use crate::{InnerPool, InnerSlot};

#[derive(Debug)]
pub(crate) struct Slot<T: 'static> {
    inner: *mut InnerSlot<T>,
}

impl<T> Slot<T> {
    pub(crate) fn new_empty(pool: &'static InnerPool<T>) -> Self {
        Self {
            inner: InnerSlot::new_empty(pool),
        }
    }

    fn inner_ref(&self) -> &'static mut InnerSlot<T> {
        unsafe { self.inner.as_mut().unwrap() }
    }

    pub(crate) fn pool(&self) -> &'static InnerPool<T> {
        self.inner_ref().pool()
    }

    pub(crate) fn take(&mut self) -> Self {
        let slot = Self { inner: self.inner };
        self.inner = std::ptr::null_mut();
        slot
    }

    pub(crate) fn recycle(self) {
        self.pool().recycle(self)
    }

    pub(crate) fn clear_value(&self) {
        self.inner_ref().clear_value()
    }

    pub(crate) fn value(&self) -> &Option<T> {
        self.inner_ref().value()
    }

    pub(crate) fn value_mut(&mut self) -> &mut Option<T> {
        self.inner_ref().value_mut()
    }

    pub(crate) fn take_value(&self) -> T {
        self.inner_ref().take_value()
    }
}

impl<T> Drop for Slot<T> {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            drop(unsafe { Box::from_raw(self.inner) })
        }
    }
}
