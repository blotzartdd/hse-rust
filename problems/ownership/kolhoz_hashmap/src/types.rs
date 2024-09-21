#![forbid(unsafe_code)]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(Debug, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(PartialOrd, Ord))]
pub struct Key(Vec<u8>);

impl Key {
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    /* Used in tests only */
    #[cfg(test)]
    pub fn new(value: &[u8]) -> Self {
        Self(Vec::from(value))
    }

    /* Used in tests only */
    #[cfg(test)]
    pub fn get(&self) -> &Vec<u8> {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(PartialOrd, Ord))]
pub struct Data(Vec<u8>);

impl Data {
    /* Used in tests only */
    #[cfg(test)]
    pub fn new(value: &[u8]) -> Self {
        Self(Vec::from(value))
    }

    /* Used in tests only */
    #[cfg(test)]
    pub fn get(&self) -> &Vec<u8> {
        &self.0
    }
}
