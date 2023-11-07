use doga_english_greetings;
use rnglib::{RNG, Language};

pub fn run_app() -> String {
    let rng = RNG::try_from(&Language::Elven).unwrap();
    doga_english_greetings::random_greeting(&rng.generate_name()).expect("random greeting to be generated")
}

#[cfg(test)]
mod test {
    use super::run_app;

    #[test]
    fn test_run_app() {
        assert!(run_app().contains("!"));
    }
}
