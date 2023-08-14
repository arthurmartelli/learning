use test_organization;

pub fn setup() -> test_organization::Tester {
    let name = String::from("Hi");
    return test_organization::Tester::new(name);
}
