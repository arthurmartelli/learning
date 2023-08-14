use test_organization;

mod common;

#[test]
fn it_adds_tow() {
    let tester: test_organization::Tester = common::setup();
    assert_eq!(2, test_organization::add_two(0));
    assert_eq!(tester.name(), "Hi")
}
