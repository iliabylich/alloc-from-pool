use std::cell::UnsafeCell;

use crate::{alloc_in::alloc_in, allocations_ptr::allocations_ptr, PoolValue};

#[derive(Debug)]
pub struct Factory<T> {
    pub(crate) slots: *const UnsafeCell<Vec<Box<T>>>,
    #[cfg(test)]
    pub(crate) allocations: *mut usize,
}

impl<T> Factory<T> {
    pub fn alloc(&self, value: T) -> PoolValue<T> {
        alloc_in(self.slots, allocations_ptr!(self), value)
    }
}

impl<T> Default for Factory<T> {
    fn default() -> Self {
        Self {
            slots: std::ptr::null(),
            #[cfg(test)]
            allocations: std::ptr::null_mut(),
        }
    }
}
