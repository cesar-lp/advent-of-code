use md5;
use rayon::prelude::*;

pub use crate::exercise::{concatenate_str, DayExercise};
pub struct Runner {}

impl Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }

    pub fn search(&self, input: &String, prefix: &str) -> usize {
        (0..1000000000)
            .into_par_iter()
            .position_first(|n| {
                let hash_input = concatenate_str(input.clone(), n.to_string());
                let digest = format!("{:?}", md5::compute(hash_input));
                digest.starts_with(prefix)
            })
            .unwrap()
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        self.search(input, "00000");
    }

    fn run_part_two(&self, input: &String) {
        self.search(input, "000000");
    }
}
