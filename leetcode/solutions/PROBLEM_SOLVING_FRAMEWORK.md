# 🧠 Framework Tư Duy Giải Quyết Bài Toán Algorithm

> **"Think first, code later"** - Một thiết kế tốt tiết kiệm 80% thời gian debug

---

## 📚 Mục Lục

1. [Quy Trình 6 Bước](#quy-trình-6-bước)
2. [Pattern Recognition Cheatsheet](#pattern-recognition-cheatsheet)
3. [Data Structure Selection Guide](#data-structure-selection-guide)
4. [Complexity Analysis](#complexity-analysis)
5. [Common Pitfalls](#common-pitfalls)
6. [Practice Workflow](#practice-workflow)

---

## 🎯 Quy Trình 6 Bước

### **BƯỚC 1: UNDERSTAND - Hiểu Bài Toán (5 phút)**

#### Checklist:

```
□ Input là gì?
  - Kiểu dữ liệu (array, string, tree, graph, etc.)
  - Range/Constraints của input
  - Có sorted không? Có duplicate không?
  - Có thể empty không?

□ Output là gì?
  - Return type
  - Format cụ thể
  - Single value hay array/list?

□ Yêu cầu chính xác là gì?
  - Đọc CHẬM và KỸ từng câu
  - Highlight các từ khóa quan trọng
  - Viết lại bằng lời của mình

□ Edge Cases?
  - Empty input
  - Single element
  - All elements same
  - Min/Max values
  - Negative numbers
```

#### Kỹ thuật đọc đề:

**Cách đọc ĐÚNG:**
```
1. Đọc lần 1: Hiểu tổng quan
2. Đọc lần 2: Highlight keywords
3. Đọc lần 3: Focus vào định nghĩa phức tạp
4. Viết lại yêu cầu bằng lời mình
5. List ra tất cả assumptions
```

**Ví dụ thực tế:**

```
Problem: "Find the x-sum of all k-long subarrays"

❌ Đọc lướt: "Tìm tổng của subarray độ dài k"
✅ Đọc kỹ: 
   - "x-sum" là gì? → Đọc definition riêng
   - "all k-long subarrays" → n-k+1 subarrays
   - Top x "most frequent" → Không phải largest!
   - Tie-break: "bigger value" → Cần custom sort
   - Sum = value × frequency → Không phải count!
```

---

### **BƯỚC 2: ANALYZE - Phân Tích Constraints (3 phút)**

#### Constraint → Time Complexity:

| n size | Max Complexity | Algorithms |
|--------|----------------|------------|
| n ≤ 10 | O(n!) | Permutation, Brute Force All |
| n ≤ 20 | O(2ⁿ) | Bitmask DP, Subset Generation |
| n ≤ 50 | O(n⁴) | DP with 4 dimensions |
| n ≤ 100 | O(n³) | Floyd-Warshall, 3-nested loops |
| n ≤ 500 | O(n³) | Careful 3-nested loops |
| n ≤ 1,000 | O(n²) | DP with 2D, All Pairs |
| n ≤ 10,000 | O(n² / 2) | Careful O(n²) with optimization |
| n ≤ 100,000 | O(n log n) | Sorting, Heap, Divide & Conquer |
| n ≤ 1,000,000 | O(n) or O(n log n) | Linear, Hash, Efficient Sorting |
| n ≤ 10,000,000 | O(n) | Hash, Two Pointers, Linear Scan |

#### Rule of Thumb:

```
• 10⁸ operations per second
• 1 second time limit → ~10⁸ operations
• 2 seconds → ~2×10⁸ operations
```

#### Phân tích thực tế:

**Ví dụ 1: n ≤ 50**
```
Constraint: n ≤ 50, k ≤ 50

Tính toán:
- O(n³) = 50³ = 125,000 ✅ OK
- O(n² log n) = 50² × 6 = 15,000 ✅ Very fast
- O(n²) = 2,500 ✅ Excellent

→ Kết luận: Simple solution đủ, không cần over-optimize
```

**Ví dụ 2: n ≤ 10⁵**
```
Constraint: n ≤ 100,000

Tính toán:
- O(n²) = 10¹⁰ ❌ TLE (Too Large)
- O(n log n) = 10⁵ × 17 ≈ 1.7×10⁶ ✅ OK
- O(n) = 10⁵ ✅ Best

→ Kết luận: Cần thuật toán hiệu quả, O(n) hoặc O(n log n)
```

---

### **BƯỚC 3: PATTERN RECOGNITION - Nhận Dạng Mẫu (5 phút)**

#### A. Nhận dạng từ KEYWORDS:

| Keyword | Gợi ý Pattern | Algorithms |
|---------|---------------|------------|
| **Subarray/Substring** | Sliding Window | Two Pointers, Sliding Window |
| "Contiguous" + "sum/max" | Prefix Sum | Kadane's, Prefix Sum + Hash |
| "All subarrays" | Iteration/DP | Brute Force, DP |
| **"Top K / Kth largest"** | Selection | Heap, QuickSelect, Sorting |
| **"Frequency/Count"** | Counting | HashMap, Counter Array |
| **Sorted Array** | Binary Search | Binary Search, Two Pointers |
| **"Shortest path"** | Graph | BFS, Dijkstra, Bellman-Ford |
| **"All paths"** | Graph Traversal | DFS, Backtracking |
| **"Minimum/Maximum steps"** | BFS/DP | BFS (unweighted), DP |
| **"Optimize choice"** | Greedy/DP | Greedy, Dynamic Programming |
| **"Permutation/Combination"** | Generate | Backtracking, Itertools |
| **"Palindrome"** | Two Pointers/DP | Expand Around Center, DP |
| **"Parentheses/Brackets"** | Stack | Stack |
| **"Tree traversal"** | DFS/BFS | Recursion, Stack, Queue |
| **"Islands/Connected"** | Union-Find/DFS | DFS, BFS, Union-Find |

#### B. Nhận dạng từ DATA STRUCTURE:

```
Input Type                  → Typical Approach
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📊 Array/List
  ├─ Sorted                 → Binary Search, Two Pointers
  ├─ Unsorted + Need order  → Sorting first
  ├─ Need frequency         → HashMap/Counter
  └─ Subarray problems      → Sliding Window, Prefix Sum

📝 String
  ├─ Palindrome             → Two Pointers, DP
  ├─ Substring              → Sliding Window
  ├─ Pattern matching       → KMP, Rabin-Karp
  └─ Anagram                → Counter/HashMap

🌳 Tree
  ├─ Binary Search Tree     → In-order traversal
  ├─ Path problems          → DFS
  ├─ Level problems         → BFS
  └─ Lowest Common Ancestor → Binary Lifting, Tarjan

🕸️ Graph
  ├─ Shortest path          → BFS (unweighted), Dijkstra
  ├─ All pairs shortest     → Floyd-Warshall
  ├─ Connected components   → DFS, BFS, Union-Find
  ├─ Cycle detection        → DFS with colors
  └─ Topological sort       → Kahn's, DFS

🔗 Linked List
  ├─ Fast & Slow pointers   → Cycle detection, Middle
  ├─ Reverse                → Three pointers
  └─ Merge                  → Two pointers

⚡ Matrix/Grid
  ├─ Path finding           → BFS, DFS
  ├─ Islands                → DFS, BFS, Union-Find
  └─ DP problems            → 2D DP
```

#### C. Nhận dạng từ PROPERTIES:

```
Property                    → Technique
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✓ Monotonic property        → Two Pointers, Mono Stack/Queue
✓ Optimal substructure      → Dynamic Programming
✓ Greedy choice property    → Greedy Algorithm
✓ Overlapping subproblems   → DP, Memoization
✓ No dependencies           → Parallel/Independent processing
✓ Small search space        → Brute Force
✓ Binary decision           → Binary Search on Answer
✓ Fixed window              → Sliding Window (fixed size)
✓ Variable window           → Sliding Window (variable size)
```

---

### **BƯỚC 4: EVALUATE APPROACHES - Đánh Giá Các Cách Tiếp Cận (5 phút)**

#### Framework đánh giá:

```
Cho mỗi approach, tự hỏi:

1. ✅ CORRECTNESS (Quan trọng nhất!)
   □ Đúng với tất cả examples?
   □ Handle edge cases?
   □ Logic sound?

2. ⚡ EFFICIENCY
   □ Time complexity pass constraints?
   □ Space complexity acceptable?
   □ Có bottleneck nào?

3. 🔧 COMPLEXITY (Code complexity)
   □ Dễ implement?
   □ Dễ debug?
   □ Dễ giải thích?

4. 🎯 TRADE-OFFS
   □ Time vs Space?
   □ Simplicity vs Performance?
   □ Edge case handling?
```

#### Template so sánh:

**Ví dụ: Problem 3318**

| Approach | Time | Space | Pros | Cons | Choose? |
|----------|------|-------|------|------|---------|
| **Brute force each window** | O(n·k·k log k) | O(k) | ✅ Simple<br>✅ Clear<br>✅ Easy debug | ⚠️ Not optimal | ✅ YES (n≤50) |
| **Sliding window optimized** | O(n·k) | O(k) | ✅ Fast | ❌ Complex<br>❌ Need 2 heaps<br>❌ Hard to debug | ❌ NO (overkill) |
| **Heap for top K** | O(n·k log x) | O(k+x) | ✅ Medium speed | ⚠️ Medium complex | ⚠️ Maybe |
| **Sort every time** | O(n·k log k) | O(k) | ✅ Clean comparator<br>✅ Easy to understand | ⚠️ Repeated sorting | ✅ YES |

**Decision:** Chọn approach 1 hoặc 4 (tương đương về complexity, nhưng sort dễ hơn)

#### Quy tắc vàng:

```
1. Correct > Fast
   → Thuật toán đúng chậm vẫn tốt hơn thuật toán sai nhanh

2. Simple > Clever (nếu pass constraints)
   → Constraint nhỏ → Simple solution thường đủ
   → Easy to debug, maintain, explain

3. Optimize khi cần thiết
   → Premature optimization is the root of all evil
   → Profile first, optimize later
```

---

### **BƯỚC 5: VERIFY - Kiểm Chứng Trước Khi Code (3 phút)**

#### Pre-coding Checklist:

```
□ Algorithm đúng với ALL examples?
  - Example 1: ✓
  - Example 2: ✓
  - Example 3 (nếu có): ✓

□ Edge Cases?
  □ Empty input
  □ Single element
  □ All elements same
  □ Min values (0, negative)
  □ Max values (overflow?)
  □ Special: x > distinct elements

□ Complexity Analysis?
  □ Time: ______ pass? ✓/✗
  □ Space: ______ pass? ✓/✗

□ Trace 1 example chi tiết?
  - Step by step
  - Write down intermediate values
  - Verify logic at each step
```

#### Kỹ thuật Trace:

**Ví dụ chi tiết:**

```
Problem: Find x-sum of [1,1,2,2,3,4], k=6, x=2

Manual Trace:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Step 1: Count frequency
  Input: [1, 1, 2, 2, 3, 4]
  Process: 
    - 1 appears: 2 times
    - 2 appears: 2 times  
    - 3 appears: 1 time
    - 4 appears: 1 time
  Result: {1:2, 2:2, 3:1, 4:1} ✓

Step 2: Convert to array
  Input: {1:2, 2:2, 3:1, 4:1}
  Process: Create pairs (value, count)
  Result: [(1,2), (2,2), (3,1), (4,1)] ✓

Step 3: Sort with custom comparator
  Input: [(1,2), (2,2), (3,1), (4,1)]
  Rules: 
    1. Higher count first
    2. If count same, higher value first
  Process:
    - (2,2) vs (1,2): count same, 2 > 1 → (2,2) first
    - (4,1) vs (3,1): count same, 4 > 3 → (4,1) first
  Result: [(2,2), (1,2), (4,1), (3,1)] ✓

Step 4: Take top x=2
  Input: [(2,2), (1,2), (4,1), (3,1)]
  Process: Take first 2 elements
  Result: [(2,2), (1,2)] ✓

Step 5: Calculate sum
  Input: [(2,2), (1,2)]
  Process: sum(value × count)
    - 2 × 2 = 4
    - 1 × 2 = 2
    - Total = 4 + 2 = 6
  Result: 6 ✓

Expected: 6 ✓✓✓ CORRECT!
```

**Lợi ích của trace:**
- Phát hiện logic error sớm
- Hiểu rõ từng bước
- Tự tin khi code
- Dễ debug sau này

---

### **BƯỚC 6: IMPLEMENT - Viết Code (10-20 phút)**

#### Code Structure Best Practices:

```rust
// Template structure

// 1. Main function với logic rõ ràng
impl Solution {
    pub fn main_function(input: Type) -> Output {
        // Step 1: Preprocessing (nếu cần)
        let preprocessed = preprocess(input);
        
        // Step 2: Main loop/logic
        let mut result = Vec::new();
        for item in preprocessed {
            let step_result = process_item(item);
            result.push(step_result);
        }
        
        // Step 3: Postprocessing (nếu cần)
        postprocess(result)
    }
    
    // 2. Helper functions - mỗi function làm 1 việc
    fn process_item(item: Type) -> Output {
        // Single responsibility
        // Easy to test separately
        // Clear purpose
    }
}

// 3. Main function để test
fn main() {
    // Test với examples
    let test1 = vec![...];
    println!("{:?}", Solution::main_function(test1));
    
    // Test edge cases
    let test_empty = vec![];
    println!("{:?}", Solution::main_function(test_empty));
}
```

#### Comment Strategy:

```rust
// ❌ BAD: Comment the obvious
let x = 5; // set x to 5

// ✅ GOOD: Comment the WHY
// Use 5 because we need to skip the header rows
let skip_rows = 5;

// ✅ GOOD: Comment complex logic
// Sort by frequency (desc), then by value (desc)
// This ensures we get the most frequent elements first,
// and break ties by choosing larger values
elements.sort_by(|a, b| {
    b.1.cmp(&a.1).then(b.0.cmp(&a.0))
});

// ✅ GOOD: Comment non-obvious decisions
// We use HashMap instead of array because values can be up to 10^9
let mut freq = HashMap::new();
```

#### Testing Strategy:

```
1. Test với examples từ đề bài
   → Should pass 100%

2. Test edge cases
   □ Empty
   □ Single element
   □ All same
   □ Min/max values

3. Test corner cases
   □ x > số phần tử distinct
   □ k = n (một subarray duy nhất)
   □ x = k (lấy tất cả)

4. Test với data tự tạo
   → Biết expected output
   → Verify từng bước
```

---

## 🎨 Pattern Recognition Cheatsheet

### 1. **Array/Subarray Problems**

#### A. Sliding Window

**Khi nào dùng:**
```
✓ "Subarray/substring of size k"
✓ "Longest/shortest subarray with condition"
✓ "All subarrays" với optimization
```

**Fixed Size Window:**
```python
def sliding_window_fixed(arr, k):
    result = []
    # Initialize first window
    window_sum = sum(arr[:k])
    result.append(window_sum)
    
    # Slide window
    for i in range(k, len(arr)):
        window_sum = window_sum - arr[i-k] + arr[i]
        result.append(window_sum)
    
    return result
```

**Variable Size Window:**
```python
def sliding_window_variable(arr, target):
    left = 0
    current_sum = 0
    min_length = float('inf')
    
    for right in range(len(arr)):
        current_sum += arr[right]
        
        # Shrink window while condition met
        while current_sum >= target:
            min_length = min(min_length, right - left + 1)
            current_sum -= arr[left]
            left += 1
    
    return min_length
```

**Examples:**
- Maximum Sum Subarray of Size K
- Longest Substring Without Repeating Characters
- Minimum Window Substring
- Sliding Window Maximum

---

#### B. Two Pointers

**Khi nào dùng:**
```
✓ Sorted array/string
✓ Có tính chất monotonic
✓ Pair/triplet problems
✓ Reverse/rearrange
```

**Pattern 1: Opposite Directions**
```python
def two_sum_sorted(arr, target):
    left, right = 0, len(arr) - 1
    
    while left < right:
        current_sum = arr[left] + arr[right]
        
        if current_sum == target:
            return [left, right]
        elif current_sum < target:
            left += 1  # Need larger sum
        else:
            right -= 1  # Need smaller sum
    
    return []
```

**Pattern 2: Same Direction**
```python
def remove_duplicates(arr):
    if not arr:
        return 0
    
    slow = 0
    for fast in range(1, len(arr)):
        if arr[fast] != arr[slow]:
            slow += 1
            arr[slow] = arr[fast]
    
    return slow + 1
```

**Examples:**
- Two Sum (sorted)
- Container With Most Water
- Trapping Rain Water
- Remove Duplicates from Sorted Array

---

#### C. Prefix Sum

**Khi nào dùng:**
```
✓ "Subarray sum equals K"
✓ "Range sum queries"
✓ Need cumulative information
```

**Template:**
```python
def subarray_sum(arr, k):
    prefix_sum = 0
    sum_count = {0: 1}  # {sum: count}
    result = 0
    
    for num in arr:
        prefix_sum += num
        
        # Check if (prefix_sum - k) exists
        if prefix_sum - k in sum_count:
            result += sum_count[prefix_sum - k]
        
        # Update count
        sum_count[prefix_sum] = sum_count.get(prefix_sum, 0) + 1
    
    return result
```

**Examples:**
- Subarray Sum Equals K
- Contiguous Array (0s and 1s)
- Range Sum Query

---

### 2. **Sorting & Searching**

#### A. Binary Search

**Khi nào dùng:**
```
✓ Sorted array
✓ "Find/Search" in sorted data
✓ "Minimum/Maximum value that satisfies condition"
✓ Binary search on answer
```

**Template 1: Classic Binary Search**
```python
def binary_search(arr, target):
    left, right = 0, len(arr) - 1
    
    while left <= right:
        mid = left + (right - left) // 2
        
        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return -1
```

**Template 2: Binary Search on Answer**
```python
def min_value_satisfying_condition(arr):
    def is_valid(value):
        # Check if this value satisfies the condition
        pass
    
    left, right = min_possible, max_possible
    result = -1
    
    while left <= right:
        mid = left + (right - left) // 2
        
        if is_valid(mid):
            result = mid
            right = mid - 1  # Try smaller
        else:
            left = mid + 1
    
    return result
```

**Examples:**
- Binary Search
- Search in Rotated Sorted Array
- Koko Eating Bananas
- Minimum in Rotated Sorted Array

---

#### B. Custom Sorting

**Khi nào dùng:**
```
✓ "Top K" với complex criteria
✓ Need specific ordering
✓ Multiple sort keys
```

**Template:**
```rust
// Rust example
elements.sort_by(|a, b| {
    // Primary criterion
    let primary = b.frequency.cmp(&a.frequency);
    
    if primary == Ordering::Equal {
        // Secondary criterion (tie-break)
        b.value.cmp(&a.value)
    } else {
        primary
    }
});
```

```python
# Python example
elements.sort(key=lambda x: (-x[1], -x[0]))
# Sort by frequency desc, then value desc
```

---

### 3. **Hash Table / HashMap**

**Khi nào dùng:**
```
✓ "Count frequency"
✓ "Find duplicates"
✓ "Two sum" variations
✓ Need O(1) lookup
```

**Pattern 1: Frequency Counter**
```python
from collections import Counter

def top_k_frequent(nums, k):
    # Count frequency
    freq = Counter(nums)
    
    # Get top k
    return [num for num, count in freq.most_common(k)]
```

**Pattern 2: Seen/Visited Tracking**
```python
def contains_duplicate(nums):
    seen = set()
    
    for num in nums:
        if num in seen:
            return True
        seen.add(num)
    
    return False
```

**Pattern 3: Complement Search**
```python
def two_sum(nums, target):
    seen = {}  # {value: index}
    
    for i, num in enumerate(nums):
        complement = target - num
        if complement in seen:
            return [seen[complement], i]
        seen[num] = i
    
    return []
```

---

### 4. **Stack & Queue**

#### A. Stack

**Khi nào dùng:**
```
✓ "Valid parentheses"
✓ "Next greater/smaller element"
✓ "Monotonic" problems
✓ DFS, backtracking
```

**Pattern 1: Matching Pairs**
```python
def is_valid_parentheses(s):
    stack = []
    pairs = {'(': ')', '[': ']', '{': '}'}
    
    for char in s:
        if char in pairs:  # Opening bracket
            stack.append(char)
        else:  # Closing bracket
            if not stack or pairs[stack.pop()] != char:
                return False
    
    return len(stack) == 0
```

**Pattern 2: Monotonic Stack**
```python
def next_greater_element(nums):
    stack = []  # Store indices
    result = [-1] * len(nums)
    
    for i in range(len(nums)):
        # While current is greater than stack top
        while stack and nums[i] > nums[stack[-1]]:
            idx = stack.pop()
            result[idx] = nums[i]
        
        stack.append(i)
    
    return result
```

---

#### B. Queue & Deque

**Khi nào dùng:**
```
✓ BFS
✓ Sliding window maximum/minimum
✓ FIFO processing
```

**Pattern: Sliding Window Maximum**
```python
from collections import deque

def sliding_window_maximum(nums, k):
    dq = deque()  # Store indices
    result = []
    
    for i in range(len(nums)):
        # Remove elements outside window
        while dq and dq[0] < i - k + 1:
            dq.popleft()
        
        # Maintain decreasing order
        while dq and nums[i] > nums[dq[-1]]:
            dq.pop()
        
        dq.append(i)
        
        # Add to result after first window
        if i >= k - 1:
            result.append(nums[dq[0]])
    
    return result
```

---

### 5. **Heap / Priority Queue**

**Khi nào dùng:**
```
✓ "Top K" elements
✓ "Kth largest/smallest"
✓ Merge K sorted lists
✓ Median finding
```

**Pattern: Top K Elements**
```python
import heapq

def top_k_frequent(nums, k):
    # Count frequency
    freq = {}
    for num in nums:
        freq[num] = freq.get(num, 0) + 1
    
    # Use min heap of size k
    heap = []
    for num, count in freq.items():
        heapq.heappush(heap, (count, num))
        if len(heap) > k:
            heapq.heappop(heap)
    
    return [num for count, num in heap]
```

**Pattern: Merge K Sorted Lists**
```python
def merge_k_sorted(lists):
    heap = []
    
    # Initialize heap with first element of each list
    for i, lst in enumerate(lists):
        if lst:
            heapq.heappush(heap, (lst[0], i, 0))
    
    result = []
    while heap:
        val, list_idx, elem_idx = heapq.heappop(heap)
        result.append(val)
        
        # Add next element from same list
        if elem_idx + 1 < len(lists[list_idx]):
            next_val = lists[list_idx][elem_idx + 1]
            heapq.heappush(heap, (next_val, list_idx, elem_idx + 1))
    
    return result
```

---

### 6. **Dynamic Programming**

**Khi nào dùng:**
```
✓ "Maximum/Minimum"
✓ "Count number of ways"
✓ Overlapping subproblems
✓ Optimal substructure
```

**Nhận diện DP:**
```
1. Có thể chia thành subproblems?
2. Subproblems lặp lại?
3. Optimal solution từ optimal subsolutions?
4. Có recurrence relation?
```

**Template:**
```python
def dp_problem(input):
    # 1. Define DP state
    # dp[i] = meaning
    
    # 2. Initialize base cases
    dp = [0] * (n + 1)
    dp[0] = base_value
    
    # 3. Fill DP table
    for i in range(1, n + 1):
        # 4. Recurrence relation
        dp[i] = function_of(dp[i-1], dp[i-2], ...)
    
    # 5. Return answer
    return dp[n]
```

**Common Patterns:**

**1D DP:**
```python
# Fibonacci-style
dp[i] = dp[i-1] + dp[i-2]

# House Robber-style
dp[i] = max(dp[i-1], dp[i-2] + nums[i])

# Climbing Stairs-style
dp[i] = dp[i-1] + dp[i-2]
```

**2D DP:**
```python
# Grid path
dp[i][j] = dp[i-1][j] + dp[i][j-1]

# Longest Common Subsequence
if s1[i] == s2[j]:
    dp[i][j] = dp[i-1][j-1] + 1
else:
    dp[i][j] = max(dp[i-1][j], dp[i][j-1])

# Knapsack
dp[i][w] = max(dp[i-1][w], dp[i-1][w-weight[i]] + value[i])
```

---

### 7. **Graph Algorithms**

#### A. BFS (Breadth-First Search)

**Khi nào dùng:**
```
✓ Shortest path (unweighted)
✓ Level-order traversal
✓ "Minimum steps/moves"
✓ Connected components
```

**Template:**
```python
from collections import deque

def bfs(graph, start):
    visited = set([start])
    queue = deque([start])
    
    while queue:
        node = queue.popleft()
        
        # Process node
        process(node)
        
        # Visit neighbors
        for neighbor in graph[node]:
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)
```

---

#### B. DFS (Depth-First Search)

**Khi nào dùng:**
```
✓ Path finding (all paths)
✓ Cycle detection
✓ Topological sort
✓ Connected components
✓ Backtracking
```

**Template (Recursive):**
```python
def dfs(graph, node, visited):
    visited.add(node)
    
    # Process node
    process(node)
    
    # Visit neighbors
    for neighbor in graph[node]:
        if neighbor not in visited:
            dfs(graph, neighbor, visited)
```

**Template (Iterative):**
```python
def dfs_iterative(graph, start):
    visited = set()
    stack = [start]
    
    while stack:
        node = stack.pop()
        
        if node in visited:
            continue
        
        visited.add(node)
        process(node)
        
        for neighbor in graph[node]:
            if neighbor not in visited:
                stack.append(neighbor)
```

---

## 🎯 Data Structure Selection Guide

### Quick Reference:

| Need | Use | Time | Space |
|------|-----|------|-------|
| **Fast lookup** | HashMap/HashSet | O(1) | O(n) |
| **Maintain order + fast lookup** | OrderedDict (Python) / BTreeMap (Rust) | O(log n) | O(n) |
| **Top K elements** | Heap | O(log k) | O(k) |
| **Range queries** | Segment Tree, Fenwick Tree | O(log n) | O(n) |
| **FIFO** | Queue | O(1) | O(n) |
| **LIFO** | Stack | O(1) | O(n) |
| **Both ends access** | Deque | O(1) | O(n) |
| **Sorted + updates** | TreeSet/TreeMap | O(log n) | O(n) |
| **Union/Find** | Union-Find (DSU) | O(α(n)) ≈ O(1) | O(n) |
| **Prefix/Suffix ops** | Trie | O(m) | O(n×m) |

---

## 📊 Complexity Analysis

### Common Complexities (Best to Worst):

```
O(1)        < O(log n)   < O(n)      < O(n log n) < O(n²)    < O(2ⁿ)   < O(n!)
Constant      Logarithmic   Linear      Linearithmic  Quadratic   Exponential Factorial

Examples:
O(1)        → Hash lookup, array access
O(log n)    → Binary search, balanced tree
O(n)        → Linear scan, single loop
O(n log n)  → Merge sort, heap sort
O(n²)       → Nested loops, bubble sort
O(2ⁿ)       → Generate all subsets
O(n!)       → Generate all permutations
```

### Space Complexity:

```
O(1)        → Few variables, in-place modification
O(log n)    → Recursion depth (binary search, balanced tree)
O(n)        → Array, hash table, single recursion
O(n²)       → 2D matrix, graph adjacency matrix
```

---

## 🚫 Common Pitfalls

### 1. **Không đọc kỹ đề**

❌ **Sai lầm:**
```
"Find top x most frequent elements"
→ Nghĩ: "Tìm x số lớn nhất"
```

✅ **Đúng:**
```
→ Đọc kỹ: "most FREQUENT" = xuất hiện nhiều nhất
→ Không phải giá trị lớn nhất!
```

---

### 2. **Bỏ qua edge cases**

❌ **Sai lầm:**
```rust
fn divide(a: i32, b: i32) -> i32 {
    a / b  // Crash when b = 0!
}
```

✅ **Đúng:**
```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
```

**Edge cases checklist:**
```
□ Empty input
□ Single element
□ All elements same
□ Division by zero
□ Integer overflow
□ Negative numbers
□ null/None values
```

---

### 3. **Off-by-one errors**

❌ **Sai lầm:**
```python
# Want: nums[i..i+k-1] (k elements)
for i in range(len(nums) - k):  # Missing last window!
    window = nums[i:i+k]
```

✅ **Đúng:**
```python
# Include last window: i can go up to (n-k)
for i in range(len(nums) - k + 1):
    window = nums[i:i+k]
```

---

### 4. **Integer overflow**

❌ **Sai lầm:**
```rust
let sum: i32 = nums.iter().sum();  // Overflow nếu sum > 2^31-1
```

✅ **Đúng:**
```rust
let sum: i64 = nums.iter().map(|&x| x as i64).sum();
// Hoặc check overflow
```

---

### 5. **Mutating while iterating**

❌ **Sai lầm:**
```python
for i in range(len(arr)):
    if condition:
        arr.pop(i)  # Indices shift! Bug!
```

✅ **Đúng:**
```python
# Option 1: Iterate backwards
for i in range(len(arr) - 1, -1, -1):
    if condition:
        arr.pop(i)

# Option 2: Create new list
arr = [x for x in arr if not condition]
```

---

### 6. **Premature optimization**

❌ **Sai lầm:**
```
"n = 50, tôi phải tối ưu xuống O(n log n)!"
→ Spend 2 hours coding complex solution
```

✅ **Đúng:**
```
"n = 50, O(n³) = 125k operations = OK"
→ Simple O(n²) solution in 15 minutes
→ Passes all tests
→ Easy to debug
```

---

## 🎓 Practice Workflow

### Phase 1: Learning (Tuần 1-4)

**Mục tiêu:** Học patterns và techniques

```
1. Chọn một topic (e.g., Sliding Window)
2. Học theory và template
3. Làm 5-10 bài cùng pattern
4. Tự implement lại từ đầu
5. Viết notes/summary
```

**Topics để học theo thứ tự:**
```
Week 1: Array & String basics
  - Two Pointers
  - Sliding Window
  - Prefix Sum

Week 2: Hash & Sort
  - HashMap/HashSet
  - Sorting techniques
  - Binary Search

Week 3: Stack & Queue
  - Stack applications
  - Queue/Deque
  - Monotonic Stack

Week 4: Basic Graph & Tree
  - BFS
  - DFS
  - Binary Tree traversal
```

---

### Phase 2: Practice (Tuần 5-12)

**Mục tiêu:** Apply patterns to new problems

```
1. Làm bài KHÔNG nhìn solution trước
2. Áp dụng 6-step framework
3. Implement solution
4. Test với examples
5. Xem solution để học cách khác
6. Viết down key insights
```

**Daily routine:**
```
□ 1-2 easy problems (warm up)
□ 1-2 medium problems (main practice)
□ 0-1 hard problem (stretch goal)
□ Review previous mistakes
```

---

### Phase 3: Mastery (Tuần 13+)

**Mục tiêu:** Speed + accuracy

```
1. Timed practice (30-45 min per problem)
2. Mock interviews
3. Explain solution out loud
4. Teach others
5. Contribute to discussions
```

**Weekly goals:**
```
□ 15-20 problems solved
□ Review 5 previous hard problems
□ Deep dive 1 advanced topic
□ Teach/explain 3 solutions
```

---

## 📝 Problem-Solving Template

**Copy template này mỗi khi làm bài mới:**

```markdown
# Problem: [Name]

## 1. Understanding
- Input: 
- Output: 
- Constraints: 
- Edge cases: 

## 2. Examples
Example 1:
  Input: 
  Output: 
  Trace:

## 3. Pattern Recognition
- Keywords spotted: 
- Data structure: 
- Algorithm pattern: 
- Similar problems: 

## 4. Approaches

### Approach 1: [Name]
- Time: 
- Space: 
- Pros: 
- Cons: 

### Approach 2: [Name]
- Time: 
- Space: 
- Pros: 
- Cons: 

**Decision:** Choose Approach X because...

## 5. Algorithm
```
Pseudocode here
```

## 6. Implementation
```rust
Code here
```

## 7. Test Cases
- [ ] Example 1
- [ ] Example 2
- [ ] Edge case 1
- [ ] Edge case 2

## 8. Complexity Analysis
- Time: O(?) because...
- Space: O(?) because...

## 9. Key Insights
- 
- 
- 
```

---

## 🎯 Final Checklist

**Before coding ANY problem:**

```
□ Read problem 3 times
□ Understand input/output format
□ List all edge cases
□ Check constraints → determine complexity
□ Recognize patterns
□ List 2-3 possible approaches
□ Choose best approach for the constraint
□ Trace 1 example manually
□ Verify logic is correct
□ NOW start coding!
```

---

## 🚀 Remember

> **"Hours of debugging can save minutes of planning"**

- Think first, code later
- Simple > clever
- Correct > fast
- Test early, test often
- Learn from mistakes
- Practice deliberately

**Good luck! 🎉**

