use crate::Slot;
use std::ops::{Deref, DerefMut};

pub struct PoolValue<T: 'static> {
    slot: Slot<T>,
}

impl<T> Clone for PoolValue<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        self.slot.pool().alloc(self.slot.value().clone().unwrap())
    }
}

impl<T> PoolValue<T> {
    pub(crate) fn new(slot: Slot<T>) -> Self {
        Self { slot }
    }

    pub fn take_value(&self) -> T {
        self.slot.take_value()
    }

    pub fn take_boxed_value(&self) -> Box<T> {
        Box::new(self.take_value())
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

impl<T> std::fmt::Debug for PoolValue<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.deref())
    }
}

impl<T> AsRef<T> for PoolValue<T> {
    fn as_ref(&self) -> &T {
        self.slot.value().as_ref().unwrap()
    }
}
