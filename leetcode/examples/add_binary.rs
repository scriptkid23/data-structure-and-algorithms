struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let max = a.len().max(b.len());

        let mut carry = 0;

        let mut rs: String = String::new();

        for i in 0..max {
            let x = if a.len() > i {
                a.len() - 1 - i % a.len()
            } else {
                max
            };
            let y = if b.len() > i {
                b.len() - 1 - i % b.len()
            } else {
                max
            };
            let mut rai = a.chars().nth(x);
            let mut rbi = b.chars().nth(y);

            let (ai, bi): (u32, u32);

            match rai.take() {
                Some(v) => ai = v.to_digit(10).unwrap(),
                None => ai = 0,
            }
            match rbi.take() {
                Some(v) => bi = v.to_digit(10).unwrap(),
                None => bi = 0,
            }
            

            if ai + bi + carry == 3 {
                rs.push('1');
                carry = 1;
            }

            if ai + bi + carry == 2 {
                rs.push('0');
                carry = 1;
            }

            if ai + bi + carry == 0 {
                rs.push('0');
                carry = 0;
            }

            if ai + bi + carry == 1 {
                rs.push('1');
                carry = 0;
            }
        }
        if carry == 1 {
            rs.push('1');
        }
        let reversed: String = rs.chars().rev().collect();
        print!("{:?}", reversed);
        return reversed;
    }
}

fn main() {


    Solution::add_binary("1111".to_string(), "1111".to_string());
}
