
// # **#80. Remove Duplicates from Sorted Array II**
// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/description/


struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 2 || nums.len() == 1 {
            return nums.len() as i32;
        }
        let mut slow: usize = 2;

        for fast in 2..nums.len() {
            if nums[fast] == nums[slow - 2] {
                continue;
            } else {
                nums[slow] = nums[fast];
                slow = slow + 1;
            }
        }
        return slow as i32;
    }
}

fn main(){

    let mut nums: Vec<i32> = vec![1,1,1,2,2,3];

    let x = Solution::remove_duplicates(&mut nums);
    print!("{:?}", nums);
    print!("{}", x);
}