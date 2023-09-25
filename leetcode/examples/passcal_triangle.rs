struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![vec![1]];

        let mut candidate: Vec<i32> = Vec::new();

        for i in 1..num_rows {
            for j in 0..=i {
                if j - 1 < 0 || j == i  {
                    candidate.push(1);
                } else {
                    candidate.push(
                        result[(i - 1) as usize][(j - 1) as usize]
                            + result[(i - 1) as usize][j  as usize],
                    );
                }
            }
            result.push(candidate.clone());
            print!("{:?}", result);
            candidate.clear();
        }

        result
    }
}

fn main() {
    Solution::generate(10);
}
