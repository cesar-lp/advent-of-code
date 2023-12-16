use crate::exercise::concatenate;
pub use crate::exercise::DayExercise;

#[derive(Eq, PartialEq)]
enum ParsingMode {
    SplitWhiteSpace,
    Concatenate,
}

pub struct Runner {}

impl<'a> Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }

    fn extract_numbers(&self, input: &str, parsing_mode: &ParsingMode) -> Vec<u32> {
        let splitted_str = input
            .lines()
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .collect::<Vec<&str>>();

        return if parsing_mode == &ParsingMode::SplitWhiteSpace {
            let splitted_str: Vec<&str> = splitted_str.get(0).unwrap().split(" ").collect();

            splitted_str
                .iter()
                .filter_map(|c| c.parse::<u32>().ok())
                .collect()
        } else {
            splitted_str
                .get(0)
                .unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        };
    }

    fn parse_input(&self, input: &str, parsing_mode: ParsingMode) -> (Vec<u32>, Vec<u32>) {
        let times_input = input.lines().next().unwrap();
        let distance_input = input.lines().skip(1).next().unwrap();

        return (
            self.extract_numbers(times_input, &parsing_mode),
            self.extract_numbers(distance_input, &parsing_mode),
        );
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let (times, distances) = self.parse_input(input, ParsingMode::SplitWhiteSpace);

        let mut total = 1;

        for (idx, time) in times.iter().enumerate() {
            let mut number_of_ways = 0;

            for time_hold in 0..=*time {
                let distance_reached = (time - time_hold) * time_hold;

                if &distance_reached > distances.get(idx).unwrap() {
                    number_of_ways += 1;
                }
            }

            total *= number_of_ways;
        }

        println!("Total is {total}");
    }

    fn run_part_two(&self, input: &String) {
        let (times, distances) = self.parse_input(input, ParsingMode::Concatenate);

        let time = concatenate(times);
        let distance = concatenate(distances);

        let mut total: u32 = 0;

        for time_hold in 0..time {
            let distance_reached: u64 = (time - time_hold) * time_hold;

            if distance_reached > distance {
                total += 1
            }
        }

        println!("Total is {total}");
    }
}
