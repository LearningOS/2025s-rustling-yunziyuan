/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
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
        match self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(ref mut node) => {
                node.insert(value);
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        fn search_node<T: Ord>(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
            match node {
                None => false,
                Some(node) => {
                    match value.cmp(&node.value) {
                        Ordering::Equal => true,
                        Ordering::Less => search_node(&node.left, value),
                        Ordering::Greater => search_node(&node.right, value),
                    }
                }
            }
        }
        search_node(&self.root, &value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {} // 重复值，不做任何操作
            Ordering::Less => {
                match self.left {
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(ref mut left) => {
                        left.insert(value);
                    }
                }
            }
            Ordering::Greater => {
                match self.right {
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                    Some(ref mut right) => {
                        right.insert(value);
                    }
                }
            }
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


