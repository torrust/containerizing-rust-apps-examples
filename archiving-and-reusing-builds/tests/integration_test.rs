use archiving_and_reusing_builds::run_app;

#[test]
fn test_app() {
    assert_eq!("Hello World!", run_app());
}