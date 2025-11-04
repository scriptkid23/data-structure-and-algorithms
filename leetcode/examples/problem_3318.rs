/*
3318. Find X-Sum of All K-Long Subarrays I
https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i/

Problem:
You are given an array of integers `nums` of length `n` and two integers `k` and `x`.

The x-sum of an array is calculated by the following steps:
1. Count the occurrences of all elements in the array.
2. Keep only the occurrences of the top `x` most frequent elements. 
   If two elements have the same number of occurrences, the element with the bigger value is considered more frequent.
3. Calculate the sum of the resulting array.

Note: If an array has less than `x` distinct elements, its x-sum is the sum of the entire array.

Your task is to return an integer array `answer` of length `n - k + 1`, 
where `answer[i]` is the x-sum of the subarray `nums[i..i + k - 1]`.

Example 1:
Input: nums = [1,1,2,2,3,4,2,3], k = 6, x = 2
Output: [6,10,12]

Explanation:
- For subarray [1, 1, 2, 2, 3, 4], only keep elements 1 and 2 in the resulting array. 
  Hence, answer[0] = 1 + 1 + 2 + 2 = 6.
- For subarray [1, 2, 2, 3, 4, 2], only keep elements 2 and 4 in the resulting array. 
  Hence, answer[1] = 2 + 2 + 2 + 4 = 10. 
  Note that 4 is kept because it is bigger than 3 and 1, which have the same occurrences.
- For subarray [2, 2, 3, 4, 2, 3], only keep elements 2 and 3 in the resulting array. 
  Hence, answer[2] = 2 + 2 + 2 + 3 + 3 = 12.

Example 2:
Input: nums = [3,8,7,8,7,5], k = 2, x = 2
Output: [11,15,15,15,12]

Explanation:
- Since k == x, answer[i] is equal to the sum of the subarray nums[i..i + k - 1].

Constraints:
- 1 <= n == nums.length <= 50
- 1 <= nums[i] <= 50
- 1 <= x <= k <= nums.length
*/

struct Solution;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        vec![]
    }
}

fn main() {
    let nums = vec![1,1,2,2,3,4,2,3];
    let k = 6;
    let x = 2;
    println!("{:?}", Solution::find_x_sum(nums, k, x));
}