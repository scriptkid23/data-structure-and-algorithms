# Problem: Next Permutation

> **LeetCode #31** - Medium  
> **Link:** https://leetcode.com/problems/next-permutation/

---

## 1. UNDERSTAND - Hiểu Bài Toán

### 📋 Checklist:

#### Input:
- **Kiểu dữ liệu:** Array of integers `nums`
- **Constraints:**
  - `1 <= nums.length <= 100`
  - `0 <= nums[i] <= 100`
- **Đặc điểm:**
  - Có thể có duplicate
  - Không sorted
  - Không thể empty (length >= 1)

#### Output:
- **Return type:** Void (modify in-place)
- **Format:** Modify mảng `nums` thành next permutation
- **Đặc biệt:** Nếu là permutation lớn nhất → chuyển về nhỏ nhất

#### Yêu cầu chính xác:
```
□ Tìm next permutation theo thứ tự từ điển (lexicographically)
□ Modify in-place (O(1) extra space)
□ Nếu không có next (đang ở max) → reverse về min permutation
```

#### Edge Cases:
```
□ Single element: [1] → [1] (không đổi)
□ All elements same: [1,1,1] → [1,1,1] (không đổi)
□ Already max permutation: [3,2,1] → [1,2,3] (reverse)
□ Two elements: [1,2] → [2,1]
□ Next is simple swap: [1,2,3] → [1,3,2]
```

### 🎯 Viết lại yêu cầu:
**"Cho một mảng số nguyên đại diện cho một permutation, tìm permutation tiếp theo lớn hơn ngay sau nó theo thứ tự từ điển. Nếu không tồn tại (đang ở max), chuyển về permutation nhỏ nhất (sorted). Phải modify in-place."**

---

## 2. ANALYZE - Phân Tích Constraints

### Constraint Analysis:

| Parameter | Value | Impact |
|-----------|-------|--------|
| n (length) | ≤ 100 | Very small |
| nums[i] | 0-100 | Small values, no overflow concern |

### Time Complexity Target:

```
n ≤ 100
→ O(n³) = 1,000,000 ✅ OK
→ O(n²) = 10,000 ✅ Excellent
→ O(n) = 100 ✅ Perfect

Kết luận: Simple O(n) solution là đủ và tối ưu
```

### Space Complexity Target:
```
Yêu cầu: O(1) extra space (in-place modification)
→ Không được tạo array mới
→ Chỉ dùng vài biến
```

---

## 3. PATTERN RECOGNITION - Nhận Dạng Mẫu

### Keywords Analysis:

| Keyword | Pattern Gợi Ý |
|---------|---------------|
| **"Permutation"** | Mathematical pattern, ordering |
| **"Next"** | Sequential, find next in order |
| **"In-place"** | Two pointers, swap operations |
| **"Lexicographically"** | Dictionary order, compare from left |

### Pattern Identified:
✅ **Two Pointers** - Để scan và swap  
✅ **Greedy** - Tìm vị trí swap tối ưu  
✅ **Array Manipulation** - Reverse, swap

### Thuộc tính quan trọng:
```
• Next permutation = permutation nhỏ nhất lớn hơn current
• Cần thay đổi ít nhất có thể (rightmost change)
• Phần còn lại phải nhỏ nhất có thể
```

### Similar Problems:
- Previous Permutation
- Permutation Sequence
- Next Greater Element

---

## 4. APPROACHES - Đánh Giá Các Cách Tiếp Cận

### Approach 1: Generate All Permutations ❌

**Ý tưởng:** Generate tất cả permutations, sort, tìm next

```
Time: O(n! × n log n) - Generate n! permutations, sort
Space: O(n!)
```

**Đánh giá:**
- ❌ Cực kỳ chậm và tốn bộ nhớ
- ❌ Không satisfy yêu cầu O(1) space
- ❌ Overkill cho bài toán này

---

### Approach 2: Mathematical Pattern (Optimal) ✅

**Ý tưởng:** Phát hiện pattern toán học của next permutation

**Quan sát quan trọng:**
```
[1, 2, 3] → [1, 3, 2] → [2, 1, 3] → [2, 3, 1] → [3, 1, 2] → [3, 2, 1]

Nhận xét:
1. Khi mảng giảm dần hoàn toàn → đang ở max permutation
   Ví dụ: [3, 2, 1] → next = [1, 2, 3]

2. Để tìm next, cần tìm "pivot point" từ phải sang trái:
   - Pivot = phần tử đầu tiên (từ phải sang) mà nums[i] < nums[i+1]
   - Ví dụ: [1, 5, 8, 4, 7, 6, 5, 3, 1]
              ↑ pivot (4 < 7)
   
3. Sau khi tìm được pivot, cần:
   - Tìm số nhỏ nhất lớn hơn pivot ở bên phải
   - Swap với pivot
   - Reverse phần sau pivot để có permutation nhỏ nhất
```

**Algorithm:**
```
Step 1: Tìm pivot từ phải sang trái
  - i = n-2 downto 0
  - Tìm i sao cho nums[i] < nums[i+1]
  - Nếu không tìm thấy → reverse toàn bộ và return

Step 2: Tìm số để swap
  - j = n-1 downto i+1
  - Tìm j sao cho nums[j] > nums[i]
  
Step 3: Swap nums[i] và nums[j]

Step 4: Reverse đoạn [i+1, n-1]
```

**Complexity:**
- **Time:** O(n)
  - Scan để tìm pivot: O(n)
  - Scan để tìm swap position: O(n)
  - Reverse: O(n)
  - Total: O(n)
  
- **Space:** O(1)
  - Chỉ dùng vài biến: i, j
  - In-place modification

**Đánh giá:**
- ✅ Optimal time complexity
- ✅ Satisfy O(1) space requirement
- ✅ Clean và elegant
- ✅ Dễ implement
- ✅ Handle tất cả edge cases

---

### Comparison Table:

| Approach | Time | Space | Pros | Cons | Choose? |
|----------|------|-------|------|------|---------|
| **Brute Force (All perms)** | O(n! × n log n) | O(n!) | ✅ Straightforward concept | ❌ Extremely slow<br>❌ Violates space constraint | ❌ NO |
| **Mathematical Pattern** | O(n) | O(1) | ✅ Optimal<br>✅ Clean<br>✅ In-place | ⚠️ Need to understand pattern | ✅ **YES** |

**Decision:** Chọn Approach 2 - Mathematical Pattern

**Lý do:**
1. Đạt optimal complexity O(n) time, O(1) space
2. Satisfy yêu cầu in-place
3. Elegant solution dựa trên pattern toán học
4. Handle tất cả edge cases tự nhiên

---

## 5. VERIFY - Kiểm Chứng Trước Khi Code

### ✅ Pre-coding Checklist:

```
□ Algorithm đúng với ALL examples? → Testing below
□ Edge Cases handled?
  □ Single element → i không tìm thấy → reverse [1] → [1] ✓
  □ All same → i không tìm thấy → reverse ✓
  □ Max permutation → i không tìm thấy → reverse ✓
  □ Two elements → Works ✓
□ Complexity Analysis?
  □ Time: O(n) ✓
  □ Space: O(1) ✓
```

### 🔍 Trace Examples:

#### Example 1: `[1, 2, 3]` → `[1, 3, 2]`

```
Input: [1, 2, 3]
         0  1  2

Step 1: Tìm pivot (i)
  - i = 1: nums[1]=2 < nums[2]=3 ✓ FOUND
  - pivot = i = 1

Step 2: Tìm swap position (j)
  - j = 2: nums[2]=3 > nums[1]=2 ✓ FOUND
  - swap_pos = j = 2

Step 3: Swap nums[1] và nums[2]
  - Before: [1, 2, 3]
  - After:  [1, 3, 2]

Step 4: Reverse [i+1, n-1] = [2, 2]
  - Chỉ 1 phần tử → không đổi
  - Result: [1, 3, 2]

Output: [1, 3, 2] ✓✓✓ CORRECT!
```

#### Example 2: `[3, 2, 1]` → `[1, 2, 3]`

```
Input: [3, 2, 1]
        0  1  2

Step 1: Tìm pivot (i)
  - i = 1: nums[1]=2 >= nums[2]=1 ✗
  - i = 0: nums[0]=3 >= nums[1]=2 ✗
  - i = -1 → NOT FOUND
  
Step 2-3: Skip (no pivot)

Step 4: Reverse toàn bộ [0, n-1]
  - Before: [3, 2, 1]
  - After:  [1, 2, 3]

Output: [1, 2, 3] ✓✓✓ CORRECT!
```

#### Example 3: `[1, 1, 5]` → `[1, 5, 1]`

```
Input: [1, 1, 5]
        0  1  2

Step 1: Tìm pivot (i)
  - i = 1: nums[1]=1 < nums[2]=5 ✓ FOUND
  - pivot = i = 1

Step 2: Tìm swap position (j)
  - j = 2: nums[2]=5 > nums[1]=1 ✓ FOUND
  - swap_pos = j = 2

Step 3: Swap nums[1] và nums[2]
  - Before: [1, 1, 5]
  - After:  [1, 5, 1]

Step 4: Reverse [i+1, n-1] = [2, 2]
  - Chỉ 1 phần tử → không đổi
  - Result: [1, 5, 1]

Output: [1, 5, 1] ✓✓✓ CORRECT!
```

#### Example 4 (Complex): `[1, 5, 8, 4, 7, 6, 5, 3, 1]`

```
Input: [1, 5, 8, 4, 7, 6, 5, 3, 1]
        0  1  2  3  4  5  6  7  8

Step 1: Tìm pivot từ phải sang trái
  - i = 7: nums[7]=3 >= nums[8]=1 ✗
  - i = 6: nums[6]=5 >= nums[7]=3 ✗
  - i = 5: nums[5]=6 >= nums[6]=5 ✗
  - i = 4: nums[4]=7 >= nums[5]=6 ✗
  - i = 3: nums[3]=4 < nums[4]=7 ✓ FOUND
  - pivot = i = 3 (value = 4)

Step 2: Tìm số nhỏ nhất lớn hơn 4 từ phải
  - j = 8: nums[8]=1 <= 4 ✗
  - j = 7: nums[7]=3 <= 4 ✗
  - j = 6: nums[6]=5 > 4 ✓ FOUND
  - swap_pos = j = 6 (value = 5)

Step 3: Swap nums[3]=4 và nums[6]=5
  - Before: [1, 5, 8, 4, 7, 6, 5, 3, 1]
  - After:  [1, 5, 8, 5, 7, 6, 4, 3, 1]
                      ↑        ↑

Step 4: Reverse [4, 8]
  - Before: [1, 5, 8, 5, | 7, 6, 4, 3, 1]
  - Reverse:              [1, 3, 4, 6, 7]
  - After:  [1, 5, 8, 5, 1, 3, 4, 6, 7]

Output: [1, 5, 8, 5, 1, 3, 4, 6, 7] ✓✓✓ CORRECT!

Verify: 
  Original: 158,471,653,1
  Next:     158,513,467 > Original ✓
  And smallest possible next ✓
```

---

## 6. IMPLEMENT - Viết Code

### Rust Implementation:

```rust
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        
        // Step 1: Tìm pivot - phần tử đầu tiên từ phải sang mà nums[i] < nums[i+1]
        let mut i = n as i32 - 2;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }
        
        // Nếu không tìm thấy pivot → mảng đang giảm dần → reverse toàn bộ
        if i >= 0 {
            // Step 2: Tìm số nhỏ nhất lớn hơn nums[i] từ phải sang
            let mut j = n as i32 - 1;
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            
            // Step 3: Swap nums[i] và nums[j]
            nums.swap(i as usize, j as usize);
        }
        
        // Step 4: Reverse đoạn [i+1, n-1]
        let start = (i + 1) as usize;
        let end = n - 1;
        Self::reverse(nums, start, end);
    }
    
    // Helper function: Reverse đoạn [start, end]
    fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
        let mut left = start;
        let mut right = end;
        
        while left < right {
            nums.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

fn main() {
    // Test Example 1
    let mut test1 = vec![1, 2, 3];
    Solution::next_permutation(&mut test1);
    println!("Test 1: {:?} → Expected: [1, 3, 2]", test1);
    
    // Test Example 2
    let mut test2 = vec![3, 2, 1];
    Solution::next_permutation(&mut test2);
    println!("Test 2: {:?} → Expected: [1, 2, 3]", test2);
    
    // Test Example 3
    let mut test3 = vec![1, 1, 5];
    Solution::next_permutation(&mut test3);
    println!("Test 3: {:?} → Expected: [1, 5, 1]", test3);
    
    // Test Edge Case: Single element
    let mut test4 = vec![1];
    Solution::next_permutation(&mut test4);
    println!("Test 4: {:?} → Expected: [1]", test4);
    
    // Test Edge Case: Two elements
    let mut test5 = vec![1, 2];
    Solution::next_permutation(&mut test5);
    println!("Test 5: {:?} → Expected: [2, 1]", test5);
    
    // Test Edge Case: Two elements max
    let mut test6 = vec![2, 1];
    Solution::next_permutation(&mut test6);
    println!("Test 6: {:?} → Expected: [1, 2]", test6);
}
```

### Code Structure:
```
1. Main function: next_permutation()
   - Step 1: Tìm pivot
   - Step 2: Tìm swap position
   - Step 3: Swap
   - Step 4: Reverse

2. Helper function: reverse()
   - Two pointers để reverse in-place
   - Single responsibility: chỉ làm 1 việc
```

### Comments Strategy:
- ✅ Comment mỗi step của algorithm
- ✅ Giải thích WHY (tại sao cần tìm pivot, tại sao reverse)
- ✅ Comment edge case handling
- ❌ KHÔNG comment obvious (như "increment i")

---

## 7. TEST CASES

### Test Results:

```
✓ Example 1: [1,2,3] → [1,3,2]
✓ Example 2: [3,2,1] → [1,2,3]
✓ Example 3: [1,1,5] → [1,5,1]
✓ Edge: [1] → [1]
✓ Edge: [1,2] → [2,1]
✓ Edge: [2,1] → [1,2]
✓ Edge: [1,1,1] → [1,1,1]
✓ Complex: [1,5,8,4,7,6,5,3,1] → [1,5,8,5,1,3,4,6,7]
```

### Edge Cases Coverage:
```
□ ✅ Empty input - N/A (constraint: n >= 1)
□ ✅ Single element: [1] → [1]
□ ✅ All elements same: [1,1,1] → [1,1,1]
□ ✅ Max permutation: [3,2,1] → [1,2,3]
□ ✅ Two elements ascending: [1,2] → [2,1]
□ ✅ Two elements descending: [2,1] → [1,2]
□ ✅ Has duplicates: [1,1,5] → [1,5,1]
```

---

## 8. COMPLEXITY ANALYSIS

### Time Complexity: **O(n)**

**Breakdown:**
```
Step 1: Tìm pivot
  - Scan từ right to left: O(n) worst case
  
Step 2: Tìm swap position
  - Scan từ right to i: O(n) worst case
  
Step 3: Swap
  - O(1)
  
Step 4: Reverse
  - O(n) để reverse đoạn [i+1, n-1]

Total: O(n) + O(n) + O(1) + O(n) = O(n)
```

**Best case:** O(1)
- Khi pivot ngay ở vị trí cuối: [1,2] → [2,1]

**Worst case:** O(n)
- Khi pivot ở đầu: [1,4,3,2] → scan toàn bộ

**Average case:** O(n)

### Space Complexity: **O(1)**

**Breakdown:**
```
Variables used:
  - i: O(1)
  - j: O(1)
  - left, right in reverse: O(1)

Total: O(1) auxiliary space
```

✅ **In-place modification** - không cần extra array

---

## 9. KEY INSIGHTS

### 🎯 Core Insights:

1. **Mathematical Pattern Recognition**
   - Next permutation là permutation nhỏ nhất lớn hơn current
   - Cần thay đổi càng phải càng tốt (rightmost change)
   - Sau khi swap, phần còn lại phải tăng dần (nhỏ nhất có thể)

2. **Pivot Point là then chốt**
   - Pivot = điểm "breaking" của descending sequence
   - Từ phải sang trái, tìm nums[i] < nums[i+1]
   - Nếu không có pivot → đang ở max permutation → reverse

3. **Greedy Swap**
   - Swap pivot với số nhỏ nhất lớn hơn nó
   - Guarantee được next permutation gần nhất

4. **Reverse để minimize**
   - Sau khi swap, phần phải đang descending
   - Reverse để biến thành ascending = smallest possible

5. **In-place là khả thi**
   - Không cần generate permutations
   - Chỉ cần 3 operations: find, swap, reverse
   - Tất cả đều O(1) space

### 📚 Similar Patterns:
- Next Greater Element (stack pattern)
- Previous Permutation (reverse logic)
- Permutation Sequence

### 🔧 Implementation Tips:
- Dùng `i32` thay vì `usize` cho index để handle i = -1 case
- Helper function `reverse()` tách riêng cho clean code
- Two pointers cho reverse operation

### ⚠️ Common Mistakes:
- Quên handle case không có pivot (max permutation)
- Swap sai phần tử (phải là số nhỏ nhất lớn hơn pivot, không phải bất kỳ)
- Quên reverse sau khi swap

---

## 10. VARIATIONS & EXTENSIONS

### Related Problems:
1. **Previous Permutation** - Reverse logic
2. **Permutation Sequence** - Find kth permutation
3. **Next Greater Element III** - Apply to number
4. **Palindrome Permutation** - Check if exists

### Follow-up Questions:
1. **Q:** Tìm previous permutation?
   **A:** Đảo logic: tìm nums[i] > nums[i+1], swap với số lớn nhất nhỏ hơn

2. **Q:** Tìm kth next permutation?
   **A:** Call next_permutation() k lần, hoặc dùng math formula

3. **Q:** Count total permutations?
   **A:** n! / (count duplicates factorial)

---

## 11. SUMMARY

### Problem Type: Array Manipulation + Mathematical Pattern

### Solution Approach: Two Pointers + Greedy

### Key Algorithm Steps:
```
1. Find pivot (rightmost ascending pair)
2. Find smallest element > pivot
3. Swap
4. Reverse suffix
```

### Complexity:
- ⏰ Time: O(n)
- 💾 Space: O(1)

### Difficulty Rating:
- **Problem Understanding:** ⭐⭐⭐ (Medium - Need to understand permutation concept)
- **Pattern Recognition:** ⭐⭐⭐⭐ (Hard - Not obvious without knowing the trick)
- **Implementation:** ⭐⭐ (Easy - Once understand pattern)

### Tags:
`#Array` `#TwoPointers` `#Math` `#Permutation` `#Greedy` `#InPlace`

---

**Time to solve:** ~30-45 minutes (including thinking)

**Recommend practice order:**
1. Understand permutation concept
2. Trace examples manually
3. Recognize pattern
4. Implement step by step
5. Test with edge cases

**Good luck! 🚀**

