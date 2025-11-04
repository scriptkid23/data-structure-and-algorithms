# 3318. Find X-Sum of All K-Long Subarrays I

## Tóm tắt bài toán

**Input:**
- Một mảng số nguyên `nums` có độ dài `n`
- Số nguyên `k`: độ dài của subarray cần xét
- Số nguyên `x`: số lượng phần tử có tần suất cao nhất cần giữ lại

**Output:**
- Mảng `answer` có độ dài `n - k + 1`
- Mỗi phần tử `answer[i]` là x-sum của subarray `nums[i..i+k-1]`

**Định nghĩa X-sum:**
1. Đếm tần suất xuất hiện của tất cả phần tử trong mảng
2. Chọn `x` phần tử có tần suất cao nhất theo quy tắc:
   - Ưu tiên phần tử có **tần suất cao hơn**
   - Nếu tần suất bằng nhau → ưu tiên phần tử có **giá trị lớn hơn**
3. Tính tổng: `sum(value × frequency)` của `x` phần tử được chọn
4. Đặc biệt: Nếu mảng có ít hơn `x` phần tử phân biệt → x-sum = tổng toàn bộ mảng

**Ví dụ:**
```
nums = [1,1,2,2,3,4,2,3], k = 6, x = 2

Subarray [1,1,2,2,3,4]:
  Frequency: {1:2, 2:2, 3:1, 4:1}
  Top 2: (2,2) và (1,2)  // cùng freq=2, nhưng 2 > 1
  X-sum = 2×2 + 1×2 = 6

Subarray [1,2,2,3,4,2]:
  Frequency: {1:1, 2:3, 3:1, 4:1}
  Top 2: (2,3) và (4,1)  // 2 có freq cao nhất, còn lại freq=1 chọn 4 vì 4>3>1
  X-sum = 2×3 + 4×1 = 10
```

---

## Hướng giải quyết

### Tại sao lại dùng thuật toán này?

#### 1. **Phân tích yêu cầu bài toán**

Bài toán yêu cầu:
- Xét **tất cả các subarray** liên tiếp có độ dài `k` → Gợi ý **Sliding Window**
- Với mỗi subarray, cần **đếm tần suất** → Gợi ý **HashMap/Frequency Counter**
- Chọn **top x phần tử** theo tiêu chí ưu tiên → Gợi ý **Sorting** hoặc **Heap**

#### 2. **Lý do chọn Sliding Window + HashMap + Sorting**

**Tại sao Sliding Window?**
- Có `n - k + 1` subarray cần xét, mỗi subarray có `k` phần tử
- Thay vì xử lý mỗi subarray độc lập O(k), ta có thể tối ưu bằng sliding window
- Tuy nhiên với constraint nhỏ (`n ≤ 50`), việc xử lý từng window độc lập vẫn hiệu quả

**Tại sao HashMap?**
- Cần đếm tần suất các phần tử → HashMap là cấu trúc tối ưu O(1) cho insert/lookup
- Dễ dàng lấy ra tất cả các cặp (value, frequency)

**Tại sao Sorting?**
- Cần chọn top `x` phần tử theo tiêu chí phức tạp (tần suất, rồi giá trị)
- Sorting cho phép định nghĩa custom comparator rõ ràng
- Với `k ≤ 50`, độ phức tạp O(k log k) hoàn toàn chấp nhận được
- Alternative: Có thể dùng Heap nhưng phức tạp hơn trong implementation

**Tại sao không dùng thuật toán khác?**
- ❌ **Two Pointers:** Không áp dụng được vì không có tính chất monotonic
- ❌ **Binary Search:** Không có mảng sắp xếp hay không gian tìm kiếm
- ❌ **Dynamic Programming:** Các subarray độc lập, không có optimal substructure
- ❌ **Greedy:** Không có tính chất greedy choice

#### 3. **Chiến lược tổng thể**

```
Mỗi Window → Count Frequency → Sort → Select Top X → Calculate Sum
     ↓              ↓               ↓          ↓              ↓
  O(k)          O(k)          O(k log k)    O(x)          O(x)
```

**Ưu điểm:**
- ✅ Straightforward, dễ implement
- ✅ Code rõ ràng, dễ debug
- ✅ Đủ hiệu quả cho constraint đã cho
- ✅ Không cần tối ưu phức tạp

**Trade-off:**
- Nếu `n` và `k` rất lớn (10⁵+), có thể cần tối ưu sliding window để reuse frequency map
- Với constraint hiện tại (`n ≤ 50`), simple solution là best solution

---

## Pseudocode

```
FUNCTION find_x_sum(nums, k, x):
    n ← length(nums)
    answer ← empty array
    
    // Duyệt qua tất cả các window có độ dài k
    FOR i FROM 0 TO (n - k):
        
        // Bước 1: Đếm tần suất các phần tử trong window hiện tại
        freq_map ← empty HashMap
        FOR j FROM i TO (i + k - 1):
            IF nums[j] NOT IN freq_map:
                freq_map[nums[j]] ← 0
            freq_map[nums[j]] ← freq_map[nums[j]] + 1
        
        // Bước 2: Chuyển HashMap thành mảng các cặp (value, frequency)
        elements ← empty array
        FOR EACH (value, count) IN freq_map:
            elements.append((value, count))
        
        // Bước 3: Sắp xếp theo tiêu chí
        // - Ưu tiên 1: frequency giảm dần (count lớn hơn)
        // - Ưu tiên 2: value giảm dần (nếu count bằng nhau)
        SORT elements BY:
            PRIMARY: count (descending)
            SECONDARY: value (descending)
        
        // Bước 4: Lấy top x phần tử và tính tổng
        x_sum ← 0
        top_x ← min(x, length(elements))
        
        FOR i FROM 0 TO (top_x - 1):
            (value, count) ← elements[i]
            x_sum ← x_sum + (value × count)
        
        // Thêm kết quả vào mảng đáp án
        answer.append(x_sum)
    
    RETURN answer
END FUNCTION
```

### Chi tiết implementation:

**Custom Comparator cho Sorting:**
```
FUNCTION compare(a, b):
    // a và b là các tuple (value, count)
    (value_a, count_a) ← a
    (value_b, count_b) ← b
    
    // So sánh frequency trước
    IF count_a ≠ count_b:
        RETURN count_a > count_b  // Descending order
    
    // Nếu frequency bằng nhau, so sánh value
    RETURN value_a > value_b  // Descending order
END FUNCTION
```

### Trace Example:

```
Input: nums = [1,1,2,2,3,4,2,3], k = 6, x = 2

i = 0: Window [1,1,2,2,3,4]
  ├─ freq_map = {1:2, 2:2, 3:1, 4:1}
  ├─ elements = [(1,2), (2,2), (3,1), (4,1)]
  ├─ after sort = [(2,2), (1,2), (4,1), (3,1)]
  │   • (2,2) vs (1,2): count same, 2 > 1 ✓
  │   • (4,1) vs (3,1): count same, 4 > 3 ✓
  ├─ top 2 = [(2,2), (1,2)]
  └─ x_sum = 2×2 + 1×2 = 6

i = 1: Window [1,2,2,3,4,2]
  ├─ freq_map = {1:1, 2:3, 3:1, 4:1}
  ├─ elements = [(1,1), (2,3), (3,1), (4,1)]
  ├─ after sort = [(2,3), (4,1), (3,1), (1,1)]
  │   • (2,3): count highest ✓
  │   • (4,1) vs (3,1) vs (1,1): count same, 4 > 3 > 1 ✓
  ├─ top 2 = [(2,3), (4,1)]
  └─ x_sum = 2×3 + 4×1 = 10

i = 2: Window [2,2,3,4,2,3]
  ├─ freq_map = {2:3, 3:2, 4:1}
  ├─ elements = [(2,3), (3,2), (4,1)]
  ├─ after sort = [(2,3), (3,2), (4,1)]
  ├─ top 2 = [(2,3), (3,2)]
  └─ x_sum = 2×3 + 3×2 = 12

Output: [6, 10, 12] ✓
```

---

## Độ phức tạp

**Time Complexity:**
- Có `n - k + 1` windows
- Mỗi window:
  - Count frequency: O(k)
  - Convert to array: O(k) trong worst case (k phần tử phân biệt)
  - Sort: O(k log k)
  - Calculate sum: O(x) ≤ O(k)
- Tổng: **O((n-k+1) × k log k)**
- Với constraint: O(50 × 50 × log 50) ≈ O(14,000) operations - rất nhanh

**Space Complexity:**
- HashMap: O(k) trong worst case
- Elements array: O(k)
- Answer array: O(n-k+1)
- Tổng: **O(n + k)**

---

## Key Takeaways

1. **Đọc kỹ yêu cầu:** "Top x most frequent, tie-break by value" - quan trọng cho comparator
2. **Constraint-driven:** Với n ≤ 50, simple solution thường là best solution
3. **Clear over Clever:** Code rõ ràng quan trọng hơn tối ưu hóa sớm
4. **Test với examples:** Trace từng bước để đảm bảo logic đúng

