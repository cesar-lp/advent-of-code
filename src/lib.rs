mod y2015;

pub fn noop() {}

pub type DayFn = fn();

pub fn get(year: u32, day: u32) -> (DayFn, DayFn) {
    return match year {
        2015 => match day {
            1 => (y2015::day_01::run_part_one, y2015::day_01::run_part_two),
            _ => {
                println!("Unknown day: {}", day);
                return (noop, noop);
            }
        },
        _ => {
            println!("Unknown year: {}", year);
            return (noop, noop);
        }
    };
}
