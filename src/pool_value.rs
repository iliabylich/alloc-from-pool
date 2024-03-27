use std::cell::UnsafeCell;
use std::ops::Deref;

use crate::{alloc_in::alloc_in, allocations_ptr::allocations_ptr};

pub struct PoolValue<T: 'static> {
    pub(crate) ptr: *mut T,
    pub(crate) slots: *const UnsafeCell<Vec<Box<T>>>,
    #[cfg(test)]
    pub(crate) allocations: *mut usize,
}

impl<T> PoolValue<T> {
    pub fn take_value(&mut self) -> T {
        let value = *unsafe { Box::from_raw(self.ptr) };
        self.ptr = std::ptr::null_mut();
        value
    }
}

impl<T> Deref for PoolValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref().unwrap_unchecked() }
    }
}

impl<T> Clone for PoolValue<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        alloc_in(self.slots, allocations_ptr!(self), self.deref().clone())
    }
}

impl<T> PartialEq for PoolValue<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T> PartialEq<T> for PoolValue<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.deref() == other
    }
}

impl<T> PartialEq<&T> for PoolValue<T>
where
    T: PartialEq + std::fmt::Debug,
{
    fn eq(&self, other: &&T) -> bool {
        self.deref() == *other
    }
}

impl<T> Drop for PoolValue<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            let ptr = self.ptr;
            self.ptr = std::ptr::null_mut();
            unsafe {
                UnsafeCell::raw_get(self.slots)
                    .as_mut()
                    .unwrap_unchecked()
                    .push(Box::from_raw(ptr))
            }
        }
    }
}

impl<T> std::fmt::Debug for PoolValue<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}
