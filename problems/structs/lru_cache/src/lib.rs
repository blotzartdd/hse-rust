#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
pub struct LRUCache<K, V> {
    capacity: usize,
    cache: HashMap<K, V>,
    next: HashMap<K, K>,
    prev: HashMap<K, K>,
    head: Option<K>,
    tail: Option<K>,
}

impl<K, V> LRUCache<K, V>
where
    K: Clone + Hash + Ord,
{
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            cache: HashMap::with_capacity(capacity),
            next: HashMap::new(),
            prev: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    fn update(&mut self, key: &K) {
        if self.head.clone().unwrap() == *key {
        } else if self.tail.clone().unwrap() == *key {
            let prev_key = self.prev.remove(&key.clone()).unwrap();
            self.next.remove(&prev_key);
            self.tail = Some(prev_key);

            self.next.insert(key.clone(), self.head.clone().unwrap());
            self.prev.insert(self.head.clone().unwrap(), key.clone());
            self.head = Some(key.clone());
        } else {
            let prev_key = self.prev.remove(&key.clone()).unwrap();
            let next_key = self.next.remove(&key.clone()).unwrap();

            self.next.remove(&prev_key.clone());
            self.prev.remove(&next_key.clone());

            self.next.insert(prev_key.clone(), next_key.clone());
            self.prev.insert(next_key.clone(), prev_key.clone());

            self.next.insert(key.clone(), self.head.clone().unwrap());
            self.prev.insert(self.head.clone().unwrap(), key.clone());
            self.head = Some(key.clone());
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.cache.contains_key(key) {
            self.update(key);
        }

        self.cache.get(&key.clone())
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.cache.contains_key(key) {
            self.update(key);
        }

        self.cache.get_mut(&key.clone())
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.head.is_none() && self.tail.is_none() {
            self.head = Some(key.clone());
            self.tail = Some(key.clone());
            self.cache.insert(key, value);
            return None;
        }

        if self.cache.contains_key(&key) {
            let old_value = self.cache.remove(&key.clone());
            self.update(&key);

            self.cache.insert(key, value);
            old_value
        } else {
            if self.len() == self.capacity && self.capacity == 1 {
                let old_value = self.cache.remove(&self.head.clone().unwrap());
                let prev = self.head.clone().unwrap();

                self.head = Some(key.clone());
                self.tail = Some(key.clone());
                self.cache.insert(key.clone(), value);

                if prev != key {
                    return None;
                }

                return old_value;
            }

            if self.len() == self.capacity {
                let prev_key = self.prev.remove(&self.tail.clone().unwrap()).unwrap();
                self.next.remove(&prev_key.clone());
                self.cache.remove(&self.tail.clone().unwrap());

                self.tail = Some(prev_key);
            }

            self.next.insert(key.clone(), self.head.clone().unwrap());
            self.prev.insert(self.head.clone().unwrap(), key.clone());
            self.head = Some(key.clone());
            self.cache.insert(key, value);

            None
        }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.next.clear();
        self.prev.clear();
        self.head = None;
        self.tail = None;
    }
}
