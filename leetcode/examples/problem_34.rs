// 34. Find First and Last Position of Element in Sorted Array
// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

// Given an array of integers nums sorted in non-decreasing order, find the starting and ending position of a given target value.
// If target is not found in the array, return [-1, -1].
// You must write an algorithm with O(log n) runtime complexity.

// Example 1:
// Input: nums = [5,7,7,8,8,10], target = 8
// Output: [3,4]

// Example 2:
// Input: nums = [5,7,7,8,8,10], target = 6
// Output: [-1,-1]

// Example 3:
// Input: nums = [], target = 0
// Output: [-1,-1]

struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        
        let left = Self::find_left_bound(&nums, target);
        let right = Self::find_right_bound(&nums, target);
        
        vec![left, right]
    }
    
    // Tìm vị trí đầu tiên (leftmost) của target
    fn find_left_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut result = -1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if nums[mid as usize] == target {
                result = mid; // Lưu vị trí tìm thấy
                right = mid - 1; // Tiếp tục tìm bên trái
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        result
    }
    
    // Tìm vị trí cuối cùng (rightmost) của target
    fn find_right_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut result = -1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if nums[mid as usize] == target {
                result = mid; // Lưu vị trí tìm thấy
                left = mid + 1; // Tiếp tục tìm bên phải
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        result
    }
}

fn main() {
    // Test case 1
    println!("{:?}", Solution::search_range(vec![5,7,7,8,8,10], 8)); // [3,4]
    
    // Test case 2
    println!("{:?}", Solution::search_range(vec![5,7,7,8,8,10], 6)); // [-1,-1]
    
    // Test case 3
    println!("{:?}", Solution::search_range(vec![], 0)); // [-1,-1]
    
    // Test case 4
    println!("{:?}", Solution::search_range(vec![1], 1)); // [0,0]
    
    // Test case 5
    println!("{:?}", Solution::search_range(vec![2,2], 2)); // [0,1]
}

// GIẢI THÍCH THUẬT TOÁN:
// 
// 1. Ý tưởng chính:
//    - Sử dụng 2 lần binary search:
//      + Lần 1: Tìm vị trí đầu tiên (leftmost) của target
//      + Lần 2: Tìm vị trí cuối cùng (rightmost) của target
//    - Time complexity: O(log n) cho mỗi lần tìm kiếm, tổng là O(log n)
//    - Space complexity: O(1)
//
// 2. Find Left Bound (Tìm biên trái):
//    - Khi tìm thấy target tại mid:
//      + Lưu lại vị trí này
//      + Tiếp tục tìm ở nửa bên TRÁI (right = mid - 1)
//      + Vì có thể còn target ở bên trái
//
// 3. Find Right Bound (Tìm biên phải):
//    - Khi tìm thấy target tại mid:
//      + Lưu lại vị trí này
//      + Tiếp tục tìm ở nửa bên PHẢI (left = mid + 1)
//      + Vì có thể còn target ở bên phải
//
// 4. Ví dụ với nums = [5,7,7,8,8,10], target = 8:
//    
//    Find Left Bound:
//    [5,7,7,8,8,10]
//     0 1 2 3 4 5
//    left=0, right=5, mid=2 -> nums[2]=7 < 8 -> left=3
//    left=3, right=5, mid=4 -> nums[4]=8 == 8 -> result=4, right=3
//    left=3, right=3, mid=3 -> nums[3]=8 == 8 -> result=3, right=2
//    left=3, right=2 -> dừng, return 3
//
//    Find Right Bound:
//    [5,7,7,8,8,10]
//     0 1 2 3 4 5
//    left=0, right=5, mid=2 -> nums[2]=7 < 8 -> left=3
//    left=3, right=5, mid=4 -> nums[4]=8 == 8 -> result=4, left=5
//    left=5, right=5, mid=5 -> nums[5]=10 > 8 -> right=4
//    left=5, right=4 -> dừng, return 4
//
//    Kết quả: [3, 4]

