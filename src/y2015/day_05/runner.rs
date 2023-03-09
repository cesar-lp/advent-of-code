use fancy_regex::Regex as FancyRegex;
use rayon::{prelude::ParallelIterator, str::ParallelString};
use regex::Regex;

pub use crate::exercise::DayExercise;
pub struct Runner {}

impl Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }

    pub fn is_nice(&self, input: &str) -> bool {
        let vowels_regex = Regex::new(r"[aeiou]").unwrap();
        let cc_regex = FancyRegex::new(r"([a-z])\1").unwrap();
        let invalid_cc_regex = Regex::new(r"ab|cd|pq|xy").unwrap();

        let has_enough_vowels = vowels_regex.captures_iter(input).count() >= 3;
        let has_valid_cc = cc_regex.is_match(input).unwrap();
        let has_not_invalid_cc = !invalid_cc_regex.is_match(input);

        has_enough_vowels && has_valid_cc && has_not_invalid_cc
    }

    fn is_nice_v2(&self, input: &str) -> bool {
        let sandwich_regex = FancyRegex::new(r"([a-z])[a-z]\1").unwrap();
        let pair_regex = FancyRegex::new(r"([a-z][a-z])[a-z]*\1").unwrap();

        let has_sandwich_char = sandwich_regex.is_match(input).unwrap();
        let has_char_pair = pair_regex.captures_iter(input).count() > 0;

        has_sandwich_char && has_char_pair
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        input.par_lines().filter(|line| self.is_nice(line)).count();
    }

    fn run_part_two(&self, input: &String) {
        input
            .par_lines()
            .filter(|line| self.is_nice_v2(line))
            .count();
    }
}
