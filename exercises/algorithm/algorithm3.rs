/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
    排序算法
*/


use std::cmp::PartialOrd;

// 冒泡排序
// 通过反复比较相邻的元素，如果顺序错误就进行交换，每一轮都将最大的元素 “浮” 到数组的末尾。
fn bubble_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    let n = array.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

// 插入排序
// 将待排序的元素插入到已排序的部分中的适当位置，从第二个元素开始，依次与已排序部分的元素比较并插入。
fn insertion_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    for i in 1..array.len() {
        let key = array[i];
        let mut j = i - 1;

        while j >= 0 && array[j] > key {
            array[j + 1] = array[j];
            j -= 1;
        }
        array[j + 1] = key;
    }
}

// 堆排序辅助函数：维护最大堆性质
// 首先将数组构建成一个最大堆，然后依次将堆顶元素与末尾元素交换，并对新的堆顶元素进行调整，以维持最大堆性质。
fn heapify<T: PartialOrd + Copy>(array: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < n && array[i] < array[l] {
        largest = l;
    }

    if r < n && array[largest] < array[r] {
        largest = r;
    }

    if largest!= i {
        array.swap(i, largest);
        heapify(array, n, largest);
    }
}

// 堆排序
fn heap_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    let n = array.len();

    for i in (n / 2 - 1)..=0 {
        heapify(array, n, i as usize);
    }

    for i in (n - 1)..0 {
        array.swap(0, i);
        heapify(array, i, 0);
    }
}

// 快排
fn quick_sort<T: PartialOrd + Copy>(array: &mut [T]) {
    quick_sort_helper(array, 0, array.len() - 1);
}

fn quick_sort_helper<T: PartialOrd + Copy>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(array, low, high);
        quick_sort_helper(array, low, pivot_index - 1);
        quick_sort_helper(array, pivot_index + 1, high);
    }
}

// 将数组中小于等于枢轴的元素放在左边，大于枢轴的元素放在右边，返回枢轴的最终位置。
fn partition<T: PartialOrd + Copy>(array: &mut [T], low: usize, high: usize) -> usize {
    let pivot = array[high];
    let mut i = low - 1;

    for j in low..high {
        if array[j] <= pivot {
            i += 1;
            array.swap(i, j);
        }
    }

    array.swap(i + 1, high);
    i + 1
}

// 统一的 sort 函数，根据传入的参数选择排序算法
fn sort<T: PartialOrd + Copy>(array: &mut [T], algorithm: &str) {
    match algorithm {
        "bubble" => bubble_sort(array),
        "insertion" => insertion_sort(array),
        "heap" => heap_sort(array),
        "quick" => quick_sort(array),
        _ => panic!("Invalid sorting algorithm"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec, "bubble");
        // sort(&mut vec, "quick");
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec, "bubble");
        // sort(&mut vec, "quick");
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec, "bubble");
        // sort(&mut vec, "quick");
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}