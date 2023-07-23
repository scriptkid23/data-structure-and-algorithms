// Collision: Chaining

use std::{collections::VecDeque, hash::Hasher};

#[derive(Clone, Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}
#[derive(Debug)]
struct HashMap<K: Clone, V: Clone> {
    size: usize,
    bins: Vec<VecDeque<Entry<K, V>>>,
}
trait HashTable<K, V> {
    fn new(size: usize) -> Self;
    fn hash(&self, key: &K) -> usize;
    fn insert(&mut self, key: K, value: V);
    fn remove(&mut self, key: K);
    fn lookup(&self, key: &K) -> Option<Entry<K, V>>;
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> HashTable<K, V> for HashMap<K, V> {
    fn new(size: usize) -> Self {
        let size = size;

        let mut bins: Vec<VecDeque<Entry<K, V>>> = Vec::with_capacity(size);

        for _ in 0..size {
            bins.push(VecDeque::new());
        }

        return HashMap {
            size: size,
            bins: bins,
        };
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        key.hash(&mut hasher);

        return hasher.finish() as usize % self.size;
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        if let Some(bin) = self.bins.get_mut(index) {
            bin.push_back(Entry {
                key: key,
                value: value,
            })
        }
    }
    fn remove(&mut self, key: K) {
        let hash_key = self.hash(&key);
        if let Some(bin) = self.bins.get_mut(hash_key) {
            if bin.is_empty() {
                return;
            }

            for (index, entry) in bin.iter().enumerate() {
                if entry.key == key {
                    bin.remove(index);
                    return;
                }
            }
        }
    }

    fn lookup(&self, key: &K) -> Option<Entry<K, V>> {
        let hash_key = self.hash(key);
        if let Some(bin) = self.bins.get(hash_key) {
            for i in bin {
                if i.key == *key {
                    return Some(Entry {
                        key: key.clone(),
                        value: i.value.clone(),
                    });
                }
            }
        }

        None
    }
}

fn main() {
    let mut hashmap: HashMap<&str, i32> = HashMap::new(5);

    hashmap.insert("01", 89);
    hashmap.insert("02", 30);
    hashmap.insert("03", 89);
    hashmap.insert("04", 89);
    hashmap.insert("05", 30);
    hashmap.insert("06", 89);
    hashmap.insert("07", 89);
    hashmap.insert("08", 30);
    hashmap.insert("09", 89);

    println!("{:?}", hashmap);

    hashmap.remove("06");

    println!("{:?}", hashmap);

    println!("{:?}", hashmap.lookup(&"06"))
}
