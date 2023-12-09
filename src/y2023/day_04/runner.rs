use std::collections::{HashMap, HashSet};

pub use crate::exercise::DayExercise;

pub struct Runner {}

impl<'a> Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }

    fn parse_numbers(&self, input: &str) -> HashSet<u32> {
        return input
            .split(" ")
            .into_iter()
            .filter_map(|value| value.parse().ok())
            .collect();
    }

    fn get_card_numbers(&self, card: &str) -> (HashSet<u32>, HashSet<u32>) {
        let input: Vec<&str> = card
            .split(": ")
            .skip(1)
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
            .split(" | ")
            .collect();

        let winning_numbers = self.parse_numbers(input.get(0).unwrap());
        let numbers_in_possesion = self.parse_numbers(input.get(1).unwrap());

        return (winning_numbers, numbers_in_possesion);
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let base: u32 = 2;
        let mut total = 0;

        for card in input.lines() {
            let (winning_numbers, in_possesion) = self.get_card_numbers(card);

            let power = winning_numbers
                .iter()
                .map(|number| if in_possesion.contains(number) { 1 } else { 0 })
                .sum::<u32>();

            total += base.pow(power) / base;
        }

        println!("Total is {total}");
    }

    fn run_part_two(&self, input: &String) {
        let original_scratchcards_amount = input.lines().count() as u32;
        let mut scratchcards_copies: HashMap<usize, u32> = HashMap::from([]);

        let scratchcards: Vec<(HashSet<u32>, HashSet<u32>)> = input
            .lines()
            .map(|card| self.get_card_numbers(card))
            .collect();

        for (card_idx, card) in scratchcards.iter().enumerate() {
            let (winning_numbers, in_possesion) = card;

            let matches = winning_numbers
                .iter()
                .map(|number| if in_possesion.contains(number) { 1 } else { 0 })
                .sum();

            if matches == 0 {
                continue;
            }

            let current_card_copies = scratchcards_copies
                .entry(card_idx + 1)
                .or_insert(0)
                .to_owned();

            let matches: Vec<usize> = (1..=matches).map(|x| x as usize).collect();

            for match_number in matches {
                let idx = card_idx + match_number + 1;

                if let Some(_) = scratchcards.iter().nth(idx) {
                    let copies = scratchcards_copies.entry(idx).or_insert(0);
                    *copies += 1 + current_card_copies;
                }
            }
        }

        let total: u32 = original_scratchcards_amount + scratchcards_copies.values().sum::<u32>();

        println!("Total is {total}");
    }
}
