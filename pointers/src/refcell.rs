use std::cell::UnsafeCell;

use crate::cell::Cell;

#[derive(Clone, Copy)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell::<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared)
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: no exlusive references have been given out since the state would be Exclusive
                Some(Ref{refcell: self})
               },
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n+1));
                // SAFETY: no exlusive references have been given out since the state would be Exclusive
                Some(Ref{refcell: self})
                },
            RefState::Exclusive => None
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_,T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            // SAFETY: no other references have been given out since state would be shared or Exclusive
            Some(RefMut{refcell: self})
        } else {
            None
        }
    }
}


pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: a Ref is only created if no exlusive references have been given out.
        // once it is give out, state is set to Shared, so no exclusive references are given out.
        // so dereferencing into a shared reference is fine.
        unsafe {
            &*self.refcell.value.get()
        }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            },
            RefState::Shared(n) =>{
                self.refcell.state.set(RefState::Shared(n-1));
            }
        }
    }
}
pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
    
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY
        // a RefMut is only created if no other references have been given out.
        // once it is given out, state is set to Exclusive, so no future references are given out.
        // so we have an exlusive lease on the inner value, so mutably dereferencing is fine.
        unsafe {&mut *self.refcell.value.get()}
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}