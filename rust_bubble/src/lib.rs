#[no_mangle]
pub extern "C" fn example_function(mut arr: *mut f64, len: i32) -> i32 {
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
