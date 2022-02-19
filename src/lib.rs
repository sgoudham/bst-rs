use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::vec::IntoIter;

pub trait BinarySearchTree<T: Ord> {
    fn empty() -> Self;
    fn new(value: T) -> Self;
    fn size(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn insert(&mut self, value: T);
    fn contains(&self, value: &T) -> bool;
    fn remove(&mut self, value: &T);
    fn retrieve(&self, value: T) -> Option<&T>;
    fn retrieve_as_mut(&mut self, value: T) -> Option<&mut T>;
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
        let mut bst = RecursiveBST::empty();
        bst.extend(iter);
        bst
    }
}

impl<T: Ord> From<Vec<T>> for RecursiveBST<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut bst = RecursiveBST::empty();
        for value in vec.into_iter() {
            bst.insert(value);
        }
        bst
    }
}

impl<T: Ord + Clone> From<&[T]> for RecursiveBST<T> {
    fn from(slice: &[T]) -> Self {
        let mut bst = RecursiveBST::empty();
        for value in slice {
            bst.insert((*value).clone());
        }
        bst
    }
}

impl<T: Ord + Clone> Clone for RecursiveBST<T> {
    fn clone(&self) -> Self {
        let mut bst = RecursiveBST::empty();

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

impl<T: Ord> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
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

    fn recursive_retrieve(&self, value: T) -> Option<&T> {
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

    fn recursive_retrieve_as_mut(&mut self, value: T) -> Option<&mut T> {
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

    fn recursive_min(&self) -> Option<&T> {
        match &self.left {
            None => Some(&self.value),
            Some(node) => node.recursive_min(),
        }
    }

    fn recursive_max(&self) -> Option<&T> {
        match &self.right {
            None => Some(&self.value),
            Some(node) => node.recursive_max(),
        }
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

    fn recursive_remove_max(root: &mut HeapNode<T>) -> Option<T> {
        if root.as_ref().unwrap().right.is_some() {
            Node::recursive_remove_max(&mut root.as_mut().unwrap().right)
        } else {
            let node = root.take().unwrap();
            *root = node.left;
            Some(node.value)
        }
    }

    fn recursive_pre_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            elements.push(&node.value);
            Node::recursive_pre_order_vec(&node.left, elements);
            Node::recursive_pre_order_vec(&node.right, elements);
        }
    }

    fn recursive_in_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::recursive_in_order_vec(&node.left, elements);
            elements.push(&node.value);
            Node::recursive_in_order_vec(&node.right, elements);
        }
    }

    fn recursive_post_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::recursive_post_order_vec(&node.left, elements);
            Node::recursive_post_order_vec(&node.right, elements);
            elements.push(&node.value);
        }
    }

    fn recursive_consume_pre_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            elements.push(node.value);
            Node::recursive_consume_pre_order_vec(node.left, elements);
            Node::recursive_consume_pre_order_vec(node.right, elements);
        }
    }

    fn recursive_consume_in_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::recursive_consume_in_order_vec(node.left, elements);
            elements.push(node.value);
            Node::recursive_consume_in_order_vec(node.right, elements);
        }
    }

    fn recursive_consume_post_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::recursive_consume_post_order_vec(node.left, elements);
            Node::recursive_consume_post_order_vec(node.right, elements);
            elements.push(node.value);
        }
    }
}

impl<T: Ord> BinarySearchTree<T> for RecursiveBST<T> {
    fn empty() -> RecursiveBST<T> {
        RecursiveBST {
            root: None,
            size: 0,
        }
    }

    fn new(value: T) -> RecursiveBST<T> {
        RecursiveBST {
            root: Some(Box::from(Node::new(value))),
            size: 1,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
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

    fn retrieve(&self, value: T) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref node) => node.recursive_retrieve(value),
        }
    }

    fn retrieve_as_mut(&mut self, value: T) -> Option<&mut T> {
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