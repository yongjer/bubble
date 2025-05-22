# Project Title: Rust Bubble Sort with Python Bindings

## Overview

This project implements the bubble sort algorithm in Rust and provides Python bindings for it using the `ctypes` library. It also includes a native Python implementation of bubble sort.

The primary purpose of this project is to:
- Demonstrate how to create Rust functions that can be called from Python (Foreign Function Interface - FFI).
- Compare the performance of the Rust implementation versus the native Python implementation for the bubble sort algorithm.

## Rust Component

The core bubble sort algorithm is implemented in Rust, located in the `rust_bubble` directory.

-   **Source Code:** `rust_bubble/src/lib.rs`
-   **Function:** `bubble_sort(arr: *mut f64, len: i32) -> i32`
    -   This function sorts an array of `f64` (double-precision floating-point numbers) in place.
    -   It's exposed as a C-compatible function for use by Python.

### Building the Rust Library

To compile the Rust library:

1.  Navigate to the Rust directory:
    ```bash
    cd rust_bubble
    ```
2.  Build the project (for a release version, which is optimized):
    ```bash
    cargo build --release
    ```
3.  After a successful build, the compiled library will be located at `rust_bubble/target/release/librust_bubble.dylib` (on macOS), `rust_bubble/target/release/librust_bubble.so` (on Linux), or `rust_bubble/target/release/rust_bubble.dll` (on Windows).
4.  This library needs to be copied to the `lib/` directory in the project root for the Python script to find it. For example, on macOS:
    ```bash
    cp rust_bubble/target/release/librust_bubble.dylib ../lib/
    ```
    Adjust the command based on your operating system and the exact library name. The Python script currently expects `lib/librust_bubble.dylib`. If you are on a different OS, you might need to adjust the library name in `src/bubble/__init__.py` or rename the compiled file to `librust_bubble.dylib` in the `lib` directory.

## Python Component

The Python part of the project is located in `src/bubble/__init__.py`.

-   **Loading the Rust Library:** The script uses Python's `ctypes` module to load the compiled Rust shared library (e.g., `lib/librust_bubble.dylib`).
    ```python
    import ctypes
    lib = ctypes.CDLL("lib/librust_bubble.dylib")
    ```
-   **Defining Argument and Return Types:** It then defines the argument types (`argtypes`) and return type (`restype`) for the `bubble_sort` function to ensure correct data marshalling between Python and Rust.
    ```python
    # Assuming 'lib' is the loaded library from ctypes
    lib.bubble_sort.argtypes = [ctypes.POINTER(ctypes.c_double), ctypes.c_int]
    lib.bubble_sort.restype = ctypes.c_int
    ```
-   **Native Python Implementation:** The script also includes a native Python implementation of bubble sort, `bubble_sort_by_python3(arr: Sequence[float]) -> list[float]`, for comparison purposes.
-   **Calling the Rust function:** To call the Rust function, a Python list of floats is converted into a C-compatible array of doubles, and then passed to the `lib.bubble_sort` function.
    ```python
    # Example:
    data = [4.0, 2.0, 5.0, 1.0, 3.0]
    array_type = ctypes.c_double * len(data)
    c_array = array_type(*data)

    result = lib.bubble_sort(c_array, len(data))

    if result == 0:
        sorted_data_from_rust = list(c_array)
        print(f"Sorted array (Rust): {sorted_data_from_rust}")
    else:
        print("Error during Rust sorting.")
    ```

## Running the Benchmark

The `src/bubble/__init__.py` script contains a benchmark that uses the `timeit` module to compare the execution speed of the Rust-based bubble sort and the native Python bubble sort.

To run the benchmark:

1.  Ensure you have built the Rust library and copied it to the `lib/` directory as described in the "Rust Component" section.
2.  From the project root directory, execute the Python script as a module:
    ```bash
    python -m src.bubble
    ```
3.  **Output:** The script will print the total execution time for a large number of sorts (currently 100,000 iterations as per the script) for both the Python version and the Rust version. This allows for a direct performance comparison.

    Example (the exact times will vary based on your hardware):
    ```
    執行時間: X.XXXXXX 秒  // Python version
    執行時間: Y.YYYYYY 秒  // Rust version
    ```
    (Note: "執行時間" is Chinese for "execution time", as seen in the current script output.)

## Prerequisites

To build and run this project, you will need:

-   **Rust:**
    -   Installed from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
    -   This includes `rustc` (the Rust compiler) and `cargo` (the Rust package manager and build tool).
-   **Python 3:**
    -   Installed from [https://www.python.org/downloads/](https://www.python.org/downloads/)
    -   The `ctypes` module is part of the standard library, so no additional Python packages are strictly required to run the core functionality.
    -   The benchmark uses `timeit`, which is also part of the standard library.

## Project Structure

```
.
├── README.md               # This file
├── lib/
│   └── librust_bubble.dylib # Compiled Rust library (example for macOS)
├── pyproject.toml          # Python project configuration (if used, e.g., for Poetry or Flit)
├── rust_bubble/
│   ├── Cargo.toml          # Rust project manifest
│   ├── src/
│   │   └── lib.rs          # Rust bubble sort implementation
│   └── target/             # Rust build artifacts
└── src/
    └── bubble/
        └── __init__.py     # Python script with FFI bindings and benchmark
```

-   `README.md`: Provides information about the project.
-   `lib/`: Contains the compiled Rust shared library. The Python script expects the library to be here.
-   `pyproject.toml`: Standard Python project metadata file. (Though in this project it seems minimal or for future use).
-   `rust_bubble/`: Contains the Rust source code and build configuration.
    -   `rust_bubble/Cargo.toml`: Defines the Rust package and its dependencies.
    -   `rust_bubble/src/lib.rs`: The Rust library code.
-   `src/bubble/`: Contains the Python source code.
    -   `src/bubble/__init__.py`: The main Python script that loads the Rust library, defines a Python bubble sort, and runs the performance benchmark.
