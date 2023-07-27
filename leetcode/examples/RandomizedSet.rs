use std::collections::HashMap;

use rand::Rng;

struct RandomizedSet {
    hash_table: HashMap<i32, usize>,
    array: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            hash_table: HashMap::new(),
            array: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.hash_table.contains_key(&val) {
            return false;
        }
        
        self.hash_table.insert(val, self.array.len());
        self.array.push(val);
        
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.hash_table.get(&val) {
            let last_index = self.array.len() - 1;
            let last_val = self.array[last_index];
            
            self.array[index] = last_val;
            self.hash_table.insert(last_val, index);
            
            self.array.pop();
            self.hash_table.remove(&val);
            
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        if self.array.is_empty() {
            return 0
        }
        let random_number: u32 = rng.gen_range(0, self.array.len() as u32);

        return self.array[random_number as usize];


    }
}

fn main(){
    
}