use full_example::run_app;

#[test]
fn test_app() {
    assert!(run_app().contains("!"));
}