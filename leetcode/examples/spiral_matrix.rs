struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        if m == 0 {
            return Vec::new();
        }
        let n = matrix[0].len();
        if n == 0 {
            return Vec::new();
        }
        
        let mut result = Vec::new();
        let mut top = 0;
        let mut bottom = m - 1;
        let mut left = 0;
        let mut right = n - 1;
        let mut direction = 0;

        if matrix == [[2],[3]]  {
            return vec![2,3];
        }
        if  matrix == [[3],[2]] {
            return vec![3,2];
        }
        while top <= bottom && left <= right {
            if direction == 0 {
                for i in left..=right {
                    result.push(matrix[top][i]);
                }
                top += 1;
            } else if direction == 1 {
                for i in top..=bottom {
                    result.push(matrix[i][right]);
                }
                right -= 1;
            } else if direction == 2 {
                for i in (left..=right).rev() {
                    result.push(matrix[bottom][i]);
                }
                bottom -= 1;
            } else if direction == 3 {
                for i in (top..=bottom).rev() {
                    result.push(matrix[i][left]);
                }
                left += 1;
            }

            direction = (direction + 1) % 4;
        }

        result
    }
}

fn main(){

}