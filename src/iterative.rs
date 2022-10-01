use std::fmt::{Debug, Display, Formatter};
use std::vec::IntoIter;

use crate::BinarySearchTree;
use crate::Node;
use crate::HeapNode;

/// Iterative Binary Search Tree implementation.
///
/// # Important
///
/// This should be preferred over [RecursiveBST] for reasons listed in crate level documentation.
#[derive(Debug)]
pub struct IterativeBST<T: Ord> {
    root: HeapNode<T>,
    size: usize,
}

impl<T: Ord> IterativeBST<T> {
    /// Creates an empty `IterativeBST<T>`
    ///
    /// No nodes are allocated on the heap yet
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// // Empty tree is created
    /// let mut bst: IterativeBST<i32> = IterativeBST::new();
    /// assert!(bst.is_empty())
    /// ```
    pub fn new() -> IterativeBST<T> {
        IterativeBST {
            root: None,
            size: 0,
        }
    }
}

impl<T: Ord> Default for IterativeBST<T> {
    /// Creates an empty `IterativeBST<T>`
    fn default() -> IterativeBST<T> {
        IterativeBST::new()
    }
}

impl<T: Ord> PartialEq for IterativeBST<T> {
    fn eq(&self, other: &Self) -> bool {
        self.asc_order_vec() == other.asc_order_vec()
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
        write!(f, "{:?}", self.asc_order_vec())
    }
}

impl<T: Ord> BinarySearchTree<T> for IterativeBST<T> {
    /// Returns the total **number of nodes** within the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(5);
    /// bst.insert(10);
    /// bst.insert(3);
    ///
    /// assert_eq!(bst.size(), 3);
    /// ```
    fn size(&self) -> usize {
        self.size
    }

    /// Returns `true` if the binary search tree contains no nodes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst: IterativeBST<i32> = IterativeBST::new();
    /// assert!(bst.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns `true` if the binary search tree contains one or more nodes.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// assert!(bst.is_empty());
    ///
    /// bst.insert(2);
    ///
    /// assert!(bst.is_not_empty());
    /// ```
    fn is_not_empty(&self) -> bool {
        self.size != 0
    }

    /// Inserts given value as a node.
    ///
    /// **Duplicate values are _not allowed_**.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    ///
    /// bst.insert(10);
    /// bst.insert(10);   // Element is not inserted
    /// bst.insert(5);
    /// bst.insert(2);
    /// bst.insert(15);
    /// bst.insert(25);
    ///
    /// assert_eq!(bst.size(), 5);
    /// ```
    fn insert(&mut self, value: T) {
        if Node::iterative_insert(&mut self.root, value).is_ok() {
            self.size += 1;
        }
    }

    /// Returns `true` if the binary search tree contains an element with the given value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(5);
    /// bst.insert(2);
    /// bst.insert(7);
    ///
    /// assert!(bst.contains(&5));
    /// assert!(!bst.contains(&10));
    /// ```
    fn contains(&self, value: &T) -> bool {
        Node::iterative_contains(&self.root, value)
    }

    /// Removes the given value.
    ///
    /// Tree will not be modified if trying to remove element that does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(5);
    /// bst.insert(2);
    /// bst.insert(7);
    /// assert_eq!(bst.size(), 3);
    ///
    /// bst.remove(&5);
    /// bst.remove(&10); // Element is not removed
    /// assert_eq!(bst.size(), 2);
    /// ```
    fn remove(&mut self, value: &T) {
        if Node::iterative_remove(&mut self.root, value).is_ok() {
            self.size -= 1;
        }
    }

    /// Returns a reference to the element or `None` if element does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(5);
    /// bst.insert(2);
    /// bst.insert(7);
    ///
    /// assert_eq!(bst.retrieve(&5), Some(&5));
    /// assert_eq!(bst.retrieve(&10), None);
    /// ```
    fn retrieve(&self, value: &T) -> Option<&T> {
        Node::iterative_retrieve(&self.root, value)
    }

    /// Returns a mutable reference to the element (see [IterativeBST::retrieve()])
    /// or `None` if element does not exist.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(10);
    /// bst.insert(5);
    ///
    /// let optional_retrieved_value_as_mut = bst.retrieve_as_mut(&5);
    /// assert_eq!(optional_retrieved_value_as_mut, Some(&mut 5));
    ///
    /// let mut retrieved_value = optional_retrieved_value_as_mut.unwrap();
    /// *retrieved_value = 2; // Change value inside tree to '2'
    ///
    /// assert_eq!(bst.retrieve_as_mut(&5), None); // 5 does not exist anymore
    /// assert_eq!(bst.retrieve_as_mut(&2), Some(&mut 2));
    /// ```
    fn retrieve_as_mut(&mut self, value: &T) -> Option<&mut T> {
        Node::iterative_retrieve_as_mut(&mut self.root, value)
    }

    /// Returns the **height** or `None` if tree is empty.
    ///
    /// The height is the number of edges between the root and it's furthest leaf node.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// // Given a tree that looks like:
    ///  //         4
    ///  //       /  \
    ///  //      2    6
    ///  //     / \  / \
    ///  //    1  3 5   7
    /// let mut bst = IterativeBST::new();
    /// assert_eq!(bst.height(), None);
    ///
    /// bst.insert(4);
    /// bst.insert(6);
    /// bst.insert(2);
    /// bst.insert(7);
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(1);
    ///
    /// // The height is 2.
    /// assert_eq!(bst.height(), Some(2));
    /// ```
    fn height(&self) -> Option<isize> {
        self.root
            .as_ref()
            .map(|_| Node::iterative_height(&self.root))
    }

    /// Returns a reference to the minimum element of the tree or `None` if tree is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// assert_eq!(bst.min(), None);
    ///
    /// bst.insert(5);
    /// bst.insert(2);
    /// bst.insert(10);
    ///
    /// assert_eq!(bst.min(), Some(&2));
    /// ```
    fn min(&self) -> Option<&T> {
        Node::iterative_min(&self.root)
    }

    /// Returns a reference to the maximum element of the tree or `None` if tree is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// assert_eq!(bst.max(), None);
    ///
    /// bst.insert(5);
    /// bst.insert(2);
    /// bst.insert(10);
    ///
    /// assert_eq!(bst.max(), Some(&10));
    /// ```
    fn max(&self) -> Option<&T> {
        Node::iterative_max(&self.root)
    }

    /// Removes and returns the minimum element from the tree or `None` if tree is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// assert_eq!(bst.remove_min(), None);
    ///
    /// bst.insert(2);
    /// bst.insert(5);
    /// bst.insert(10);
    ///
    /// assert_eq!(bst.size(), 3);
    /// assert_eq!(bst.remove_min(), Some(2));
    /// assert_eq!(bst.size(), 2);
    /// ```
    fn remove_min(&mut self) -> Option<T> {
        let removed_min = Node::iterative_remove_min(&mut self.root);
        if removed_min.is_some() {
            self.size -= 1;
        }
        removed_min
    }

    /// Removes and returns the maximum element from the tree or `None` if tree is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// assert_eq!(bst.remove_max(), None);
    ///
    /// bst.insert(2);
    /// bst.insert(5);
    /// bst.insert(10);
    ///
    /// assert_eq!(bst.size(), 3);
    /// assert_eq!(bst.remove_max(), Some(10));
    /// assert_eq!(bst.size(), 2);
    /// ```
    fn remove_max(&mut self) -> Option<T> {
        let removed_max = Node::iterative_remove_max(&mut self.root);
        if removed_max.is_some() {
            self.size -= 1;
        }
        removed_max
    }

    /// Returns references to the elements of the tree in **ascending order.**`
    ///
    /// # Important
    ///
    /// This function is analogous to [IterativeBST::in_order_vec()] as the underlying
    /// behaviour is **_exactly the same_.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(4);
    /// bst.insert(6);
    /// bst.insert(2);
    /// bst.insert(7);
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(1);
    ///
    /// assert_eq!(bst.asc_order_vec(), vec![&1, &2, &3, &4, &5, &6, &7]);
    /// ```
    fn asc_order_vec(&self) -> Vec<&T> {
        self.in_order_vec()
    }

    /// Returns references to the elements of the tree in the order of a **pre-order traversal.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// // Given a tree that looks like:
    ///  //         4
    ///  //       /  \
    ///  //      2    6
    ///  //     / \  / \
    ///  //    1  3 5   7
    /// let mut bst = IterativeBST::new();
    /// bst.insert(4);
    /// bst.insert(6);
    /// bst.insert(2);
    /// bst.insert(7);
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(1);
    ///
    /// // The pre_order_vec is: [&4, &2, &1, &3, &6, &5, &7]
    /// assert_eq!(bst.pre_order_vec(), vec![&4, &2, &1, &3, &6, &5, &7]);
    /// ```
    fn pre_order_vec(&self) -> Vec<&T> {
        Node::iterative_pre_order_vec(&self.root)
    }

    /// Returns references to the elements of the tree in the order of an **in-order traversal.**
    ///
    /// # Important
    ///
    /// This function is analogous to [IterativeBST::asc_order_vec()] as the underlying
    /// behaviour is **_exactly the same_.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// // Given a tree that looks like:
    ///  //         4
    ///  //       /  \
    ///  //      2    6
    ///  //     / \  / \
    ///  //    1  3 5   7
    /// let mut bst = IterativeBST::new();
    /// bst.insert(4);
    /// bst.insert(6);
    /// bst.insert(2);
    /// bst.insert(7);
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(1);
    ///
    /// // The in_order_vec is: [&1, &2, &3, &4, &5, &6, &7]
    /// assert_eq!(bst.in_order_vec(), vec![&1, &2, &3, &4, &5, &6, &7]);
    /// ```
    fn in_order_vec(&self) -> Vec<&T> {
        Node::iterative_in_order_vec(&self.root)
    }

    /// Returns references to the elements of the tree in the order of a **post-order traversal.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// // Given a tree that looks like:
    ///  //         4
    ///  //       /  \
    ///  //      2    6
    ///  //     / \  / \
    ///  //    1  3 5   7
    /// let mut bst = IterativeBST::new();
    /// bst.insert(4);
    /// bst.insert(6);
    /// bst.insert(2);
    /// bst.insert(7);
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(1);
    ///
    /// // The post_order_vec is: [&1, &3, &2, &5, &7, &6, &4]
    /// assert_eq!(bst.post_order_vec(), vec![&1, &3, &2, &5, &7, &6, &4]);
    /// ```
    fn post_order_vec(&self) -> Vec<&T> {
        Node::iterative_post_order_vec(&self.root)
    }

    /// Returns references to the elements of the tree in the order of a **level-order traversal.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// // Given a tree that looks like:
    ///  //         4
    ///  //       /  \
    ///  //      2    6
    ///  //     / \  / \
    ///  //    1  3 5   7
    /// let mut bst = IterativeBST::new();
    /// bst.insert(4);
    /// bst.insert(6);
    /// bst.insert(2);
    /// bst.insert(7);
    /// bst.insert(5);
    /// bst.insert(3);
    /// bst.insert(1);
    ///
    /// // The level_order_vec is: [&4, &2, &6, &1, &3, &5, &7]
    /// assert_eq!(bst.level_order_vec(), vec![&4, &2, &6, &1, &3, &5, &7]);
    /// ```
    fn level_order_vec(&self) -> Vec<&T> {
        Node::iterative_level_order_vec(&self.root)
    }

    /// Returns an iterator over [IterativeBST::asc_order_vec()].
    ///
    /// # Important
    ///
    /// This function is analogous to [IterativeBST::in_order_iter()] as the underlying
    /// behaviour is **_exactly the same_.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut asc_order_iter = bst.asc_order_iter();
    ///
    /// assert_eq!(asc_order_iter.next(), Some(&1));
    /// assert_eq!(asc_order_iter.next(), Some(&2));
    /// assert_eq!(asc_order_iter.next(), Some(&3));
    /// assert_eq!(asc_order_iter.next(), Some(&4));
    /// assert_eq!(asc_order_iter.next(), Some(&5));
    /// assert_eq!(asc_order_iter.next(), None);
    /// ```
    fn asc_order_iter(&self) -> IntoIter<&T> {
        self.in_order_iter()
    }

    /// Returns an iterator over [IterativeBST::pre_order_vec()].
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut pre_order_iter = bst.pre_order_iter();
    ///
    /// assert_eq!(pre_order_iter.next(), Some(&3));
    /// assert_eq!(pre_order_iter.next(), Some(&1));
    /// assert_eq!(pre_order_iter.next(), Some(&2));
    /// assert_eq!(pre_order_iter.next(), Some(&4));
    /// assert_eq!(pre_order_iter.next(), Some(&5));
    /// assert_eq!(pre_order_iter.next(), None);
    /// ```
    fn pre_order_iter(&self) -> IntoIter<&T> {
        Node::iterative_pre_order_vec(&self.root).into_iter()
    }

    /// Returns an iterator over [IterativeBST::in_order_vec()].
    ///
    /// # Important
    ///
    /// This function is analogous to [IterativeBST::asc_order_iter()] as the underlying
    /// behaviour is **_exactly the same_.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut in_order_iter = bst.in_order_iter();
    ///
    /// assert_eq!(in_order_iter.next(), Some(&1));
    /// assert_eq!(in_order_iter.next(), Some(&2));
    /// assert_eq!(in_order_iter.next(), Some(&3));
    /// assert_eq!(in_order_iter.next(), Some(&4));
    /// assert_eq!(in_order_iter.next(), Some(&5));
    /// assert_eq!(in_order_iter.next(), None);
    /// ```
    fn in_order_iter(&self) -> IntoIter<&T> {
        Node::iterative_in_order_vec(&self.root).into_iter()
    }

    /// Returns an iterator over [IterativeBST::post_order_vec()].
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut post_order_iter = bst.post_order_iter();
    ///
    /// assert_eq!(post_order_iter.next(), Some(&2));
    /// assert_eq!(post_order_iter.next(), Some(&1));
    /// assert_eq!(post_order_iter.next(), Some(&5));
    /// assert_eq!(post_order_iter.next(), Some(&4));
    /// assert_eq!(post_order_iter.next(), Some(&3));
    /// assert_eq!(post_order_iter.next(), None);
    /// ```
    fn post_order_iter(&self) -> IntoIter<&T> {
        Node::iterative_post_order_vec(&self.root).into_iter()
    }

    /// Returns an iterator over [IterativeBST::level_order_vec()].
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut level_order_iter = bst.level_order_iter();
    ///
    /// assert_eq!(level_order_iter.next(), Some(&3));
    /// assert_eq!(level_order_iter.next(), Some(&1));
    /// assert_eq!(level_order_iter.next(), Some(&4));
    /// assert_eq!(level_order_iter.next(), Some(&2));
    /// assert_eq!(level_order_iter.next(), Some(&5));
    /// assert_eq!(level_order_iter.next(), None);
    /// ```
    fn level_order_iter(&self) -> IntoIter<&T> {
        Node::iterative_level_order_vec(&self.root).into_iter()
    }

    /// Returns [IterativeBST::asc_order_iter()] **AND** consumes the tree.
    ///
    /// # Important
    ///
    /// This function is analogous to [IterativeBST::into_in_order_iter()] as the
    /// underlying behaviour is **_exactly the same_.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut into_asc_order_iter = bst.into_asc_order_iter();
    ///
    /// assert_eq!(into_asc_order_iter.next(), Some(1));
    /// assert_eq!(into_asc_order_iter.next(), Some(2));
    /// assert_eq!(into_asc_order_iter.next(), Some(3));
    /// assert_eq!(into_asc_order_iter.next(), Some(4));
    /// assert_eq!(into_asc_order_iter.next(), Some(5));
    /// assert_eq!(into_asc_order_iter.next(), None);
    ///
    /// // bst.insert(10); -> COMPILE ERROR
    /// ```
    fn into_asc_order_iter(self) -> IntoIter<T> {
        self.into_in_order_iter()
    }

    /// Returns [IterativeBST::pre_order_iter()] **AND** consumes the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut into_pre_order_iter = bst.into_pre_order_iter();
    ///
    /// assert_eq!(into_pre_order_iter.next(), Some(3));
    /// assert_eq!(into_pre_order_iter.next(), Some(1));
    /// assert_eq!(into_pre_order_iter.next(), Some(2));
    /// assert_eq!(into_pre_order_iter.next(), Some(4));
    /// assert_eq!(into_pre_order_iter.next(), Some(5));
    /// assert_eq!(into_pre_order_iter.next(), None);
    ///
    /// // bst.insert(10); -> COMPILE ERROR
    /// ```
    fn into_pre_order_iter(self) -> IntoIter<T> {
        Node::iterative_consume_pre_order_vec(self.root).into_iter()
    }

    /// Returns [IterativeBST::in_order_iter()] **AND** consumes the tree.
    ///
    /// # Important
    ///
    /// This function is analogous to [IterativeBST::asc_order_iter()] as the
    /// underlying behaviour is **_exactly the same_.**
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut into_in_order_iter = bst.into_in_order_iter();
    ///
    /// assert_eq!(into_in_order_iter.next(), Some(1));
    /// assert_eq!(into_in_order_iter.next(), Some(2));
    /// assert_eq!(into_in_order_iter.next(), Some(3));
    /// assert_eq!(into_in_order_iter.next(), Some(4));
    /// assert_eq!(into_in_order_iter.next(), Some(5));
    /// assert_eq!(into_in_order_iter.next(), None);
    ///
    /// // bst.insert(10); -> COMPILE ERROR
    /// ```
    fn into_in_order_iter(self) -> IntoIter<T> {
        Node::iterative_consume_in_order_vec(self.root).into_iter()
    }

    /// Returns [IterativeBST::post_order_iter()] **AND** consumes the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut into_post_order_iter = bst.into_post_order_iter();
    ///
    /// assert_eq!(into_post_order_iter.next(), Some(2));
    /// assert_eq!(into_post_order_iter.next(), Some(1));
    /// assert_eq!(into_post_order_iter.next(), Some(5));
    /// assert_eq!(into_post_order_iter.next(), Some(4));
    /// assert_eq!(into_post_order_iter.next(), Some(3));
    /// assert_eq!(into_post_order_iter.next(), None);
    ///
    /// // bst.insert(10); -> COMPILE ERROR
    /// ```
    fn into_post_order_iter(self) -> IntoIter<T> {
        Node::iterative_consume_post_order_vec(self.root).into_iter()
    }

    /// Returns [IterativeBST::level_order_iter()] **AND** consumes the tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bst_rs::{BinarySearchTree, IterativeBST};
    ///
    /// let mut bst = IterativeBST::new();
    /// bst.insert(3);
    /// bst.insert(4);
    /// bst.insert(5);
    /// bst.insert(1);
    /// bst.insert(2);
    ///
    /// let mut into_level_order_iter = bst.into_level_order_iter();
    ///
    /// assert_eq!(into_level_order_iter.next(), Some(3));
    /// assert_eq!(into_level_order_iter.next(), Some(1));
    /// assert_eq!(into_level_order_iter.next(), Some(4));
    /// assert_eq!(into_level_order_iter.next(), Some(2));
    /// assert_eq!(into_level_order_iter.next(), Some(5));
    /// assert_eq!(into_level_order_iter.next(), None);
    ///
    /// // bst.insert(10); -> COMPILE ERROR
    /// ```
    fn into_level_order_iter(self) -> IntoIter<T> {
        Node::iterative_consume_level_order_vec(self.root).into_iter()
    }
}
