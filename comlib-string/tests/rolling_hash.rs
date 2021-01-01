use comlib_math::Mod1e9p7;
use comlib_string::RollingHash;

#[test]
fn test_rolling_hash() {
    let mut hash: RollingHash<Mod1e9p7> = RollingHash::new("abcxyzabc");
    assert_eq!(hash.get_hash(0..=2), hash.get_hash(6..));
    assert_ne!(hash.get_hash(3..=5), hash.get_hash(6..));
    // Change the string to "abdxyzabc"
    hash.set_char(2, 'd');
    assert_ne!(hash.get_hash(0..=2), hash.get_hash(6..));
    assert_ne!(hash.get_hash(3..=5), hash.get_hash(6..));
    assert_eq!(hash.get_hash(0..=1), hash.get_hash(6..8));
    // Change the string to "abdxyzabd"
    hash.set_char(8, 'd');
    assert_eq!(hash.get_hash(0..=2), hash.get_hash(6..));
    assert_ne!(hash.get_hash(3..=5), hash.get_hash(6..));
    assert_eq!(hash.get_hash(0..=1), hash.get_hash(6..8));
}
