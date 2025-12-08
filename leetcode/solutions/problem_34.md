# Bài toán: Find First and Last Position of Element in Sorted Array

> LeetCode #34 — Medium  
> Link: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

---

## 1. HIỂU BÀI

- Input: `nums: Vec<i32>` là mảng tăng dần (non-decreasing), `target: i32`
- Output: trả về `[start, end]` là vị trí đầu tiên và cuối cùng của `target`, nếu không có trả `[-1, -1]`
- Constraints: `0 <= nums.len() <= 10^5`, `-10^9 <= nums[i] <= 10^9`, mảng không giảm (có thể trùng)
- Edge cases: mảng rỗng; target không tồn tại; target xuất hiện 1 lần; toàn bộ mảng là target

## 2. VÍ DỤ

- nums = [5,7,7,8,8,10], target = 8 → [3,4]
- nums = [5,7,7,8,8,10], target = 6 → [-1,-1]
- nums = [], target = 0 → [-1,-1]
- nums = [1], target = 1 → [0,0]
- nums = [2,2], target = 2 → [0,1]

## 3. NHẬN DẠNG MẪU

- Từ khóa: sorted array, find position, O(log n) → Binary Search
- Cấu trúc: mảng tăng dần có phần tử trùng lặp
- Kỹ thuật: **Modified Binary Search** - tìm biên trái (leftmost) và biên phải (rightmost)

## 4. CÁCH TIẾP CẬN

Thực hiện **2 lần Binary Search** độc lập:

### Tìm biên trái (First Position):
- Khi tìm thấy `target` tại `mid`: 
  - Lưu lại vị trí này
  - Tiếp tục tìm ở **nửa bên trái** (right = mid - 1)
  - Vì có thể còn target ở bên trái

### Tìm biên phải (Last Position):
- Khi tìm thấy `target` tại `mid`:
  - Lưu lại vị trí này  
  - Tiếp tục tìm ở **nửa bên phải** (left = mid + 1)
  - Vì có thể còn target ở bên phải

### So sánh với Binary Search thông thường:
- Binary search thông thường: tìm thấy → **dừng ngay**
- Bài này: tìm thấy → **tiếp tục tìm** để chắc chắn đó là biên

Độ phức tạp: Thời gian O(log n), bộ nhớ O(1)

## 5. THUẬT TOÁN (Giả mã)

```
function search_range(nums, target):
    if nums is empty: return [-1, -1]
    
    left_bound = find_left_bound(nums, target)
    right_bound = find_right_bound(nums, target)
    
    return [left_bound, right_bound]

function find_left_bound(nums, target):
    left = 0, right = n-1, result = -1
    
    while left <= right:
        mid = left + (right - left) / 2
        
        if nums[mid] == target:
            result = mid          # Lưu vị trí tìm thấy
            right = mid - 1       # Tiếp tục tìm bên trái
        else if nums[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return result

function find_right_bound(nums, target):
    left = 0, right = n-1, result = -1
    
    while left <= right:
        mid = left + (right - left) / 2
        
        if nums[mid] == target:
            result = mid          # Lưu vị trí tìm thấy
            left = mid + 1        # Tiếp tục tìm bên phải
        else if nums[mid] < target:
            left = mid + 1
        else:
            right = mid - 1
    
    return result
```

## 6. CODE (Rust)

```rust
struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }
        
        let left = Self::find_left_bound(&nums, target);
        let right = Self::find_right_bound(&nums, target);
        
        vec![left, right]
    }
    
    // Tìm vị trí đầu tiên (leftmost) của target
    fn find_left_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut result = -1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if nums[mid as usize] == target {
                result = mid;        // Lưu vị trí
                right = mid - 1;     // Tìm tiếp bên trái
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        result
    }
    
    // Tìm vị trí cuối cùng (rightmost) của target
    fn find_right_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut result = -1;
        
        while left <= right {
            let mid = left + (right - left) / 2;
            
            if nums[mid as usize] == target {
                result = mid;        // Lưu vị trí
                left = mid + 1;      // Tìm tiếp bên phải
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        
        result
    }
}
```

## 7. TEST

- [ ] [5,7,7,8,8,10], 8 → [3,4]
- [ ] [5,7,7,8,8,10], 6 → [-1,-1]
- [ ] [], 0 → [-1,-1]
- [ ] [1], 1 → [0,0]
- [ ] [2,2], 2 → [0,1]
- [ ] [1,2,3], 2 → [1,1] (chỉ xuất hiện 1 lần)
- [ ] [8,8,8,8,8], 8 → [0,4] (toàn bộ mảng)

## 8. ĐỘ PHỨC TẠP

- Thời gian: O(log n) — 2 lần binary search độc lập, mỗi lần O(log n)
- Bộ nhớ: O(1) — chỉ dùng biến cố định

## 9. Ý CHÍNH

- Khi cần tìm **biên/boundary** trong mảng sorted, nghĩ đến **Modified Binary Search**
- Khác với binary search thông thường (tìm thấy → dừng), ta **không dừng ngay** mà tiếp tục tìm để chắc chắn đó là biên
- Tìm biên trái: đi sang trái (right = mid - 1)
- Tìm biên phải: đi sang phải (left = mid + 1)

---

## 10. TRỰC GIÁC & VÌ SAO ĐÚNG

### Trực giác

- Trong mảng sorted có duplicate, target có thể xuất hiện nhiều lần liên tiếp tạo thành một "đoạn"
- Ta cần tìm 2 đầu mút của đoạn này
- Binary search thông thường chỉ cho ta **một vị trí bất kỳ** trong đoạn
- Modified binary search cho phép ta **"lùi về biên"** của đoạn

### Tại sao tiếp tục tìm sau khi tìm thấy?

**Ví dụ: [5,7,7,8,8,10], target = 8**

```
Nếu dừng ngay khi tìm thấy:
  mid=4 → nums[4]=8 → return 4
  ❌ Nhưng vị trí đầu tiên là 3!

Với modified binary search (tìm biên trái):
  mid=4 → nums[4]=8 → lưu result=4, tiếp tục tìm trái
  mid=3 → nums[3]=8 → lưu result=3, tiếp tục tìm trái
  mid=2 → nums[2]=7 < 8 → không còn target bên trái
  ✓ Trả về 3 (vị trí đầu tiên)
```

### Phác thảo tính đúng đắn

**Invariant (bất biến):** 
- Biến `result` luôn lưu vị trí **gần nhất với biên** mà ta đã tìm thấy

**Tìm biên trái:**
1. Mỗi lần tìm thấy target, lưu vị trí và **loại bỏ nửa phải** (vì biên trái chắc chắn ở bên trái hoặc tại đây)
2. Vòng lặp dừng khi `left > right`, khi đó `result` là vị trí trái nhất đã thấy
3. Nếu không tìm thấy target nào, `result` vẫn là -1

**Tìm biên phải:**
1. Mỗi lần tìm thấy target, lưu vị trí và **loại bỏ nửa trái** (vì biên phải chắc chắn ở bên phải hoặc tại đây)
2. Logic tương tự với tìm biên trái

### Tại sao không thể dùng linear search từ biên?

```
Linear scan: O(n) trong worst case [8,8,8,...,8]
Binary search: O(log n) luôn
```

---

## 11. TRUY VẾT CHI TIẾT (Step-by-step Trace)

### Ví dụ: `nums = [5,7,7,8,8,10], target = 8`

#### Tìm biên trái (find_left_bound):

```
Bước 0: nums = [5, 7, 7, 8, 8, 10]
        index   0  1  2  3  4   5
        left=0, right=5, result=-1

Bước 1: mid = 0 + (5-0)/2 = 2
        nums[2] = 7 < 8
        → đi phải: left = 3
        [5, 7, 7 | 8, 8, 10]
                   ^

Bước 2: left=3, right=5, mid=4
        nums[4] = 8 == 8 ✓
        → result = 4, tiếp tục tìm trái: right = 3
        [5, 7, 7, 8 | 8, 10]
                      ^
                   tìm thấy

Bước 3: left=3, right=3, mid=3
        nums[3] = 8 == 8 ✓
        → result = 3, tiếp tục tìm trái: right = 2
        [5, 7, 7 | 8, 8, 10]
                   ^
               tìm thấy, gần biên hơn!

Bước 4: left=3, right=2
        → left > right, dừng
        → return result = 3 ✓
```

#### Tìm biên phải (find_right_bound):

```
Bước 0: nums = [5, 7, 7, 8, 8, 10]
        index   0  1  2  3  4   5
        left=0, right=5, result=-1

Bước 1: mid = 2
        nums[2] = 7 < 8
        → đi phải: left = 3

Bước 2: left=3, right=5, mid=4
        nums[4] = 8 == 8 ✓
        → result = 4, tiếp tục tìm phải: left = 5
        [5, 7, 7, 8, 8 | 10]
                      ^
                   tìm thấy

Bước 3: left=5, right=5, mid=5
        nums[5] = 10 > 8
        → đi trái: right = 4

Bước 4: left=5, right=4
        → left > right, dừng
        → return result = 4 ✓

Kết quả cuối: [3, 4] ✓
```

---

## 12. BIẾN THỂ & CÁC CÁCH KHÁC

### Biến thể 1: Tìm số lượng target xuất hiện

```rust
impl Solution {
    pub fn count_target(nums: Vec<i32>, target: i32) -> i32 {
        let range = Self::search_range(nums, target);
        if range[0] == -1 {
            return 0;
        }
        range[1] - range[0] + 1
    }
}
```

### Biến thể 2: Tìm range của tất cả các phần tử

```rust
// Tìm first và last position của mỗi phần tử unique
// Output: HashMap<i32, (i32, i32)>
```

### Cách khác: Linear scan từ binary search result

```
1. Binary search tìm một vị trí bất kỳ của target: O(log n)
2. Scan trái để tìm biên trái: O(k)
3. Scan phải để tìm biên phải: O(k)

Time: O(log n + k), k là số lượng target
Worst case: O(n) khi toàn bộ mảng là target
→ Kém hơn cách 2 binary search: O(log n)
```

### Template: Lower bound & Upper bound

Nhiều library cung cấp:
- `lower_bound`: vị trí đầu tiên >= target
- `upper_bound`: vị trí đầu tiên > target

```rust
// Có thể dùng:
let left = lower_bound(nums, target);
let right = upper_bound(nums, target) - 1;
```

---

## 13. LỖI THƯỜNG GẶP

### 1. Dừng ngay khi tìm thấy target

```rust
// ❌ SAI
if nums[mid] == target {
    return mid;  // Không biết đây là biên hay không!
}
```

### 2. Không lưu result trước khi thu hẹp

```rust
// ❌ SAI
if nums[mid] == target {
    right = mid - 1;  // Quên lưu result!
}

// ✓ ĐÚNG
if nums[mid] == target {
    result = mid;      // Lưu trước
    right = mid - 1;   // Rồi mới thu hẹp
}
```

### 3. Nhầm lẫn điều kiện left/right

```rust
// Tìm biên TRÁI:
if nums[mid] == target {
    result = mid;
    right = mid - 1;  // Đi TRÁI ✓
}

// Tìm biên PHẢI:
if nums[mid] == target {
    result = mid;
    left = mid + 1;   // Đi PHẢI ✓
}
```

### 4. Không xử lý mảng rỗng

```rust
// ❌ SAI: Panic khi nums.len() = 0
let mut right = nums.len() - 1;  // Underflow!

// ✓ ĐÚNG
if nums.is_empty() {
    return vec![-1, -1];
}
let mut right = nums.len() as i32 - 1;
```

### 5. Sử dụng `<` thay vì `<=` trong while

```rust
// ❌ SAI: Bỏ sót trường hợp left == right
while left < right {
    // Khi left == right, có thể đó là vị trí cần tìm!
}

// ✓ ĐÚNG
while left <= right {
    // Xét hết tất cả vị trí có thể
}
```

---

## 14. NHỮNG ĐIỀU CẦN NHỚ

### Pattern Recognition

Khi thấy:
- ✅ Mảng sorted
- ✅ Tìm boundary/biên
- ✅ Yêu cầu O(log n)

→ Nghĩ ngay đến **Modified Binary Search**

### Core Concept

```
Standard Binary Search:     tìm thấy → DỪNG
Modified Binary Search:     tìm thấy → TIẾP TỤC (để tìm biên)
```

### Decision Tree

```
Tìm thấy target?
├─ YES
│  ├─ Tìm biên trái? → đi trái (right = mid - 1)
│  └─ Tìm biên phải? → đi phải (left = mid + 1)
└─ NO
   ├─ nums[mid] < target → đi phải
   └─ nums[mid] > target → đi trái
```

### Related Problems

- LeetCode 35: Search Insert Position (tìm vị trí insert)
- LeetCode 278: First Bad Version (tìm biên đầu tiên)
- LeetCode 374: Guess Number Higher or Lower
- LeetCode 704: Binary Search (cơ bản)

---

## 15. KẾT LUẬN

Bài toán này là **ví dụ điển hình** của Modified Binary Search pattern:
- Không phải tìm một phần tử đơn lẻ
- Mà tìm **boundary** của một range trong mảng sorted
- Kỹ thuật: **tìm thấy → không dừng → tiếp tục tìm về phía biên**

Thành thạo pattern này rất quan trọng vì:
- ✅ Xuất hiện nhiều trong interview
- ✅ Là nền tảng cho các bài binary search nâng cao
- ✅ Có thể apply cho nhiều bài toán tìm boundary khác

**Key takeaway:** Khi cần tìm biên trong sorted array → Modified Binary Search!

