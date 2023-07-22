#[derive(Clone, Debug)]
struct Entry {
    key: usize,
    value: i32,
}
#[derive(Debug)]
struct HashTable {
    size: usize,
    bins: Vec<Option<Entry>>,
}

impl HashTable {
    pub fn hash(&self, value: i32) -> usize {
        return value as usize % self.size;
    }
    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            bins: vec![None; size],
        }
    }

    pub fn add(&mut self, value: i32) -> usize {
        let key = self.hash(value);

        let entry = Entry {
            key: key,
            value: value,
        };

        if let Some(bin) = self.bins.get_mut(key) {
            *bin = Some(entry);
        }

        return key;
    }

    pub fn get(&self, value: i32) -> &Option<Entry> {
        let key = self.hash(value);

        return self.bins.get(key).unwrap();
    }
}

fn main() {
    let vec_size = std::mem::size_of::<i64>();
    println!("Kích thước của Vec<String>: {} bytes", vec_size);
}
// a: 42 (0x72174ff900...0x72174ff907)
