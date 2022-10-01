use bst_rs::bst;

#[test]
fn successfully_construct_bst_from_macro() {
    let mut actual_bst = IterativeBST::new();
    actual_bst.insert(3);
    actual_bst.insert(2);
    let expected_bst = bst![3,2];
    assert_eq!(actual_bst, expected_bst);
}

#[test]
fn verify_permutations_produce_same_tree() {
    let expected_bst = bst![2,3];
    let expected_bst = bst![3,2];
    assert_eq!(actual_bst, expected_bst);
}
