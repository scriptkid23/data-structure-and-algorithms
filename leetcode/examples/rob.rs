struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![];
        if nums.len() == 1 {
            return nums[0]
        }
        dp.push(nums[0]);
        dp.push(nums[0].max(nums[1]));

        for i in 2..nums.len() {
            dp.push(i32::max(dp[i-1], dp[i-2] + nums[i]));
        }


        return  dp[dp.len() - 1];
    }
}

fn main(){
    let x = Solution::rob(vec![1,2,3,1]);

    print!("{x}");
}