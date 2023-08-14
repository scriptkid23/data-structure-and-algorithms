#[derive(Debug)]
struct MinStack {
    store: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            store: Vec::new(),
            min: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.store.push(val);
        let index = self.min.binary_search(&val).unwrap_or_else(|i| i);
        self.min.insert(index, val);
    }

    fn pop(&mut self) {
        if let Some(x) = self.store.pop() {
            let index = self.min.binary_search(&x).expect("Failed");

            self.min.remove(index);
            
           
        }
    }

    fn top(&self) -> i32 {
        let len = self.store.len();

        if len < 1 {
            return 0;
        }
        if let Some(x) = self.store.get(len - 1) {
            return *x;
        }

        return 0;
    }

    fn get_min(&self) -> i32 {
        let len = self.min.len();
        return self.min[0];
    }
}

fn main() {
    let mut ms = MinStack::new();

    ms.push(5);
    ms.push(3);
    ms.push(4);

    print!("{:?}", ms.get_min());
   
   ms.pop();
   ms.pop();

   print!("{:?}", ms);
   ms.top();

   print!("{:?}", ms.get_min());



}
