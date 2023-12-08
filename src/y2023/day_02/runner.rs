use regex::Regex;
use std::collections::HashMap;

pub use crate::exercise::DayExercise;

pub struct Runner<'a> {
    bucket_limits: HashMap<&'a str, u32>,
}

impl<'a> Runner<'a> {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {
            bucket_limits: HashMap::from([("red", 12), ("green", 13), ("blue", 14)]),
        })
    }

    pub fn get_bucket_amount(&self, capture: regex::Captures<'a>) -> (u32, &'a str) {
        let (_, [amount, bucket]) = capture.extract();

        return (amount.parse::<u32>().unwrap(), bucket);
    }

    pub fn surpass_limit(&self, bucket: &str, amount: u32) -> bool {
        let limit = self.bucket_limits.get(bucket).copied().unwrap();

        return amount > limit;
    }
}

impl<'a> DayExercise for Runner<'a> {
    fn run_part_one(&self, input: &String) {
        let mut possible_games_ids_sum = 0;

        let regex = Regex::new(r"\s(\d+)\s(\w+)").unwrap();

        for (game_id, game) in input.lines().enumerate() {
            let invalid_game = regex
                .captures_iter(game)
                .map(|capture| self.get_bucket_amount(capture))
                .any(|(amount, bucket)| self.surpass_limit(bucket, amount));

            if !invalid_game {
                possible_games_ids_sum += game_id + 1;
            }
        }

        println!("Total is {possible_games_ids_sum}");
    }

    fn run_part_two(&self, input: &String) {
        let regex = Regex::new(r"\s(\d+)\s(\w+)").unwrap();
        let mut power_sum = 0;

        for (_, game) in input.lines().enumerate() {
            let mut minimum_buckets_amount = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

            regex
                .captures_iter(game)
                .map(|capture| self.get_bucket_amount(capture))
                .for_each(|(amount, bucket)| {
                    minimum_buckets_amount
                        .entry(&bucket)
                        .and_modify(|k| *k = if amount > *k { amount } else { *k });
                });

            power_sum += minimum_buckets_amount.values().product::<u32>();
        }

        println!("Total is {power_sum}");
    }
}
