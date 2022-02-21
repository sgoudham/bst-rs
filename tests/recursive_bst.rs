use std::vec::IntoIter;

use bst_rs::{BinarySearchTree, RecursiveBST};

#[test]
fn successfully_insert_elements_into_bst() {
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(0);
    expected_bst.insert(1);
    expected_bst.insert(2);
    expected_bst.insert(-20);

    let mut actual_bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
    assert!(bst.is_empty());

    bst.insert(1);
    assert!(!bst.is_empty());
}

#[test]
fn check_if_bst_is_not_empty() {
    let mut bst = RecursiveBST::new();
    assert!(!bst.is_not_empty());

    bst.insert(1);
    assert!(bst.is_not_empty());
}

#[test]
fn check_if_bst_contains_elements() {
    let mut bst = RecursiveBST::new();
    assert!(!bst.contains(&10));

    bst.insert(1);
    bst.insert(5);

    assert!(!bst.contains(&10));
    assert!(bst.contains(&1));
    assert!(bst.contains(&5));
}

#[test]
fn successfully_remove_root_node_from_bst() {
    let mut bst = RecursiveBST::new();
    bst.insert(0);

    bst.remove(&0);

    assert!(bst.is_empty());
    assert_eq!(bst.size(), 0)
}

#[test]
fn successfully_remove_leaf_node() {
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(5);
    expected_bst.insert(4);
    expected_bst.insert(6);
    let mut actual_bst = RecursiveBST::new();
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
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(5);
    expected_bst.insert(4);
    expected_bst.insert(7);
    expected_bst.insert(8);
    let mut actual_bst = RecursiveBST::new();
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
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(5);
    expected_bst.insert(3);
    expected_bst.insert(2);
    expected_bst.insert(6);
    let mut actual_bst = RecursiveBST::new();
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
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(10);
    expected_bst.insert(3);
    expected_bst.insert(8);
    expected_bst.insert(15);
    let mut actual_bst = RecursiveBST::new();
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
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(10);
    expected_bst.insert(5);
    expected_bst.insert(8);
    expected_bst.insert(3);
    expected_bst.insert(15);

    let mut actual_bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
    bst.insert(5);
    bst.insert(10);

    let retrieved_value = bst.retrieve(&5);
    let invalid_value = bst.retrieve(&15);

    assert_eq!(retrieved_value, Some(&5));
    assert_eq!(invalid_value, None);
}

#[test]
fn successfully_retrieve_element_as_mut_and_modify_bst() {
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(10);
    expected_bst.insert(2);

    let mut actual_bst = RecursiveBST::new();
    actual_bst.insert(10);
    actual_bst.insert(5);

    let _retrieved_value_as_mut: &mut i32 = actual_bst.retrieve_as_mut(&5).unwrap();
    *_retrieved_value_as_mut = 2;

    assert_eq!(actual_bst, expected_bst);
}

#[test]
fn successfully_get_height_of_bst() {
    let mut bst = RecursiveBST::new();
    assert_eq!(bst.height(), 0);

    bst.insert(15);
    bst.insert(10);
    bst.insert(20);
    bst.insert(8);
    bst.insert(12);
    bst.insert(16);
    bst.insert(25);
    assert_eq!(bst.height(), 3);
}

#[test]
fn successfully_get_min_from_bst() {
    let mut bst = RecursiveBST::new();
    assert_eq!(bst.min(), None);

    bst.insert(5);
    bst.insert(3);
    bst.insert(1);
    bst.insert(15);

    assert_eq!(bst.min(), Some(&1));
}

#[test]
fn successfully_get_max_from_bst() {
    let mut bst = RecursiveBST::new();
    assert_eq!(bst.max(), None);

    bst.insert(5);
    bst.insert(12);
    bst.insert(1);
    bst.insert(15);

    assert_eq!(bst.max(), Some(&15));
}

#[test]
fn successfully_remove_min_from_bst() {
    let mut bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
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
    let bst: RecursiveBST<i32> = RecursiveBST::new();

    let mut pre_order_traversal = bst.into_pre_order_iter();

    assert_eq!(pre_order_traversal.next(), None);
}

#[test]
fn into_pre_order_iter_with_one_element() {
    let mut bst = RecursiveBST::new();
    bst.insert(3);

    let mut pre_order_traversal = bst.into_pre_order_iter();

    assert_eq!(pre_order_traversal.next(), Some(3));
    assert_eq!(pre_order_traversal.next(), None);
}

#[test]
fn into_pre_order_iter() {
    let mut iter: IntoIter<i32> = RecursiveBST::new().into_pre_order_iter();
    assert_eq!(iter.next(), None);

    let mut bst = RecursiveBST::new();
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
    let bst: RecursiveBST<i32> = RecursiveBST::new();

    let mut in_order_traversal = bst.into_in_order_iter();

    assert_eq!(in_order_traversal.next(), None);
}

#[test]
fn into_in_order_iter_with_one_element() {
    let mut bst = RecursiveBST::new();
    bst.insert(3);

    let mut in_order_traversal = bst.into_in_order_iter();

    assert_eq!(in_order_traversal.next(), Some(3));
    assert_eq!(in_order_traversal.next(), None);
}

#[test]
fn into_in_order_iter() {
    let another_bst: RecursiveBST<i32> = RecursiveBST::new();
    let mut iter = another_bst.into_in_order_iter();
    assert_eq!(iter.next(), None);

    let mut bst = RecursiveBST::new();
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
    let bst: RecursiveBST<i32> = RecursiveBST::new();

    let mut post_order_traversal = bst.into_post_order_iter();

    assert_eq!(post_order_traversal.next(), None);
}

#[test]
fn into_post_order_iter_with_one_element() {
    let mut bst = RecursiveBST::new();
    bst.insert(3);

    let mut post_order_traversal = bst.into_post_order_iter();

    assert_eq!(post_order_traversal.next(), Some(3));
    assert_eq!(post_order_traversal.next(), None);
}

#[test]
fn into_post_order_iter_with_many_elements() {
    let mut bst = RecursiveBST::new();
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
    let bst: RecursiveBST<i32> = RecursiveBST::new();

    let mut level_order_traversal = bst.into_level_order_iter();

    assert_eq!(level_order_traversal.next(), None);
}

#[test]
fn into_level_order_iter_with_one_element() {
    let mut bst = RecursiveBST::new();
    bst.insert(3);

    let mut level_order_traversal = bst.into_level_order_iter();

    assert_eq!(level_order_traversal.next(), Some(3));
    assert_eq!(level_order_traversal.next(), None);
}

#[test]
fn into_level_order_iter_with_many_elements() {
    let mut bst = RecursiveBST::new();
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
fn successfully_get_sorted_vec() {
    let bst: RecursiveBST<i32> = RecursiveBST::new();
    assert!(bst.sorted_vec().is_empty());

    let mut bst = RecursiveBST::new();
    bst.insert(3);
    bst.insert(4);
    bst.insert(5);
    bst.insert(1);
    bst.insert(2);

    assert_eq!(bst.sorted_vec(), vec![&1, &2, &3, &4, &5]);
}

#[test]
fn successfully_transfer_bst_into_sorted_vec() {
    let mut bst = RecursiveBST::new();
    bst.insert(3);
    bst.insert(4);
    bst.insert(5);
    bst.insert(1);
    bst.insert(2);

    assert_eq!(bst.into_sorted_vec(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn successfully_get_pre_order_vec() {
    let mut bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
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
    let mut bst = RecursiveBST::new();
    assert!(bst.level_order_vec().is_empty());

    bst.insert(15);
    bst.insert(20);
    bst.insert(10);
    bst.insert(8);
    bst.insert(12);
    bst.insert(16);
    bst.insert(25);

    assert_eq!(bst.level_order_vec(), vec![&15, &10, &20, &8, &12, &16, &25]);
}

#[test]
fn successfully_create_bst_from_vec() {
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(10);
    expected_bst.insert(20);
    expected_bst.insert(5);
    expected_bst.insert(30);

    let actual_bst = RecursiveBST::from(vec![10, 20, 5, 30]);

    assert_eq!(actual_bst, expected_bst);
}

#[test]
fn successfully_create_bst_from_slice() {
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(10);
    expected_bst.insert(20);
    expected_bst.insert(5);
    expected_bst.insert(30);

    let actual_bst = RecursiveBST::from(vec![10, 20, 5, 30].as_slice());

    assert_eq!(actual_bst, expected_bst);
}

#[test]
fn successfully_create_bst_from_into_vec() {
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(10);
    expected_bst.insert(20);
    expected_bst.insert(5);
    expected_bst.insert(30);

    let actual_bst: RecursiveBST<i32> = vec![10, 20, 5, 30].into();

    assert_eq!(actual_bst, expected_bst);
}

#[test]
fn successfully_extend_bst_from_iter() {
    let vec = vec![8, 1, 10];
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(3);
    expected_bst.insert(2);
    expected_bst.insert(5);
    expected_bst.insert(8);
    expected_bst.insert(1);
    expected_bst.insert(10);
    let mut actual_bst = RecursiveBST::new();
    actual_bst.insert(3);
    actual_bst.insert(2);
    actual_bst.insert(5);

    actual_bst.extend(vec.into_iter());

    assert_eq!(actual_bst.size(), 6);
    assert_eq!(actual_bst, expected_bst);
}

#[test]
fn successfully_create_bst_from_iter() {
    let mut expected_bst = RecursiveBST::new();
    expected_bst.insert(3);
    expected_bst.insert(2);
    expected_bst.insert(5);
    expected_bst.insert(8);
    expected_bst.insert(1);
    expected_bst.insert(10);

    let actual_bst = RecursiveBST::from_iter(vec![3, 2, 5, 8, 1, 10].into_iter());

    assert_eq!(actual_bst, expected_bst);
}

#[test]
fn successfully_clone_bst() {
    let mut expected_bst = RecursiveBST::new();
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
    let mut actual_bst = RecursiveBST::new();
    actual_bst.insert(3);
    actual_bst.insert(2);
    let mut expected_bst = RecursiveBST::new();
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