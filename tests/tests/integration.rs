use tests::functions::unit;

mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(unit::add(1, 2), 3);
}
