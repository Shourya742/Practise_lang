use std::{cell::UnsafeCell, sync::atomic::{compiler_fence, AtomicUsize}};


#[repr(align(64))]
pub struct Seqlock<T> {
    version: AtomicUsize,
    _pad: [u8; 56],
    data: UnsafeCell<T>
}

impl<T: Default> Default for Seqlock<T> {
    fn default() -> Self {
        Self { version: AtomicUsize::new(0), _pad: [0;56], data: UnsafeCell::default() }
    }
}

unsafe impl<T: Send>  Send for Seqlock<T> {}
unsafe impl<T: Sync> Sync for Seqlock<T>  {}

impl<T: Copy> Seqlock<T> {
    pub fn new(data: T) -> Self {
        Self { version: AtomicUsize::new(0), _pad: [0;56], data: UnsafeCell::new(data) }
    }

    #[inline(never)]
    pub fn read(&self, result: &mut T) {
        loop {
            let v1 = self.version.load(std::sync::atomic::Ordering::Acquire);
            compiler_fence(std::sync::atomic::Ordering::AcqRel);
            *result = unsafe {
                *self.data.get()
            };
            compiler_fence(std::sync::atomic::Ordering::AcqRel);
            let v2 = self.version.load(std::sync::atomic::Ordering::Acquire);
            if v1 == v2 && v1 & 1 == 0 {
                return;
            }
        }
    }

    #[inline(never)]
    pub fn pessimistic_read(&self, result: &mut T) {
        loop {
            let v1  = self.version.load(std::sync::atomic::Ordering::Acquire);
            if v1&1 == 1 {
                continue;
            }
            compiler_fence(std::sync::atomic::Ordering::AcqRel);
            *result = unsafe {
                *self.data.get()
            };
            compiler_fence(std::sync::atomic::Ordering::AcqRel);
            let v2 = self.version.load(std::sync::atomic::Ordering::Acquire);
            if v1 == v2 {
                return;
            }

        }
    }

    #[inline(never)]
    pub fn write_old(&self, val: &T) {
        let v = self.version.load(std::sync::atomic::Ordering::Relaxed);
        self.version.store(v, std::sync::atomic::Ordering::Relaxed);
        unsafe { *self.data.get() = *val};
        self.version.store(v.wrapping_add(1), std::sync::atomic::Ordering::Relaxed);
    }

    pub fn write(&self, val: &T) {
        let v = self.version.fetch_add(1, std::sync::atomic::Ordering::Release);
        compiler_fence(std::sync::atomic::Ordering::AcqRel);
        unsafe {*self.data.get() = *val};
        compiler_fence(std::sync::atomic::Ordering::AcqRel);
        self.version.store(v.wrapping_add(2), std::sync::atomic::Ordering::Release);
    }
}


fn main() {
    println!("Hello, world!");
}
