use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut store: HashMap<i32, i32> = HashMap::new();
      
        let times  = nums.len() / 2 + 1;
        for ch in nums {
            store
                .entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }

        let data = store.iter().find( |&(_,x)| {
            *x >= times as i32
        }) ;


        return *data.unwrap().0;
    }
}

fn main() {
    let nums: Vec<i32> = vec![2,2,1,1,1,2,2];

    Solution::majority_element(nums);
}
