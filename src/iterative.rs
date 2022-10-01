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

#[cfg(test)]
mod tests {
    use std::vec::IntoIter;

    use crate::{BinarySearchTree, IterativeBST};

    #[test]
    fn successfully_insert_elements_into_bst() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(0);
        expected_bst.insert(1);
        expected_bst.insert(2);
        expected_bst.insert(-20);

        let mut actual_bst = IterativeBST::new();
        actual_bst.insert(0);
        actual_bst.insert(1);
        actual_bst.insert(1);
        actual_bst.insert(2);
        actual_bst.insert(-20);

        assert_eq!(actual_bst, expected_bst);
        assert_eq!(actual_bst.size(), 4);
    }

    #[test]
    fn check_if_bst_is_empty() {
        let mut bst = IterativeBST::new();
        assert!(bst.is_empty());

        bst.insert(1);
        assert!(!bst.is_empty());
    }

    #[test]
    fn check_if_bst_is_not_empty() {
        let mut bst = IterativeBST::new();
        assert!(!bst.is_not_empty());

        bst.insert(1);
        assert!(bst.is_not_empty());
    }

    #[test]
    fn check_if_bst_contains_elements() {
        let mut bst = IterativeBST::new();
        assert!(!bst.contains(&10));

        bst.insert(1);
        bst.insert(5);

        assert!(!bst.contains(&10));
        assert!(bst.contains(&1));
        assert!(bst.contains(&5));
    }

    #[test]
    fn successfully_remove_root_node_from_bst() {
        let mut bst = IterativeBST::new();
        bst.insert(0);

        bst.remove(&0);

        assert!(bst.is_empty());
        assert_eq!(bst.size(), 0)
    }

    #[test]
    fn successfully_remove_leaf_node() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(5);
        expected_bst.insert(4);
        expected_bst.insert(6);
        let mut actual_bst = IterativeBST::new();
        actual_bst.insert(5);
        actual_bst.insert(4);
        actual_bst.insert(6);
        actual_bst.insert(7);

        actual_bst.remove(&7);

        assert_eq!(actual_bst.size(), 3);
        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_remove_single_right_node_with_children() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(5);
        expected_bst.insert(4);
        expected_bst.insert(7);
        expected_bst.insert(8);
        let mut actual_bst = IterativeBST::new();
        actual_bst.insert(5);
        actual_bst.insert(4);
        actual_bst.insert(6);
        actual_bst.insert(7);
        actual_bst.insert(8);

        actual_bst.remove(&6);

        println!("{}", actual_bst);
        assert_eq!(actual_bst.size(), 4);
        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_remove_single_left_node_with_children() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(5);
        expected_bst.insert(3);
        expected_bst.insert(2);
        expected_bst.insert(6);
        let mut actual_bst = IterativeBST::new();
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
    fn successfully_remove_node_with_two_children() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(10);
        expected_bst.insert(3);
        expected_bst.insert(8);
        expected_bst.insert(15);
        let mut actual_bst = IterativeBST::new();
        actual_bst.insert(10);
        actual_bst.insert(5);
        actual_bst.insert(8);
        actual_bst.insert(3);
        actual_bst.insert(15);

        actual_bst.remove(&5);

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_does_not_fail_when_removing_non_existing_element() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(10);
        expected_bst.insert(5);
        expected_bst.insert(8);
        expected_bst.insert(3);
        expected_bst.insert(15);

        let mut actual_bst = IterativeBST::new();
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
    fn successfully_retrieve_element() {
        let mut bst = IterativeBST::new();
        bst.insert(5);
        bst.insert(10);

        let retrieved_value = bst.retrieve(&5);
        let invalid_value = bst.retrieve(&15);

        assert_eq!(retrieved_value, Some(&5));
        assert_eq!(invalid_value, None);
    }

    #[test]
    fn successfully_retrieve_element_as_mut_and_modify_bst() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(10);
        expected_bst.insert(2);

        let mut actual_bst = IterativeBST::new();
        actual_bst.insert(10);
        actual_bst.insert(5);

        let _retrieved_value_as_mut: &mut i32 = actual_bst.retrieve_as_mut(&5).unwrap();
        *_retrieved_value_as_mut = 2;

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_get_height_of_bst() {
        let mut bst = IterativeBST::new();
        assert_eq!(bst.height(), None);

        bst.insert(4);
        assert_eq!(bst.height(), Some(0));

        bst.insert(2);
        bst.insert(6);
        bst.insert(1);
        bst.insert(3);
        bst.insert(4);
        bst.insert(7);
        assert_eq!(bst.height(), Some(2));

        bst.insert(8);
        assert_eq!(bst.height(), Some(3));
    }

    #[test]
    fn successfully_get_min_from_bst() {
        let mut bst = IterativeBST::new();
        assert_eq!(bst.min(), None);

        bst.insert(5);
        bst.insert(3);
        bst.insert(1);
        bst.insert(15);

        assert_eq!(bst.min(), Some(&1));
    }

    #[test]
    fn successfully_get_max_from_bst() {
        let mut bst = IterativeBST::new();
        assert_eq!(bst.max(), None);

        bst.insert(5);
        bst.insert(12);
        bst.insert(1);
        bst.insert(15);

        assert_eq!(bst.max(), Some(&15));
    }

    #[test]
    fn successfully_remove_min_from_bst() {
        let mut bst = IterativeBST::new();
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
    fn successfully_remove_max_from_bst() {
        let mut bst = IterativeBST::new();
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
    fn pre_order_iter() {
        let mut bst = IterativeBST::new();
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
    fn in_order_iter() {
        let mut bst = IterativeBST::new();
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
    fn post_order_iter() {
        let mut bst = IterativeBST::new();
        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        let mut post_order_iter = bst.post_order_iter();

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
    fn level_order_iter() {
        let mut bst = IterativeBST::new();
        bst.insert(15);
        bst.insert(20);
        bst.insert(10);
        bst.insert(8);
        bst.insert(12);
        bst.insert(16);
        bst.insert(25);

        let mut level_order_iter = bst.level_order_iter();

        assert_eq!(level_order_iter.next(), Some(&15));
        assert_eq!(level_order_iter.next(), Some(&10));
        assert_eq!(level_order_iter.next(), Some(&20));
        assert_eq!(level_order_iter.next(), Some(&8));
        assert_eq!(level_order_iter.next(), Some(&12));
        assert_eq!(level_order_iter.next(), Some(&16));
        assert_eq!(level_order_iter.next(), Some(&25));
        assert_eq!(level_order_iter.next(), None);

        bst.insert(4);

        let mut another_level_order_iter = bst.level_order_iter();

        assert_eq!(another_level_order_iter.next(), Some(&15));
        assert_eq!(another_level_order_iter.next(), Some(&10));
        assert_eq!(another_level_order_iter.next(), Some(&20));
        assert_eq!(another_level_order_iter.next(), Some(&8));
        assert_eq!(another_level_order_iter.next(), Some(&12));
        assert_eq!(another_level_order_iter.next(), Some(&16));
        assert_eq!(another_level_order_iter.next(), Some(&25));
        assert_eq!(another_level_order_iter.next(), Some(&4));
        assert_eq!(another_level_order_iter.next(), None);
    }

    #[test]
    fn into_pre_order_iter_with_no_elements() {
        let bst: IterativeBST<i32> = IterativeBST::new();

        let mut pre_order_traversal = bst.into_pre_order_iter();

        assert_eq!(pre_order_traversal.next(), None);
    }

    #[test]
    fn into_pre_order_iter_with_one_element() {
        let mut bst = IterativeBST::new();
        bst.insert(3);

        let mut pre_order_traversal = bst.into_pre_order_iter();

        assert_eq!(pre_order_traversal.next(), Some(3));
        assert_eq!(pre_order_traversal.next(), None);
    }

    #[test]
    fn into_pre_order_iter() {
        let mut iter: IntoIter<i32> = IterativeBST::new().into_pre_order_iter();
        assert_eq!(iter.next(), None);

        let mut bst = IterativeBST::new();
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
    fn into_in_order_iter_with_no_elements() {
        let bst: IterativeBST<i32> = IterativeBST::new();

        let mut in_order_traversal = bst.into_in_order_iter();

        assert_eq!(in_order_traversal.next(), None);
    }

    #[test]
    fn into_in_order_iter_with_one_element() {
        let mut bst = IterativeBST::new();
        bst.insert(3);

        let mut in_order_traversal = bst.into_in_order_iter();

        assert_eq!(in_order_traversal.next(), Some(3));
        assert_eq!(in_order_traversal.next(), None);
    }

    #[test]
    fn into_in_order_iter() {
        let another_bst: IterativeBST<i32> = IterativeBST::new();
        let mut iter = another_bst.into_in_order_iter();
        assert_eq!(iter.next(), None);

        let mut bst = IterativeBST::new();
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
    fn into_post_order_iter_with_no_elements() {
        let bst: IterativeBST<i32> = IterativeBST::new();

        let mut post_order_traversal = bst.into_post_order_iter();

        assert_eq!(post_order_traversal.next(), None);
    }

    #[test]
    fn into_post_order_iter_with_one_element() {
        let mut bst = IterativeBST::new();
        bst.insert(3);

        let mut post_order_traversal = bst.into_post_order_iter();

        assert_eq!(post_order_traversal.next(), Some(3));
        assert_eq!(post_order_traversal.next(), None);
    }

    #[test]
    fn into_post_order_iter_with_many_elements() {
        let mut bst = IterativeBST::new();
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
    fn into_level_order_iter_with_no_elements() {
        let bst: IterativeBST<i32> = IterativeBST::new();

        let mut level_order_traversal = bst.into_level_order_iter();

        assert_eq!(level_order_traversal.next(), None);
    }

    #[test]
    fn into_level_order_iter_with_one_element() {
        let mut bst = IterativeBST::new();
        bst.insert(3);

        let mut level_order_traversal = bst.into_level_order_iter();

        assert_eq!(level_order_traversal.next(), Some(3));
        assert_eq!(level_order_traversal.next(), None);
    }

    #[test]
    fn into_level_order_iter_with_many_elements() {
        let mut bst = IterativeBST::new();
        bst.insert(3);
        bst.insert(5);
        bst.insert(4);
        bst.insert(1);
        bst.insert(2);

        let mut level_order_traversal = bst.into_level_order_iter();

        assert_eq!(level_order_traversal.next(), Some(3));
        assert_eq!(level_order_traversal.next(), Some(1));
        assert_eq!(level_order_traversal.next(), Some(5));
        assert_eq!(level_order_traversal.next(), Some(2));
        assert_eq!(level_order_traversal.next(), Some(4));
        assert_eq!(level_order_traversal.next(), None);
    }

    #[test]
    fn successfully_get_pre_order_vec() {
        let mut bst = IterativeBST::new();
        assert!(bst.pre_order_vec().is_empty());

        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.pre_order_vec(), vec![&3, &1, &2, &4, &5]);
    }

    #[test]
    fn successfully_get_in_order_vec() {
        let mut bst = IterativeBST::new();
        assert!(bst.in_order_vec().is_empty());

        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.in_order_vec(), vec![&1, &2, &3, &4, &5]);
    }

    #[test]
    fn successfully_get_post_order_vec() {
        let mut bst = IterativeBST::new();
        assert!(bst.post_order_vec().is_empty());

        bst.insert(3);
        bst.insert(4);
        bst.insert(5);
        bst.insert(1);
        bst.insert(2);

        assert_eq!(bst.post_order_vec(), vec![&2, &1, &5, &4, &3]);
    }

    #[test]
    fn successfully_get_level_order_vec() {
        let mut bst = IterativeBST::new();
        assert!(bst.level_order_vec().is_empty());

        bst.insert(15);
        bst.insert(20);
        bst.insert(10);
        bst.insert(8);
        bst.insert(12);
        bst.insert(16);
        bst.insert(25);

        assert_eq!(
            bst.level_order_vec(),
            vec![&15, &10, &20, &8, &12, &16, &25]
        );
    }

    #[test]
    fn successfully_create_bst_from_vec() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(10);
        expected_bst.insert(20);
        expected_bst.insert(5);
        expected_bst.insert(30);

        let actual_bst = IterativeBST::from(vec![10, 20, 5, 30]);

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_create_bst_from_slice() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(10);
        expected_bst.insert(20);
        expected_bst.insert(5);
        expected_bst.insert(30);

        let actual_bst = IterativeBST::from(vec![10, 20, 5, 30].as_slice());

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_create_bst_from_into_vec() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(10);
        expected_bst.insert(20);
        expected_bst.insert(5);
        expected_bst.insert(30);

        let actual_bst: IterativeBST<i32> = vec![10, 20, 5, 30].into();

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_extend_bst_from_iter() {
        let vec = vec![8, 1, 10];
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(3);
        expected_bst.insert(2);
        expected_bst.insert(5);
        expected_bst.insert(8);
        expected_bst.insert(1);
        expected_bst.insert(10);
        let mut actual_bst = IterativeBST::new();
        actual_bst.insert(3);
        actual_bst.insert(2);
        actual_bst.insert(5);

        actual_bst.extend(vec.into_iter());

        assert_eq!(actual_bst.size(), 6);
        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_create_bst_from_iter() {
        let mut expected_bst = IterativeBST::new();
        expected_bst.insert(3);
        expected_bst.insert(2);
        expected_bst.insert(5);
        expected_bst.insert(8);
        expected_bst.insert(1);
        expected_bst.insert(10);

        let actual_bst = IterativeBST::from_iter(vec![3, 2, 5, 8, 1, 10].into_iter());

        assert_eq!(actual_bst, expected_bst);
    }

    #[test]
    fn successfully_clone_bst() {
        let mut expected_bst = IterativeBST::new();
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
    fn successfully_clone_into_another_bst() {
        let mut actual_bst = IterativeBST::new();
        actual_bst.insert(3);
        actual_bst.insert(2);
        let mut expected_bst = IterativeBST::new();
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
