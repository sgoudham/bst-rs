//! This crate contains Recursive & Iterative Binary Search Tree Implementations. All common operations are included along with common traversal iterators.
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
//! as **ownership**, **borrowing**, **generics** and **lifetimes**. I cannot promise that the implementations are
//! particularly efficient, or if they are, it was not at the forefront of my mind.
//!
//! That being said, there are some areas I would love to improve upon which include:
//! - Write idiomatic code.
//! - Effectively use **macro_rules!** to reduce large portions of repetitive code.
//! - Implement a **pretty_print()** function to display the binary search trees nicely.
//! - Implement [Drop] trait for iterative node cleanup.
//! - Pre-allocate space on the heap for nodes to reduce inefficiency of inserts.
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
//! // View pre-order, in-order, post-order and level-order traversals
//! assert_eq!(iterative_bst.pre_order_vec(), vec![&15, &5, &2, &25]);
//! assert_eq!(iterative_bst.in_order_vec(), vec![&2, &5, &15, &25]);
//! assert_eq!(iterative_bst.post_order_vec(), vec![&2, &5, &25, &15]);
//! assert_eq!(iterative_bst.level_order_vec(), vec![&15, &5, &25, &2]);
//!
//! assert_eq!(recursive_bst.pre_order_vec(), vec![&15, &5, &2, &25]);
//! assert_eq!(recursive_bst.in_order_vec(), vec![&2, &5, &15, &25]);
//! assert_eq!(recursive_bst.post_order_vec(), vec![&2, &5, &25, &15]);
//! assert_eq!(recursive_bst.level_order_vec(), vec![&15, &5, &25, &2]);
//!
//! // Compare equality/in-equality of trees
//! assert_eq!(iterative_bst.asc_order_vec(), recursive_bst.asc_order_vec());
//! assert_ne!(iterative_bst, IterativeBST::new());
//! assert_ne!(recursive_bst, RecursiveBST::new());
//! ```

use crate::node::{HeapNode, Node};
use std::vec::IntoIter;

mod node;
mod iterative;
mod recursive;
pub use recursive::RecursiveBST;
pub use iterative::IterativeBST;

/// Creates a [`IterativeBST`] containing the arguments.
///
/// # Important
///
/// If given no arguments this will be equivalent to calling
/// `IterativeBST::new()`
///
/// # Example
/// - Create a [`IterativeBST`] containing a given list of elements:
///
/// ```rust
/// use bst_rs::{BinarySearchTree, IterativeBST, bst};
///
/// let t1 = bst![1, 2, 3];
/// // Which is functionally equivalent to
/// let t2 = IterativeBST::from_iter(vec![1,2,3]);
/// // and produces the following tree
/// //    2
/// //   / \
/// //  1   3
/// assert_eq!(t1, t2);
/// ```
///
/// [`IterativeBST`]: crate::IterativeBST
#[macro_export]
macro_rules! bst {
    () => (
        $crate::IterativeBST::new()
    );
    ($($x:expr),+ $(,)?) => (
        $crate::IterativeBST::from_iter(vec![$($x),+].into_iter())
    );
}

/// A trait containing all the common operations of Binary Search Trees.
///
/// # Examples
/// Examples are extended from crate level "Quick Start"
///
/// ```rust
/// use bst_rs::{BinarySearchTree, IterativeBST, RecursiveBST};
///
/// // Create new empty binary search trees
/// let mut iterative_bst = IterativeBST::new();
/// assert!(iterative_bst.is_empty());///
///
/// let mut recursive_bst = RecursiveBST::new();
/// assert!(recursive_bst.is_empty());
///
/// // Insert elements (no duplicates are allowed)
/// iterative_bst.insert(10);
/// iterative_bst.insert(10);   // Element is not inserted
/// iterative_bst.insert(5);
/// iterative_bst.insert(2);
/// iterative_bst.insert(15);
/// iterative_bst.insert(25);
/// assert_eq!(iterative_bst.size(), 5);
///
/// recursive_bst.insert(10);
/// recursive_bst.insert(10);   // Element is not inserted
/// recursive_bst.insert(5);
/// recursive_bst.insert(2);
/// recursive_bst.insert(15);
/// recursive_bst.insert(25);
/// assert_eq!(recursive_bst.size(), 5);
///
/// // Check if element exists
/// assert!(iterative_bst.contains(&5));    // true
/// assert!(!iterative_bst.contains(&0));   // false
///
/// assert!(recursive_bst.contains(&5));    // true
/// assert!(!recursive_bst.contains(&0));   // false
///
/// // Remove elements
/// iterative_bst.remove(&10);
/// iterative_bst.remove(&50);              // No change to tree as element does not exist
/// assert_eq!(iterative_bst.size(), 4);
///
/// recursive_bst.remove(&10);
/// recursive_bst.remove(&50);              // No change to tree as element does not exist
/// assert_eq!(recursive_bst.size(), 4);
///
/// // Get height of tree
/// assert_eq!(iterative_bst.height(), Some(2));
/// assert_eq!(recursive_bst.height(), Some(2));
///
/// // Get minimum element of tree
/// assert_eq!(iterative_bst.min(), Some(&2));
/// assert_eq!(recursive_bst.min(), Some(&2));
///
/// // Get maximum element of tree
/// assert_eq!(iterative_bst.max(), Some(&25));
/// assert_eq!(recursive_bst.max(), Some(&25));
///
/// // Retrieve reference to element in tree
/// assert_eq!(iterative_bst.retrieve(&5), Some(&5));
/// assert_eq!(iterative_bst.retrieve(&100), None); // Element does not exist so None is returned
/// assert_eq!(recursive_bst.retrieve(&5), Some(&5));
/// assert_eq!(recursive_bst.retrieve(&100), None); // Element does not exist so None is returned
///
/// // View pre-order, in-order, post-order and level-order traversals
/// assert_eq!(iterative_bst.pre_order_vec(), vec![&15, &5, &2, &25]);
/// assert_eq!(iterative_bst.in_order_vec(), vec![&2, &5, &15, &25]);
/// assert_eq!(iterative_bst.post_order_vec(), vec![&2, &5, &25, &15]);
/// assert_eq!(iterative_bst.level_order_vec(), vec![&15, &5, &25, &2]);
///
/// assert_eq!(recursive_bst.pre_order_vec(), vec![&15, &5, &2, &25]);
/// assert_eq!(recursive_bst.in_order_vec(), vec![&2, &5, &15, &25]);
/// assert_eq!(recursive_bst.post_order_vec(), vec![&2, &5, &25, &15]);
/// assert_eq!(recursive_bst.level_order_vec(), vec![&15, &5, &25, &2]);
///
/// // Compare equality/in-equality of trees
/// assert_eq!(iterative_bst.asc_order_vec(), recursive_bst.asc_order_vec());
/// assert_ne!(iterative_bst, IterativeBST::new());
/// assert_ne!(recursive_bst, RecursiveBST::new());
/// ```
pub trait BinarySearchTree<T: Ord> {
    /// Returns the total **number of nodes** within the tree.
    fn size(&self) -> usize;

    /// Returns `true` if the binary search tree contains no nodes.
    fn is_empty(&self) -> bool;

    /// Returns `true` if the binary search tree contains one or more nodes.
    fn is_not_empty(&self) -> bool;

    /// Inserts given value as a node.
    ///
    /// **Duplicate values are _not allowed_**.
    fn insert(&mut self, value: T);

    /// Returns `true` if the binary search tree contains an element with the given value.
    fn contains(&self, value: &T) -> bool;

    /// Removes the given value.
    ///
    /// Tree will not be modified if trying to remove element that does not exist.
    fn remove(&mut self, value: &T);

    /// Returns a reference to the element or `None` if element does not exist.
    fn retrieve(&self, value: &T) -> Option<&T>;

    /// Returns a mutable reference to the element (see [`retrieve`](Self::retrieve()))
    /// or `None` if element does not exist.
    fn retrieve_as_mut(&mut self, value: &T) -> Option<&mut T>;

    /// Returns the **height** or `None` if tree is empty.
    ///
    /// The height is the number of edges between the root and it's furthest leaf node.
    ///
    /// # Example
    ///
    /// Given a tree that looks like:
    ///
    /// ```text
    ///           4
    ///         /  \
    ///        2    6
    ///       / \  / \
    ///      1  3 5   7
    /// ```
    ///
    /// The height is: **2**
    fn height(&self) -> Option<isize>;

    /// Returns a reference to the minimum element of the tree or `None` if tree is empty.
    fn min(&self) -> Option<&T>;

    /// Returns a reference to the maximum element of the tree or `None` if tree is empty.
    fn max(&self) -> Option<&T>;

    /// Removes and returns the minimum element from the tree or `None` if tree is empty.
    fn remove_min(&mut self) -> Option<T>;

    /// Removes and returns the maximum element from the tree or `None` if tree is empty.
    fn remove_max(&mut self) -> Option<T>;

    /// Returns references to the elements of the tree in **ascending order.**
    ///
    /// # Important
    ///
    /// This function is analogous to [in_order_vec](Self::in_order_vec()) as the underlying
    /// behaviour is **_exactly the same_.**
    fn asc_order_vec(&self) -> Vec<&T>;

    /// Returns references to the elements of the tree in the order of a **pre-order traversal.**
    ///
    /// # Example
    ///
    /// Given a tree that looks like:
    /// ```text
    ///           4
    ///         /  \
    ///        2    6
    ///       / \  / \
    ///      1  3 5   7
    /// ```
    /// The pre_order_vec is: **[&4, &2, &1, &3, &6, &5, &7].**
    fn pre_order_vec(&self) -> Vec<&T>;

    /// Returns references to the elements of the tree in the order of an **in-order traversal.**
    ///
    /// # Important
    ///
    /// This function is analogous to [asc_order_vec](Self::asc_order_vec()) as the underlying
    /// behaviour is **_exactly the same_.**
    ///
    /// # Example
    ///
    /// Given a tree that looks like:
    /// ```text
    ///           4
    ///         /  \
    ///        2    6
    ///       / \  / \
    ///      1  3 5   7
    /// ```
    /// The in_order_vec is: **[&1, &2, &3, &4, &5, &6, &7].**
    fn in_order_vec(&self) -> Vec<&T>;

    /// Returns references to the elements of the tree in the order of a **post-order traversal.**
    ///
    /// # Example
    ///
    /// Given a tree that looks like:
    /// ```text
    ///           4
    ///         /  \
    ///        2    6
    ///       / \  / \
    ///      1  3 5   7
    /// ```
    /// The post_order_vec is: **[&1, &3, &2, &5, &7, &6, &4].**
    fn post_order_vec(&self) -> Vec<&T>;

    /// Returns references to the elements of the tree in the order of a **level-order traversal.**
    ///
    /// # Example
    ///
    /// Given a tree that looks like:
    /// ```text
    ///           4
    ///         /  \
    ///        2    6
    ///       / \  / \
    ///      1  3 5   7
    /// ```
    /// The post_order_vec is: **[&4, &2, &6, &1, &3, &5, &7].**
    fn level_order_vec(&self) -> Vec<&T>;

    /// Returns an iterator over [asc_order_vec](Self::asc_order_vec()).
    ///
    /// # Important
    ///
    /// This function is analogous to [in_order_iter](Self::in_order_iter()) as the underlying
    /// behaviour is **_exactly the same_.**
    fn asc_order_iter(&self) -> IntoIter<&T>;

    /// Returns an iterator over [pre_order_vec](Self::pre_order_vec()).
    fn pre_order_iter(&self) -> IntoIter<&T>;

    /// Returns an iterator over [in_order_vec](Self::in_order_vec()).
    ///
    /// # Important
    ///
    /// This function is analogous to [asc_order_iter](Self::asc_order_iter()) as the underlying
    /// behaviour is **_exactly the same_.**
    fn in_order_iter(&self) -> IntoIter<&T>;

    /// Returns an iterator over [post_order_vec](Self::post_order_vec()).
    fn post_order_iter(&self) -> IntoIter<&T>;

    /// Returns an iterator over [level_order_vec](Self::level_order_vec()).
    fn level_order_iter(&self) -> IntoIter<&T>;

    /// Returns [asc_order_iter](Self::asc_order_iter()) **AND** consumes the tree.
    ///
    /// # Important
    ///
    /// This function is analogous to [into_in_order_iter](Self::into_in_order_iter()) as the
    /// underlying behaviour is **_exactly the same_.**
    fn into_asc_order_iter(self) -> IntoIter<T>;

    /// Returns [pre_order_iter](Self::pre_order_iter()) **AND** consumes the tree.
    fn into_pre_order_iter(self) -> IntoIter<T>;

    /// Returns [in_order_iter](Self::in_order_iter()) **AND** consumes the tree.
    ///
    /// # Important
    ///
    /// This function is analogous to [into_asc_order_iter](Self::into_asc_order_iter()) as the
    /// underlying behaviour is **_exactly the same_.**
    fn into_in_order_iter(self) -> IntoIter<T>;

    /// Returns [post_order_iter](Self::post_order_iter()) **AND** consumes the tree.
    fn into_post_order_iter(self) -> IntoIter<T>;

    /// Returns [level_order_iter](Self::level_order_iter()) **AND** consumes the tree.
    fn into_level_order_iter(self) -> IntoIter<T>;
}
