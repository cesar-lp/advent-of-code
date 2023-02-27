use std::env;
use std::io;
use std::time::{Duration, Instant};

use advent_of_code::{get, noop};

mod y2015;

fn fmt_time(ms: f64) -> String {
    if ms <= 1.0 {
        let micro_sec = ms * 1000.0;
        return String::from(format!("{}µs", micro_sec.round()));
    }

    if ms < 1000.0 {
        let whole_ms = ms.floor();
        let rem_ms = ms - whole_ms;
        return String::from(format!("{}ms ", whole_ms) + &fmt_time(rem_ms));
    }

    let sec: f64 = ms / 1000.0;
    if sec < 60.0 {
        let whole_sec = sec.floor();
        let rem_ms = ms - whole_sec * 1000.0;

        return format!("{}s ", whole_sec) + &fmt_time(rem_ms);
    }

    let min: f64 = sec / 60.0;
    return format!("{}m ", min.floor()) + &fmt_time((sec % 60.0) * 1000.0);
}

fn fmt_dur(dur: Duration) -> String {
    return fmt_time(dur.as_secs_f64() * 1000.0);
}

fn main() {
    // Get day string
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();
    let mut year = String::new();

    if args.len() >= 2 {
        year = args[1].clone();
        day = args[2].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }

    // Parse year, day as number
    day = day.trim().to_string();
    let day_num: u32 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };

    year = year.trim().to_string();
    let year_num: u32 = match year.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid year number: {}", year);
            return;
        }
    };

    // Get corresponding function
    let to_run = get(year_num, day_num);

    // Time it
    if to_run.0 != noop {
        println!("Running Part 1");
        let part1_start = Instant::now();
        to_run.0();
        let part1_dur = part1_start.elapsed();
        println!("Took {}", fmt_dur(part1_dur));
    }

    if to_run.1 != noop {
        println!("Running Part 2");
        let part2_start = Instant::now();
        to_run.1();
        let part2_dur = part2_start.elapsed();
        println!("Took {}", fmt_dur(part2_dur));
    }
}
