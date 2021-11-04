use crate::Slot;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct PoolValue<T: 'static> {
    slot: Slot<T>,
}

impl<T> PoolValue<T> {
    pub(crate) fn new(slot: Slot<T>) -> Self {
        Self { slot }
    }

    pub fn to_value(&self) -> T {
        self.slot.to_value()
    }

    pub fn to_boxed_value(&self) -> Box<T> {
        Box::new(self.to_value())
    }
}

impl<T> Deref for PoolValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.slot.value().as_ref().unwrap()
    }
}

impl<T> DerefMut for PoolValue<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.slot.value_mut().as_mut().unwrap()
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
    T: PartialEq,
{
    fn eq(&self, other: &&T) -> bool {
        self.deref() == *other
    }
}

impl<T> Drop for PoolValue<T> {
    fn drop(&mut self) {
        self.slot.take().recycle()
    }
}
