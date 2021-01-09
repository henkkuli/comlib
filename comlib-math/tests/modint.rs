use comlib_math::*;

#[test]
fn test_modular_power() {
    assert_eq!(
        ModInt::<Mod1e9p7>::from(10u64).pow(10),
        ModInt::<Mod1e9p7>::from(999999937u64)
    );
    assert_eq!(
        ModInt::<Mod1e9p7>::from(5u64).pow(3),
        ModInt::<Mod1e9p7>::from(125u64)
    );
    assert_eq!(
        ModInt::<Mod1e9p7>::from(5u64).pow(0),
        ModInt::<Mod1e9p7>::from(1u64)
    );
    assert_eq!(
        ModInt::<Mod1e9p7>::from(0u64).pow(0),
        ModInt::<Mod1e9p7>::from(1u64)
    );
}
