// 39. Combination Sum
struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut combinations: Vec<Vec<i32>> = Vec::new();
        let mut current_combination: Vec<i32> = Vec::new();
        
        Self::backtrack(&candidates, target, 0, &mut current_combination, &mut combinations);
        
        
        combinations
    }
    
    fn backtrack(
        candidates: &[i32],
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            combinations.push(current_combination.clone());
            return;
        }
        
        for i in start..candidates.len() {
            let candidate = candidates[i];
            if candidate <= target {
                current_combination.push(candidate);
                Self::backtrack(candidates, target - candidate, i, current_combination, combinations);
                current_combination.pop();
            }
        }
    }
}
fn main() {
    Solution::combination_sum(vec![2,3,6,7], 8);
}
