
struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut cache = Vec::with_capacity(len);
        for n in nums {
            /*
            
            The binary_search method returns a Result enum, which can have two possible variants: Ok or Err. 
            If the element is found, it returns Ok with the index of the element; 
            otherwise, 
            it returns Err with the index where the element should be inserted to maintain the sorted order. 
            
            */
            let idx = cache.binary_search(&n).unwrap_or_else(|i| i);
        
            // update cache content

            if idx < cache.len() {
                cache[idx] = n;
            } else {
                cache.push(n);
            }  

        }
        cache.len() as _
    }
}

fn main() {
    Solution::length_of_lis(vec![9, 3, 6, 4, 9, 4, 10, 5, 6]);
}
