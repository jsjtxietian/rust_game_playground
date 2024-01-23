use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

struct Pool<T> {
    items: RefCell<Vec<T>>,
}

impl<T: PoolItem> Pool<T> {
    pub fn new() -> Self {
        Self {
            items: RefCell::new(Vec::new()),
        }
    }

    pub fn get(&self) -> PoolGuard<T> {
        let item = match self.items.borrow_mut().pop() {
            Some(item) => item,
            None => T::new(),
        };
        PoolGuard {
            inner: Some(item),
            items: &self.items,
        }
    }
}

struct PoolGuard<'a, T: PoolItem> {
    inner: Option<T>,
    items: &'a RefCell<Vec<T>>,
}

impl<T: PoolItem> Drop for PoolGuard<'_, T> {
    fn drop(&mut self) {
        let mut item = self.inner.take().unwrap();
        item.reset();
        self.items.borrow_mut().push(item);
    }
}

impl<T: PoolItem> Deref for PoolGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner.as_ref().unwrap()
    }
}

impl<T: PoolItem> DerefMut for PoolGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.as_mut().unwrap()
    }
}

trait PoolItem {
    fn new() -> Self;
    fn reset(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        struct Awesome(usize);
        impl Awesome {
            fn get(&self) -> usize {
                self.0
            }

            fn inc(&mut self) {
                self.0 += 1;
            }
        }

        impl PoolItem for Awesome {
            fn new() -> Self {
                Awesome(0)
            }

            fn reset(&mut self) {
                self.0 = 0;
            }
        }

        let pool: Pool<Awesome> = Pool::new();
        let mut item1 = pool.get();

        assert_eq!(item1.get(), 0);
        item1.inc();
        assert_eq!(item1.get(), 1);
        drop(item1);

        let item2 = pool.get();
        assert_eq!(item2.get(), 0);
    }
}
