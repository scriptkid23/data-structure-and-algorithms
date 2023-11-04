struct Solution;

impl Solution {
    fn tower_of_hanoi(s: char, d: char, e: char, n:i32) {
         if n < 0 {
            return;
        }

        Self::tower_of_hanoi(s, e, d, n - 1);

        println!("Move Disk-{} FROM {} TO {}", n,s,d);

        Self::tower_of_hanoi(e, d, s, n - 1);
    } 
}

fn main() {
    Solution::tower_of_hanoi('s','d', 'e', 3);
}