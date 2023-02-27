use std::fs;

pub fn run_part_one() {
    let contents = fs::read_to_string("src/y2015/day_01/input.txt").unwrap();

    let mut _floor = 0;

    for char in contents.chars() {
        if char == '(' {
            _floor += 1;
        } else if char == ')' {
            _floor -= 1;
        }
    }
}

pub fn run_part_two() {
    let contents = fs::read_to_string("src/y2015/day_01/input.txt").unwrap();
    let chars_length = contents.chars().count();
    let mut chars_iter = contents.chars();

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
