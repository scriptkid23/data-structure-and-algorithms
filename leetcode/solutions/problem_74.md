# Bài toán: Search a 2D Matrix

> **LeetCode #74** - Medium  
> **Link:** https://leetcode.com/problems/search-a-2d-matrix/

---

## 1. HIỂU BÀI

### 📋 Checklist:

#### Input:
- **Kiểu dữ liệu:** Ma trận 2D `matrix: Vec<Vec<i32>>` và `target: i32`
- **Constraints:**
  - `m == matrix.length`
  - `n == matrix[i].length`
  - `1 <= m, n <= 100`
  - `-10^4 <= matrix[i][j], target <= 10^4`
- **Đặc điểm:**
  - Mỗi hàng được sắp xếp theo thứ tự tăng dần (từ trái sang phải)
  - Phần tử đầu tiên của mỗi hàng lớn hơn phần tử cuối cùng của hàng trước đó
  - Ma trận có thể coi như một mảng 1D đã được sắp xếp

#### Output:
- **Return type:** `bool`
- **Format:** Trả về `true` nếu tìm thấy `target`, ngược lại trả về `false`

#### Yêu cầu chính xác:
```
□ Tìm kiếm target trong ma trận đã sắp xếp
□ Ma trận có tính chất: hàng tăng dần, và hàng sau > hàng trước
□ Cần tối ưu: O(log(m*n)) thời gian
```

#### Edge Cases:
```
□ Ma trận 1x1: [[1]], target=1 → true
□ Ma trận 1x1: [[1]], target=0 → false
□ Target nhỏ hơn phần tử đầu: [[1,3,5]], target=0 → false
□ Target lớn hơn phần tử cuối: [[1,3,5]], target=6 → false
□ Target ở giữa: [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target=3 → true
```

### 🎯 Viết lại yêu cầu:
**"Cho một ma trận 2D đã được sắp xếp (mỗi hàng tăng dần, và hàng sau lớn hơn hàng trước), tìm kiếm giá trị target. Trả về true nếu tìm thấy, false nếu không."**

---

## 2. ANALYZE - Phân Tích Constraints

### Constraint Analysis:

| Parameter | Value | Impact |
|-----------|-------|--------|
| m (rows) | ≤ 100 | Nhỏ |
| n (cols) | ≤ 100 | Nhỏ |
| Total elements | ≤ 10,000 | Có thể dùng O(m*n) nhưng cần tối ưu |

### Time Complexity Target:

```
m ≤ 100, n ≤ 100
→ O(m*n) = 10,000 ✅ OK (brute force)
→ O(m*log n) = 100*7 ≈ 700 ✅ Tốt (binary search mỗi hàng)
→ O(log(m*n)) = log(10,000) ≈ 13 ✅ Tối ưu (coi như mảng 1D)

Kết luận: Nên dùng binary search trên toàn bộ ma trận như mảng 1D
```

### Space Complexity Target:
```
Yêu cầu: O(1) extra space
→ Chỉ dùng vài biến: left, right, mid, row, col
```

---

## 3. PATTERN RECOGNITION - Nhận Dạng Mẫu

### Keywords Analysis:

| Keyword | Pattern Gợi Ý |
|---------|---------------|
| **"Sorted matrix"** | Binary Search |
| **"Search"** | Binary Search |
| **"Each row sorted"** | Có thể binary search mỗi hàng |
| **"First element > last of previous"** | Có thể coi như mảng 1D sorted |

### Pattern Identified:
✅ **Binary Search** - Coi ma trận như mảng 1D đã sorted  
✅ **Index Mapping** - Ánh xạ chỉ số 1D sang 2D (row, col)

### Thuộc tính quan trọng:
```
• Ma trận có tính chất: phần tử đầu hàng i > phần tử cuối hàng i-1
• Điều này đảm bảo toàn bộ ma trận có thể coi như mảng 1D tăng dần
• Có thể dùng binary search trực tiếp trên "mảng 1D" này
```

### Similar Problems:
- Search in Rotated Sorted Array
- Search a 2D Matrix II (hàng và cột đều sorted)
- Find Peak Element

---

## 4. APPROACHES - Đánh Giá Các Cách Tiếp Cận

### Approach 1: Brute Force ❌

**Ý tưởng:** Duyệt toàn bộ ma trận, so sánh từng phần tử

```
Time: O(m*n)
Space: O(1)
```

**Đánh giá:**
- ✅ Đơn giản, dễ implement
- ❌ Không tận dụng tính sorted
- ❌ Không tối ưu

---

### Approach 2: Binary Search Mỗi Hàng ⚠️

**Ý tưởng:** Duyệt từng hàng, binary search trong mỗi hàng

```
Time: O(m*log n)
Space: O(1)
```

**Đánh giá:**
- ✅ Tận dụng tính sorted của hàng
- ⚠️ Chưa tối ưu nhất
- ⚠️ Không tận dụng tính chất "hàng sau > hàng trước"

---

### Approach 3: Binary Search trên Toàn Bộ Ma Trận (Optimal) ✅

**Ý tưởng:** Coi ma trận như mảng 1D đã sorted, dùng binary search

**Key Insight:**
```
Ma trận m×n có thể coi như mảng 1D có m*n phần tử
Chỉ số 1D: 0, 1, 2, ..., m*n-1

Ánh xạ 1D → 2D:
  row = index / n
  col = index % n

Ánh xạ 2D → 1D:
  index = row * n + col
```

**Algorithm:**
```
Step 1: Khởi tạo
  left = 0 (phần tử đầu)
  right = m*n - 1 (phần tử cuối)

Step 2: Binary search
  while left <= right:
    mid = (left + right) / 2
    row = mid / n
    col = mid % n
    mid_value = matrix[row][col]
    
    if mid_value == target:
      return true
    elif mid_value < target:
      left = mid + 1
    else:
      right = mid - 1

Step 3: Không tìm thấy
  return false
```

**Complexity:**
- **Time:** O(log(m*n))
  - Binary search trên m*n phần tử
  - Mỗi bước loại bỏ một nửa
- **Space:** O(1)
  - Chỉ dùng vài biến: left, right, mid, row, col

**Đánh giá:**
- ✅ Optimal time complexity
- ✅ Tận dụng tối đa tính sorted
- ✅ Code gọn, dễ hiểu
- ✅ O(1) space

---

### Comparison Table:

| Approach | Time | Space | Pros | Cons | Choose? |
|----------|------|-------|------|------|---------|
| **Brute Force** | O(m*n) | O(1) | ✅ Đơn giản | ❌ Không tối ưu | ❌ NO |
| **Binary Search mỗi hàng** | O(m*log n) | O(1) | ✅ Tốt | ⚠️ Chưa tối ưu nhất | ⚠️ Maybe |
| **Binary Search toàn bộ** | O(log(m*n)) | O(1) | ✅ Optimal<br>✅ Elegant | ⚠️ Cần hiểu ánh xạ | ✅ **YES** |

**Decision:** Chọn Approach 3 - Binary Search trên Toàn Bộ Ma Trận

**Lý do:**
1. Đạt optimal complexity O(log(m*n))
2. Tận dụng tối đa tính chất sorted của ma trận
3. Code gọn và dễ hiểu
4. O(1) space

---

## 5. VERIFY - Kiểm Chứng Trước Khi Code

### ✅ Pre-coding Checklist:

```
□ Algorithm đúng với ALL examples? → Testing below
□ Edge Cases handled?
  □ Ma trận 1x1 → Works ✓
  □ Target không tồn tại → Works ✓
  □ Target ở biên → Works ✓
□ Complexity Analysis?
  □ Time: O(log(m*n)) ✓
  □ Space: O(1) ✓
```

### 🔍 Trace Examples:

#### Example 1: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3`

```
Ma trận 3×4, coi như mảng 1D có 12 phần tử:
[1,3,5,7,10,11,16,20,23,30,34,60]
        0 1 2 3  4  5  6  7  8  9 10 11

Step 1: left=0, right=11, mid=5
  row = 5/4 = 1, col = 5%4 = 1
  matrix[1][1] = 11
  11 > 3 → right = 4

Step 2: left=0, right=4, mid=2
  row = 2/4 = 0, col = 2%4 = 2
  matrix[0][2] = 5
  5 > 3 → right = 1

Step 3: left=0, right=1, mid=0
  row = 0/4 = 0, col = 0%4 = 0
  matrix[0][0] = 1
  1 < 3 → left = 1

Step 4: left=1, right=1, mid=1
  row = 1/4 = 0, col = 1%4 = 1
  matrix[0][1] = 3
  3 == 3 → return true ✓✓✓ CORRECT!
```

#### Example 2: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13`

```
Step 1: left=0, right=11, mid=5
  matrix[1][1] = 11
  11 < 13 → left = 6

Step 2: left=6, right=11, mid=8
  row = 8/4 = 2, col = 8%4 = 0
  matrix[2][0] = 23
  23 > 13 → right = 7

Step 3: left=6, right=7, mid=6
  row = 6/4 = 1, col = 6%4 = 2
  matrix[1][2] = 16
  16 > 13 → right = 5

Step 4: left=6, right=5 → left > right
  return false ✓✓✓ CORRECT!
```

---

## 6. IMPLEMENT - Viết Code

### Rust Implementation:

```rust
struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        
        let m = matrix.len();
        let n = matrix[0].len();
        let mut left = 0_i32;
        let mut right = (m * n - 1) as i32;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            let row = (mid / n as i32) as usize;
            let col = (mid % n as i32) as usize;
            let mid_val = matrix[row][col];
            
            if mid_val == target {
                return true;
            } else if mid_val < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        false
    }
}

fn main() {
    // Test Example 1
    let matrix1 = vec![
        vec![1,3,5,7],
        vec![10,11,16,20],
        vec![23,30,34,60]
    ];
    println!("Test 1: {} → Expected: true", Solution::search_matrix(matrix1, 3));
    
    // Test Example 2
    let matrix2 = vec![
        vec![1,3,5,7],
        vec![10,11,16,20],
        vec![23,30,34,60]
    ];
    println!("Test 2: {} → Expected: false", Solution::search_matrix(matrix2, 13));
    
    // Test Edge Case: 1x1
    let matrix3 = vec![vec![1]];
    println!("Test 3: {} → Expected: true", Solution::search_matrix(matrix3, 1));
    println!("Test 4: {} → Expected: false", Solution::search_matrix(matrix3, 0));
}
```

### Code Structure:
```
1. Main function: search_matrix()
   - Kiểm tra empty
   - Khởi tạo left, right
   - Binary search với ánh xạ 1D → 2D
   - Trả về kết quả
```

### Comments Strategy:
- ✅ Comment logic ánh xạ index
- ✅ Giải thích tại sao coi như mảng 1D
- ✅ Comment edge case handling

---

## 7. TEST CASES

### Test Results:

```
✓ Example 1: [[1,3,5,7],[10,11,16,20],[23,30,34,60]], 3 → true
✓ Example 2: [[1,3,5,7],[10,11,16,20],[23,30,34,60]], 13 → false
✓ Edge: [[1]], 1 → true
✓ Edge: [[1]], 0 → false
✓ Edge: [[1,3,5]], 0 → false
✓ Edge: [[1,3,5]], 6 → false
```

### Edge Cases Coverage:
```
□ ✅ Empty matrix - N/A (constraint: m,n >= 1)
□ ✅ Single element: [[1]], 1 → true
□ ✅ Single element: [[1]], 0 → false
□ ✅ Target < first element → false
□ ✅ Target > last element → false
□ ✅ Target ở đầu ma trận → true
□ ✅ Target ở cuối ma trận → true
```

---

## 8. COMPLEXITY ANALYSIS

### Time Complexity: **O(log(m*n))**

**Breakdown:**
```
Binary search trên m*n phần tử:
  - Mỗi bước loại bỏ một nửa
  - Tổng số bước: log₂(m*n)
  
Total: O(log(m*n))
```

**Best case:** O(1)
- Target ở vị trí mid đầu tiên

**Worst case:** O(log(m*n))
- Target không tồn tại hoặc ở biên

**Average case:** O(log(m*n))

### Space Complexity: **O(1)**

**Breakdown:**
```
Variables used:
  - left, right, mid: O(1)
  - row, col: O(1)
  - mid_val: O(1)

Total: O(1) auxiliary space
```

✅ **Không cần extra array** - chỉ dùng vài biến

---

## 9. KEY INSIGHTS

### 🎯 Core Insights:

1. **Tính Chất Ma Trận**
   - Mỗi hàng tăng dần (từ trái sang phải)
   - Hàng sau > hàng trước (phần tử đầu hàng i > phần tử cuối hàng i-1)
   - Điều này đảm bảo toàn bộ ma trận có thể coi như mảng 1D tăng dần

2. **Ánh Xạ 1D ↔ 2D**
   - 1D → 2D: `row = index / n`, `col = index % n`
   - 2D → 1D: `index = row * n + col`
   - Đây là kỹ thuật quan trọng để áp dụng binary search

3. **Binary Search Trực Tiếp**
   - Không cần tìm hàng trước, rồi tìm cột sau
   - Có thể binary search trực tiếp trên "mảng 1D" này
   - Tối ưu hơn so với binary search mỗi hàng

4. **Tại Sao Không Cần Tìm Hàng Trước?**
   - Vì ma trận có tính chất: hàng sau > hàng trước
   - Nếu target > phần tử cuối hàng i, chắc chắn không ở hàng i
   - Nếu target < phần tử đầu hàng i, chắc chắn không ở hàng i
   - Binary search tự động xử lý điều này

### 📚 Similar Patterns:
- Search in Rotated Sorted Array (cũng dùng binary search có điều chỉnh)
- Search a 2D Matrix II (hàng và cột đều sorted, dùng khác)
- Find Peak Element (binary search với điều kiện đặc biệt)

### 🔧 Implementation Tips:
- Dùng `i32` cho left/right để tránh overflow khi tính `m*n`
- Ánh xạ index: `row = mid / n`, `col = mid % n`
- Kiểm tra empty matrix trước

### ⚠️ Common Mistakes:
- Quên kiểm tra empty matrix
- Ánh xạ index sai (nhầm row/col)
- Dùng `m` thay vì `n` khi ánh xạ (phải dùng số cột `n`)
- Không hiểu tại sao có thể coi như mảng 1D

---

## 10. TRỰC GIÁC & VÌ SAO ĐÚNG

### Trực giác
- Ma trận có tính chất đặc biệt: hàng sau > hàng trước. Điều này đảm bảo khi "duỗi" ma trận thành mảng 1D, ta có một mảng tăng dần hoàn chỉnh.
- Binary search hoạt động trên mảng tăng dần → áp dụng trực tiếp được.

### Phác thảo tính đúng đắn
- Bất biến: khoảng `[left, right]` (trong không gian 1D) luôn chứa nghiệm nếu tồn tại.
- Mỗi vòng lặp: so sánh `mid_value` với `target`. Nếu `mid_value < target`, loại bỏ nửa trái (vì mảng tăng dần, tất cả phần tử trái đều < target). Tương tự cho nửa phải. Mỗi bước loại ít nhất một nửa → O(log(m*n)) bước.

### Ánh xạ index
- Công thức `row = mid / n`, `col = mid % n` đảm bảo ánh xạ chính xác từ chỉ số 1D sang 2D.
- Ví dụ: `mid = 5`, `n = 4` → `row = 1`, `col = 1` → `matrix[1][1]` đúng.

---

## 11. TRUY VẾT CHI TIẾT (Step-by-step Trace)

Ví dụ: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3`

**Ma trận 3×4, coi như mảng 1D:**
```
Index:  0  1  2  3   4  5  6  7   8  9 10 11
Value:  1  3  5  7  10 11 16 20  23 30 34 60
```

1) `left=0`, `right=11`, `mid=5`
   - `row = 5/4 = 1`, `col = 5%4 = 1`
   - `matrix[1][1] = 11`
   - `11 > 3` → `right = 4`

2) `left=0`, `right=4`, `mid=2`
   - `row = 2/4 = 0`, `col = 2%4 = 2`
   - `matrix[0][2] = 5`
   - `5 > 3` → `right = 1`

3) `left=0`, `right=1`, `mid=0`
   - `row = 0/4 = 0`, `col = 0%4 = 0`
   - `matrix[0][0] = 1`
   - `1 < 3` → `left = 1`

4) `left=1`, `right=1`, `mid=1`
   - `row = 1/4 = 0`, `col = 1%4 = 1`
   - `matrix[0][1] = 3`
   - `3 == 3` → return `true` ✓

---

## 12. BIẾN THỂ: TÌM HÀNG RỒI TÌM CỘT

### Ý tưởng
- Bước 1: Binary search để tìm hàng chứa target (so sánh với phần tử đầu/cuối mỗi hàng).
- Bước 2: Binary search trong hàng đó để tìm cột.

### Đánh giá
- Cũng O(log m + log n) = O(log(m*n)) thời gian.
- Code dài hơn, phức tạp hơn so với cách coi như mảng 1D.

---

## 13. LỖI THƯỜNG GẶP

- Ánh xạ index sai: dùng `m` thay vì `n` khi tính `row = mid / n` (phải dùng số cột `n`).
- Quên kiểm tra empty matrix hoặc hàng rỗng.
- Không hiểu tại sao có thể coi ma trận như mảng 1D (cần hiểu tính chất "hàng sau > hàng trước").
- Overflow khi tính `m*n` (nên dùng `i32` hoặc `i64`).

---

## 14. SUMMARY

### Problem Type: Binary Search + Index Mapping

### Solution Approach: Binary Search trên Toàn Bộ Ma Trận

### Key Algorithm Steps:
```
1. Coi ma trận như mảng 1D có m*n phần tử
2. Binary search với left=0, right=m*n-1
3. Ánh xạ mid (1D) → (row, col) (2D)
4. So sánh và thu hẹp khoảng tìm kiếm
```

### Complexity:
- ⏰ Time: O(log(m*n))
- 💾 Space: O(1)

### Difficulty Rating:
- **Problem Understanding:** ⭐⭐ (Easy - Cần hiểu tính chất ma trận)
- **Pattern Recognition:** ⭐⭐⭐ (Medium - Cần nhận ra có thể coi như mảng 1D)
- **Implementation:** ⭐⭐ (Easy - Binary search chuẩn + ánh xạ index)

### Tags:
`#Array` `#BinarySearch` `#Matrix` `#IndexMapping`

---

**Time to solve:** ~15-20 minutes (including thinking)

**Recommended practice order:**
1. Hiểu tính chất ma trận (hàng sau > hàng trước)
2. Hiểu ánh xạ 1D ↔ 2D
3. Implement binary search với ánh xạ
4. Test với edge cases

**Good luck! 🚀**

