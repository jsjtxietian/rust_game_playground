use std::{
    alloc::{self, Layout},
    ptr::{null, NonNull},
};

struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if std::mem::size_of::<T>() == 0 {
            panic!("No zero sized types");
        }

        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Could not alloc more");
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not alloc memory");
            unsafe {
                // the memory previouls at ptr is not read
                ptr.as_ptr().write(item);
            }
            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped size");
            unsafe {
                self.ptr.as_ptr().add(self.len).write(item);
            }
            self.len = self.len + 1;
        } else {
            debug_assert!(self.len == self.capacity);

            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");
            let new_size = std::mem::size_of::<T>() * new_capacity;

            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("Can't alloc");
            self.ptr = unsafe {
                let layout = Layout::from_size_align_unchecked(size, align);
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not alloc memory");
                ptr.as_ptr().add(self.len).write(item);
                ptr
            };

            self.capacity = new_capacity;
            self.len = self.len + 1;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );
            std::alloc::dealloc(self.ptr.as_ptr() as _, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MyVec;

    #[derive(PartialEq,Debug)]
    struct Dropped(usize);

    impl Drop for Dropped {
        fn drop(&mut self) {
            println!("Dropping");
        }
    }

    #[test]
    fn it_works() {
        let mut vec = MyVec::<Dropped>::new();
        vec.push(Dropped(1));
        vec.push(Dropped(2));

        assert_eq!(vec.get(1),Some(&Dropped(2)));
    }
}
