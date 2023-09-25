// 35. Search Insert Position

// Given a sorted array of distinct integers and a target value, return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.

// You must write an algorithm with O(log n) runtime complexity.
struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        return nums.binary_search(&target).unwrap_or_else(|i| i) as i32;
    }
}

fn main(){}

