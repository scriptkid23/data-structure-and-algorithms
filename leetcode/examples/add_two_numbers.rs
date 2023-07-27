// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> ListNode {
       return ListNode { val: val, next: None }
    }

    fn append(mut self, val:i32) -> ListNode {
        let value = Box::new(ListNode {val: val, next: None});
        self.next = Some(value);
        return self;
    }
}
struct Solution;

impl Solution {
    pub fn add_two_numbers() -> Option<Box<ListNode>> {
            let x = ListNode::new(3).append(4);
            print!("{x:?}");
            return Some(Box::new(x));
    }
}
fn main(){
    Solution::add_two_numbers();
}