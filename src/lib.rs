use std::cmp::Ordering;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct BinarySearchTree<T: Ord> {
    root: Child<T>,
}

type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Child<T>,
    right: Child<T>,
}

impl<T: Ord> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            Ordering::Equal => {}
            Ordering::Less => match self.left {
                None => self.left = Some(Box::from(Node::new(value))),
                Some(ref mut node) => node.insert(value),
            },
            Ordering::Greater => match self.right {
                None => self.right = Some(Box::from(Node::new(value))),
                Some(ref mut node) => node.insert(value),
            },
        }
    }

    fn contains(&self, value: T) -> bool {
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

    // https://users.rust-lang.org/t/binary-search-tree-node-removal-without-unsafe-code/56078
    fn remove(mut root: &mut Child<T>, value: T) {
        while let Some(ref mut node) = root {
            match node.value.cmp(&value) {
                Ordering::Less => root = &mut root.as_mut().unwrap().right,
                Ordering::Greater => root = &mut root.as_mut().unwrap().left,
                Ordering::Equal => match (node.left.as_mut(), node.right.as_mut()) {
                    (None, None) => *root = None,
                    (Some(_), None) => *root = node.left.take(),
                    (None, Some(_)) => *root = node.right.take(),
                    (Some(_), Some(_)) => {
                        root.as_mut().unwrap().value = Self::extract_min(&mut node.right).unwrap()
                    }
                },
            }
        }
    }

    // https://users.rust-lang.org/t/binary-search-tree-node-removal-without-unsafe-code/56078
    fn extract_min(node: &mut Child<T>) -> Option<T> {
        let mut value = None;

        if node.is_some() {
            let mut current = node;

            while current.as_ref().unwrap().left.is_some() {
                current = &mut current.as_mut().unwrap().left;
            }

            let node = current.take().unwrap();
            value = Some(node.value);
            *current = node.right;
        }

        value
    }

    fn pre_order_traversal<'a>(node: &'a Child<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            elements.push(&node.value);
            Self::pre_order_traversal(&node.left, elements);
            Self::pre_order_traversal(&node.right, elements);
        }
    }

    fn in_order_traversal<'a>(node: &'a Child<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Self::in_order_traversal(&node.left, elements);
            elements.push(&node.value);
            Self::in_order_traversal(&node.right, elements);
        }
    }

    fn post_order_traversal<'a>(node: &'a Child<T>, elements: &mut Vec<&'a T>) {
        if let Some(ref node) = node {
            Self::post_order_traversal(&node.left, elements);
            Self::post_order_traversal(&node.right, elements);
            elements.push(&node.value);
        }
    }

    fn into_pre_order_traversal(node: Child<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            elements.push(node.value);
            Self::into_pre_order_traversal(node.left, elements);
            Self::into_pre_order_traversal(node.right, elements);
        }
    }

    fn into_in_order_traversal(node: Child<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Self::into_in_order_traversal(node.left, elements);
            elements.push(node.value);
            Self::into_in_order_traversal(node.right, elements);
        }
    }

    fn into_post_order_traversal(node: Child<T>, elements: &mut Vec<T>) {
        if let Some(node) = node {
            Self::into_post_order_traversal(node.left, elements);
            Self::into_post_order_traversal(node.right, elements);
            elements.push(node.value);
        }
    }
}

impl<T: Ord + Copy> Node<T> {
    fn pre_order_traversal_copy(node: &Child<T>, elements: &mut Vec<T>) {
        if let Some(ref node) = node {
            elements.push(node.value);
            Self::pre_order_traversal_copy(&node.left, elements);
            Self::pre_order_traversal_copy(&node.right, elements);
        }
    }

    fn in_order_traversal_copy(node: &Child<T>, elements: &mut Vec<T>) {
        if let Some(ref node) = node {
            Self::in_order_traversal_copy(&node.left, elements);
            elements.push(node.value);
            Self::in_order_traversal_copy(&node.right, elements);
        }
    }

    fn post_order_traversal_copy(node: &Child<T>, elements: &mut Vec<T>) {
        if let Some(ref node) = node {
            Self::post_order_traversal_copy(&node.left, elements);
            Self::post_order_traversal_copy(&node.right, elements);
            elements.push(node.value);
        }
    }
}

impl<T: Ord + Copy> BinarySearchTree<T> {
    pub fn pre_order(&self) -> Vec<T> {
        let mut elements: Vec<T> = Vec::new();
        Node::pre_order_traversal_copy(&self.root, &mut elements);
        elements
    }

    pub fn in_order(&self) -> Vec<T> {
        let mut elements: Vec<T> = Vec::new();
        Node::in_order_traversal_copy(&self.root, &mut elements);
        elements
    }

    pub fn post_order(&self) -> Vec<T> {
        let mut elements: Vec<T> = Vec::new();
        Node::post_order_traversal_copy(&self.root, &mut elements);
        elements
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn empty() -> Self {
        Self { root: None }
    }

    pub fn new(value: T) -> Self {
        Self {
            root: Some(Box::from(Node::new(value))),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => self.root = Some(Box::from(Node::new(value))),
            Some(ref mut node) => node.insert(value),
        }
    }

    pub fn contains(&self, value: T) -> bool {
        match self.root {
            None => false,
            Some(ref node) => node.contains(value),
        }
    }

    pub fn remove(&mut self, value: T) {
        Node::remove(&mut self.root, value);
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

        assert_eq!(bst.retrieve(-1), Some(&-1));
        assert_eq!(bst.retrieve(0), Some(&0));
        assert_eq!(bst.retrieve(1), Some(&1));
        assert_eq!(bst.retrieve(2), Some(&2));
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

        assert!(!bst.contains(10));
        assert!(bst.contains(1));
        assert!(bst.contains(5));
    }

    #[test]
    fn remove_root_element() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(0);

        bst.remove(0);

        assert!(bst.is_empty())
    }

    #[test]
    fn remove_leaf_node() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(5);
        bst.insert(4);
        bst.insert(6);
        bst.insert(7);

        bst.remove(7);

        assert!(!bst.contains(7));
        assert_eq!(bst.in_order(), vec![4, 5, 6])
    }

    #[test]
    fn remove_single_right_node_with_children() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(5);
        bst.insert(4);
        bst.insert(6);
        bst.insert(7);
        bst.insert(8);

        bst.remove(6);

        assert!(!bst.contains(6));
        assert_eq!(bst.in_order(), vec![4, 5, 7, 8])
    }

    #[test]
    fn remove_single_left_node_with_children() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(5);
        bst.insert(4);
        bst.insert(6);
        bst.insert(3);
        bst.insert(2);

        bst.remove(4);

        assert!(!bst.contains(4));
        assert_eq!(bst.in_order(), vec![2, 3, 5, 6])
    }

    #[test]
    fn remove_node_with_two_children() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(10);
        bst.insert(5);
        bst.insert(8);
        bst.insert(3);
        bst.insert(15);

        bst.remove(5);

        assert!(!bst.contains(5));
        assert_eq!(bst.in_order(), vec![3, 8, 10, 15])
    }

    #[test]
    fn does_not_fail_when_removing_non_existing_element() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(10);
        bst.insert(5);
        bst.insert(8);
        bst.insert(3);
        bst.insert(15);

        bst.remove(20);

        assert!(!bst.contains(20));
        assert_eq!(bst.in_order(), vec![3, 5, 8, 10, 15])
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
        let mut bst = BinarySearchTree::empty();
        bst.insert(10);
        bst.insert(5);

        let _retrieved_value_as_mut: &mut i32 = bst.retrieve_as_mut(5).unwrap();
        *_retrieved_value_as_mut = 2;

        assert!(bst.contains(10));
        assert!(bst.contains(2));
        assert!(!bst.contains(5));
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
    fn pre_order_traversal_copy() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.pre_order(), vec![3, 1, 2, 4, 5]);
    }

    #[test]
    fn in_order_traversal_copy() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.in_order(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn post_order_traversal_copy() {
        let mut bst = BinarySearchTree::empty();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.post_order(), vec![2, 1, 5, 4, 3]);
    }
}