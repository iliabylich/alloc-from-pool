use crate::InnerPool;

#[derive(Debug)]
pub(crate) struct InnerSlot<T: 'static> {
    value: Option<T>,
    pool: &'static InnerPool<T>,
}

impl<T> InnerSlot<T> {
    pub(crate) fn new_empty(pool: &'static InnerPool<T>) -> *mut Self {
        Box::leak(Box::new(Self { value: None, pool }))
    }

    pub(crate) fn pool(&self) -> &'static InnerPool<T> {
        self.pool
    }

    pub(crate) fn clear_value(&mut self) {
        self.value = None;
    }

    pub(crate) fn take_value(&mut self) -> T {
        self.value.take().unwrap()
    }

    pub(crate) fn value(&self) -> &Option<T> {
        &self.value
    }

    pub(crate) fn value_mut(&mut self) -> &mut Option<T> {
        &mut self.value
    }
}
