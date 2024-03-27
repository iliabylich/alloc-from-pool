use std::cell::UnsafeCell;

use crate::PoolValue;

pub(crate) fn alloc_in<T>(
    slots: *const UnsafeCell<Vec<Box<T>>>,
    _allocations: *mut usize,
    value: T,
) -> PoolValue<T> {
    let slots_ref = unsafe { UnsafeCell::raw_get(slots).as_mut().unwrap_unchecked() };
    let boxed = if let Some(mut boxed) = slots_ref.pop() {
        *boxed = value;
        boxed
    } else {
        #[cfg(test)]
        unsafe {
            *_allocations += 1
        }
        Box::new(value)
    };
    let ptr = Box::leak(boxed);
    PoolValue {
        ptr,
        slots,
        #[cfg(test)]
        allocations: _allocations,
    }
}
