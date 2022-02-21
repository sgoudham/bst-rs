//! This crate contains Recursive & Iterative Binary Search Tree Implementations. All common
//! operations are included along with common traversal iterators.
//!
//! All elements within the Binary Search Trees _must_ implement the [Ord] trait.
//!
//! It is also important to note that [RecursiveBST] is more likely to `blow the stack.`
//! For more information on why that is the case, please have a look at
//! [The Story of Tail Call Optimizations in Rust.](https://seanchen1991.github.io/posts/tco-story/)
//!
//! ## Author Notes
//!
//! I have made this library with the personal goals of learning and solidifying concepts such
//! as ownership, borrowing, generics and lifetimes. I cannot promise that the implementations are
//! particularly efficient, or if they are, it was not at the forefront of my mind.
//!
//! That being said, there are some areas I would love to improve/include:
//! - Write Rust more idiomatically.
//! - Implement a `pretty_print()` function to display the binary search trees nicely.
//! - Implementing the Drop trait for iterative node cleanup.
//! - Pre-allocating space on the heap for nodes to reduce inefficiency of inserts.
//!
//! I'm more than happy to accept (and encourage) contributions if anyone is kind enough to do so.
//!
//! # Quick Start
//!
//! ```rust
//! use bst_rs::{BinarySearchTree, IterativeBST, RecursiveBST};
//!
//! // Create new empty binary search trees
//! let mut iterative_bst = IterativeBST::new();
//! assert!(iterative_bst.is_empty());
//!
//! let mut recursive_bst = RecursiveBST::new();
//! assert!(recursive_bst.is_empty());
//!
//! // Insert elements (no duplicates are allowed)
//! iterative_bst.insert(10);
//! iterative_bst.insert(10);   // Element is not inserted
//! iterative_bst.insert(5);
//! iterative_bst.insert(2);
//! iterative_bst.insert(15);
//! iterative_bst.insert(25);
//! assert_eq!(iterative_bst.size(), 5);
//!
//! recursive_bst.insert(10);
//! recursive_bst.insert(10);   // Element is not inserted
//! recursive_bst.insert(5);
//! recursive_bst.insert(2);
//! recursive_bst.insert(15);
//! recursive_bst.insert(25);
//! assert_eq!(recursive_bst.size(), 5);
//!
//! // Check if element exists
//! assert!(iterative_bst.contains(&5));    // true
//! assert!(!iterative_bst.contains(&0));   // false
//!
//! assert!(recursive_bst.contains(&5));    // true
//! assert!(!recursive_bst.contains(&0));   // false
//!
//! // Remove elements
//! iterative_bst.remove(&10);
//! iterative_bst.remove(&50);              // No change to tree as element does not exist
//! assert_eq!(iterative_bst.size(), 4);
//!
//! recursive_bst.remove(&10);
//! recursive_bst.remove(&50);              // No change to tree as element does not exist
//! assert_eq!(recursive_bst.size(), 4);
//!
//! // View pre-order, in-order and post-order traversals
//! assert_eq!(iterative_bst.pre_order_vec(), vec![&15, &5, &2, &25]);
//! assert_eq!(iterative_bst.in_order_vec(), vec![&2, &5, &15, &25]);
//! assert_eq!(iterative_bst.post_order_vec(), vec![&2, &5, &25, &15]);
//!
//! assert_eq!(recursive_bst.pre_order_vec(), vec![&15, &5, &2, &25]);
//! assert_eq!(recursive_bst.in_order_vec(), vec![&2, &5, &15, &25]);
//! assert_eq!(recursive_bst.post_order_vec(), vec![&2, &5, &25, &15]);
//!
//! // Compare equality of trees
//! assert_eq!(iterative_bst.sorted_vec(), recursive_bst.sorted_vec());
//! assert_ne!(iterative_bst, IterativeBST::new());
//! assert_ne!(recursive_bst, RecursiveBST::new());
//! ```

use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::vec::IntoIter;

/// A trait containing all the common operations of Binary Search Trees.
///
/// # Examples
/// _Examples are extended from crate level "Quick Start"_
///
/// ```rust
/// use bst_rs::{BinarySearchTree, IterativeBST};
///
/// // Create a new empty Iterative Binary Search Tree
/// let mut new_bst = IterativeBST::new();
/// assert!(new_bst.is_empty());
///
/// // Populate new_bst with 5 elements (no duplicates are allowed)
/// new_bst.insert(10);
/// new_bst.insert(5);
/// new_bst.insert(15);
/// new_bst.insert(18);
/// new_bst.insert(2);
/// assert_eq!(new_bst.size(), 5);
///
/// // We can also create a IterativeBST from vecs and iterators
/// let mut bst_from_vec = IterativeBST::from(vec![10, 5, 15, 18, 2]);
/// let mut bst_from_iter = IterativeBST::from_iter((10..20).map(|node| node > 15));
///
/// // Retrieve the in_order_vec() and sorted_vec() which retrieve the elements in order
/// assert_eq!(bst_from_vec.in_order_vec(), vec![&2, &5, &10, &15, &18]);
/// assert_eq!(bst_from_vec.sorted_vec(), vec![&2, &5, &10, &15, &18]);
pub trait BinarySearchTree<T: Ord> {
    fn new() -> Self;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn is_not_empty(&self) -> bool;
    fn insert(&mut self, value: T);
    fn contains(&self, value: &T) -> bool;
    fn remove(&mut self, value: &T);
    fn retrieve(&self, value: &T) -> Option<&T>;
    fn retrieve_as_mut(&mut self, value: &T) -> Option<&mut T>;
    fn min(&self) -> Option<&T>;
    fn max(&self) -> Option<&T>;
    fn remove_min(&mut self) -> Option<T>;
    fn remove_max(&mut self) -> Option<T>;
    fn sorted_vec(&self) -> Vec<&T>;
    fn into_sorted_vec(self) -> Vec<T>;
    fn pre_order_vec(&self) -> Vec<&T>;
    fn in_order_vec(&self) -> Vec<&T>;
    fn post_order_vec(&self) -> Vec<&T>;
    fn pre_order_iter(&self) -> IntoIter<&T>;
    fn in_order_iter(&self) -> IntoIter<&T>;
    fn post_order_iter(&self) -> IntoIter<&T>;
    fn into_pre_order_iter(self) -> IntoIter<T>;
    fn into_in_order_iter(self) -> IntoIter<T>;
    fn into_post_order_iter(self) -> IntoIter<T>;
}

type HeapNode<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct RecursiveBST<T: Ord> {
    root: HeapNode<T>,
    size: usize,
}

#[derive(Debug)]
pub struct IterativeBST<T: Ord> {
    root: HeapNode<T>,
    size: usize,
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: HeapNode<T>,
    right: HeapNode<T>,
}

impl<T: Ord> PartialEq for IterativeBST<T> {
    fn eq(&self, other: &Self) -> bool {
        self.sorted_vec() == other.sorted_vec()
    }
}

impl<T: Ord> Extend<T> for IterativeBST<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter.into_iter() {
            self.insert(value)
        }
    }
}

impl<T: Ord> FromIterator<T> for IterativeBST<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut bst = IterativeBST::new();
        bst.extend(iter);
        bst
    }
}

impl<T: Ord> From<Vec<T>> for IterativeBST<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut bst = IterativeBST::new();
        for value in vec.into_iter() {
            bst.insert(value);
        }
        bst
    }
}

impl<T: Ord + Clone> From<&[T]> for IterativeBST<T> {
    fn from(slice: &[T]) -> Self {
        let mut bst = IterativeBST::new();
        for value in slice {
            bst.insert((*value).clone());
        }
        bst
    }
}

impl<T: Ord + Clone> Clone for IterativeBST<T> {
    fn clone(&self) -> Self {
        let mut bst = IterativeBST::new();

        for value in self.in_order_iter() {
            bst.insert((*value).clone());
        }

        bst
    }
}

impl<T: Ord + Debug> Display for IterativeBST<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.sorted_vec())
    }
}

impl<T: Ord> PartialEq for RecursiveBST<T> {
    fn eq(&self, other: &Self) -> bool {
        self.sorted_vec() == other.sorted_vec()
    }
}

impl<T: Ord> Extend<T> for RecursiveBST<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter.into_iter() {
            self.insert(value)
        }
    }
}

impl<T: Ord> FromIterator<T> for RecursiveBST<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut bst = RecursiveBST::new();
        bst.extend(iter);
        bst
    }
}

impl<T: Ord> From<Vec<T>> for RecursiveBST<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut bst = RecursiveBST::new();
        for value in vec.into_iter() {
            bst.insert(value);
        }
        bst
    }
}

impl<T: Ord + Clone> From<&[T]> for RecursiveBST<T> {
    fn from(slice: &[T]) -> Self {
        let mut bst = RecursiveBST::new();
        for value in slice {
            bst.insert((*value).clone());
        }
        bst
    }
}

impl<T: Ord + Clone> Clone for RecursiveBST<T> {
    fn clone(&self) -> Self {
        let mut bst = RecursiveBST::new();

        for value in self.in_order_iter() {
            bst.insert((*value).clone());
        }

        bst
    }
}

impl<T: Ord + Debug> Display for RecursiveBST<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.sorted_vec())
    }
}

impl<T: Ord> BinarySearchTree<T> for IterativeBST<T> {
    fn new() -> IterativeBST<T> {
        IterativeBST {
            root: None,
            size: 0,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_not_empty(&self) -> bool {
        self.size != 0
    }

    fn insert(&mut self, value: T) {
        if Node::iterative_insert(&mut self.root, value).is_ok() {
            self.size += 1;
        }
    }

    fn contains(&self, value: &T) -> bool {
        Node::iterative_contains(&self.root, value)
    }

    fn remove(&mut self, value: &T) {
        if Node::iterative_remove(&mut self.root, value).is_ok() {
            self.size -= 1;
        }
    }

    fn retrieve(&self, value: &T) -> Option<&T> {
        Node::iterative_retrieve(&self.root, value)
    }

    fn retrieve_as_mut(&mut self, value: &T) -> Option<&mut T> {
        Node::iterative_retrieve_as_mut(&mut self.root, value)
    }

    fn min(&self) -> Option<&T> {
        Node::iterative_min(&self.root)
    }

    fn max(&self) -> Option<&T> {
        Node::iterative_max(&self.root)
    }

    fn remove_min(&mut self) -> Option<T> {
        let removed_min = Node::iterative_remove_min(&mut self.root);
        if removed_min.is_some() {
            self.size -= 1;
        }
        removed_min
    }

    fn remove_max(&mut self) -> Option<T> {
        let removed_max = Node::iterative_remove_max(&mut self.root);
        if removed_max.is_some() {
            self.size -= 1;
        }
        removed_max
    }

    fn sorted_vec(&self) -> Vec<&T> {
        Node::iterative_in_order_vec(&self.root)
    }

    fn into_sorted_vec(self) -> Vec<T> {
        Node::iterative_consume_in_order_vec(self.root)
    }

    fn pre_order_vec(&self) -> Vec<&T> {
        Node::iterative_pre_order_vec(&self.root)
    }

    fn in_order_vec(&self) -> Vec<&T> {
        Node::iterative_in_order_vec(&self.root)
    }

    fn post_order_vec(&self) -> Vec<&T> {
        Node::iterative_post_order_vec(&self.root)
    }

    fn pre_order_iter(&self) -> IntoIter<&T> {
        Node::iterative_pre_order_vec(&self.root).into_iter()
    }

    fn in_order_iter(&self) -> IntoIter<&T> {
        Node::iterative_in_order_vec(&self.root).into_iter()
    }

    fn post_order_iter(&self) -> IntoIter<&T> {
        Node::iterative_post_order_vec(&self.root).into_iter()
    }

    fn into_pre_order_iter(self) -> IntoIter<T> {
        Node::iterative_consume_pre_order_vec(self.root).into_iter()
    }

    fn into_in_order_iter(self) -> IntoIter<T> {
        Node::iterative_consume_in_order_vec(self.root).into_iter()
    }

    fn into_post_order_iter(self) -> IntoIter<T> {
        Node::iterative_consume_post_order_vec(self.root).into_iter()
    }
}

impl<T: Ord> BinarySearchTree<T> for RecursiveBST<T> {
    fn new() -> RecursiveBST<T> {
        RecursiveBST {
            root: None,
            size: 0,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_not_empty(&self) -> bool {
        self.size != 0
    }

    fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Some(Box::from(Node::new(value)));
                self.size += 1;
            }
            Some(ref mut node) => {
                if node.recursive_insert(value).is_ok() {
                    self.size += 1;
                }
            }
        }
    }

    fn contains(&self, value: &T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.recursive_contains(value),
        }
    }

    fn remove(&mut self, value: &T) {
        if Node::recursive_remove(&mut self.root, value).is_ok() {
            self.size -= 1;
        }
    }

    fn retrieve(&self, value: &T) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref node) => node.recursive_retrieve(value),
        }
    }

    fn retrieve_as_mut(&mut self, value: &T) -> Option<&mut T> {
        match self.root {
            None => None,
            Some(ref mut node) => node.recursive_retrieve_as_mut(value),
        }
    }

    fn min(&self) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref node) => node.recursive_min(),
        }
    }

    fn max(&self) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref node) => node.recursive_max(),
        }
    }

    fn remove_min(&mut self) -> Option<T> {
        let removed_min = match self.root {
            None => None,
            Some(_) => Node::recursive_remove_min(&mut self.root),
        };

        if removed_min.is_some() {
            self.size -= 1;
        }

        removed_min
    }

    fn remove_max(&mut self) -> Option<T> {
        let removed_max = match self.root {
            None => None,
            Some(_) => Node::recursive_remove_max(&mut self.root),
        };

        if removed_max.is_some() {
            self.size -= 1;
        }

        removed_max
    }

    fn sorted_vec(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::recursive_in_order_vec(&self.root, &mut elements);
        elements
    }

    fn into_sorted_vec(self) -> Vec<T> {
        let mut elements = Vec::new();
        Node::recursive_consume_in_order_vec(self.root, &mut elements);
        elements
    }

    fn pre_order_vec(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::recursive_pre_order_vec(&self.root, &mut elements);
        elements
    }

    fn in_order_vec(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::recursive_in_order_vec(&self.root, &mut elements);
        elements
    }

    fn post_order_vec(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::recursive_post_order_vec(&self.root, &mut elements);
        elements
    }

    fn pre_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::recursive_pre_order_vec(&self.root, &mut elements);
        elements.into_iter()
    }

    fn in_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::recursive_in_order_vec(&self.root, &mut elements);
        elements.into_iter()
    }

    fn post_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::recursive_post_order_vec(&self.root, &mut elements);
        elements.into_iter()
    }

    fn into_pre_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::recursive_consume_pre_order_vec(self.root, &mut elements);
        elements.into_iter()
    }

    fn into_in_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::recursive_consume_in_order_vec(self.root, &mut elements);
        elements.into_iter()
    }

    fn into_post_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::recursive_consume_post_order_vec(self.root, &mut elements);
        elements.into_iter()
    }
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn iterative_insert(mut root: &mut HeapNode<T>, value: T) -> Result<(), ()> {
        while let Some(ref mut node) = root {
            match value.cmp(&node.value) {
                Ordering::Equal => return Err(()),
                Ordering::Less => root = &mut node.left,
                Ordering::Greater => root = &mut node.right,
            }
        }
        *root = Some(Box::new(Node::new(value)));

        Ok(())
    }

    fn recursive_insert(&mut self, value: T) -> Result<(), ()> {
        match value.cmp(&self.value) {
            Ordering::Equal => Err(()),
            Ordering::Less => match self.left {
                None => {
                    self.left = Some(Box::from(Node::new(value)));
                    Ok(())
                }
                Some(ref mut node) => node.recursive_insert(value),
            },
            Ordering::Greater => match self.right {
                None => {
                    self.right = Some(Box::from(Node::new(value)));
                    Ok(())
                }
                Some(ref mut node) => node.recursive_insert(value),
            },
        }
    }

    fn iterative_contains(mut root: &HeapNode<T>, value: &T) -> bool {
        while let Some(current) = root {
            match value.cmp(&current.value) {
                Ordering::Equal => return true,
                Ordering::Less => root = &current.left,
                Ordering::Greater => root = &current.right,
            }
        }

        false
    }

    fn recursive_contains(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => match self.left {
                None => false,
                Some(ref node) => node.recursive_contains(value),
            },
            Ordering::Greater => match self.right {
                None => false,
                Some(ref node) => node.recursive_contains(value),
            },
        }
    }

    fn iterative_retrieve<'a>(mut root: &'a HeapNode<T>, value: &T) -> Option<&'a T> {
        while let Some(current) = root {
            match value.cmp(&current.value) {
                Ordering::Equal => return Some(&current.value),
                Ordering::Less => root = &current.left,
                Ordering::Greater => root = &current.right,
            }
        }

        None
    }

    fn recursive_retrieve(&self, value: &T) -> Option<&T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => match self.left {
                None => None,
                Some(ref node) => node.recursive_retrieve(value),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref node) => node.recursive_retrieve(value),
            },
        }
    }

    fn iterative_retrieve_as_mut<'a>(
        mut root: &'a mut HeapNode<T>,
        value: &T,
    ) -> Option<&'a mut T> {
        while let Some(current) = root {
            match value.cmp(&current.value) {
                Ordering::Equal => return Some(&mut current.value),
                Ordering::Less => root = &mut current.left,
                Ordering::Greater => root = &mut current.right,
            }
        }

        None
    }

    fn recursive_retrieve_as_mut(&mut self, value: &T) -> Option<&mut T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&mut self.value),
            Ordering::Less => match self.left {
                None => None,
                Some(ref mut node) => node.recursive_retrieve_as_mut(value),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref mut node) => node.recursive_retrieve_as_mut(value),
            },
        }
    }

    fn iterative_remove(mut root: &mut HeapNode<T>, value: &T) -> Result<(), ()> {
        while let Some(ref mut current) = root {
            match value.cmp(&current.value) {
                Ordering::Less => root = &mut root.as_mut().unwrap().left,
                Ordering::Greater => root = &mut root.as_mut().unwrap().right,
                Ordering::Equal => {
                    match (current.left.as_mut(), current.right.as_mut()) {
                        (None, None) => *root = None,
                        (Some(_), None) => *root = current.left.take(),
                        (None, Some(_)) => *root = current.right.take(),
                        (Some(_), Some(_)) => {
                            root.as_mut().unwrap().value =
                                Node::recursive_remove_min(&mut current.right).unwrap()
                        }
                    }

                    return Ok(());
                }
            }
        }

        Err(())
    }

    fn recursive_remove(root: &mut HeapNode<T>, value: &T) -> Result<(), ()> {
        if let Some(ref mut node) = root {
            return match value.cmp(&node.value) {
                Ordering::Less => Node::recursive_remove(&mut node.left, value),
                Ordering::Greater => Node::recursive_remove(&mut node.right, value),
                Ordering::Equal => {
                    match (&node.left, &node.right) {
                        (None, None) => *root = None,
                        (Some(_), None) => *root = node.left.take(),
                        (None, Some(_)) => *root = node.right.take(),
                        (Some(_), Some(_)) => {
                            node.value = Node::recursive_remove_min(&mut node.right).unwrap()
                        }
                    }

                    Ok(())
                }
            };
        }

        Err(())
    }

    fn iterative_min(mut root: &HeapNode<T>) -> Option<&T> {
        while let Some(current) = root {
            if current.left.is_none() {
                return Some(&current.value);
            }
            root = &current.left;
        }

        None
    }

    fn recursive_min(&self) -> Option<&T> {
        match &self.left {
            None => Some(&self.value),
            Some(node) => node.recursive_min(),
        }
    }

    fn iterative_max(mut root: &HeapNode<T>) -> Option<&T> {
        while let Some(current) = root {
            if current.right.is_none() {
                return Some(&current.value);
            }
            root = &current.right;
        }

        None
    }

    fn recursive_max(&self) -> Option<&T> {
        match &self.right {
            None => Some(&self.value),
            Some(node) => node.recursive_max(),
        }
    }

    fn iterative_remove_min(mut root: &mut HeapNode<T>) -> Option<T> {
        if root.is_some() {
            while root.as_ref().unwrap().left.is_some() {
                root = &mut root.as_mut().unwrap().left
            }

            let node = root.take().unwrap();
            *root = node.right;
            return Some(node.value);
        }

        None
    }

    fn recursive_remove_min(root: &mut HeapNode<T>) -> Option<T> {
        if root.as_ref().unwrap().left.is_some() {
            Node::recursive_remove_min(&mut root.as_mut().unwrap().left)
        } else {
            let node = root.take().unwrap();
            *root = node.right;
            Some(node.value)
        }
    }

    fn iterative_remove_max(mut root: &mut HeapNode<T>) -> Option<T> {
        if root.is_some() {
            while root.as_ref().unwrap().right.is_some() {
                root = &mut root.as_mut().unwrap().right
            }

            let node = root.take().unwrap();
            *root = node.left;
            return Some(node.value);
        }

        None
    }

    fn recursive_remove_max(root: &mut HeapNode<T>) -> Option<T> {
        if root.as_ref().unwrap().right.is_some() {
            Node::recursive_remove_max(&mut root.as_mut().unwrap().right)
        } else {
            let node = root.take().unwrap();
            *root = node.left;
            Some(node.value)
        }
    }

    fn iterative_pre_order_vec(node: &HeapNode<T>) -> Vec<&T> {
        let mut elements = Vec::new();
        let mut stack = vec![node.as_ref()];

        while let Some(current) = stack.pop().unwrap_or(None) {
            elements.push(&current.value);
            if current.right.is_some() {
                stack.push(current.right.as_ref());
            }
            if current.left.is_some() {
                stack.push(current.left.as_ref());
            }
        }

        elements
    }

    fn recursive_pre_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            elements.push(&node.value);
            Node::recursive_pre_order_vec(&node.left, elements);
            Node::recursive_pre_order_vec(&node.right, elements);
        }
    }

    fn iterative_in_order_vec(mut root: &HeapNode<T>) -> Vec<&T> {
        let mut elements = Vec::new();
        let mut stack = Vec::new();

        while !stack.is_empty() || root.is_some() {
            if root.is_some() {
                stack.push(root);
                root = &root.as_ref().unwrap().left;
            } else {
                let node = stack.pop().unwrap();
                elements.push(&node.as_ref().unwrap().value);
                root = &node.as_ref().unwrap().right;
            }
        }

        elements
    }

    fn recursive_in_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::recursive_in_order_vec(&node.left, elements);
            elements.push(&node.value);
            Node::recursive_in_order_vec(&node.right, elements);
        }
    }

    fn iterative_post_order_vec(root: &HeapNode<T>) -> Vec<&T> {
        let mut elements = Vec::new();
        let mut stack_one = vec![root];
        let mut stack_two = vec![];

        while let Some(node) = stack_one.pop().unwrap_or(&None) {
            if node.left.is_some() {
                stack_one.push(&node.left);
            }
            if node.right.is_some() {
                stack_one.push(&node.right);
            }
            stack_two.push(node);
        }

        while let Some(node) = stack_two.pop() {
            elements.push(&node.value);
        }

        elements
    }

    fn recursive_post_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::recursive_post_order_vec(&node.left, elements);
            Node::recursive_post_order_vec(&node.right, elements);
            elements.push(&node.value);
        }
    }

    fn iterative_consume_pre_order_vec(node: HeapNode<T>) -> Vec<T> {
        let mut elements = Vec::new();
        let mut stack = vec![node];

        while let Some(current) = stack.pop().unwrap_or(None) {
            elements.push(current.value);
            if current.right.is_some() {
                stack.push(current.right);
            }
            if current.left.is_some() {
                stack.push(current.left);
            }
        }

        elements
    }

    fn recursive_consume_pre_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            elements.push(node.value);
            Node::recursive_consume_pre_order_vec(node.left, elements);
            Node::recursive_consume_pre_order_vec(node.right, elements);
        }
    }

    fn iterative_consume_in_order_vec(root: HeapNode<T>) -> Vec<T> {
        let mut elements = Vec::new();
        let mut stack = vec![root];

        while !stack.is_empty() {
            if let Some(mut current) = stack.pop().unwrap() {
                if current.left.is_some() {
                    let left_node = current.left.take();
                    stack.push(Some(current));
                    stack.push(left_node);
                } else {
                    let right_node = current.right.take();
                    elements.push(current.value);
                    stack.push(right_node);
                }
            }
        }

        elements
    }

    fn recursive_consume_in_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::recursive_consume_in_order_vec(node.left, elements);
            elements.push(node.value);
            Node::recursive_consume_in_order_vec(node.right, elements);
        }
    }

    fn iterative_consume_post_order_vec(root: HeapNode<T>) -> Vec<T> {
        let mut elements = Vec::new();
        let mut stack_one = vec![root];
        let mut stack_two = vec![];

        while let Some(mut node) = stack_one.pop().unwrap_or(None) {
            if let Some(left_node) = node.left.take() {
                stack_one.push(Some(left_node));
            }
            if let Some(right_node) = node.right.take() {
                stack_one.push(Some(right_node));
            }
            stack_two.push(node);
        }

        while let Some(node) = stack_two.pop() {
            elements.push(node.value);
        }

        elements
    }

    fn recursive_consume_post_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::recursive_consume_post_order_vec(node.left, elements);
            Node::recursive_consume_post_order_vec(node.right, elements);
            elements.push(node.value);
        }
    }
}