use crate::{InnerPool, PoolValue};

#[derive(Debug)]
pub struct Factory<T: 'static> {
    inner: *mut InnerPool<T>,
}

impl<T> Factory<T> {
    pub(crate) fn new(inner: *mut InnerPool<T>) -> Self {
        Self { inner }
    }

    pub fn alloc(&self, value: T) -> PoolValue<T> {
        let inner_ref = unsafe { self.inner.as_ref().unwrap() };
        inner_ref.alloc(value)
    }
}
