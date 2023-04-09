use test_org;

mod common;

#[test]
fn it_adds_2 () {
    common::setup();
    assert_eq!(4, test_org::add_two(2));
}
