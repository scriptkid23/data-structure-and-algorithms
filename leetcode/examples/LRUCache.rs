use std::{cell::RefCell, rc::Rc, collections::HashMap};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: i32,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: None,
        }))
    }
}


struct Queue {
    front: Link,
    back: Link,
    ht: HashMap<i32, Link>
}
impl Queue {
    pub fn new() -> Queue {
        Queue {
            front: None,
            back: None,
            ht: HashMap::new()
        }
    }

    pub fn push_front(&mut self, val: i32) {
        let node = Node::new(val);

        self.ht.insert(val, Some(node.clone()));

        match self.back.take() {
            Some(old) => {
                old.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(old);
            }

            None => {
                self.front = Some(node.clone());
            }
        }
        self.back = Some(node);
    }
    fn pop_back(&mut self) -> i32 {
        0
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
fn main() {
    let mut q = Queue::new();
    q.push_front(3);
    
    println!("Number of nodes in queue: {}", count_nodes(&q));
}
