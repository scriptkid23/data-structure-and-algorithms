use std::vec;

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut combination = Vec::new();
        let start = 1;

        combine_recursive(&mut result, &mut combination, start, n, k);

        result
    }
}
fn combine_recursive(
    result: &mut Vec<Vec<i32>>,
    combination: &mut Vec<i32>,
    start: i32,
    n: i32,
    k: i32,
) {
    if k == 0 {
        result.push(combination.clone());
        return;
    }

    for i in start..=n {
        combination.push(i);
        combine_recursive(result, combination, i + 1, n, k - 1);
        combination.pop();
    }
}

fn main() {
    Solution::combine(4, 2);
}
