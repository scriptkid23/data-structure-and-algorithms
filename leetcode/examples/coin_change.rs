struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount+1; (amount + 1) as usize];

      

        dp[0] = 0;

        let mut coints_x = coins.clone();
        coints_x.sort();

       
        print!("{:?}", coints_x);

        for i in 1..=amount {
            for j in coints_x.clone() {
                if i >= j {
                    dp[i as usize] = i32::min(dp[i as usize], dp[(i - j) as usize] + 1);
                }
            }
        }
   
   print!("{:?}", dp);


        if amount == 0 {
            return 0
        }

        if dp[amount as usize] == amount + 1{
            return -1
        } 
     

        return dp[amount as usize];
    }
}

fn main() {

    Solution::coin_change(vec![1], 1);
}
