use std::{cell::RefCell, rc::Rc};

type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}
impl Node {
    // A nice and short way of creating a new node
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    length: i32,
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        };
        self.length += 1;
        self.tail = Some(new);
    }
}

fn main(){

}