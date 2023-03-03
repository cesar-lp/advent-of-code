pub use crate::exercise::DayExercise;

pub struct Runner {}

impl Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let mut _floor = 0;

        for char in input.chars() {
            if char == '(' {
                _floor += 1;
            } else if char == ')' {
                _floor -= 1;
            }
        }
    }

    fn run_part_two(&self, input: &String) {
        let chars_length = input.chars().count();
        let mut chars_iter = input.chars();

        let mut floor = 0;
        let mut i = 0;

        while floor != -1 && i < chars_length {
            if let Some(ch) = chars_iter.next() {
                if ch == '(' {
                    floor += 1;
                } else if ch == ')' {
                    floor -= 1;
                }

                i += 1;
            }
        }

        let _basement_pos = if floor == -1 { i } else { 0 };
    }
}
