# Problem: Search in Rotated Sorted Array

> LeetCode #33 — Medium  
> Link: https://leetcode.com/problems/search-in-rotated-sorted-array/

---

## 1. UNDERSTAND

- Input: `nums: Vec<i32>` rotated ascending array with distinct values, `target: i32`
- Output: index of `target` in `nums`, or `-1` if not found
- Constraints: `1 <= nums.len() <= 5000`, values are distinct, rotation done once (possibly 0)
- Edge cases: size 1; not rotated; target at boundaries; target absent

## 2. EXAMPLES

- nums = [4,5,6,7,0,1,2], target = 0 → 4
- nums = [4,5,6,7,0,1,2], target = 3 → -1
- nums = [1], target = 0 → -1

## 3. PATTERN RECOGNITION

- Keywords: sorted, rotated, search → Binary Search with rotation logic
- Data structure: array
- Technique: modified binary search maintaining O(log n)

## 4. APPROACH

Binary search while identifying which half is sorted each step:

- At mid, either [left..mid] is sorted or [mid..right] is sorted
- If left half is sorted, check if target in [left..mid]; else search right
- If right half is sorted, check if target in [mid..right]; else search left

Complexity: Time O(log n), Space O(1)

## 5. ALGORITHM (Pseudocode)

```
left = 0, right = n-1
while left <= right:
  mid = (left+right)//2
  if nums[mid] == target: return mid
  if nums[left] <= nums[mid]:  # left half sorted
    if nums[left] <= target < nums[mid]: right = mid-1
    else: left = mid+1
  else:  # right half sorted
    if nums[mid] < target <= nums[right]: left = mid+1
    else: right = mid-1
return -1
```

## 6. IMPLEMENTATION (Rust)

```rust
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];
            if mid_val == target {
                return mid;
            }

            // Determine which half is sorted
            if nums[left as usize] <= mid_val {
                // Left half [left..mid] is sorted
                if nums[left as usize] <= target && target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // Right half [mid..right] is sorted
                if mid_val < target && target <= nums[right as usize] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }
        -1
    }
}
```

## 7. TEST CASES

- [ ] [4,5,6,7,0,1,2], 0 → 4
- [ ] [4,5,6,7,0,1,2], 3 → -1
- [ ] [1], 0 → -1
- [ ] [1,3], 3 → 1
- [ ] [5,1,3], 5 → 0

## 8. COMPLEXITY

- Time: O(log n) via binary search decisions per step
- Space: O(1) constant extra variables

## 9. KEY INSIGHTS

- In a rotated sorted array with distinct values, at least one side of `mid` is sorted.
- Choose the half to continue by checking whether `target` falls inside the sorted half's range.


---

## 10. INTUITION & CORRECTNESS

### Intuition
- The original array is globally ascending; rotation only introduces a single "fold" at the pivot (the minimum). Around any `mid`, at least one side remains strictly sorted.
- Always inspect the sorted side: if `target` cannot lie inside that side’s precise range, safely discard it and continue with the other half.

### Correctness sketch
- Invariant: the active interval `[left, right]` always contains the answer if it exists.
- Each iteration identifies a sorted half. Because the half is sorted, we know exact bounds; if `target` is outside, we discard that half without losing a potential answer. Each step removes at least half the interval → O(log n) steps.

### Boundary and comparisons
- Use `<=` on left boundary checks and `<=` on right boundary checks exactly as written to not miss boundary targets.
- Distinct values guarantee no ambiguity in comparisons.

---

## 11. DETAILED TRACE (Step-by-step)

Example: `nums = [4,5,6,7,0,1,2], target = 0`

1) `left=0 (4)`, `right=6 (2)`, `mid=3 (7)`
   - `nums[left]=4 <= 7=nums[mid]` → left half `[4,5,6,7]` is sorted
   - `target=0` not in `[4..7]` → move right: `left=mid+1=4`

2) `left=4 (0)`, `right=6 (2)`, `mid=5 (1)`
   - `nums[left]=0 <= 1=nums[mid]` → left half `[0,1]` is sorted
   - `target=0` in `[0..1)` → `right=mid-1=4`

3) `left=4`, `right=4`, `mid=4` → `nums[mid]=0` → return `4`.

---

## 12. VARIANT: FIND PIVOT THEN BINARY SEARCH

### Idea
- Step 1: Find the pivot index (minimum element) via binary search in O(log n).
- Step 2: Decide whether `target` lies in `[0..pivot-1]` or `[pivot..n-1]`.
- Step 3: Run classic binary search in that half.

### Assessment
- Also O(log n) time and O(1) space. Slightly more verbose; the “sorted half decision” approach is often more concise in a single loop.

---

## 13. COMMON PITFALLS

- Using incorrect `<`/`<=` at boundaries, which misses targets at edges.
- Checking target range before correctly determining which half is sorted.
- Accidentally comparing with stale `left/right` after updates.
- Not testing size-1 or non-rotated cases (logic still works; tests should cover them).


