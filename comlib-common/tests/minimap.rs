use std::collections::HashMap;

#[test]
fn test_minimap() {
    let mut map = HashMap::new();
    assert_eq!(map.insert("Hello", 1), None);
    assert_eq!(map.get("Hello"), Some(&1));
    assert_eq!(map.insert("Hello", 2), Some(1));
    assert_eq!(map.get("Hello"), Some(&2));
    assert_eq!(map.insert("World", 3), None);
    assert_eq!(map.get("Hello"), Some(&2));
    assert_eq!(map.get("World"), Some(&3));
}
