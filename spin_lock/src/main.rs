use std::{cell::UnsafeCell, ops::{Deref, DerefMut}, sync::atomic::AtomicBool, thread};



pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send{}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock
        unsafe { &*self.lock.value.get()}
    }
}

impl<T> DerefMut for Guard<'_,T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}


impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, std::sync::atomic::Ordering::Release);
    }
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self { locked: AtomicBool::new(false), value: UnsafeCell::new(value) }
    }

    // pub fn lock<'a>(&'a self) -> &'a mut T {
    //     while self.locked.swap(true, std::sync::atomic::Ordering::Acquire) {
    //         std::hint::spin_loop();
    //     }

    //     unsafe { &mut *self.value.get()}
    // }

    pub fn lock(& self) -> Guard<T> {
        while self.locked.swap(true, std::sync::atomic::Ordering::Acquire) {
            std::hint::spin_loop();
        }

        Guard { lock: self }
    }

    /// Safety: The &mut T from lock() must be gone!
    /// (And no cheating by keeping to fields to fields to that T around!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, std::sync::atomic::Ordering::Release);
    }
}
fn main() {
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });
    let g = x.lock();
    println!("{:?}", g.as_slice());
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
