fn main() {
    println!("{}", run_app());
}

fn run_app() -> String {
    "Hello World!".to_string()
}

#[cfg(test)]
mod test {
    use super::run_app;

    #[test]
    fn test_run_app() {
        assert!(run_app().contains("Hello World!"));
    }
}
