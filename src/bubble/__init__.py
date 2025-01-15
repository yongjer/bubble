import timeit
from typing import Sequence
import ctypes

# 載入.dylib文件
lib = ctypes.CDLL("lib/librust_bubble.dylib")
"""
pub extern "C" fn bubble_sort(arr: *mut f64, len: i32) -> i32 {
    if arr.is_null() || len <= 0 {
        return -1;
    }

    // 將指針轉換為 Rust 切片
    let slice = unsafe { std::slice::from_raw_parts_mut(arr, len as usize) };
    
    // 執行氣泡排序
    let n = slice.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - i - 1 {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    
    0 // 返回0表示成功
}
"""

def bubble_sort_by_python3(arr: Sequence[float]) -> list[float]:
    arr = list(arr)  # 轉換為列表
    n = len(arr)
    for i in range(n):
        swapped = False
        for j in range(0, n - i - 1):
            if arr[j] > arr[j + 1]:
                arr[j], arr[j + 1] = arr[j + 1], arr[j]
                swapped = True
        if not swapped:
            break
    return arr

# TimeIt testing
if __name__ == "__main__":
    arr = [64, 34, 25, 12, 22, 11, 90]

    # python3
    execution_time = timeit.timeit(lambda: bubble_sort_by_python3([float(x) for x in arr.copy()]), number=1000000)
    print(f"執行時間: {execution_time} 秒")

    # rust
    # 將列表轉換為指針
    arr = (ctypes.c_double * len(arr))(*arr)
    execution_time = timeit.timeit(lambda: lib.bubble_sort(arr, len(arr)), number=1000000)
    print(f"執行時間: {execution_time} 秒")
