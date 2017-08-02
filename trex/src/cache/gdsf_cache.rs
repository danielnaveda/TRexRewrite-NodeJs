use std::collections::{BTreeSet, HashMap};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash};

// TODO clean everything up (maybe using hand crafted collections)
// TODO maybe compute the cost as the SQL statement teoretical cost

pub trait HasSize {
    fn size(&self) -> usize;
}

pub trait HasCost {
    fn cost(&self) -> usize;
}

struct StorageEntry<K, V>
    where V: HasSize + HasCost
{
    key: *const K,
    value: V,
    frequency: usize,
    clock: usize,
}

impl<K, V> StorageEntry<K, V>
    where V: HasSize + HasCost
{
    fn new(key: *const K, value: V, clock: usize) -> Self {
        StorageEntry {
            key: key,
            value: value,
            frequency: 1,
            clock: clock,
        }
    }
    fn priority(&self) -> usize {
        self.clock + self.frequency * self.value.cost() / (self.value.size() + 1)
    }
    fn update(&mut self, clock: usize) -> usize {
        self.clock = clock;
        self.frequency += 1;
        self.priority()
    }
}

// Greedy Dual Size Frequency cache
pub struct GDSFCache<K, V, S = RandomState>
    where K: Eq + Hash,
          V: HasSize + HasCost,
          S: BuildHasher
{
    capacity: usize,
    used: usize,
    storage: HashMap<Box<K>, Box<StorageEntry<K, V>>, S>,
    queue: BTreeSet<(usize, *mut StorageEntry<K, V>)>,
    clock: usize,
}

unsafe impl<K, V, S> Send for GDSFCache<K, V, S>
    where K: Eq + Hash + Send,
          V: HasSize + HasCost + Send,
          S: BuildHasher + Send
{
}

impl<K, V, S> GDSFCache<K, V, S>
    where K: Eq + Hash,
          V: HasSize + HasCost,
          S: BuildHasher + Default
{
    pub fn new(capacity: usize) -> Self {
        GDSFCache {
            capacity: capacity,
            used: 0,
            storage: HashMap::default(),
            queue: BTreeSet::default(),
            clock: 0,
        }
    }
}

impl<K, V, S> GDSFCache<K, V, S>
    where K: Eq + Hash,
          V: HasSize + HasCost,
          S: BuildHasher
{
    pub fn contains_key(&self, key: &K) -> bool { self.storage.contains_key(key) }
    pub fn insert(&mut self, key: K, value: V) -> Result<&V, V> {
        self.remove(&key);

        let size = value.size();
        let key = Box::new(key);
        let key_ptr = key.as_ref() as *const _;
        let mut entry = Box::new(StorageEntry::new(key_ptr, value, self.clock));
        let entry_ptr = entry.as_mut() as *mut _;
        let priority = entry.priority();

        let free = self.capacity - self.used;
        if size > free {
            let excess = size - free;
            let pos = self.queue
                .iter()
                .take_while(|it| it.0 <= priority)
                .scan(0, |acc, it| {
                    *acc += unsafe { (*it.1).value.size() };
                    Some(*acc)
                })
                .position(|it| it >= excess);
            if let Some(num) = pos {
                for _ in 0..(num + 1) {
                    let &(pri, entry_ptr) = self.queue.iter().next().unwrap();
                    // TODO get max priority from previous scan?
                    self.clock = pri;
                    unsafe {
                        self.remove(&*(*entry_ptr).key);
                    };
                }
            } else {
                self.clock = priority;
            }
        };

        if self.used + size <= self.capacity {
            self.storage.insert(key, entry);
            self.queue.insert((priority, entry_ptr));
            self.used += size;
            unsafe { Ok(&(*entry_ptr).value) }
        } else {
            Err(entry.value)
        }
    }
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let queue = &mut self.queue;
        let clock = self.clock;
        self.storage.get_mut(key).map(|entry| {
            let entry_ptr = entry.as_mut() as *mut _;
            queue.remove(&(entry.priority(), entry_ptr));
            queue.insert((entry.update(clock), entry_ptr));
            &entry.value
        })
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.storage.remove(key).map(|mut entry| {
            self.used -= entry.value.size();
            let entry_ptr = entry.as_mut() as *mut _;
            self.queue.remove(&(entry.priority(), entry_ptr));
            entry.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{GDSFCache, HasCost, HasSize, StorageEntry};

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    struct TestStruct {
        id: usize,
        cost: usize,
        size: usize,
    }

    impl HasCost for TestStruct {
        fn cost(&self) -> usize { self.cost }
    }

    impl HasSize for TestStruct {
        fn size(&self) -> usize { self.size }
    }

    #[test]
    fn simple_priority() {
        let key = Box::new(1);
        let value = TestStruct {
            id: 1,
            cost: 1,
            size: 1,
        };
        let mut entry = StorageEntry::new(key.as_ref(), value, 0);
        // The priority is simply: cost * frequency / size + clock
        assert_eq!(entry.priority(), 1);
        // Updates the clock and increases the frequency
        assert_eq!(entry.update(1), 3);
        // The value returned by update is the new priority
        assert_eq!(entry.priority(), 3);
    }

    #[test]
    fn complex_priority() {
        let key = Box::new(1);
        let value = TestStruct {
            id: 1,
            cost: 13000,
            size: 150,
        };
        let mut entry = StorageEntry::new(key.as_ref(), value, 0);
        // priority calculation uses integer division (so floor rounding)
        assert_eq!(entry.priority(), 86);
        // update both clock and frequency
        assert_eq!(entry.update(256), 429);
        // same clock, update only frequency
        assert_eq!(entry.update(256), 516);
    }

    #[test]
    fn successful_insertion() {
        let mut cache = GDSFCache::<usize, TestStruct>::new(100);
        let mut value = TestStruct {
            id: 1,
            cost: 1,
            size: 100,
        };
        {
            let result = cache.insert(1, value.clone());
            assert_eq!(result, Ok(&value));
        }
        {
            let result = cache.get(&1);
            assert_eq!(result, Some(&value));
        }
    }

    #[test]
    fn unsuccessful_insertion() {
        let mut cache = GDSFCache::<usize, TestStruct>::new(100);
        let mut value = TestStruct {
            id: 1,
            cost: 1,
            size: 101,
        };
        {
            let result = cache.insert(1, value.clone());
            assert_eq!(result, Err(value));
        }
        {
            let result = cache.get(&1);
            assert_eq!(result, None);
        }
    }

    #[test]
    fn single_eviction() {
        let mut cache = GDSFCache::<usize, TestStruct>::new(3);
        for i in 1..4 {
            cache.insert(i,
                         TestStruct {
                             id: i,
                             cost: i,
                             size: 1,
                         });
        }
        assert!(cache.queue.iter().map(|&(pri, _)| pri).eq((1..4)));
        cache.insert(4,
                     TestStruct {
                         id: 4,
                         cost: 4,
                         size: 1,
                     });
        assert!(cache.queue.iter().map(|&(pri, _)| pri).eq((2..5)));
    }

    #[test]
    fn multiple_eviction() {
        let mut cache = GDSFCache::<usize, TestStruct>::new(3);
        for i in 1..4 {
            cache.insert(i,
                         TestStruct {
                             id: i,
                             cost: i,
                             size: 1,
                         });
        }
        assert!(cache.queue.iter().map(|&(pri, _)| pri).eq((1..4)));
        cache.insert(4,
                     TestStruct {
                         id: 4,
                         cost: 8,
                         size: 2,
                     });
        assert!(cache.queue.iter().map(|&(pri, _)| pri).eq((3..5)));
    }
}
