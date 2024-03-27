use std::cell::UnsafeCell;

use crate::{alloc_in::alloc_in, allocations_ptr::allocations_ptr, Factory, PoolValue};

pub struct Pool<T: 'static> {
    slots: *const UnsafeCell<Vec<Box<T>>>,
    #[cfg(test)]
    allocations: *mut usize,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc(&self, value: T) -> PoolValue<T> {
        alloc_in(self.slots, allocations_ptr!(self), value)
    }

    pub fn factory(&self) -> Factory<T> {
        Factory {
            slots: self.slots,
            #[cfg(test)]
            allocations: self.allocations,
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            UnsafeCell::raw_get(self.slots)
                .as_ref()
                .unwrap_unchecked()
                .len()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self {
            slots: Box::leak(Box::new(UnsafeCell::new(vec![]))),
            #[cfg(test)]
            allocations: Box::leak(Box::new(0)),
        }
    }
}

#[cfg(test)]
impl<T> Pool<T> {
    pub(crate) fn total_allocations(&self) -> usize {
        unsafe { *self.allocations }
    }
}

impl<T> std::fmt::Debug for Pool<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pool",)
    }
}

impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.slots as *mut UnsafeCell<Vec<Box<T>>>));
            #[cfg(test)]
            drop(Box::from_raw(self.allocations));
        }
    }
}
