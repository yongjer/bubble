/// Sorts an array of 64-bit floating-point numbers in ascending order using the bubble sort algorithm.
///
/// This function takes a mutable pointer to the first element of an array and the number of elements in the array as input.
/// It sorts the array in place and returns 0 if successful, or -1 if the input is invalid (e.g., null pointer or non-positive length).
///
/// # Arguments
///
/// * `arr` - A mutable pointer to the first element of the array to be sorted.
/// * `len` - The number of elements in the array.
///
/// # Returns
///
/// * `0` - If the array was sorted successfully.
/// * `-1` - If the input is invalid (e.g., null pointer or non-positive length).
///
/// # Safety
///
/// This function uses unsafe code to convert the raw pointer to a mutable slice.
/// It is the caller's responsibility to ensure that the pointer is valid and that the memory pointed to is properly allocated.
///
/// # Examples
///
/// ```
/// use std::ffi::c_double;
/// use std::ffi::c_int;
/// use std::mem::size_of;
/// use std::os::raw::c_void;
/// use std::ptr;
/// use std::slice;
///
///
/// extern "C" {
///     fn bubble_sort(arr: *mut c_double, len: c_int) -> c_int;
/// }
///
/// let mut data = [4.0, 2.0, 5.0, 1.0, 3.0];
/// let len = data.len() as i32;
///
/// unsafe {
///     let result = bubble_sort(data.as_mut_ptr(), len);
///     assert_eq!(result, 0);
/// }
///
/// assert_eq!(data, [1.0, 2.0, 3.0, 4.0, 5.0]);
/// ```
///
/// # Python Usage (using ctypes)
///
/// ```python
/// import ctypes
///
/// # Load the shared library
/// lib = ctypes.CDLL("./your_library.so")  # Replace with the actual path to your library
///
/// # Define the argument and return types for the function
/// lib.bubble_sort.argtypes = [ctypes.POINTER(ctypes.c_double), ctypes.c_int]
/// lib.bubble_sort.restype = ctypes.c_int
///
/// # Create a Python list and convert it to a C array
/// data = [4.0, 2.0, 5.0, 1.0, 3.0]
/// array_type = ctypes.c_double * len(data)
/// c_array = array_type(*data)
///
/// # Call the Rust function
/// result = lib.bubble_sort(c_array, len(data))
///
/// # Check the result
/// if result == 0:
///     # Convert the C array back to a Python list
///     sorted_data = list(c_array)
///     print("Sorted array:", sorted_data)
/// else:
///     print("Error during sorting")
/// ```
#[no_mangle]
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