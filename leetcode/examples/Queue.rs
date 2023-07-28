use std::{cell::RefCell, rc::Rc};

mod Que
type Link = Option<Rc<RefCell<Node>>>;



#[derive(Clone, Debug)]
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
}
impl Queue {
    pub fn new() -> Queue {
        Queue {
            front: None,
            back: None,
        }
    }

    pub fn push_front(&mut self, val: i32) {
        let node = Node::new(val);

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
    fn pop_back(&mut self) {
        if self.front.is_none() {
            return;
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
    }
}

pub fn count_nodes(queue: &Queue) -> usize {
    let mut count = 0;
    let mut current = queue.front.clone();

    while let Some(node) = current {
        count += 1;
        current = node.borrow().next.clone();
    }

    count
}