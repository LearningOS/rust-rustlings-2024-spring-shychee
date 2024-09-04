/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
    二叉搜索树
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        // 如果根节点为空，创建一个新的根节点并存储该值
        if let None = self.root {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }

        // 从根节点开始遍历
        let mut current = &mut self.root;
        while let Some(ref mut node) = *current {
            // 比较要插入的值和当前节点的值
            match value.cmp(&node.value) {
                // 如果要插入的值较小
                Ordering::Less => {
                    // 如果左子节点为空，创建新的左子节点并存储该值
                    if let None = node.left {
                        node.left = Some(Box::new(TreeNode::new(value)));
                        return;
                    }
                    // 否则继续在左子树中查找插入位置
                    current = &mut node.left;
                }
                // 如果要插入的值较大
                Ordering::Greater => {
                    // 如果右子节点为空，创建新的右子节点并存储该值
                    if let None = node.right {
                        node.right = Some(Box::new(TreeNode::new(value)));
                        return;
                    }
                    // 否则继续在右子树中查找插入位置
                    current = &mut node.right;
                }
                // 如果要插入的值与当前节点的值相等，直接返回
                Ordering::Equal => return,
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        // 从根节点开始搜索
        let mut current = &self.root;
        while let Some(ref node) = *current {
            // 比较要搜索的值和当前节点的值
            match value.cmp(&node.value) {
                // 如果要搜索的值较小，在左子树中继续搜索
                Ordering::Less => current = &node.left,
                // 如果要搜索的值较大，在右子树中继续搜索
                Ordering::Greater => current = &node.right,
                // 如果要搜索的值与当前节点的值相等，返回 true
                Ordering::Equal => return true,
            }
        }
        // 未找到返回 false
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        // 比较要插入的值和当前节点的值
        match value.cmp(&self.value) {
            // 如果要插入的值较小
            Ordering::Less => {
                // 如果左子节点为空，创建新的左子节点并存储该值
                if let None = self.left {
                    self.left = Some(Box::new(TreeNode::new(value)));
                } else {
                    // 否则在左子树中递归插入
                    self.left.as_mut().unwrap().insert(value);
                }
            }
            // 如果要插入的值较大
            Ordering::Greater => {
                // 如果右子节点为空，创建新的右子节点并存储该值
                if let None = self.right {
                    self.right = Some(Box::new(TreeNode::new(value)));
                } else {
                    // 否则在右子树中递归插入
                    self.right.as_mut().unwrap().insert(value);
                }
            }
            // 如果要插入的值与当前节点的值相等，不做任何操作
            Ordering::Equal => {}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


