#![feature(generic_associated_types)]

mod lru;
mod shard;
mod utils;
pub use lru::*;
pub use shard::*;

use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash},
};

#[derive(Clone)]
pub struct Options<S = RandomState> {
    pub capacity: usize,
    pub shard_bits: usize,
    pub hash_builder: S,
}

pub trait CacheHandle {
    type Value;
    fn value(&self) -> &Self::Value;
}

pub trait Cache: Send + Sync {
    // We can use GAT feature for the returned handle,
    // but it will cause this trait not object save.
    // User may want use this trait like Box<dyn Cache<K, V, RandomState>>

    type Key;
    type Value;
    type HashBuilder;

    fn with_options(opts: &Options<Self::HashBuilder>) -> Self
    where
        Self::HashBuilder: Clone,
        Self: Sized;

    fn insert(
        &self,
        key: Self::Key,
        value: Self::Value,
        charge: usize,
    ) -> Box<dyn CacheHandle<Value = Self::Value> + '_>
    where
        Self::Key: Hash + Eq,
        Self::HashBuilder: BuildHasher;

    fn lookup(&self, key: &Self::Key) -> Option<Box<dyn CacheHandle<Value = Self::Value> + '_>>
    where
        Self::Key: Hash + Eq,
        Self::HashBuilder: BuildHasher;

    fn erase(&self, key: &Self::Key)
    where
        Self::Key: Hash + Eq,
        Self::HashBuilder: BuildHasher;

    fn prune(&self)
    where
        Self::Key: Eq;
}

pub trait CacheShard: Send + Sync {
    type Key;
    type Value;
    type HashBuilder;

    fn shard_with_options(opts: &Options<Self::HashBuilder>) -> Self
    where
        Self::HashBuilder: Clone,
        Self: Sized;

    fn shard_insert(
        &self,
        key: Self::Key,
        value: Self::Value,
        hash: u64,
        charge: usize,
    ) -> Box<dyn CacheHandle<Value = Self::Value> + '_>
    where
        Self::Key: Eq;

    fn shard_lookup(
        &self,
        key: &Self::Key,
        hash: u64,
    ) -> Option<Box<dyn CacheHandle<Value = Self::Value> + '_>>
    where
        Self::Key: Eq;

    fn shard_erase(&self, key: &Self::Key, hash: u64)
    where
        Self::Key: Eq;

    fn shard_prune(&self)
    where
        Self::Key: Eq;
}

#[cfg(test)]
mod tests {}
