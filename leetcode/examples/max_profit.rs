struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        let mut dp:i32 = 0;
        let mut mx:i32 = prices[0];

        for i in 0..prices.len()  {   
           mx = mx.min(prices[i]);
           dp = dp.max(prices[i] - mx);
        }        
       
        return  dp;
    }
}

fn main(){
    let nums = vec![7,6,4,3,1];

    Solution::max_profit(nums);
}