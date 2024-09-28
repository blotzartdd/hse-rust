#![forbid(unsafe_code)]

use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    // TODO: your code goes here.
}

impl<K, V> LRUCache<K, V>
where
    K: Clone + Hash + Ord,
{
    pub fn new(capacity: usize) -> Self {
        // TODO: your code goes here.
    }

    pub fn len(&self) -> usize {
        // TODO: your code goes here.
    }

    pub fn is_empty(&self) -> bool {
        // TODO: your code goes here.
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        // TODO: your code goes here.
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        // TODO: your code goes here.
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // TODO: your code goes here.
    }

    pub fn clear(&mut self) {
        // TODO: your code goes here.
    }
}
