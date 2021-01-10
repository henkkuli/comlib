use comlib_math::subsets;

#[test]
fn test_subsets() {
    let items = [1, 2, 3];

    let subsets: Vec<Vec<_>> = subsets(items.len())
        .map(|subset| subset.select(&items).copied().collect())
        .collect();

    assert_eq!(
        subsets,
        [
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3]
        ]
    );
}

#[test]
fn test_subset_is_empty() {
    let mut subsets = subsets(3);
    assert!(subsets.next().unwrap().is_empty());
    assert!(!subsets.next().unwrap().is_empty());
    assert!(!subsets.next().unwrap().is_empty());
    assert!(!subsets.next().unwrap().is_empty());
    assert!(!subsets.next().unwrap().is_empty());
    assert!(!subsets.next().unwrap().is_empty());
    assert!(!subsets.next().unwrap().is_empty());
    assert!(!subsets.next().unwrap().is_empty());
    assert!(subsets.next().is_none());
}

#[test]
fn test_subset_contains() {
    let items = [0, 1, 2];

    for subset in subsets(items.len()) {
        let elements: Vec<_> = subset.select(&items).copied().collect();
        for &item in &items {
            if elements.contains(&item) {
                assert!(subset.contains(item));
            } else {
                assert!(!subset.contains(item));
            }
        }
    }
}
