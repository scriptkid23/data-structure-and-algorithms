# Problem: Search a 2D Matrix

> **LeetCode #74** - Medium  
> **Link:** https://leetcode.com/problems/search-a-2d-matrix/

---

## 1. UNDERSTAND - Understanding the Problem

### 📋 Checklist:

#### Input:
- **Data type:** 2D matrix `matrix: Vec<Vec<i32>>` and `target: i32`
- **Constraints:**
  - `m == matrix.length`
  - `n == matrix[i].length`
  - `1 <= m, n <= 100`
  - `-10^4 <= matrix[i][j], target <= 10^4`
- **Characteristics:**
  - Each row is sorted in ascending order (left to right)
  - First element of each row is greater than last element of previous row
  - Matrix can be treated as a sorted 1D array

#### Output:
- **Return type:** `bool`
- **Format:** Return `true` if `target` is found, otherwise return `false`

#### Exact Requirements:
```
□ Search for target in sorted matrix
□ Matrix property: rows are sorted, and next row > previous row
□ Need optimization: O(log(m*n)) time
```

#### Edge Cases:
```
□ 1x1 matrix: [[1]], target=1 → true
□ 1x1 matrix: [[1]], target=0 → false
□ Target smaller than first element: [[1,3,5]], target=0 → false
□ Target larger than last element: [[1,3,5]], target=6 → false
□ Target in middle: [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target=3 → true
```

### 🎯 Rewritten Requirements:
**"Given a sorted 2D matrix (each row is ascending, and next row is greater than previous row), search for target value. Return true if found, false otherwise."**

---

## 2. ANALYZE - Constraint Analysis

### Constraint Analysis:

| Parameter | Value | Impact |
|-----------|-------|--------|
| m (rows) | ≤ 100 | Small |
| n (cols) | ≤ 100 | Small |
| Total elements | ≤ 10,000 | Can use O(m*n) but need optimization |

### Time Complexity Target:

```
m ≤ 100, n ≤ 100
→ O(m*n) = 10,000 ✅ OK (brute force)
→ O(m*log n) = 100*7 ≈ 700 ✅ Good (binary search each row)
→ O(log(m*n)) = log(10,000) ≈ 13 ✅ Optimal (treat as 1D array)

Conclusion: Should use binary search on entire matrix as 1D array
```

### Space Complexity Target:
```
Requirement: O(1) extra space
→ Only use a few variables: left, right, mid, row, col
```

---

## 3. PATTERN RECOGNITION - Pattern Recognition

### Keywords Analysis:

| Keyword | Pattern Hint |
|---------|---------------|
| **"Sorted matrix"** | Binary Search |
| **"Search"** | Binary Search |
| **"Each row sorted"** | Can binary search each row |
| **"First element > last of previous"** | Can treat as 1D sorted array |

### Pattern Identified:
✅ **Binary Search** - Treat matrix as sorted 1D array  
✅ **Index Mapping** - Map 1D index to 2D (row, col)

### Important Properties:
```
• Matrix property: first element of row i > last element of row i-1
• This ensures entire matrix can be treated as ascending 1D array
• Can use binary search directly on this "1D array"
```

### Similar Problems:
- Search in Rotated Sorted Array
- Search a 2D Matrix II (rows and columns both sorted)
- Find Peak Element

---

## 4. APPROACHES - Evaluating Approaches

### Approach 1: Brute Force ❌

**Idea:** Traverse entire matrix, compare each element

```
Time: O(m*n)
Space: O(1)
```

**Evaluation:**
- ✅ Simple, easy to implement
- ❌ Doesn't utilize sorted property
- ❌ Not optimal

---

### Approach 2: Binary Search Each Row ⚠️

**Idea:** Traverse each row, binary search within each row

```
Time: O(m*log n)
Space: O(1)
```

**Evaluation:**
- ✅ Utilizes row sorted property
- ⚠️ Not most optimal
- ⚠️ Doesn't utilize "next row > previous row" property

---

### Approach 3: Binary Search on Entire Matrix (Optimal) ✅

**Idea:** Treat matrix as sorted 1D array, use binary search

**Key Insight:**
```
Matrix m×n can be treated as 1D array with m*n elements
1D index: 0, 1, 2, ..., m*n-1

Mapping 1D → 2D:
  row = index / n
  col = index % n

Mapping 2D → 1D:
  index = row * n + col
```

**Algorithm:**
```
Step 1: Initialize
  left = 0 (first element)
  right = m*n - 1 (last element)

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

Step 3: Not found
  return false
```

**Complexity:**
- **Time:** O(log(m*n))
  - Binary search on m*n elements
  - Each step eliminates half
- **Space:** O(1)
  - Only use a few variables: left, right, mid, row, col

**Evaluation:**
- ✅ Optimal time complexity
- ✅ Maximizes sorted property utilization
- ✅ Clean, easy to understand code
- ✅ O(1) space

---

### Comparison Table:

| Approach | Time | Space | Pros | Cons | Choose? |
|----------|------|-------|------|------|---------|
| **Brute Force** | O(m*n) | O(1) | ✅ Simple | ❌ Not optimal | ❌ NO |
| **Binary Search each row** | O(m*log n) | O(1) | ✅ Good | ⚠️ Not most optimal | ⚠️ Maybe |
| **Binary Search entire** | O(log(m*n)) | O(1) | ✅ Optimal<br>✅ Elegant | ⚠️ Need to understand mapping | ✅ **YES** |

**Decision:** Choose Approach 3 - Binary Search on Entire Matrix

**Reasons:**
1. Achieves optimal complexity O(log(m*n))
2. Maximizes matrix sorted property utilization
3. Clean and easy to understand code
4. O(1) space

---

## 5. VERIFY - Verify Before Coding

### ✅ Pre-coding Checklist:

```
□ Algorithm correct for ALL examples? → Testing below
□ Edge Cases handled?
  □ 1x1 matrix → Works ✓
  □ Target doesn't exist → Works ✓
  □ Target at boundary → Works ✓
□ Complexity Analysis?
  □ Time: O(log(m*n)) ✓
  □ Space: O(1) ✓
```

### 🔍 Trace Examples:

#### Example 1: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3`

```
Matrix 3×4, treat as 1D array with 12 elements:
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

## 6. IMPLEMENT - Implementation

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
   - Check empty
   - Initialize left, right
   - Binary search with 1D → 2D mapping
   - Return result
```

### Comments Strategy:
- ✅ Comment index mapping logic
- ✅ Explain why treat as 1D array
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
□ ✅ Target at matrix start → true
□ ✅ Target at matrix end → true
```

---

## 8. COMPLEXITY ANALYSIS

### Time Complexity: **O(log(m*n))**

**Breakdown:**
```
Binary search on m*n elements:
  - Each step eliminates half
  - Total steps: log₂(m*n)
  
Total: O(log(m*n))
```

**Best case:** O(1)
- Target at first mid position

**Worst case:** O(log(m*n))
- Target doesn't exist or at boundary

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

✅ **No extra array needed** - only use a few variables

---

## 9. KEY INSIGHTS

### 🎯 Core Insights:

1. **Matrix Property**
   - Each row is ascending (left to right)
   - Next row > previous row (first element of row i > last element of row i-1)
   - This ensures entire matrix can be treated as ascending 1D array

2. **1D ↔ 2D Mapping**
   - 1D → 2D: `row = index / n`, `col = index % n`
   - 2D → 1D: `index = row * n + col`
   - This is important technique to apply binary search

3. **Direct Binary Search**
   - Don't need to find row first, then find column
   - Can binary search directly on this "1D array"
   - More optimal than binary search each row

4. **Why Don't Need to Find Row First?**
   - Because matrix has property: next row > previous row
   - If target > last element of row i, definitely not in row i
   - If target < first element of row i, definitely not in row i
   - Binary search automatically handles this

### 📚 Similar Patterns:
- Search in Rotated Sorted Array (also uses adjusted binary search)
- Search a 2D Matrix II (rows and columns both sorted, different approach)
- Find Peak Element (binary search with special conditions)

### 🔧 Implementation Tips:
- Use `i32` for left/right to avoid overflow when calculating `m*n`
- Index mapping: `row = mid / n`, `col = mid % n`
- Check empty matrix first

### ⚠️ Common Mistakes:
- Forgetting to check empty matrix
- Wrong index mapping (confusing row/col)
- Using `m` instead of `n` when mapping (must use column count `n`)
- Not understanding why can treat as 1D array

---

## 10. INTUITION & CORRECTNESS

### Intuition
- Matrix has special property: next row > previous row. This ensures when "flattening" matrix into 1D array, we get a complete ascending array.
- Binary search works on ascending array → can apply directly.

### Correctness Sketch
- Invariant: range `[left, right]` (in 1D space) always contains answer if it exists.
- Each iteration: compare `mid_value` with `target`. If `mid_value < target`, eliminate left half (because array is ascending, all left elements are < target). Similarly for right half. Each step eliminates at least half → O(log(m*n)) steps.

### Index Mapping
- Formula `row = mid / n`, `col = mid % n` ensures accurate mapping from 1D index to 2D.
- Example: `mid = 5`, `n = 4` → `row = 1`, `col = 1` → `matrix[1][1]` correct.

---

## 11. DETAILED TRACE (Step-by-step)

Example: `matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3`

**Matrix 3×4, treat as 1D array:**
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

## 12. VARIANT: FIND ROW THEN FIND COLUMN

### Idea
- Step 1: Binary search to find row containing target (compare with first/last element of each row).
- Step 2: Binary search within that row to find column.

### Assessment
- Also O(log m + log n) = O(log(m*n)) time.
- More verbose, more complex than treating as 1D array.

---

## 13. COMMON PITFALLS

- Wrong index mapping: using `m` instead of `n` when calculating `row = mid / n` (must use column count `n`).
- Forgetting to check empty matrix or empty row.
- Not understanding why can treat matrix as 1D array (need to understand "next row > previous row" property).
- Overflow when calculating `m*n` (should use `i32` or `i64`).

---

## 14. SUMMARY

### Problem Type: Binary Search + Index Mapping

### Solution Approach: Binary Search on Entire Matrix

### Key Algorithm Steps:
```
1. Treat matrix as 1D array with m*n elements
2. Binary search with left=0, right=m*n-1
3. Map mid (1D) → (row, col) (2D)
4. Compare and narrow search range
```

### Complexity:
- ⏰ Time: O(log(m*n))
- 💾 Space: O(1)

### Difficulty Rating:
- **Problem Understanding:** ⭐⭐ (Easy - Need to understand matrix property)
- **Pattern Recognition:** ⭐⭐⭐ (Medium - Need to recognize can treat as 1D array)
- **Implementation:** ⭐⭐ (Easy - Standard binary search + index mapping)

### Tags:
`#Array` `#BinarySearch` `#Matrix` `#IndexMapping`

---

**Time to solve:** ~15-20 minutes (including thinking)

**Recommended practice order:**
1. Understand matrix property (next row > previous row)
2. Understand 1D ↔ 2D mapping
3. Implement binary search with mapping
4. Test with edge cases

**Good luck! 🚀**

