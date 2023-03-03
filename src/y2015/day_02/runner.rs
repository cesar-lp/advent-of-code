use std::cmp;

pub use crate::exercise::DayExercise;

pub struct Runner {}

impl Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }

    fn get_sides(&self, measurement: Vec<u32>) -> (u32, u32, u32) {
        let l = measurement.get(0).unwrap().to_owned();
        let w = measurement.get(1).unwrap().to_owned();
        let h = measurement.get(2).unwrap().to_owned();

        (l, w, h)
    }

    fn get_smallest_perimeter(&self, l: u32, w: u32, h: u32) -> u32 {
        let perimeter1 = 2 * l + 2 * w;
        let perimeter2 = 2 * l + 2 * h;
        let perimeter3 = 2 * w + 2 * h;

        cmp::min(perimeter1, cmp::min(perimeter2, perimeter3))
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let mut _total = 0;

        for line in input.lines() {
            let measurement: Vec<u32> = line.split("x").map(|m| m.parse().unwrap()).collect();

            let (l, w, h) = self.get_sides(measurement);

            let smallest_area = cmp::min(l * w, cmp::min(l * h, w * h));

            _total += 2 * l * w + 2 * w * h + 2 * h * l + smallest_area;
        }
    }

    fn run_part_two(&self, input: &String) {
        let mut _total = 0;

        for line in input.lines() {
            let measurement: Vec<u32> = line.split("x").map(|m| m.parse().unwrap()).collect();

            let (l, w, h) = self.get_sides(measurement);

            let bow = l * w * h;
            let ribbon = self.get_smallest_perimeter(l, w, h);

            _total += ribbon + bow;
        }
    }
}
