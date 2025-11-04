# Problem: Next Permutation

> **LeetCode #31** - Medium  
> **Link:** https://leetcode.com/problems/next-permutation/

---

## 1. UNDERSTAND - Understanding the Problem

### 📋 Checklist:

#### Input:
- **Data type:** Array of integers `nums`
- **Constraints:**
  - `1 <= nums.length <= 100`
  - `0 <= nums[i] <= 100`
- **Characteristics:**
  - May have duplicates
  - Not sorted
  - Cannot be empty (length >= 1)

#### Output:
- **Return type:** Void (modify in-place)
- **Format:** Modify array `nums` to become next permutation
- **Special:** If it's the largest permutation → convert to smallest

#### Exact Requirements:
```
□ Find next permutation in lexicographical order
□ Modify in-place (O(1) extra space)
□ If no next exists (at max) → reverse to min permutation
```

#### Edge Cases:
```
□ Single element: [1] → [1] (no change)
□ All elements same: [1,1,1] → [1,1,1] (no change)
□ Already max permutation: [3,2,1] → [1,2,3] (reverse)
□ Two elements: [1,2] → [2,1]
□ Next is simple swap: [1,2,3] → [1,3,2]
```

### 🎯 Rewritten Requirements:
**"Given an array of integers representing a permutation, find the next lexicographically greater permutation. If it doesn't exist (already at max), convert to the smallest permutation (sorted). Must modify in-place."**

---

## 2. ANALYZE - Constraint Analysis

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

Conclusion: Simple O(n) solution is sufficient and optimal
```

### Space Complexity Target:
```
Requirement: O(1) extra space (in-place modification)
→ Cannot create new array
→ Only use a few variables
```

---

## 3. PATTERN RECOGNITION - Pattern Recognition

### Keywords Analysis:

| Keyword | Pattern Hint |
|---------|---------------|
| **"Permutation"** | Mathematical pattern, ordering |
| **"Next"** | Sequential, find next in order |
| **"In-place"** | Two pointers, swap operations |
| **"Lexicographically"** | Dictionary order, compare from left |

### Pattern Identified:
✅ **Two Pointers** - For scanning and swapping  
✅ **Greedy** - Find optimal swap position  
✅ **Array Manipulation** - Reverse, swap

### Important Properties:
```
• Next permutation = smallest permutation greater than current
• Need minimal change (rightmost change)
• Remaining part must be as small as possible
```

### Similar Problems:
- Previous Permutation
- Permutation Sequence
- Next Greater Element

---

## 4. APPROACHES - Evaluating Approaches

### Approach 1: Generate All Permutations ❌

**Idea:** Generate all permutations, sort, find next

```
Time: O(n! × n log n) - Generate n! permutations, sort
Space: O(n!)
```

**Evaluation:**
- ❌ Extremely slow and memory intensive
- ❌ Doesn't satisfy O(1) space requirement
- ❌ Overkill for this problem

---

### Approach 2: Mathematical Pattern (Optimal) ✅

**Idea:** Discover the mathematical pattern of next permutation

**Key Observations:**
```
[1, 2, 3] → [1, 3, 2] → [2, 1, 3] → [2, 3, 1] → [3, 1, 2] → [3, 2, 1]

Notes:
1. When array is completely descending → at max permutation
   Example: [3, 2, 1] → next = [1, 2, 3]

2. To find next, need to find "pivot point" from right to left:
   - Pivot = first element (from right) where nums[i] < nums[i+1]
   - Example: [1, 5, 8, 4, 7, 6, 5, 3, 1]
              ↑ pivot (4 < 7)
   
3. After finding pivot, need to:
   - Find smallest number greater than pivot on the right
   - Swap with pivot
   - Reverse part after pivot to get smallest permutation
```

**Algorithm:**
```
Step 1: Find pivot from right to left
  - i = n-2 downto 0
  - Find i such that nums[i] < nums[i+1]
  - If not found → reverse entire array and return

Step 2: Find number to swap
  - j = n-1 downto i+1
  - Find j such that nums[j] > nums[i]
  
Step 3: Swap nums[i] and nums[j]

Step 4: Reverse segment [i+1, n-1]
```

**Complexity:**
- **Time:** O(n)
  - Scan to find pivot: O(n)
  - Scan to find swap position: O(n)
  - Reverse: O(n)
  - Total: O(n)
  
- **Space:** O(1)
  - Only use a few variables: i, j
  - In-place modification

**Evaluation:**
- ✅ Optimal time complexity
- ✅ Satisfies O(1) space requirement
- ✅ Clean and elegant
- ✅ Easy to implement
- ✅ Handles all edge cases

---

### Comparison Table:

| Approach | Time | Space | Pros | Cons | Choose? |
|----------|------|-------|------|------|---------|
| **Brute Force (All perms)** | O(n! × n log n) | O(n!) | ✅ Straightforward concept | ❌ Extremely slow<br>❌ Violates space constraint | ❌ NO |
| **Mathematical Pattern** | O(n) | O(1) | ✅ Optimal<br>✅ Clean<br>✅ In-place | ⚠️ Need to understand pattern | ✅ **YES** |

**Decision:** Choose Approach 2 - Mathematical Pattern

**Reasons:**
1. Achieves optimal complexity O(n) time, O(1) space
2. Satisfies in-place requirement
3. Elegant solution based on mathematical pattern
4. Handles all edge cases naturally

---

## 5. VERIFY - Verify Before Coding

### ✅ Pre-coding Checklist:

```
□ Algorithm correct for ALL examples? → Testing below
□ Edge Cases handled?
  □ Single element → i not found → reverse [1] → [1] ✓
  □ All same → i not found → reverse ✓
  □ Max permutation → i not found → reverse ✓
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

Step 1: Find pivot (i)
  - i = 1: nums[1]=2 < nums[2]=3 ✓ FOUND
  - pivot = i = 1

Step 2: Find swap position (j)
  - j = 2: nums[2]=3 > nums[1]=2 ✓ FOUND
  - swap_pos = j = 2

Step 3: Swap nums[1] and nums[2]
  - Before: [1, 2, 3]
  - After:  [1, 3, 2]

Step 4: Reverse [i+1, n-1] = [2, 2]
  - Only 1 element → no change
  - Result: [1, 3, 2]

Output: [1, 3, 2] ✓✓✓ CORRECT!
```

#### Example 2: `[3, 2, 1]` → `[1, 2, 3]`

```
Input: [3, 2, 1]
       0  1  2

Step 1: Find pivot (i)
  - i = 1: nums[1]=2 >= nums[2]=1 ✗
  - i = 0: nums[0]=3 >= nums[1]=2 ✗
  - i = -1 → NOT FOUND
  
Step 2-3: Skip (no pivot)

Step 4: Reverse entire [0, n-1]
  - Before: [3, 2, 1]
  - After:  [1, 2, 3]

Output: [1, 2, 3] ✓✓✓ CORRECT!
```

#### Example 3: `[1, 1, 5]` → `[1, 5, 1]`

```
Input: [1, 1, 5]
       0  1  2

Step 1: Find pivot (i)
  - i = 1: nums[1]=1 < nums[2]=5 ✓ FOUND
  - pivot = i = 1

Step 2: Find swap position (j)
  - j = 2: nums[2]=5 > nums[1]=1 ✓ FOUND
  - swap_pos = j = 2

Step 3: Swap nums[1] and nums[2]
  - Before: [1, 1, 5]
  - After:  [1, 5, 1]

Step 4: Reverse [i+1, n-1] = [2, 2]
  - Only 1 element → no change
  - Result: [1, 5, 1]

Output: [1, 5, 1] ✓✓✓ CORRECT!
```

#### Example 4 (Complex): `[1, 5, 8, 4, 7, 6, 5, 3, 1]`

```
Input: [1, 5, 8, 4, 7, 6, 5, 3, 1]
       0  1  2  3  4  5  6  7  8

Step 1: Find pivot from right to left
  - i = 7: nums[7]=3 >= nums[8]=1 ✗
  - i = 6: nums[6]=5 >= nums[7]=3 ✗
  - i = 5: nums[5]=6 >= nums[6]=5 ✗
  - i = 4: nums[4]=7 >= nums[5]=6 ✗
  - i = 3: nums[3]=4 < nums[4]=7 ✓ FOUND
  - pivot = i = 3 (value = 4)

Step 2: Find smallest number greater than 4 from right
  - j = 8: nums[8]=1 <= 4 ✗
  - j = 7: nums[7]=3 <= 4 ✗
  - j = 6: nums[6]=5 > 4 ✓ FOUND
  - swap_pos = j = 6 (value = 5)

Step 3: Swap nums[3]=4 and nums[6]=5
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

## 6. IMPLEMENT - Implementation

### Rust Implementation:

```rust
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        
        // Step 1: Find pivot - first element from right where nums[i] < nums[i+1]
        let mut i = n as i32 - 2;
        while i >= 0 && nums[i as usize] >= nums[(i + 1) as usize] {
            i -= 1;
        }
        
        // If no pivot found → array is descending → reverse entire array
        if i >= 0 {
            // Step 2: Find smallest number greater than nums[i] from right
            let mut j = n as i32 - 1;
            while nums[j as usize] <= nums[i as usize] {
                j -= 1;
            }
            
            // Step 3: Swap nums[i] and nums[j]
            nums.swap(i as usize, j as usize);
        }
        
        // Step 4: Reverse segment [i+1, n-1]
        let start = (i + 1) as usize;
        let end = n - 1;
        Self::reverse(nums, start, end);
    }
    
    // Helper function: Reverse segment [start, end]
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
   - Step 1: Find pivot
   - Step 2: Find swap position
   - Step 3: Swap
   - Step 4: Reverse

2. Helper function: reverse()
   - Two pointers for in-place reversal
   - Single responsibility: does one thing
```

### Comments Strategy:
- ✅ Comment each step of algorithm
- ✅ Explain WHY (why need pivot, why reverse)
- ✅ Comment edge case handling
- ❌ DON'T comment obvious things (like "increment i")

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
Step 1: Find pivot
  - Scan from right to left: O(n) worst case
  
Step 2: Find swap position
  - Scan from right to i: O(n) worst case
  
Step 3: Swap
  - O(1)
  
Step 4: Reverse
  - O(n) to reverse segment [i+1, n-1]

Total: O(n) + O(n) + O(1) + O(n) = O(n)
```

**Best case:** O(1)
- When pivot is at the end: [1,2] → [2,1]

**Worst case:** O(n)
- When pivot is at the beginning: [1,4,3,2] → scan entire array

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

✅ **In-place modification** - no extra array needed

---

## 9. KEY INSIGHTS

### 🎯 Core Insights:

1. **Mathematical Pattern Recognition**
   - Next permutation is the smallest permutation greater than current
   - Need to change as far right as possible (rightmost change)
   - After swap, remaining part must be ascending (smallest possible)

2. **Pivot Point is Key**
   - Pivot = "breaking" point of descending sequence
   - From right to left, find nums[i] < nums[i+1]
   - If no pivot → at max permutation → reverse

3. **Greedy Swap**
   - Swap pivot with smallest number greater than it
   - Guarantees closest next permutation

4. **Reverse to Minimize**
   - After swap, right part is descending
   - Reverse to make it ascending = smallest possible

5. **In-place is Feasible**
   - Don't need to generate permutations
   - Only need 3 operations: find, swap, reverse
   - All are O(1) space

### 📚 Similar Patterns:
- Next Greater Element (stack pattern)
- Previous Permutation (reverse logic)
- Permutation Sequence

### 🔧 Implementation Tips:
- Use `i32` instead of `usize` for index to handle i = -1 case
- Helper function `reverse()` separate for clean code
- Two pointers for reverse operation

### ⚠️ Common Mistakes:
- Forgetting to handle case with no pivot (max permutation)
- Swapping wrong element (must be smallest greater than pivot, not any)
- Forgetting to reverse after swap

---

## 10. VARIATIONS & EXTENSIONS

### Related Problems:
1. **Previous Permutation** - Reverse logic
2. **Permutation Sequence** - Find kth permutation
3. **Next Greater Element III** - Apply to number
4. **Palindrome Permutation** - Check if exists

### Follow-up Questions:
1. **Q:** Find previous permutation?
   **A:** Reverse logic: find nums[i] > nums[i+1], swap with largest smaller

2. **Q:** Find kth next permutation?
   **A:** Call next_permutation() k times, or use math formula

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

**Recommended practice order:**
1. Understand permutation concept
2. Trace examples manually
3. Recognize pattern
4. Implement step by step
5. Test with edge cases

**Good luck! 🚀**

