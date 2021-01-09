use comlib_math::next_permutation;

#[test]
fn test_next_permutation() {
    let mut perm = [1, 2, 3];
    assert!(next_permutation(&mut perm));
    assert_eq!(perm, [1, 3, 2]);
    assert!(next_permutation(&mut perm));
    assert_eq!(perm, [2, 1, 3]);
    assert!(next_permutation(&mut perm));
    assert_eq!(perm, [2, 3, 1]);
    assert!(next_permutation(&mut perm));
    assert_eq!(perm, [3, 1, 2]);
    assert!(next_permutation(&mut perm));
    assert_eq!(perm, [3, 2, 1]);
    assert!(!next_permutation(&mut perm));
    assert_eq!(perm, [1, 2, 3]);
}

#[test]
fn test_next_permutation_duplicate_elements() {
    let mut perm = [1, 2, 2];
    assert!(next_permutation(&mut perm));
    assert_eq!(perm, [2, 1, 2]);
    assert!(next_permutation(&mut perm));
    assert_eq!(perm, [2, 2, 1]);
    assert!(!next_permutation(&mut perm));
    assert_eq!(perm, [1, 2, 2]);
}
