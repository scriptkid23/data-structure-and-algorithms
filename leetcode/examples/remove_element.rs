

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        
        nums.retain(|&x| {x != val});

        return nums.len() as i32;
    }
}

fn main(){
    let mut nums:Vec<i32> = vec![0,1,2,2,3,0,4,2];

    Solution::remove_element(&mut nums, 2);
}