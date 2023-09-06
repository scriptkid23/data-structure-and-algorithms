struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let count_zero = nums
            .iter()
            .fold(0, |acc, &x| if x == 0 { acc + 1 } else { acc });

        nums.retain(|&x| x != 0);

        nums.append(&mut vec![0; count_zero]);
    }
}

fn main() {
    Solution::move_zeroes(&mut vec![4, 2, 4, 0, 0, 3, 0, 5, 1, 0])
}
