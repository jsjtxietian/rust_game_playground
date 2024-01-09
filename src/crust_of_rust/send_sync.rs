// Send and sync are marker & auto traits

// Rc and mutex guards(thread local state) are not send
// A type T is Sync if and only if &T is Send

// - Send + Sync: applies to "most" types, no restrictions.
// - !Send + Sync: cross-thread transfer of ownership is not fine, but transfer of a &-reference is fine.
//   Other threads can access the value but are not allowed to drop it, nor &mut-modify it.
//   Prototypical example: lock guards (like MutexGuard<T>), "per-thread" allocation via thread_local/TLS (?).
// - !Send + !Sync: neither owned nor &-reference transfer are sound. This is because there are methods accessible via &-references that break some invariant if called from a different thread. 
//   Prototypical example: Rc<T>, it is !Send because of its non-atomic reference counter (can cause a double-free (UB!) or leak (not so serious)), !Sync because clone() also manipulates said counter. Arc is Send + Sync by using atomics (performance tradeoff).
//   Raw pointers are purposefully !Send + !Sync to "contaminate" any enclosing types, just as an extra security measure.
// - Send + !Sync: weirdest case, this applies when we want to have all references in the same thread as the owning one, but as the same time we want to allow cross-thread ownership transfer. This is the case of interior mutability wrapper types.
//   Prototypical example: {Ref,Unsafe}Cell.

struct MutexGuard<'a, T> {
    i: &'a mut T,
    _not_send : std::marker::PhantomData<std::rc::Rc<()>>
}

struct Rc<T> {
    inner: *mut Inner<T>,
}

struct Inner<T> {
    count: usize,
    value: T,
}

impl<T> Rc<T> {
    fn new(v: T) -> Self {
        Rc {
            inner: Box::into_raw(Box::new(Inner { count: 1, value: v })),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe { &mut *self.inner }.count += 1;
        Rc { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let cnt = &mut unsafe { &mut *self.inner }.count;
        if *cnt == 1 {
            let _ = unsafe { Box::from_raw(self.inner) };
        } else {
            *cnt -= 1;
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}

fn main() {
    let x = Rc::new(1);
    let y = x.clone();

    std::thread::spawn(move || {
        drop(y);
    });
    drop(x);
}
