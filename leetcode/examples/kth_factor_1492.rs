struct Solution;

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let maximum = f32::sqrt(n as f32) as i32;
        let mut arr: Vec<i32> = Vec::new();
        if k > maximum * 2 {
            return -1;
        }

        for i in 1..=maximum {
            if n % i == 0 {
                let idx = arr.binary_search(&i).unwrap_or_else(|i| i);
                arr.insert(idx, i);
                if n / i != i {
                    let idx_next = arr.binary_search(&(n / i)).unwrap_or_else(|i| i);
                    arr.insert(idx_next, n / i);
                }
            }
        }
        if arr.len() < k as usize {
            return -1;
        }

        return arr[(k - 1) as usize % arr.len()];
    }
}

fn main() {
    print!("{}", Solution::kth_factor(4, 4));
}
