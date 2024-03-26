use crate::{Factory, InnerPool, PoolValue};

#[derive(Debug)]
pub struct Pool<T: 'static> {
    inner: *mut InnerPool<T>,
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        let inner = Box::leak(Box::new(InnerPool::new()));
        Self { inner }
    }
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self::default()
    }

    fn inner_ref(&self) -> &InnerPool<T> {
        unsafe { self.inner.as_ref().unwrap() }
    }

    pub fn alloc(&self, value: T) -> PoolValue<T> {
        self.inner_ref().alloc(value)
    }

    pub fn factory(&self) -> Factory<T> {
        Factory::new(self.inner)
    }
}

#[cfg(test)]
impl<T> Pool<T> {
    pub(crate) fn total_allocations(&self) -> usize {
        self.inner_ref().total_allocations()
    }

    pub(crate) fn size(&self) -> usize {
        self.inner_ref().size()
    }
}

impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.inner) })
    }
}
