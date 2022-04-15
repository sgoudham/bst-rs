# bst-rs

[![build](https://github.com/sgoudham/bst-rs/actions/workflows/build.yml/badge.svg)](https://github.com/sgoudham/bst-rs/actions/workflows/build.yml)
[![crate.io](https://img.shields.io/crates/v/bst-rs)](https://crates.io/crates/bst-rs)
[![downloads](https://img.shields.io/crates/d/bst-rs)](https://crates.io/crates/bst-rs)
[![license](https://img.shields.io/github/license/sgoudham/bst-rs)](LICENSE)

> Recursive & Iterative Binary Search Tree Implementations within Rust

## Table of Contents

- [About](#About)
- [Personal Goals](#Personal-Goals)
- [Quick Start](#Quick-Start)
- [License](#License)
- [Contributing](#Contributing)
- [Inspiration](#Inspiration)

## About

This crate contains Recursive & Iterative Binary Search Tree Implementations. All common operations are included along
with common traversal iterators.

All elements within the Binary Search Trees _must_ implement
the [Ord](https://doc.rust-lang.org/core/cmp/trait.Ord.html) trait.

It is also important to note that [RecursiveBST](src/lib.rs) is more likely to `blow the stack.`
For more information on why that is the case, please have a look at
[The Story of Tail Call Optimizations in Rust.](https://seanchen1991.github.io/posts/tco-story/)

## Personal Goals

I have made this library with the personal goals of learning and solidifying concepts such as `ownership`, `borrowing`
, `generics` and `lifetimes`. I cannot promise that the implementations are particularly efficient, or if they are, it
was not at the forefront of my mind.

That being said, there are some areas I would love to improve upon which include:

- [ ] Write idiomatic code.
- [ ] Effectively use **macro_rules!** to reduce large portions of repetitive code.
- [ ] Implement a **pretty_print()** function to display the binary search trees nicely.
- [ ] Implementing the Drop trait for iterative node cleanup.
- [ ] Pre-allocating space on the heap for nodes to reduce inefficiency of inserts.

I'm more than happy to accept (and encourage) contributions if anyone is kind enough to do so. (Please look
at [CONTRIBUTING!](#Contributing))

## Quick Start

 ```rust
use bst_rs::{BinarySearchTree, IterativeBST, RecursiveBST};

// Create new empty binary search trees
let mut iterative_bst = IterativeBST::new();
assert!(iterative_bst.is_empty());

let mut recursive_bst = RecursiveBST::new();
assert!(recursive_bst.is_empty());

// Insert elements (no duplicates are allowed)
iterative_bst.insert(10);
iterative_bst.insert(10);   // Element is not inserted
iterative_bst.insert(5);
iterative_bst.insert(2);
iterative_bst.insert(15);
iterative_bst.insert(25);
assert_eq!(iterative_bst.size(), 5);

recursive_bst.insert(10);
recursive_bst.insert(10);   // Element is not inserted
recursive_bst.insert(5);
recursive_bst.insert(2);
recursive_bst.insert(15);
recursive_bst.insert(25);
assert_eq!(recursive_bst.size(), 5);

// Check if element exists
assert!(iterative_bst.contains(&5));    // true
assert!(!iterative_bst.contains(&0));   // false

assert!(recursive_bst.contains(&5));    // true
assert!(!recursive_bst.contains(&0));   // false

// Remove elements
iterative_bst.remove(&10);
iterative_bst.remove(&50); // No change to tree as element does not exist
assert_eq!(iterative_bst.size(), 4);

recursive_bst.remove(&10);
recursive_bst.remove(&50); // No change to tree as element does not exist
assert_eq!(recursive_bst.size(), 4);

// Get height of tree
assert_eq!(iterative_bst.height(), Some(2));
assert_eq!(recursive_bst.height(), Some(2));

// Get minimum element of tree
assert_eq!(iterative_bst.min(), Some(&2));
assert_eq!(recursive_bst.min(), Some(&2));

// Get maximum element of tree
assert_eq!(iterative_bst.max(), Some(&25));
assert_eq!(recursive_bst.max(), Some(&25));

// Retrieve reference to element in tree
assert_eq!(iterative_bst.retrieve(&5), Some(&5));
assert_eq!(iterative_bst.retrieve(&100), None); // Element does not exist so None is returned
assert_eq!(recursive_bst.retrieve(&5), Some(&5));
assert_eq!(recursive_bst.retrieve(&100), None); // Element does not exist so None is returned

// View pre-order, in-order, post-order and level-order traversals
assert_eq!(iterative_bst.pre_order_vec(), vec![&15, &5, &2, &25]);
assert_eq!(iterative_bst.in_order_vec(), vec![&2, &5, &15, &25]);
assert_eq!(iterative_bst.post_order_vec(), vec![&2, &5, &25, &15]);
assert_eq!(iterative_bst.level_order_vec(), vec![&15, &5, &25, &2]);

assert_eq!(recursive_bst.pre_order_vec(), vec![&15, &5, &2, &25]);
assert_eq!(recursive_bst.in_order_vec(), vec![&2, &5, &15, &25]);
assert_eq!(recursive_bst.post_order_vec(), vec![&2, &5, &25, &15]);
assert_eq!(recursive_bst.level_order_vec(), vec![&15, &5, &25, &2]);

// Compare equality/in-equality of trees
assert_eq!(iterative_bst.asc_order_vec(), recursive_bst.asc_order_vec());
assert_ne!(iterative_bst, IterativeBST::new());
assert_ne!(recursive_bst, RecursiveBST::new());
 ```

## License

[MIT License](LICENSE)

## Contributing

Please read the [CONTRIBUTING.md](CONTRIBUTING.md) before contributing! (Thank you!)

## Inspiration

The book [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) inspired me
to try and implement Binary Search Trees within the language. I had also been wanting to create my first library for
other crates to use.
