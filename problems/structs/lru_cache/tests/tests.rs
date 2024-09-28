use lru_cache::LRUCache;
use ntest::timeout;
use rand::Rng;

struct NaiveLRUCache<K, V> {
    capacity: usize,
    cache: Vec<(K, V)>,
}

impl<K: Eq, V> NaiveLRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            cache: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = self.cache.iter().position(|(a, _)| a == key)?;
        let pair = self.cache.remove(index);
        self.cache.push(pair);
        Some(&self.cache.last().unwrap().1)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.cache.iter().position(|(a, _)| a == key)?;
        let pair = self.cache.remove(index);
        self.cache.push(pair);
        Some(&mut self.cache.last_mut().unwrap().1)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(index) = self.cache.iter().position(|(a, _)| a == &key) {
            let (_, old_value) = self.cache.remove(index);
            self.cache.push((key, value));
            Some(old_value)
        } else {
            if self.cache.len() == self.capacity {
                self.cache.remove(0);
            }
            self.cache.push((key, value));
            None
        }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

#[test]
#[should_panic]
fn check_zero_capacity() {
    LRUCache::<i32, i32>::new(0);
}

#[test]
fn should_compile() {
    #[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct Key {
        _key: i32,
    }
    struct Value {
        _value: i32,
    }
    LRUCache::<Key, Value>::new(1);
}

#[test]
fn it_works1() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
    assert_eq!(cache.insert(1, 1), None);
    assert_eq!(cache.len(), 1);
    assert!(!cache.is_empty());
    assert_eq!(cache.insert(2, 2), None);
    assert_eq!(cache.len(), 2);
    assert!(!cache.is_empty());
    assert_eq!(cache.get(&1), Some(&1));
    assert_eq!(cache.insert(3, 3), None);
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.insert(4, 4), None);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&3), Some(&3));
    assert_eq!(cache.get(&4), Some(&4));
    assert_eq!(cache.len(), 2);
    cache.clear();
    assert_eq!(cache.len(), 0);
    assert!(cache.is_empty());
}

#[test]
fn it_works2() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.insert(2, 6), None);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.insert(1, 5), None);
    assert_eq!(cache.insert(1, 2), Some(5));
    assert_eq!(cache.get(&1), Some(&2));
    assert_eq!(cache.get(&2), Some(&6));
}

#[test]
fn it_works3() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.insert(2, 1), None);
    assert_eq!(cache.insert(2, 2), Some(1));
    assert_eq!(cache.get(&2), Some(&2));
    assert_eq!(cache.insert(1, 1), None);
    assert_eq!(cache.insert(4, 1), None);
    assert_eq!(cache.get(&2), None);
}

#[test]
fn it_works4() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.insert(2, 1), None);
    assert_eq!(cache.insert(1, 1), None);
    assert_eq!(cache.get(&2), Some(&1));
    assert_eq!(cache.insert(4, 1), None);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&1));
}

#[test]
fn it_works5() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.insert(1, 1), None);
    assert_eq!(cache.insert(2, 2), None);
    assert_eq!(cache.get(&2), Some(&2));
    *cache.get_mut(&2).unwrap() = -2;
    assert_eq!(cache.get(&1), Some(&1));
    assert_eq!(cache.get(&2), Some(&-2));
}

#[test]
fn it_works6() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.insert(2, 2), None);
    assert_eq!(cache.insert(1, 1), None);
    *cache.get_mut(&2).unwrap() = -2;
    assert_eq!(cache.insert(3, 3), None);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&-2));
    assert_eq!(cache.get(&3), Some(&3));
}

#[test]
fn size_one() {
    let mut cache = LRUCache::new(1);
    assert_eq!(cache.insert(1, 100), None);
    assert_eq!(cache.insert(1, -100), Some(100));
    assert_eq!(cache.insert(2, 200), None);
    assert_eq!(cache.len(), 1);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&200));
    *cache.get_mut(&2).unwrap() = -200;
    assert_eq!(cache.get(&2), Some(&-200));
    assert_eq!(cache.len(), 1);
}

#[test]
fn small_capacity() {
    let mut cache = LRUCache::new(10);
    let mut naive = NaiveLRUCache::new(10);
    let mut rng = rand::thread_rng();
    for _ in 0..500000 {
        match rng.gen_range(0..3000) {
            0 => {
                cache.clear();
                naive.clear();
            }
            1..=999 => {
                let key = rng.gen_range(0..30);
                assert_eq!(cache.get(&key), naive.get(&key));
                assert_eq!(cache.len(), naive.len());
            }
            1000..=1999 => {
                let key = rng.gen_range(0..30);
                if let Some(naive_value) = naive.get_mut(&key) {
                    let new_value = rng.gen::<i8>();
                    let value = cache.get_mut(&key).expect(&format!(
                        "No value found by key {key}, but expected {}",
                        *naive_value
                    ));
                    *naive_value = new_value;
                    *value = new_value;
                } else {
                    assert_eq!(cache.get_mut(&key), None);
                }
                assert_eq!(cache.len(), naive.len());
            }
            _ => {
                let key = rng.gen_range(0..30);
                let value = rng.gen::<i8>();
                assert_eq!(cache.insert(key, value), naive.insert(key, value));
                assert_eq!(cache.len(), naive.len());
            }
        }
    }
}

#[test]
fn big_capacity() {
    let mut cache = LRUCache::new(1000);
    let mut naive = NaiveLRUCache::new(1000);
    let mut rng = rand::thread_rng();
    for _ in 0..500000 {
        match rng.gen_range(0..3000) {
            0 => {
                naive.clear();
                cache.clear();
                assert!(cache.is_empty());
                assert_eq!(cache.len(), 0);
            }
            1..=999 => {
                let key = rng.gen_range(0..3000);
                assert_eq!(cache.get(&key), naive.get(&key));
                assert_eq!(cache.len(), naive.len());
            }
            1000..=1999 => {
                let key = rng.gen_range(0..3000);
                assert_eq!(cache.get_mut(&key), naive.get_mut(&key));
                assert_eq!(cache.len(), naive.len());
            }
            _ => {
                let key = rng.gen_range(0..3000);
                let value = rng.gen::<i8>();
                assert_eq!(cache.insert(key, value), naive.insert(key, value));
                assert_eq!(cache.len(), naive.len());
            }
        }
    }
}

#[test]
#[timeout(4000)]
fn stress() {
    let mut cache = LRUCache::new(100000);
    let mut rng = rand::thread_rng();
    for _ in 0..500000 {
        cache.len();
        cache.is_empty();
        match rng.gen_range(0..5) {
            0 => {
                let key = rng.gen_range(0..1000000);
                cache.get(&key);
            }
            1 => {
                let key = rng.gen_range(0..1000000);
                cache.get_mut(&key);
            }
            _ => {
                let key = rng.gen_range(0..1000000);
                let value = rng.gen::<i8>();
                cache.insert(key, value);
            }
        }
    }
}
