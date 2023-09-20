struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut f: Vec<i32> = vec![i32::MIN; nums.len() + 1];

        f[0] = nums[0];

        let mut current_max = nums[0];

        for i in 1..nums.len() {
            f[i] = nums[i].max(f[i - 1] + nums[i]);

            current_max = current_max.max(f[i]);
        }
        
        current_max
    }
}

fn main() {
    Solution::max_sub_array(vec![-1,-2]);
}
