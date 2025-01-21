#![allow(dead_code)]
use std::{alloc::{alloc, realloc, Layout}, ptr::NonNull};



pub struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize
}


impl<T> RawVec<T> {
    pub fn new() -> Self {
        RawVec { ptr: NonNull::dangling(), cap: 0 }
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut rv = Self::new();
        rv.grow_to_cap(cap, None);
        rv
    }

    fn grow_to_cap(&mut self, new_cap: usize, old_layout: Option<Layout>) {

        let layout = Layout::array::<T>(new_cap).unwrap();

        if let Some(old_layout) = old_layout {
            unsafe {
                let ptr = realloc(self.ptr.as_ptr() as *mut u8, old_layout, layout.size());
                self.ptr = NonNull::new(ptr as *mut T).unwrap();
            }
        } else {
            unsafe {
                let ptr = alloc(layout);
                self.ptr = NonNull::new(ptr as *mut T).unwrap();
            }
        }
        self.cap = new_cap;
    }

    fn grow(&mut self) {
        let (new_cap, old_layout) = if self.cap == 0 {
            (4, None)
        } else {
            let layout = Layout::array::<T>(self.cap).unwrap();
            (self.cap * 2, Some(layout))
        };
        self.grow_to_cap(new_cap, old_layout);
    }
}