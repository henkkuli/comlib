use comlib_range::Bit;

#[test]
fn test_bit_sum() {
    let bit = Bit::from(vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(bit.sum(0..0), 0);
    assert_eq!(bit.sum(..=0), 1);
    assert_eq!(bit.sum(..=1), 3);
    assert_eq!(bit.sum(..=2), 6);
    assert_eq!(bit.sum(..=3), 10);
    assert_eq!(bit.sum(..=4), 15);
    assert_eq!(bit.sum(..=5), 21);
    assert_eq!(bit.sum(..=6), 28);

    assert_eq!(bit.sum(5..=5), 6);
    assert_eq!(bit.sum(1..=5), 20);

    assert_eq!(bit.sum(..), 28);
    assert_eq!(bit.sum(2..), 25);
    assert_eq!(bit.sum(..3), 6);
}

#[test]
fn test_bit_add() {
    let mut bit = Bit::from(vec![1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(bit.sum(0..7), 28);
    bit.add(0, 3);
    assert_eq!(bit.sum(0..7), 31);
    bit.sub(5, 2);
    assert_eq!(bit.sum(1..6), 18);
}
