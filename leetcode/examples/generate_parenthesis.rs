struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut s: String = String::new();
        let mut  result:Vec<String> = Vec::new();
        generate(3, 3, &mut s, &mut result);

        return result;
    }
}

fn generate(left:i32, right: i32, s: &mut String, result: &mut Vec<String>) {

    if left == 0 && right == 0 {
        result.push(s.to_string());
    }

    if left > right || left < 0 || right <0 {
        return;
    }

    s.push('{');
    generate(left - 1, right, s, result);
    s.pop();

    s.push('}');
    generate(left, right - 1, s, result);
    s.pop();

}
fn main() {
 
    Solution::generate_parenthesis(3);
}
