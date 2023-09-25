// 35. Search Insert Position

// Given a sorted array of distinct integers and a target value, return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.

// You must write an algorithm with O(log n) runtime complexity.
struct Solution;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
      let mut left = 0;
    let mut right = nums.len() as i32 - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] == target {
            return mid;
        } else if nums[mid as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    left
    }
}

fn main(){

    println!("{}", Solution::search_insert(vec![1,3,5,6], 4))
}

