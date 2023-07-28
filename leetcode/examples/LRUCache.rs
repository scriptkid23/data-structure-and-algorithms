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
        self.clone().value == other.clone().value && self.clone().key == other.clone().key
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
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            front: None,
            back: None,
        }
    }

    pub fn push_front(&mut self, key: i32, value: i32) -> Rc<RefCell<Node>> {
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

        return node;
    }
    fn remove(&mut self, node: Link) {
        let node_prev = node.as_ref().unwrap().borrow().prev.clone();
        let node_next = node.as_ref().unwrap().borrow().next.clone();

        if let Some(node_prev) = node_prev {
            node_prev.borrow_mut().next = node.as_ref().unwrap().borrow().next.clone();
        }

        if let Some(node_next) = node_next {
            node_next.borrow_mut().prev = node.as_ref().unwrap().borrow().prev.clone();
        }

        if node == self.front {
            self.front = self.front.clone().unwrap().borrow().next.clone();
        }

        if node == self.back {
            self.back = self.back.clone().unwrap().borrow().prev.clone();
        }
    }

    fn pop_back(&mut self) -> Option<i32> {
        if self.front.is_none() {
            return None;
        }

        let mut value: Option<i32> = None;
        match self.front.clone().take() {
            Some(x) => {
                value = Some(x.borrow().clone().value);
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

        return value;
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
        let mut result: i32 = -1;
        match value.clone().take() {
            Some(x) => {
                let mut buffer = x.clone();
                match buffer.take() {
                    Some(b) => {
                        result = b.borrow().clone().value;
                        self.q.remove(Some(b));
                        self.q.push_front(key, result);
                    }
                    None => {}
                }
            }
            None => {}
        }
        return result;
    }

    fn put(&mut self, key: i32, value: i32) {
        let entry = self.ht.get(&key);

        if entry.is_none() {
            if self.ht.len() >= self.capacity as usize {
                let key_to_remove = self.q.pop_back().unwrap();
                self.ht.remove_entry(&key_to_remove);
            }

            let value = self.q.push_front(key, value);

            self.ht.insert(key, Some(value));
        } else {
            self.q.remove(entry.unwrap().clone());
            self.q.push_front(key, value);
        }
    }
}
fn main() {
    let mut lRUCache = LRUCache::new(2);
    lRUCache.put(1, 0); // cache is {1=1}
    lRUCache.put(2, 2); // cache is {1=1}
    lRUCache.put(3, 3);
    lRUCache.get(2);
    lRUCache.put(4, 4);

    print!("{}", lRUCache.get(1));
    // lRUCache.get(2);

    print!("{:?}", lRUCache.q);
}
