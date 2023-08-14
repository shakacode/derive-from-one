use trybuild::TestCases;

#[test]
fn ui() {
    let t = TestCases::new();

    t.pass("tests/success/*.rs");
    t.compile_fail("tests/failure/*.rs");
}
