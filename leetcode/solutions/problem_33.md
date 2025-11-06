# Bài toán: Search in Rotated Sorted Array

> LeetCode #33 — Medium  
> Link: https://leetcode.com/problems/search-in-rotated-sorted-array/

---

## 1. HIỂU BÀI

- Input: `nums: Vec<i32>` là mảng tăng dần đã bị rotate (các phần tử distinct), `target: i32`
- Output: trả về index của `target` trong `nums`, nếu không có trả `-1`
- Constraints: `1 <= nums.len() <= 5000`, phần tử phân biệt, có thể không rotate
- Edge cases: kích thước 1; không rotate; target ở biên; target không tồn tại

## 2. VÍ DỤ

- nums = [4,5,6,7,0,1,2], target = 0 → 4
- nums = [4,5,6,7,0,1,2], target = 3 → -1
- nums = [1], target = 0 → -1

## 3. NHẬN DẠNG MẪU

- Từ khóa: sorted, rotated, search → Binary Search có điều chỉnh
- Cấu trúc: mảng
- Kỹ thuật: binary search, mỗi bước quyết định nửa trái/phải dựa trên tính sorted

## 4. CÁCH TIẾP CẬN

Binary search với logic nhận biết nửa nào đang sorted ở mỗi vòng lặp:

- Tại `mid`, hoặc [left..mid] sorted hoặc [mid..right] sorted
- Nếu nửa trái sorted, kiểm tra target có thuộc [left..mid] không; nếu có → đi trái, ngược lại → đi phải
- Nếu nửa phải sorted, kiểm tra target có thuộc [mid..right] không; nếu có → đi phải, ngược lại → đi trái

Độ phức tạp: Thời gian O(log n), bộ nhớ O(1)

## 5. THUẬT TOÁN (Giả mã)

```
left = 0, right = n-1
while left <= right:
  mid = (left+right)//2
  if nums[mid] == target: return mid
  if nums[left] <= nums[mid]:  # nửa trái sorted
    if nums[left] <= target < nums[mid]: right = mid-1
    else: left = mid+1
  else:  # nửa phải sorted
    if nums[mid] < target <= nums[right]: left = mid+1
    else: right = mid-1
return -1
```

## 6. CODE (Rust)

```rust
struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() { return -1; }
        let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);

        while left <= right {
            let mid = left + (right - left) / 2;
            let mid_val = nums[mid as usize];
            if mid_val == target { return mid; }

            if nums[left as usize] <= mid_val {
                // Nửa trái sorted
                if nums[left as usize] <= target && target < mid_val {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                // Nửa phải sorted
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

## 7. TEST

- [ ] [4,5,6,7,0,1,2], 0 → 4
- [ ] [4,5,6,7,0,1,2], 3 → -1
- [ ] [1], 0 → -1
- [ ] [1,3], 3 → 1
- [ ] [5,1,3], 5 → 0

## 8. ĐỘ PHỨC TẠP

- Thời gian: O(log n) — quyết định nửa mảng mỗi bước
- Bộ nhớ: O(1)

## 9. Ý CHÍNH

- Trong mảng tăng bị rotate, luôn tồn tại ít nhất một nửa quanh `mid` là sorted.
- So sánh phạm vi để chọn đúng nửa cần tìm tiếp, giữ O(log n).


---

## 10. TRỰC GIÁC & VÌ SAO ĐÚNG

### Trực giác
- Mảng gốc là tăng dần; khi rotate, chỉ “nếp gấp” tại pivot (vị trí nhỏ nhất) làm hỏng tính tăng toàn cục. Tuy nhiên, quanh một `mid` bất kỳ, ít nhất một phía vẫn còn nguyên tính tăng chuẩn.
- Ta luôn chọn phía tăng chuẩn để kiểm tra phạm vi bao chứa `target`. Nếu `target` không thể nằm trong phạm vi tăng chuẩn này, loại nó an toàn và chuyển sang nửa còn lại.

### Phác thảo tính đúng đắn
- Bất biến: khoảng `[left, right]` luôn chứa nghiệm nếu tồn tại.
- Mỗi vòng lặp: xác định nửa tăng chuẩn. Nếu `target` không thuộc phạm vi nửa tăng chuẩn (đã biết giới hạn chính xác vì tăng dần), loại bỏ được nửa đó mà không mất nghiệm. Nếu thuộc, giữ lại nửa đó. Do mỗi bước loại ít nhất một nửa, thuật toán dừng sau O(log n) bước.

### Biên và so sánh
- Dùng `<=` ở biên trái và `<=` ở biên phải theo đúng logic để không bỏ lỡ `target` ở biên.
- Vì các phần tử distinct, không có trường hợp mơ hồ khi so sánh.

---

## 11. TRUY VẾT CHI TIẾT (Step-by-step Trace)

Ví dụ: `nums = [4,5,6,7,0,1,2], target = 0`

1) `left=0 (4)`, `right=6 (2)`, `mid=3 (7)`
   - `nums[left]=4 <= 7=nums[mid]` → nửa trái `[4,5,6,7]` tăng chuẩn
   - `target=0` không thuộc `[4..7]` → bỏ trái, `left=mid+1=4`

2) `left=4 (0)`, `right=6 (2)`, `mid=5 (1)`
   - `nums[left]=0 <= 1=nums[mid]` → nửa trái `[0,1]` tăng chuẩn
   - `target=0` thuộc `[0..1)` → `right=mid-1=4`

3) `left=4`, `right=4`, `mid=4` → `nums[mid]=0` → trả `4`.

---

## 12. BIẾN THỂ: TÌM PIVOT RỒI NHỊ PHÂN

### Ý tưởng
- Bước 1: Tìm `pivot` (chỉ số phần tử nhỏ nhất) bằng binary search O(log n).
- Bước 2: Quyết định `target` nằm ở nửa nào: `[0..pivot-1]` hay `[pivot..n-1]`.
- Bước 3: Binary search chuẩn trong nửa tương ứng.

### Đánh giá
- Cũng O(log n) thời gian, O(1) bộ nhớ. Code dài hơn; cách “nhận biết nửa tăng chuẩn” gộp thành một vòng lặp thường gọn hơn.

---

## 13. LỖI THƯỜNG GẶP

- Dùng sai dấu `<`/`<=` ở biên khiến bỏ lỡ target ở đầu/cuối nửa mảng.
- Không xác định đúng nửa tăng chuẩn trước khi kiểm tra phạm vi.
- Lỡ tay so sánh với chỉ số sai (trộn `left/right` sau khi cập nhật).
- Bỏ qua trường hợp mảng kích thước 1 hoặc không rotate (logic vẫn hoạt động, nhưng test nên bao phủ).


