// Hash Table: Design and implement a hash table which uses chaining (linked lists) to handle
// collisions

use std::collections::hash_map::DefaultHasher;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};



#[derive(Debug)]
struct HashMap<K, V>
where
    K: Hash + PartialEq + Clone + Copy,
    V: Clone + Copy,
{
    table: Vec<LinkedList<Entry<K, V>>>,
}

#[derive(Debug, Clone, Copy)]
struct Entry<K, V>
where
    K: Hash + PartialEq + Clone + Copy,
    V: Clone + Copy,
{
    key: K,
    value: V,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + PartialEq + Clone + Copy,
    V: Clone + Copy,
{
    fn new(size: usize) -> Self {
        let mut table = Vec::with_capacity(size);
        table.resize_with(size, || LinkedList::new());

        HashMap { table }
    }

    fn put(&mut self, key: K, value: V) {
        let bucket = self.get_bucket(&key);
        let list = &mut self.table[bucket];
        for e in list.iter_mut() {
            if e.key == key {
                e.value = value;
                return;
            }
        }
        list.push_back(Entry { key, value })
    }

    fn get(&self, key: K) -> Option<&V> {
        let bucket = self.get_bucket(&key);
        self.table[bucket]
            .iter()
            .find(|e| e.key == key)
            .map(|e| &e.value)
    }

    fn remove(&mut self, key: K) {
        let bucket = self.get_bucket(&key);
        let list = &mut self.table[bucket];

        // stable rust doesn't have remove fn for LL, my LL impl for part 2 doesn't
        // have a mut iter so this is... well.. it's something
        let f = list
            .iter()
            .filter(|e| e.key != key)
            .map(|e| *e)
            .collect::<LinkedList<Entry<K, V>>>();

        list.clear();

        f.iter().for_each(|e| list.push_back(*e))
    }

    fn get_bucket(&self, key: &K) -> usize {
        self.calculate_hash(key) % self.table.len()
    }

    fn calculate_hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_put_and_get_from_map() {
        let mut h_map = HashMap::new(5);
        for i in 0..20 {
            h_map.put(i, i);
        }

        for i in 0..20 {
            assert_eq!(i, *h_map.get(i).unwrap());
        }
    }

    #[test]
    fn should_put_remove_and_get_from_map() {
        let mut h_map = HashMap::new(5);
        for i in 0..20 {
            h_map.put(i, i);
        }

        for i in 1..10 {
            h_map.remove(i)
        }

        assert_eq!(5, h_map.table.len());
        for i in 10..20 {
            assert_eq!(i, *h_map.get(i).unwrap());
        }
    }
}
