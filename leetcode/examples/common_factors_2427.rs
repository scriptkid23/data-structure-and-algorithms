
struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let max = a.max(b);
        let mut count = 0;
        for i in 1..=max {
            if (a % i + b % i) == 0 {
                count+=1;
            }
        }
        count
    }
}

fn main() {
    print!("{}", Solution::common_factors(12, 6));
}