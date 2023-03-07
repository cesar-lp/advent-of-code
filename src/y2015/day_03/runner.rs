pub use crate::exercise::DayExercise;
use std::collections::HashMap;

pub struct Runner {}

impl Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Santa {
    Normal,
    Robot,
}

enum Directions {
    East,
    West,
    North,
    South,
}

struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn move_x(&mut self, units: i8) {
        self.x += units as i32;
    }

    pub fn move_y(&mut self, units: i8) {
        self.y += units as i32;
    }

    pub fn get(&self) -> String {
        format!("{}_{}", self.x, self.y)
    }
}

struct Map {
    current_position: HashMap<Santa, Coordinate>,
    houses: HashMap<String, i32>,
}

impl Map {
    pub fn default(santas: Vec<Santa>) -> Self {
        Self {
            current_position: santas
                .into_iter()
                .map(|santa| (santa, Coordinate::default()))
                .collect(),
            houses: HashMap::from([("0_0".to_string(), 1)]),
        }
    }

    pub fn travel(&mut self, direction: Directions, santa: Santa) {
        if let Some(coordinate) = self.current_position.get_mut(&santa) {
            match direction {
                Directions::East => coordinate.move_x(1),
                Directions::West => coordinate.move_x(-1),
                Directions::North => coordinate.move_y(1),
                Directions::South => coordinate.move_y(-1),
            };

            self.houses
                .entry(coordinate.get())
                .and_modify(|visits| *visits += 1)
                .or_insert(1);
        }
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let mut map = Map::default(vec![Santa::Normal]);

        for char in input.chars() {
            match char {
                '>' => map.travel(Directions::East, Santa::Normal),
                '<' => map.travel(Directions::West, Santa::Normal),
                '^' => map.travel(Directions::North, Santa::Normal),
                'v' => map.travel(Directions::South, Santa::Normal),
                _ => {}
            }
        }
    }

    fn run_part_two(&self, input: &String) {
        let mut map = Map::default(vec![Santa::Normal, Santa::Robot]);

        for (mov_number, char) in input.chars().enumerate() {
            let santa = if mov_number % 2 == 0 {
                Santa::Normal
            } else {
                Santa::Robot
            };

            match char {
                '>' => map.travel(Directions::East, santa),
                '<' => map.travel(Directions::West, santa),
                '^' => map.travel(Directions::North, santa),
                'v' => map.travel(Directions::South, santa),
                _ => {}
            }
        }
    }
}
