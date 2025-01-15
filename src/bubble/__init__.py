import timeit
from typing import Sequence

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
    sorted_arr = bubble_sort_by_python3(arr)
    print("排序後的數組：", sorted_arr)

    execution_time = timeit.timeit(lambda: bubble_sort_by_python3([float(x) for x in arr.copy()]), number=1000000)
    print(f"執行時間: {execution_time} 秒")
