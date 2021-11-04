use crate::{Factory, InnerPool, PoolValue};

#[derive(Debug)]
pub struct Pool<T: 'static> {
    inner: *mut InnerPool<T>,
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        let inner = Box::leak(Box::new(InnerPool::new()));
        Self { inner }
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

    #[cfg(test)]
    pub(crate) fn total_allocations(&self) -> usize {
        self.inner_ref().total_allocations()
    }

    #[cfg(test)]
    pub(crate) fn size(&self) -> usize {
        self.inner_ref().size()
    }
}

impl<T> Drop for Pool<T> {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.inner) })
    }
}
