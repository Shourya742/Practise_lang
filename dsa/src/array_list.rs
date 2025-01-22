#![allow(dead_code)]

use std::{ops::{Index, IndexMut}, ptr};

use crate::rawvec::RawVec;


pub struct ArrayList<T> {
    buf: RawVec<T>,
    len: usize
}

impl<T> ArrayList<T> {
    pub fn new() -> Self {
        ArrayList { buf: RawVec::new(), len: 0 }
    } 

    fn ptr_at_offset(&self, offset: usize) -> *mut T {
        unsafe {
            self.buf.ptr.as_ptr().add(offset)
        }
    }

    pub fn push_front(&mut self, val: T) {
        if self.len == self.buf.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::copy(self.ptr_at_offset(0), self.ptr_at_offset(1), self.len);
            ptr::write(self.ptr_at_offset(0), val);
        }
        self.len += 1;
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.buf.cap() {
            self.buf.grow();
        }

        unsafe  {
            ptr::write(self.ptr_at_offset(self.len), val);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            unsafe  {
                Some(ptr::read(self.ptr_at_offset(self.len+1)))
            }
        } else {
            None
        }
    }

}

impl<T> Index<usize> for ArrayList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("Out of bound error");
        }

        unsafe  {
            ptr::read(self.ptr_at_offset(index) as *const &T)
        }
    }
}


impl<T> IndexMut<usize> for ArrayList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.len {
            panic!("Out of bound");
        }
        
        unsafe {
            let p = self.ptr_at_offset(index);
            &mut *p
        }
    }
}


pub struct ArrayListIter<T> {
    ptr: *mut T,
    list: ArrayList<T>
}

impl<T> ArrayListIter<T> {
    fn new(list: ArrayList<T>) -> Self {
        ArrayListIter { ptr: list.buf.ptr.as_ptr(), list }
    }
}


impl<T> IntoIterator for ArrayList<T> {
    type Item = T;
    type IntoIter = ArrayListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ArrayListIter::new(self)
    }
}

impl<T> Iterator for ArrayListIter<T> {

    type  Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.list.len > 0 {
            let n = unsafe { ptr::read(self.ptr) };
            self.list.len -= 1;
            if self.list.len > 0 {
                self.ptr = unsafe { self.ptr.offset(1) };
            }
            Some(n)
        } else {
            None
        }
    }
}

impl<T> Drop for ArrayListIter<T> {
    fn drop(&mut self) {
        while let Some(v) = self.next() {
            drop(v);
        }
    }
}

impl<T> Drop for ArrayList<T> {
    fn drop(&mut self) {
        while let Some(v) = self.pop() {
            drop(v);
        }
    }
}