/*
	heap
	This question requires you to implement a binary heap function
    二叉堆
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone,
{
    // 堆的大小
    count: usize,
    // 堆的元素
    items: Vec<T>,
    // 比较器
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    // 返回堆的大小
    pub fn len(&self) -> usize {
        self.count
    }

    // 判断堆是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // 添加元素
    pub fn add(&mut self, value: T) {
        self.count += 1;
        self.items.push(value);
        self.sift_up(self.count);
    }

    // 上浮操作
    fn sift_up(&mut self, mut idx: usize) {
        while idx > 1 {
            let parent = self.parent_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[parent]) {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    // 返回父节点的索引
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    // 判断子节点是否存在
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    // 返回左子节点的索引
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    // 返回右子节点的索引
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    // 返回最优的子节点索引
    fn optimal_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            return self.left_child_idx(idx);
        }
        
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        if (self.comparator)(&self.items[left], &self.items[right]) {
            left
        } else {
            right
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

// 实现迭代器 trait
impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let result = self.items[1].clone();
        self.items[1] = self.items[self.count].clone();
        self.items.pop();
        self.count -= 1;

        if !self.is_empty() {
            self.sift_down(1);
        }

        Some(result)
    }
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    fn sift_down(&mut self, mut idx: usize) {
        while self.children_present(idx) {
            let child = self.optimal_child_idx(idx);
            if !(self.comparator)(&self.items[idx], &self.items[child]) {
                self.items.swap(idx, child);
                idx = child;
            } else {
                break;
            }
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}