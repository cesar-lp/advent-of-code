pub use crate::exercise::DayExercise;

pub struct Runner {}

impl<'a> Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }

    pub fn is_symbol(&self, char: char) -> bool {
        return char != '.' && !char.is_digit(10);
    }

    pub fn is_gear(&self, char: char) -> bool {
        return char == '*';
    }

    fn get_diagonal_numbers(&self, line: Option<&str>, symbol_idx: usize) -> (u32, u32) {
        if let None = line {
            return (0, 0);
        }

        let line = line.unwrap();
        let diagonal_windows: Vec<usize> = vec![3, 2, 1];

        let diagonal_left_number = diagonal_windows
            .iter()
            .find_map(|window| self.get_left_number(line, symbol_idx, window))
            .unwrap_or(0);

        let diagonal_right_number = diagonal_windows
            .iter()
            .find_map(|window| self.get_right_number(line, symbol_idx, window))
            .unwrap_or(0);

        return (diagonal_left_number, diagonal_right_number);
    }

    fn get_left_number(&self, line: &str, symbol_idx: usize, window: &usize) -> Option<u32> {
        return symbol_idx.checked_sub(*window).and_then(|start| {
            line.get(start..symbol_idx)
                .and_then(|value| value.parse::<u32>().ok())
        });
    }

    fn get_right_number(&self, line: &str, symbol_idx: usize, window: &usize) -> Option<u32> {
        return line
            .get(symbol_idx + 1..=symbol_idx + window)
            .and_then(|value| value.parse::<u32>().ok());
    }

    fn get_window_total(
        &self,
        line: &str,
        symbol_idx: usize,
        window: (usize, usize),
    ) -> Option<u32> {
        let (l_limit, r_limit) = window;

        let start = symbol_idx.checked_sub(l_limit);
        let end = symbol_idx.checked_add(r_limit);

        if start.is_none() || end.is_none() {
            return None;
        }

        return line
            .get(start.unwrap()..=end.unwrap())
            .and_then(|value| value.parse::<u32>().ok());
    }

    fn get_surrounding_symbol_line_numbers(
        &self,
        line: Option<&str>,
        symbol_idx: usize,
    ) -> (u32, u32, u32) {
        let mid = self.get_near_numbers(line, symbol_idx).unwrap_or(0);
        let (left, right) = self.get_diagonal_numbers(line, symbol_idx);

        return (left, mid, right);
    }

    fn get_near_numbers(&self, line: Option<&str>, symbol_idx: usize) -> Option<u32> {
        if let None = line {
            return None;
        }

        let line = line.unwrap();
        let windows: Vec<(usize, usize)> = vec![(1, 1), (0, 2), (2, 0), (0, 1), (1, 0), (0, 0)];

        return windows
            .into_iter()
            .find_map(|window| self.get_window_total(line, symbol_idx, window));
    }

    fn get_surrounding_numbers(
        &self,
        lines: (Option<&str>, &str, Option<&str>),
        symbol_idx: usize,
    ) -> Vec<u32> {
        let windows = vec![3, 2, 1];
        let (line_above, current_line, line_below) = lines;

        let left_number = windows
            .iter()
            .find_map(|window| self.get_left_number(current_line, symbol_idx, window))
            .unwrap_or(0);

        let right_number = windows
            .iter()
            .find_map(|window| self.get_right_number(current_line, symbol_idx, window))
            .unwrap_or(0);

        let (upper_left, upper_mid, upper_right) =
            self.get_surrounding_symbol_line_numbers(line_above, symbol_idx);
        let (bottom_left, bottom_mid, bottom_right) =
            self.get_surrounding_symbol_line_numbers(line_below, symbol_idx);

        let mut values = vec![left_number, right_number];

        if upper_mid != 0 {
            values.push(upper_mid)
        } else {
            values.push(upper_left);
            values.push(upper_right);
        }

        if bottom_mid != 0 {
            values.push(bottom_mid)
        } else {
            values.push(bottom_left);
            values.push(bottom_right);
        }

        return values;
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let mut total = 0;

        for (idx, line) in input.lines().enumerate() {
            for (symbol_idx, char) in line.char_indices() {
                if !self.is_symbol(char) {
                    continue;
                }

                let line_above = idx.checked_sub(1).and_then(|idx| input.lines().nth(idx));
                let line_below = input.lines().nth(idx + 1);

                let numbers =
                    self.get_surrounding_numbers((line_above, line, line_below), symbol_idx);

                total += numbers.iter().sum::<u32>();
            }
        }

        println!("Total is {total}");
    }

    fn run_part_two(&self, input: &String) {
        let mut total: u64 = 0;

        for (idx, line) in input.lines().enumerate() {
            for (symbol_idx, char) in line.char_indices() {
                if !self.is_gear(char) {
                    continue;
                }

                let line_above = idx.checked_sub(1).and_then(|idx| input.lines().nth(idx));
                let line_below = input.lines().nth(idx + 1);

                let numbers =
                    self.get_surrounding_numbers((line_above, line, line_below), symbol_idx);

                let gear_ratio_numbers: Vec<u32> =
                    numbers.into_iter().filter(|value| *value > 0).collect();

                if gear_ratio_numbers.len() < 2 {
                    continue;
                }

                let gear_ratio = gear_ratio_numbers.into_iter().fold(1, |acc, e| acc * e);

                total += gear_ratio as u64;
            }
        }

        println!("Total is {total}");
    }
}
