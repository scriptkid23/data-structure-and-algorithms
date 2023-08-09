struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let index_valid = (digits.len() - 1) as i32;

        let x = &mut digits.clone();
        sum(x, 1, 0, index_valid);

        return x.clone();
    }
}

fn sum(digits: &mut Vec<i32>, num: i32, carry: i32, index: i32) {
    let is_carry = digits[index as usize] + carry + num == 10;

    digits[index as usize] = (digits[index as usize] + num + carry) % 10;

    if is_carry {
        if index == 0 && is_carry {
            digits.insert(0, 1);
            return;
        }

        sum(digits, 0, 1, index - 1);
    }
}

fn main() {
    Solution::plus_one(vec![9]);
}
