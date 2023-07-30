use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    key: i32,
    value: i32,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            next: None,
            prev: None,
        }))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.clone().key == other.clone().key
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print!("{}", self.clone().key);
        Ok(())
    }
}
struct Queue {
    front: Link,
    back: Link,
    capacity: i32,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            front: None,
            back: None,
            capacity: 0,
        }
    }

    pub fn push_back(&mut self, key: i32, value: i32) -> Rc<RefCell<Node>> {
        let node = Node::new(key, value);

        match self.back.take() {
            Some(old) => {
                old.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old);
            }

            None => {
                self.front = Some(node.clone());
            }
        }
        self.back = Some(node.clone());

        self.capacity += 1;
        return node;
    }

    fn to_back(&mut self, node: Link) {
        if self.back == node {
            return;
        }
        
        if self.front == node {
          
            let new_front = node.as_ref().unwrap().borrow().next.clone();

            new_front.clone().unwrap().borrow_mut().prev = None;

            self.front = new_front;

            self.back.as_ref().unwrap().borrow_mut().next = node.clone();
            node.as_ref().unwrap().borrow_mut().prev = self.back.clone();
            node.as_ref().unwrap().borrow_mut().next = None;

            self.back = node;

            return;
        }

      
        let node_prev = node.as_ref().unwrap().borrow().prev.clone();
        let node_next = node.as_ref().unwrap().borrow().next.clone();

        if let Some(node_prev) = node_prev {
            node_prev.borrow_mut().next = node.as_ref().unwrap().borrow().next.clone();
        }

        if let Some(node_next) = node_next {
            node_next.borrow_mut().prev = node.as_ref().unwrap().borrow().prev.clone();
        }

        node.as_ref().unwrap().borrow_mut().prev = self.back.clone();
        self.back.as_ref().unwrap().borrow_mut().next = node.clone();

        node.as_ref().unwrap().borrow_mut().next = None;
        self.back = node;
    }

    fn pop_front(&mut self) -> Option<i32> {
        if self.front.is_none() {
            return None;
        }

        let mut key: Option<i32> = None;
        match self.front.clone().take() {
            Some(x) => {
                key = Some(x.borrow().clone().key);
            }
            None => {}
        }

        match self.front.take() {
            Some(old) => {
                self.front = old.borrow().clone().to_owned().next;
            }
            None => {}
        }

        if self.front.is_none() {
            self.back = None;
        } else {
            match self.front.clone().take() {
                Some(x) => {
                    x.borrow_mut().prev = None;
                }
                None => {}
            }
        }

        self.capacity -= 1;
        return key;
    }
}

fn count_nodes(queue: &Queue) -> usize {
    let mut count = 0;
    let mut current = queue.front.clone();

    while let Some(node) = current {
        count += 1;
        current = node.borrow().next.clone();
    }

    count
}

struct LRUCache {
    capacity: i32,
    ht: HashMap<i32, Link>,
    q: Queue,
}

impl Debug for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self.front.clone();

        while let Some(node) = current.clone() {
            print!("{:?}", current);
            current = node.borrow().next.clone();
        }
        Ok(())
    }
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        return LRUCache {
            capacity: capacity,
            ht: HashMap::new(),
            q: Queue::new(),
        };
    }

    fn get(&mut self, key: i32) -> i32 {
        let value = self.ht.get(&key);

        if value.is_none() {
            return -1;
        }
        let mut result: i32 = -1;
        if let Some(x) = value.clone().take() {
            if let Some(b) = x.clone().take() {
                result = b.borrow().clone().value;
                self.q.to_back(Some(b));
            }
        }

        return result;
    }

    fn put(&mut self, key: i32, value: i32) {
        let entry = self.ht.get_mut(&key);

        if entry.is_none() {
            if self.q.capacity >= self.capacity {
                let key_to_remove = self.q.pop_front().unwrap();

                self.ht.remove_entry(&key_to_remove);
            }

            let value = self.q.push_back(key, value);

            self.ht.insert(key, Some(value));
        } else {
            let node = entry.unwrap().clone().unwrap();
            node.borrow_mut().value = value;

            self.q.to_back(Some(node));
        }
    }
}
fn main() {
    let mut c = LRUCache::new(1);
    c.put(2, 1);
    c.get(2);
    println!("{:?}", c.q);
}
