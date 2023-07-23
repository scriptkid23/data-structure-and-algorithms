// Collision: Linear Probing

use std::hash::Hasher;

#[derive(Clone, Debug, PartialEq)]
struct Entry<K: PartialEq, V: PartialEq> {
    key: K,
    value: V,
}
#[derive(Debug)]
struct HashMap<K: Clone + PartialEq, V: Clone + PartialEq> {
    size: usize,
    bins: Vec<Option<Entry<K, V>>>,
}
trait HashTable<K: PartialEq, V: PartialEq> {
    fn new(size: usize) -> Self;
    fn hash(&self, key: &K) -> usize;
    fn insert(&mut self, key: K, value: V);
    fn remove(&mut self, key: K);
    // fn lookup(&self, key: &K) -> Option<Entry<K, V>>;
}

impl<K: std::hash::Hash + PartialEq + Clone, V: Clone + PartialEq> HashTable<K, V>
    for HashMap<K, V>
{
    fn new(size: usize) -> Self {
        let size = size;

        return HashMap {
            size: size,
            bins: vec![None; size],
        };
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        key.hash(&mut hasher);

        return hasher.finish() as usize % self.size;
    }

    fn insert(&mut self, key: K, value: V) {
        let mut index = self.hash(&key);
        let mut count = 0;

        let mut current = self.bins[index].as_ref();

        while current.is_some() && current.unwrap().key != key && count != self.size {
            index = index + 1;

            if index >= self.size {
                index = 0
            }
            current = self.bins[index].as_ref();
            count = count + 1
        }
        print!("{:?}", count);
        if count == self.size {
            return;
        }

        if !current.is_some() {
            self.bins[index] = Some(Entry {
                key: key,
                value: value,
            });
        }
    }
    fn remove(&mut self, key: K) {}

    // fn lookup(&self, key: &K) -> Option<Entry<K, V>> {}
}

fn main() {
    let x: i32 = 10;
    let reference: &i32 = &x;
    let y = 100;
    
    println!("Address: {:p}", &reference);
    println!("Address: {:p}", reference);
}