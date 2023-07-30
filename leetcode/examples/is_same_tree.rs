// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn insert_left(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        let current = node.as_ref().unwrap().borrow().left.clone();

        if current.is_none() {
            node.clone().unwrap().borrow_mut().left =
                Some(Rc::new(RefCell::new(TreeNode::new(val))));
            return;
        }
        TreeNode::insert_left(&current, val);
    }

    pub fn insert_right(node: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
        let current = node.as_ref().unwrap().borrow().right.clone();

        if current.is_none() {
            node.clone().unwrap().borrow_mut().right =
                Some(Rc::new(RefCell::new(TreeNode::new(val))));
            return;
        }
        TreeNode::insert_right(&current, val);
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut vec_p = Vec::new();
        scan(&p, &mut vec_p);

        let mut vec_q = Vec::new();
        scan(&q, &mut vec_q);

        let result = vec_p.cmp(&vec_q);

        print!("{result:?}");
        return result.is_eq();
    
    }
}

pub fn scan(tree: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<Option<i32>>) {
    if tree.is_none() {
        return;
    };

    let current = tree.as_ref().unwrap().borrow_mut();
    vec.push(Some(current.val));

    if !current.left.is_none() {
        scan(&current.left, vec);
    }
    else {
        vec.push(None);
    }

    if !current.right.is_none() {
        scan(&current.right, vec);
    }
    else {
        vec.push(None);
    }
}

fn main() {
    let p = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    TreeNode::insert_left(&p, 2);


    let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    TreeNode::insert_right(&q, 2);

    Solution::is_same_tree(p, q);
}
