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

    #[allow(mutable_transmutes)]
    #[allow(clippy::mut_from_ref)]
    fn as_mut(&self) -> &mut Self {
        unsafe { std::mem::transmute(self) }
    }

    pub(crate) fn pool(&self) -> &'static InnerPool<T> {
        self.pool
    }

    pub(crate) fn clear_value(&self) {
        self.as_mut().value = None;
    }

    pub(crate) fn to_value(&self) -> T {
        self.as_mut().value.take().unwrap()
    }

    pub(crate) fn value(&self) -> &Option<T> {
        &self.value
    }

    pub(crate) fn value_mut(&mut self) -> &mut Option<T> {
        &mut self.value
    }
}
