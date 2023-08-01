use std::{cell::RefCell, rc::Rc};

struct Trie {
    root: Option<Rc<RefCell<TrieNode>>>,
}
#[derive(Debug)]
struct TrieNode {
    value: Option<String>,
    is_entry: bool,
    children: Vec<Option<Rc<RefCell<TrieNode>>>>, //TODO: HashMap<Option<Rc<RefCell<TrieNode>>>>
}

impl TrieNode {
    fn letter_to_index(letter: char) -> Option<usize> {
        match letter {
            'A'..='Z' => Some((letter as u8 - b'A') as usize),
            'a'..='z' => Some((letter as u8 - b'a') as usize),
            _ => None,
        }
    }

    fn search(
        current: Option<Rc<RefCell<TrieNode>>>,
        word: String,
        index: usize,
        starts_with: bool,
    ) -> Option<String> {
        if current.is_none() {
            return None;
        }

        if index == word.len() {
            if let Some(node) = current {
                let borrow_node = node.borrow();
                let node_value = borrow_node.value.clone();

                if !borrow_node.is_entry && !starts_with {
                    return None;
                }
                return node_value;
            } else {
                return None;
            }
        }

        let next_letter = word.chars().nth(index).unwrap();
        let next_index = TrieNode::letter_to_index(next_letter);
        let next_child =
            current.as_ref().unwrap().borrow_mut().children[next_index.unwrap()].clone();
        if next_child.is_none() {
            return None;
        } else {
            return TrieNode::search(next_child, word, index + 1, starts_with);
        }
    }
    fn insert(current: Option<Rc<RefCell<TrieNode>>>, val: String, index: usize) {
        if index == val.len() {
            current.as_ref().unwrap().borrow_mut().is_entry = true;
            return;
        } else {
            let next_letter = val.chars().nth(index).unwrap();
            let next_index = TrieNode::letter_to_index(next_letter);
            match next_index {
                Some(i) => {
                    let mut next_child = current.as_ref().unwrap().borrow_mut().children[i].clone();
                    if next_child.is_none() {
                        next_child = Some(Rc::new(RefCell::new(TrieNode {
                            value: Some(val[0..=index].to_string()),
                            is_entry: false,
                            children: vec![None; 26],
                        })));
                        current.as_ref().unwrap().borrow_mut().children[i] = next_child.clone();
                    }
                    TrieNode::insert(next_child, val, index + 1);
                }
                None => {}
            }
        }
    }
}

impl Trie {
    fn new() -> Self {
        Trie { root: None }
    }

    fn starts_with(&self, prefix: String) -> bool {
        let word = TrieNode::search(self.root.clone(), prefix, 0, true).take();

        match word {
            Some(x) => {
                print!("{}", x);
                true
            }
            None => false,
        }
    }

    fn search(&self, word: String) -> bool {
        let word = TrieNode::search(self.root.clone(), word, 0, false).take();

        match word {
            Some(x) => {
                print!("{}", x);
                true
            }
            None => false,
        }
    }

    fn insert(&mut self, word: String) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(TrieNode {
                value: None,
                is_entry: false,
                children: vec![None; 26],
            })));
        }
        TrieNode::insert(self.root.clone(), word, 0);
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("hellom".to_string());
    trie.insert("hellow".to_string());

    print!("{}", trie.starts_with("he".to_string()));
}
