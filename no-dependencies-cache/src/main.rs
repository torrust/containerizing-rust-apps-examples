use doga_english_greetings;
use rnglib::{RNG, Language};

fn main() {
    let rng = RNG::try_from(&Language::Elven).unwrap();
    println!("{}", doga_english_greetings::random_greeting(&rng.generate_name()).unwrap());
}
