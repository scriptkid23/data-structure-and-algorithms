struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {

        let mut buffer = x;

        let mut result = 0;

        let mut stack:Vec<i32> = Vec::new();
        loop  {
            stack.push(buffer % 10);
            buffer = buffer / 10;
            if buffer == 0 {
                break;
            }

        }

        stack.retain(|&x| {x != 0});
        
        print!("{:?}", stack);
        return 0;
    }
}

fn main(){
    Solution::reverse(0123000000);
}