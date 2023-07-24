struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
            let buffer =  k  % nums.len() as i32;

            nums.rotate_right(buffer as usize);
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 11);
}
