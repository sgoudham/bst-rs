use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::vec::IntoIter;

#[derive(Debug, Eq)]
pub struct BinarySearchTree<T: Ord> {
    root: HeapNode<T>,
    size: usize,
}

#[derive(Debug, Eq)]
struct Node<T: Ord> {
    value: T,
    left: HeapNode<T>,
    right: HeapNode<T>,
}

type HeapNode<T> = Option<Box<Node<T>>>;

impl<T: Ord + Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T: Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
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

    fn insert(&mut self, value: T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => false,
            Ordering::Less => match self.left {
                None => {
                    self.left = Some(Box::from(Node::new(value)));
                    true
                }
                Some(ref mut node) => node.insert(value),
            },
            Ordering::Greater => match self.right {
                None => {
                    self.right = Some(Box::from(Node::new(value)));
                    true
                }
                Some(ref mut node) => node.insert(value),
            },
        }
    }

    fn contains(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            Ordering::Equal => true,
            Ordering::Less => match self.left {
                None => false,
                Some(ref node) => node.contains(value),
            },
            Ordering::Greater => match self.right {
                None => false,
                Some(ref node) => node.contains(value),
            },
        }
    }

    fn retrieve(&self, value: T) -> Option<&T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => match self.left {
                None => None,
                Some(ref node) => node.retrieve(value),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref node) => node.retrieve(value),
            },
        }
    }

    fn retrieve_as_mut(&mut self, value: T) -> Option<&mut T> {
        match value.cmp(&self.value) {
            Ordering::Equal => Some(&mut self.value),
            Ordering::Less => match self.left {
                None => None,
                Some(ref mut node) => node.retrieve_as_mut(value),
            },
            Ordering::Greater => match self.right {
                None => None,
                Some(ref mut node) => node.retrieve_as_mut(value),
            },
        }
    }

    fn remove(root: &mut HeapNode<T>, value: &T) -> bool {
        if let Some(ref mut node) = root {
            return match value.cmp(&node.value) {
                Ordering::Less => Node::remove(&mut node.left, value),
                Ordering::Greater => Node::remove(&mut node.right, value),
                Ordering::Equal => {
                    match (&node.left, &node.right) {
                        (None, None) => *root = None,
                        (Some(_), None) => *root = node.left.take(),
                        (None, Some(_)) => *root = node.right.take(),
                        (Some(_), Some(_)) => node.value = Node::extract_min(&mut node.right),
                    }

                    true
                }
            };
        }

        false
    }

    fn extract_min(root: &mut HeapNode<T>) -> T {
        if root.as_ref().unwrap().left.is_some() {
            Node::extract_min(&mut root.as_mut().unwrap().left)
        } else {
            let node = root.take().unwrap();
            *root = node.right;
            node.value
        }
    }

    fn pre_order_traversal<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            elements.push(&node.value);
            Node::pre_order_traversal(&node.left, elements);
            Node::pre_order_traversal(&node.right, elements);
        }
    }

    fn in_order_traversal<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::in_order_traversal(&node.left, elements);
            elements.push(&node.value);
            Node::in_order_traversal(&node.right, elements);
        }
    }

    fn post_order_traversal<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::post_order_traversal(&node.left, elements);
            Node::post_order_traversal(&node.right, elements);
            elements.push(&node.value);
        }
    }

    fn into_pre_order_traversal(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            elements.push(node.value);
            Node::into_pre_order_traversal(node.left, elements);
            Node::into_pre_order_traversal(node.right, elements);
        }
    }

    fn into_in_order_traversal(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::into_in_order_traversal(node.left, elements);
            elements.push(node.value);
            Node::into_in_order_traversal(node.right, elements);
        }
    }

    fn into_post_order_traversal(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::into_post_order_traversal(node.left, elements);
            Node::into_post_order_traversal(node.right, elements);
            elements.push(node.value);
        }
    }
}

impl<T: Ord + Display> Display for BinarySearchTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let vec = self.in_order_iter();
        let len = vec.len();
        for (index, elem) in vec.enumerate() {
            if index == len - 1 {
                write!(f, "{}", elem)?;
            } else {
                write!(f, "{}, ", elem)?;
            }
        }

        write!(f, "]")
    }
}

impl<T: Ord> PartialEq for BinarySearchTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.in_order() == other.in_order()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn empty() -> BinarySearchTree<T> {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    pub fn new(value: T) -> BinarySearchTree<T> {
        BinarySearchTree {
            root: Some(Box::from(Node::new(value))),
            size: 1,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Some(Box::from(Node::new(value)));
                self.size += 1;
            }
            Some(ref mut node) => {
                if node.insert(value) {
                    self.size += 1;
                }
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.contains(value),
        }
    }

    pub fn remove(&mut self, value: &T) {
        if Node::remove(&mut self.root, value) {
            self.size -= 1;
        }
    }

    pub fn retrieve(&self, value: T) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref node) => node.retrieve(value),
        }
    }

    pub fn retrieve_as_mut(&mut self, value: T) -> Option<&mut T> {
        match self.root {
            None => None,
            Some(ref mut node) => node.retrieve_as_mut(value),
        }
    }

    pub fn pre_order(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::pre_order_traversal(&self.root, &mut elements);
        elements
    }

    pub fn in_order(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::in_order_traversal(&self.root, &mut elements);
        elements
    }

    pub fn post_order(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::post_order_traversal(&self.root, &mut elements);
        elements
    }

    pub fn pre_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::pre_order_traversal(&self.root, &mut elements);
        elements.into_iter()
    }

    pub fn in_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::in_order_traversal(&self.root, &mut elements);
        elements.into_iter()
    }

    pub fn post_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::post_order_traversal(&self.root, &mut elements);
        elements.into_iter()
    }

    pub fn into_pre_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::into_pre_order_traversal(self.root, &mut elements);
        elements.into_iter()
    }

    pub fn into_in_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::into_in_order_traversal(self.root, &mut elements);
        elements.into_iter()
    }

    pub fn into_post_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::into_post_order_traversal(self.root, &mut elements);
        elements.into_iter()
    }
}

// impl<T: Ord> Drop for BST<T> {
//     fn drop(&mut self) {
//         todo!("I need help with this :sob: :sob:")
//
//         // let mut stack = vec![&self.root];
//         // let mut out: Vec<&LinkNode<T>> = Vec::new();
//         //
//         // while !stack.is_empty() {
//         //     let cur_node = stack.pop().unwrap();
//         //     out.push(cur_node);
//         //
//         //     if let Some(node) = &cur_node.as_ref().unwrap().left {
//         //         stack.push()
//         //     }
//         // }
//     }
// }

#[cfg(test)]
mod bst_test {
    use super::BinarySearchTree;

    #[test]
    fn can_insert_element() {
        let mut bst = BinarySearchTree::new(-1);

        bst.insert(0);
        bst.insert(1);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.size(), 4);
    }

    #[test]
    fn check_if_bst_is_empty() {
        let mut bst = BinarySearchTree::empty();
        assert!(bst.is_empty());

        bst.insert(1);
        assert!(!bst.is_empty());
    }

    #[test]
    fn check_element_exists() {
        let mut bst = BinarySearchTree::empty();

        bst.insert(1);
        bst.insert(5);

        assert!(!bst.contains(&10));
        assert!(bst.contains(&1));
        assert!(bst.contains(&5));
    }

    #[test]
    fn remove_root_element() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(0);

        assert!(!bst.is_empty());
        assert_eq!(bst.size(), 1);

        bst.remove(&0);

        assert!(bst.is_empty());
        assert_eq!(bst.size(), 0)
    }

    #[test]
    fn remove_leaf_node() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(5);
        expected_bst.insert(4);
        expected_bst.insert(6);
        let mut actual_bst = BinarySearchTree::empty();
        actual_bst.insert(5);
        actual_bst.insert(4);
        actual_bst.insert(6);
        actual_bst.insert(7);

        actual_bst.remove(&7);

        assert_eq!(actual_bst.size(), 3);
        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn remove_single_right_node_with_children() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(5);
        expected_bst.insert(4);
        expected_bst.insert(7);
        expected_bst.insert(8);
        let mut actual_bst = BinarySearchTree::empty();
        actual_bst.insert(5);
        actual_bst.insert(4);
        actual_bst.insert(6);
        actual_bst.insert(7);
        actual_bst.insert(8);

        actual_bst.remove(&6);

        assert_eq!(actual_bst.size(), 4);
        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn remove_single_left_node_with_children() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(5);
        expected_bst.insert(3);
        expected_bst.insert(2);
        expected_bst.insert(6);
        let mut actual_bst = BinarySearchTree::empty();
        actual_bst.insert(5);
        actual_bst.insert(4);
        actual_bst.insert(6);
        actual_bst.insert(3);
        actual_bst.insert(2);

        actual_bst.remove(&4);

        assert_eq!(actual_bst.size(), 4);
        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn remove_node_with_two_children() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(10);
        expected_bst.insert(3);
        expected_bst.insert(8);
        expected_bst.insert(15);
        let mut actual_bst = BinarySearchTree::empty();
        actual_bst.insert(10);
        actual_bst.insert(5);
        actual_bst.insert(8);
        actual_bst.insert(3);
        actual_bst.insert(15);

        actual_bst.remove(&5);

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn does_not_fail_when_removing_non_existing_element() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(10);
        expected_bst.insert(5);
        expected_bst.insert(8);
        expected_bst.insert(3);
        expected_bst.insert(15);

        let mut actual_bst = BinarySearchTree::empty();
        actual_bst.insert(10);
        actual_bst.insert(5);
        actual_bst.insert(8);
        actual_bst.insert(3);
        actual_bst.insert(15);

        actual_bst.remove(&20);

        assert_eq!(actual_bst.size(), 5);
        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn retrieve_element() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(5);
        bst.insert(10);

        let retrieved_value = bst.retrieve(5);
        let invalid_value = bst.retrieve(15);

        assert_eq!(retrieved_value, Some(&5));
        assert_eq!(invalid_value, None);
    }

    #[test]
    fn retrieve_element_as_mut_and_modify_bst() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(10);
        expected_bst.insert(2);

        let mut actual_bst = BinarySearchTree::empty();
        actual_bst.insert(10);
        actual_bst.insert(5);

        let _retrieved_value_as_mut: &mut i32 = actual_bst.retrieve_as_mut(5).unwrap();
        *_retrieved_value_as_mut = 2;

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn pre_order_traversal() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut pre_order_iter = bst.pre_order_iter();

        assert_eq!(pre_order_iter.next(), Some(&3));
        assert_eq!(pre_order_iter.next(), Some(&1));
        assert_eq!(pre_order_iter.next(), Some(&2));
        assert_eq!(pre_order_iter.next(), Some(&4));
        assert_eq!(pre_order_iter.next(), Some(&5));
        assert_eq!(pre_order_iter.next(), None);

        bst.insert(10);

        let mut another_pre_order_iter = bst.pre_order_iter();

        assert_eq!(another_pre_order_iter.next(), Some(&3));
        assert_eq!(another_pre_order_iter.next(), Some(&1));
        assert_eq!(another_pre_order_iter.next(), Some(&2));
        assert_eq!(another_pre_order_iter.next(), Some(&4));
        assert_eq!(another_pre_order_iter.next(), Some(&5));
        assert_eq!(another_pre_order_iter.next(), Some(&10));
        assert_eq!(another_pre_order_iter.next(), None);
    }

    #[test]
    fn in_order_traversal() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut in_order_iter = bst.in_order_iter();

        assert_eq!(in_order_iter.next(), Some(&1));
        assert_eq!(in_order_iter.next(), Some(&2));
        assert_eq!(in_order_iter.next(), Some(&3));
        assert_eq!(in_order_iter.next(), Some(&4));
        assert_eq!(in_order_iter.next(), Some(&5));
        assert_eq!(in_order_iter.next(), None);

        bst.insert(6);

        let mut another_in_order_iter = bst.in_order_iter();

        assert_eq!(another_in_order_iter.next(), Some(&1));
        assert_eq!(another_in_order_iter.next(), Some(&2));
        assert_eq!(another_in_order_iter.next(), Some(&3));
        assert_eq!(another_in_order_iter.next(), Some(&4));
        assert_eq!(another_in_order_iter.next(), Some(&5));
        assert_eq!(another_in_order_iter.next(), Some(&6));
        assert_eq!(another_in_order_iter.next(), None);
    }

    #[test]
    fn post_order_traversal() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut post_order_iter = bst.post_order_iter();
        println!("{:?}", bst);

        assert_eq!(post_order_iter.next(), Some(&2));
        assert_eq!(post_order_iter.next(), Some(&1));
        assert_eq!(post_order_iter.next(), Some(&5));
        assert_eq!(post_order_iter.next(), Some(&4));
        assert_eq!(post_order_iter.next(), Some(&3));
        assert_eq!(post_order_iter.next(), None);

        bst.insert(10);

        let mut another_post_order_iter = bst.post_order_iter();

        assert_eq!(another_post_order_iter.next(), Some(&2));
        assert_eq!(another_post_order_iter.next(), Some(&1));
        assert_eq!(another_post_order_iter.next(), Some(&10));
        assert_eq!(another_post_order_iter.next(), Some(&5));
        assert_eq!(another_post_order_iter.next(), Some(&4));
        assert_eq!(another_post_order_iter.next(), Some(&3));
        assert_eq!(another_post_order_iter.next(), None);
    }

    #[test]
    fn into_pre_order_traversal() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut pre_order_iter = bst.into_pre_order_iter();

        assert_eq!(pre_order_iter.next(), Some(3));
        assert_eq!(pre_order_iter.next(), Some(1));
        assert_eq!(pre_order_iter.next(), Some(2));
        assert_eq!(pre_order_iter.next(), Some(4));
        assert_eq!(pre_order_iter.next(), Some(5));
        assert_eq!(pre_order_iter.next(), None);
    }

    #[test]
    fn into_in_order_traversal() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut in_order_iter = bst.into_in_order_iter();

        assert_eq!(in_order_iter.next(), Some(1));
        assert_eq!(in_order_iter.next(), Some(2));
        assert_eq!(in_order_iter.next(), Some(3));
        assert_eq!(in_order_iter.next(), Some(4));
        assert_eq!(in_order_iter.next(), Some(5));
        assert_eq!(in_order_iter.next(), None);
    }

    #[test]
    fn into_post_order_traversal() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut post_order_traversal = bst.into_post_order_iter();

        assert_eq!(post_order_traversal.next(), Some(2));
        assert_eq!(post_order_traversal.next(), Some(1));
        assert_eq!(post_order_traversal.next(), Some(5));
        assert_eq!(post_order_traversal.next(), Some(4));
        assert_eq!(post_order_traversal.next(), Some(3));
        assert_eq!(post_order_traversal.next(), None);
    }

    #[test]
    fn pre_order() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.pre_order(), vec![&3, &1, &2, &4, &5]);
    }

    #[test]
    fn in_order() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.in_order(), vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn post_order() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.post_order(), vec![&2, &1, &5, &4, &3]);
    }
}