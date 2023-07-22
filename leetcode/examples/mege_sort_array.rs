/*
You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and two integers m and n, 
representing the number of elements in nums1 and nums2 respectively.

Merge nums1 and nums2 into a single array sorted in non-decreasing order.

The final sorted array should not be returned by the function, but instead be stored inside the array nums1. 
To accommodate this, nums1 has a length of m + n, 
where the first m elements denote the elements that should be merged, and the last n elements are set to 0 and should be ignored. nums2 has a length of n.
*/
struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(m as usize);
        nums2.truncate(n as usize);

        nums1.append(nums2);
        nums1.sort();

    }
}
fn main() {
    let mut nums1: Vec<i32> = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
    let mut nums2: Vec<i32> = vec![1, 2, 2];

    Solution::merge(&mut nums1, 6, &mut nums2, 3);
}
