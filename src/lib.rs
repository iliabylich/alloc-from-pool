#![doc = include_str!("../README.md")]

mod factory;
mod inner_pool;
mod inner_slot;
mod pool;
mod pool_value;
mod slot;

pub use factory::Factory;
pub(crate) use inner_pool::InnerPool;
pub(crate) use inner_slot::InnerSlot;
pub use pool::Pool;
pub use pool_value::PoolValue;
pub(crate) use slot::Slot;

#[cfg(test)]
mod tests;
