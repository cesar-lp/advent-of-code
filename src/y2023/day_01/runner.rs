use std::{
    collections::HashMap,
    ops::{Add, Mul},
};

pub use crate::exercise::DayExercise;

#[derive(PartialEq)]
enum TraversalMode {
    Normal,
    Reverse,
}

pub struct Runner {}

impl Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }

    fn get_digit(
        &self,
        char_idx: (usize, char),
        digits_names: &HashMap<&str, u32>,
        line: &str,
        mode: TraversalMode,
    ) -> Option<u32> {
        let (idx, char) = char_idx;

        let remaining_line_content = if mode == TraversalMode::Normal {
            &line.chars().as_str()[idx..]
        } else {
            &line.chars().as_str()[0..=idx]
        };

        char.to_digit(10).or_else(|| {
            IntoIterator::into_iter([5, 4, 3])
                .map(|position| {
                    if mode == TraversalMode::Normal {
                        remaining_line_content.get(0..position)
                    } else {
                        remaining_line_content
                            .len()
                            .checked_sub(position)
                            .and_then(|start_position| remaining_line_content.get(start_position..))
                    }
                })
                .flatten()
                .find_map(|word| digits_names.get(word))
                .copied()
        })
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let mut total = 0;

        for line in input.lines() {
            let chars: Vec<char> = line.chars().collect();

            total += chars
                .iter()
                .find_map(|c| c.to_digit(10))
                .unwrap_or(0)
                .mul(10)
                .add(chars.iter().rev().find_map(|c| c.to_digit(10)).unwrap_or(0));
        }

        println!("Total is {total}");
    }

    fn run_part_two(&self, input: &String) {
        let mut total: u32 = 0;

        let digits_names: HashMap<&str, u32> = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]);

        for line in input.lines() {
            let first_digit = line
                .char_indices()
                .find_map(|char_idx| {
                    self.get_digit(char_idx, &digits_names, line, TraversalMode::Normal)
                })
                .unwrap();

            let second_digit = line
                .char_indices()
                .rev()
                .find_map(|char_idx| {
                    self.get_digit(char_idx, &digits_names, line, TraversalMode::Reverse)
                })
                .unwrap();

            total += first_digit * 10 + second_digit;
        }

        println!("Total is {total}");
    }
}
