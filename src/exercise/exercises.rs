use crate::exercise::DayExercise;

use crate::{y2015, y2023};

pub fn get(year: u32, day: u32) -> Option<Box<dyn DayExercise>> {
    let day_exercise: Option<Box<dyn DayExercise>> = match year {
        2015 => match day {
            1 => Some(y2015::day_01::Runner::boxed()),
            2 => Some(y2015::day_02::Runner::boxed()),
            3 => Some(y2015::day_03::Runner::boxed()),
            4 => Some(y2015::day_04::Runner::boxed()),
            5 => Some(y2015::day_05::Runner::boxed()),
            6 => Some(y2015::day_06::Runner::boxed()),
            7 => Some(y2015::day_07::Runner::boxed()),
            _ => {
                println!("Unknown day: {}", day);
                None
            }
        },

        2023 => match day {
            1 => Some(y2023::day_01::Runner::boxed()),
            _ => {
                println!("Unknown day: {}", day);
                None
            }
        }
        _ => {
            println!("Unknown year: {}", year);
            None
        }
    };

    day_exercise
}
