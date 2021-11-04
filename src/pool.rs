use crate::{PoolValue, Slot};

#[derive(Debug)]
pub struct Pool<T: 'static> {
    slots: Vec<Slot<T>>,
    #[cfg(test)]
    allocations: usize,
}

impl<T> Default for Pool<T> {
    fn default() -> Self {
        Self {
            slots: vec![],
            #[cfg(test)]
            allocations: 0,
        }
    }
}

impl<T> Pool<T> {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(mutable_transmutes)]
    #[allow(clippy::mut_from_ref)]
    fn as_mut(&self) -> &'static mut Self {
        unsafe { std::mem::transmute(self) }
    }

    pub fn alloc(&self, value: T) -> PoolValue<T> {
        let mut slot = self.find_slot();
        *slot.value_mut() = Some(value);
        PoolValue::new(slot)
    }

    fn find_slot(&self) -> Slot<T> {
        if let Some(slot) = self.as_mut().slots.pop() {
            slot
        } else {
            #[cfg(test)]
            {
                self.as_mut().allocations += 1;
            }
            Slot::new_empty(self.as_mut())
        }
    }

    pub(crate) fn recycle(&self, slot: Slot<T>) {
        slot.clear_value();
        self.as_mut().slots.push(slot);
    }

    #[cfg(test)]
    pub(crate) fn total_allocations(&self) -> usize {
        self.allocations
    }

    #[cfg(test)]
    pub(crate) fn size(&self) -> usize {
        self.slots.len()
    }
}
