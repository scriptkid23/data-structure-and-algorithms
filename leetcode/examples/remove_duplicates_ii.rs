struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        
     nums.dedup_by_key( |&mut i| { i  % 2 == 1});
 nums
    print!("{:?}", nums);
      return nums.len() as i32;
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![0,0,1,1,1,1,2,3,3];
    
    let x = Solution::remove_duplicates(&mut nums);
    print!("{}", x);
}

/* version 1
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut index_buffer = 0;

        if nums.len() as i32 == 1 {return 1};

        loop {
            
            if nums[index_buffer] == nums[index_buffer + 1] {
                nums.remove(index_buffer + 1);
            } else {
                index_buffer = index_buffer + 1;
            }
            if index_buffer == nums.len() - 1 {
                break;
            }
        }
        print!("{:?}", nums);
        
        return nums.len() as i32;
    }

*/