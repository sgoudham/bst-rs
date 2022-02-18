use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::vec::IntoIter;

#[derive(Debug)]
pub struct BinarySearchTree<T: Ord> {
    root: HeapNode<T>,
    size: usize,
}

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: HeapNode<T>,
    right: HeapNode<T>,
}

type HeapNode<T> = Option<Box<Node<T>>>;

impl<T: Ord> PartialEq for BinarySearchTree<T> {
    fn eq(&self, other: &Self) -> bool {
        self.in_order() == other.in_order()
    }
}

impl<T: Ord> Extend<T> for BinarySearchTree<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for value in iter.into_iter() {
            self.insert(value)
        }
    }
}

impl<T: Ord> FromIterator<T> for BinarySearchTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut bst = BinarySearchTree::empty();
        bst.extend(iter);
        bst
    }
}

impl<T: Ord> From<Vec<T>> for BinarySearchTree<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut bst = BinarySearchTree::empty();
        for value in vec.into_iter() {
            bst.insert(value);
        }
        bst
    }
}

impl<T: Ord + Clone> From<&[T]> for BinarySearchTree<T> {
    fn from(slice: &[T]) -> Self {
        let mut bst = BinarySearchTree::empty();
        for value in slice {
            bst.insert((*value).clone());
        }
        bst
    }
}

impl<T: Ord + Clone> Clone for BinarySearchTree<T> {
    fn clone(&self) -> Self {
        let mut bst = BinarySearchTree::empty();

        for value in self.in_order_iter() {
            bst.insert((*value).clone());
        }

        bst
    }
}

impl<T: Ord + Debug> Display for BinarySearchTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.in_order())
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

    fn insert(&mut self, value: T) -> Result<(), ()> {
        match value.cmp(&self.value) {
            Ordering::Equal => Err(()),
            Ordering::Less => match self.left {
                None => {
                    self.left = Some(Box::from(Node::new(value)));
                    Ok(())
                }
                Some(ref mut node) => node.insert(value),
            },
            Ordering::Greater => match self.right {
                None => {
                    self.right = Some(Box::from(Node::new(value)));
                    Ok(())
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

    fn remove(root: &mut HeapNode<T>, value: &T) -> Result<(), ()> {
        if let Some(ref mut node) = root {
            return match value.cmp(&node.value) {
                Ordering::Less => Node::remove(&mut node.left, value),
                Ordering::Greater => Node::remove(&mut node.right, value),
                Ordering::Equal => {
                    match (&node.left, &node.right) {
                        (None, None) => *root = None,
                        (Some(_), None) => *root = node.left.take(),
                        (None, Some(_)) => *root = node.right.take(),
                        (Some(_), Some(_)) => {
                            node.value = Node::remove_min(&mut node.right).unwrap()
                        }
                    }

                    Ok(())
                }
            };
        }

        Err(())
    }

    fn min(&self) -> Option<&T> {
        match &self.left {
            None => Some(&self.value),
            Some(node) => node.min(),
        }
    }

    fn max(&self) -> Option<&T> {
        match &self.right {
            None => Some(&self.value),
            Some(node) => node.max(),
        }
    }

    fn remove_min(root: &mut HeapNode<T>) -> Option<T> {
        if root.as_ref().unwrap().left.is_some() {
            Node::remove_min(&mut root.as_mut().unwrap().left)
        } else {
            let node = root.take().unwrap();
            *root = node.right;
            Some(node.value)
        }
    }

    fn remove_max(root: &mut HeapNode<T>) -> Option<T> {
        if root.as_ref().unwrap().right.is_some() {
            Node::remove_max(&mut root.as_mut().unwrap().right)
        } else {
            let node = root.take().unwrap();
            *root = node.left;
            Some(node.value)
        }
    }

    fn pre_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            elements.push(&node.value);
            Node::pre_order_vec(&node.left, elements);
            Node::pre_order_vec(&node.right, elements);
        }
    }

    fn in_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::in_order_vec(&node.left, elements);
            elements.push(&node.value);
            Node::in_order_vec(&node.right, elements);
        }
    }

    fn post_order_vec<'a>(node: &'a HeapNode<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Node::post_order_vec(&node.left, elements);
            Node::post_order_vec(&node.right, elements);
            elements.push(&node.value);
        }
    }

    fn consume_pre_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            elements.push(node.value);
            Node::consume_pre_order_vec(node.left, elements);
            Node::consume_pre_order_vec(node.right, elements);
        }
    }

    fn consume_in_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::consume_in_order_vec(node.left, elements);
            elements.push(node.value);
            Node::consume_in_order_vec(node.right, elements);
        }
    }

    fn consume_post_order_vec(node: HeapNode<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Node::consume_post_order_vec(node.left, elements);
            Node::consume_post_order_vec(node.right, elements);
            elements.push(node.value);
        }
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
                if node.insert(value).is_ok() {
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
        if Node::remove(&mut self.root, value).is_ok() {
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

    pub fn min(&self) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref node) => node.min(),
        }
    }

    pub fn max(&self) -> Option<&T> {
        match self.root {
            None => None,
            Some(ref node) => node.max(),
        }
    }

    pub fn remove_min(&mut self) -> Option<T> {
        let removed_min = match self.root {
            None => None,
            Some(_) => Node::remove_min(&mut self.root),
        };

        if removed_min.is_some() {
            self.size -= 1;
        }

        removed_min
    }

    pub fn remove_max(&mut self) -> Option<T> {
        let removed_max = match self.root {
            None => None,
            Some(_) => Node::remove_max(&mut self.root),
        };

        if removed_max.is_some() {
            self.size -= 1;
        }

        removed_max
    }

    pub fn pre_order(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::pre_order_vec(&self.root, &mut elements);
        elements
    }

    pub fn in_order(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::in_order_vec(&self.root, &mut elements);
        elements
    }

    pub fn post_order(&self) -> Vec<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::post_order_vec(&self.root, &mut elements);
        elements
    }

    pub fn pre_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::pre_order_vec(&self.root, &mut elements);
        elements.into_iter()
    }

    pub fn in_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::in_order_vec(&self.root, &mut elements);
        elements.into_iter()
    }

    pub fn post_order_iter(&self) -> IntoIter<&T> {
        let mut elements: Vec<&T> = Vec::new();
        Node::post_order_vec(&self.root, &mut elements);
        elements.into_iter()
    }

    pub fn into_pre_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::consume_pre_order_vec(self.root, &mut elements);
        elements.into_iter()
    }

    pub fn into_in_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::consume_in_order_vec(self.root, &mut elements);
        elements.into_iter()
    }

    pub fn into_post_order_iter(self) -> IntoIter<T> {
        let mut elements = Vec::new();
        Node::consume_post_order_vec(self.root, &mut elements);
        elements.into_iter()
    }
}

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
    fn get_min_from_bst() {
        let mut bst = BinarySearchTree::empty();
        assert_eq!(bst.min(), None);

        bst.insert(5);
        bst.insert(3);
        bst.insert(1);
        bst.insert(15);

        assert_eq!(bst.min(), Some(&1));
    }

    #[test]
    fn get_max_from_bst() {
        let mut bst = BinarySearchTree::empty();
        assert_eq!(bst.max(), None);

        bst.insert(5);
        bst.insert(12);
        bst.insert(1);
        bst.insert(15);

        assert_eq!(bst.max(), Some(&15));
    }

    #[test]
    fn remove_min_from_bst() {
        let mut bst = BinarySearchTree::empty();
        assert_eq!(bst.remove_min(), None);

        bst.insert(5);
        assert_eq!(bst.remove_min(), Some(5));
        assert_eq!(bst.size(), 0);

        bst.insert(3);
        bst.insert(1);
        bst.insert(2);
        bst.insert(15);

        assert_eq!(bst.remove_min(), Some(1));
        assert!(bst.contains(&2));
        assert_eq!(bst.size(), 3);
    }

    #[test]
    fn remove_max_from_bst() {
        let mut bst = BinarySearchTree::empty();
        assert_eq!(bst.remove_max(), None);

        bst.insert(5);
        assert_eq!(bst.remove_max(), Some(5));
        assert_eq!(bst.size(), 0);

        bst.insert(3);
        bst.insert(1);
        bst.insert(15);
        bst.insert(10);

        assert_eq!(bst.remove_max(), Some(15));
        assert!(bst.contains(&10));
        assert_eq!(bst.size(), 3);
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

    #[test]
    fn create_bst_from_vec() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(10);
        expected_bst.insert(20);
        expected_bst.insert(5);
        expected_bst.insert(30);

        let actual_bst = BinarySearchTree::from(vec![10, 20, 5, 30]);

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn create_bst_from_slice() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(10);
        expected_bst.insert(20);
        expected_bst.insert(5);
        expected_bst.insert(30);

        let actual_bst = BinarySearchTree::from(vec![10, 20, 5, 30].as_slice());

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn create_bst_from_into_vec() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(10);
        expected_bst.insert(20);
        expected_bst.insert(5);
        expected_bst.insert(30);

        let actual_bst: BinarySearchTree<i32> = vec![10, 20, 5, 30].into();

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn extend_bst_from_iter() {
        let vec = vec![8, 1, 10];
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(3);
        expected_bst.insert(2);
        expected_bst.insert(5);
        expected_bst.insert(8);
        expected_bst.insert(1);
        expected_bst.insert(10);
        let mut actual_bst = BinarySearchTree::empty();
        actual_bst.insert(3);
        actual_bst.insert(2);
        actual_bst.insert(5);

        actual_bst.extend(vec.into_iter());

        assert_eq!(actual_bst.size(), 6);
        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn create_bst_from_iter() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(3);
        expected_bst.insert(2);
        expected_bst.insert(5);
        expected_bst.insert(8);
        expected_bst.insert(1);
        expected_bst.insert(10);

        let actual_bst = BinarySearchTree::from_iter(vec![3, 2, 5, 8, 1, 10].into_iter());

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn clone_bst() {
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(3);
        expected_bst.insert(2);
        expected_bst.insert(5);
        expected_bst.insert(8);
        expected_bst.insert(1);
        expected_bst.insert(10);

        let cloned_bst = expected_bst.clone();

        assert_eq!(cloned_bst, expected_bst);
    }

    #[test]
    fn clone_into_another_bst() {
        let mut actual_bst = BinarySearchTree::empty();
        actual_bst.insert(3);
        actual_bst.insert(2);
        let mut expected_bst = BinarySearchTree::empty();
        expected_bst.insert(3);
        expected_bst.insert(2);
        expected_bst.insert(5);
        expected_bst.insert(8);
        expected_bst.insert(1);
        expected_bst.insert(10);
        assert_ne!(actual_bst, expected_bst);

        actual_bst.clone_from(&expected_bst);

        assert_eq!(actual_bst, expected_bst);
    }
}