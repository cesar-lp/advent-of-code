pub use crate::exercise::DayExercise;

const ROWS: usize = 1000;
const COLUMNS: usize = 1000;

pub struct Runner {}

impl Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub enum Switch {
    ON,
    OFF,
    TOGGLE,
}

impl Switch {
    pub fn get_lighting(&self, value: u32) -> u32 {
        match self {
            Switch::ON => 1,
            Switch::OFF => 0,
            Switch::TOGGLE => {
                if value > 0 {
                    0
                } else {
                    1
                }
            }
        }
    }

    pub fn get_brightness(&self, value: u32) -> u32 {
        match self {
            Switch::ON => value + 1,
            Switch::OFF => {
                if value == 0 {
                    0
                } else {
                    value - 1
                }
            }
            Switch::TOGGLE => value + 2,
        }
    }
}

pub struct Operation {
    start: Coordinate,
    end: Coordinate,
    switch: Switch,
}

pub enum TileOperation {
    Lightness,
    Brightness,
}

pub struct Grid {
    coordinates: [[u32; COLUMNS]; ROWS],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            coordinates: [[0; COLUMNS]; ROWS],
        }
    }

    pub fn adjust_cells(&mut self, operation: Operation, adjustment: TileOperation) {
        let Operation { start, end, switch } = operation;

        for x in start.x..=end.x {
            for y in start.y..=end.y {
                self.coordinates[x][y] = match adjustment {
                    TileOperation::Lightness => switch.get_lighting(self.coordinates[x][y]),
                    TileOperation::Brightness => switch.get_brightness(self.coordinates[x][y]),
                };
            }
        }
    }

    pub fn get_lights_turned_on(&self) -> u32 {
        let mut lights_turned_on = 0;

        for row in 0..ROWS {
            for column in 0..COLUMNS {
                if self.coordinates[row][column] == 1 {
                    lights_turned_on += 1;
                }
            }
        }

        lights_turned_on
    }

    pub fn get_total_brightness(&self) -> u32 {
        let mut total_brightness: u32 = 0;

        for row in 0..ROWS {
            for column in 0..COLUMNS {
                total_brightness += self.coordinates[row][column];
            }
        }

        total_brightness
    }
}

fn get_coordinate(coordinate_pair: &str) -> Coordinate {
    let coordinates = coordinate_pair
        .split(",")
        .map(|position| str::parse::<usize>(position).unwrap())
        .collect::<Vec<usize>>();

    Coordinate::new(*coordinates.get(0).unwrap(), *coordinates.get(1).unwrap())
}

fn get_operation(input_line: &str) -> Operation {
    let switch: Switch;

    let keywords: Vec<&str> = input_line.split_whitespace().collect();

    let end_coordinate = get_coordinate(keywords.last().unwrap());
    let start_coordinate: Coordinate;

    let op = keywords.get(0).unwrap().to_owned();

    if op == "toggle" {
        switch = Switch::TOGGLE;
        start_coordinate = get_coordinate(keywords.get(1).unwrap());
    } else {
        start_coordinate = get_coordinate(keywords.get(2).unwrap());
        let next_op = keywords.get(1).unwrap().to_owned();

        switch = if next_op == "on" {
            Switch::ON
        } else {
            Switch::OFF
        };
    }

    Operation {
        start: start_coordinate,
        end: end_coordinate,
        switch,
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let mut grid = Grid::new();

        input
            .lines()
            .for_each(|line| grid.adjust_cells(get_operation(line), TileOperation::Lightness));

        let _lights_turned_on = grid.get_lights_turned_on();
    }

    fn run_part_two(&self, input: &String) {
        let mut grid = Grid::new();

        input
            .lines()
            .for_each(|line| grid.adjust_cells(get_operation(line), TileOperation::Brightness));

        let _total_brightness = grid.get_total_brightness();
    }
}
