use std::collections::HashMap;

pub use crate::exercise::DayExercise;

fn split_instruction_by(input: &String, split_by: &str) -> (String, String) {
    let signal_pair: Vec<&str> = input.split(split_by).collect();

    let left = signal_pair.get(0).unwrap().to_owned();
    let right = signal_pair.get(1).unwrap().to_owned();

    (left.to_string(), right.to_string())
}

pub enum LogicGateOperand {
    And,
    Or,
}

pub enum ShiftType {
    LShift,
    RShift,
}

pub enum WireValueOp {
    Not,
    Wire,
}

pub enum InstructionType {
    Logic(String, LogicGateOperand, String),
    Shift(String, ShiftType, u32),
    WireValue(String, WireValueOp),
    Value(u32),
}

impl InstructionType {
    pub fn from(signal: &String) -> Self {
        return match signal.parse::<u32>() {
            Ok(v) => InstructionType::Value(v),
            Err(_) => {
                return if signal.contains("AND") {
                    let (left, right) = split_instruction_by(signal, " AND ");

                    InstructionType::Logic(left, LogicGateOperand::And, right)
                } else if signal.contains("OR") {
                    let (left, right) = split_instruction_by(signal, " OR ");

                    InstructionType::Logic(left, LogicGateOperand::Or, right)
                } else if signal.contains("NOT") {
                    let signal_pair: Vec<&str> = signal.split("NOT ").collect();

                    let signal_wire = signal_pair.get(1).unwrap().to_owned();

                    InstructionType::WireValue(signal_wire.to_string(), WireValueOp::Not)
                } else if signal.contains("LSHIFT") {
                    let (left, right) = split_instruction_by(signal, " LSHIFT ");

                    InstructionType::Shift(left, ShiftType::LShift, right.parse().unwrap())
                } else if signal.contains("RSHIFT") {
                    let (left, right) = split_instruction_by(signal, " RSHIFT ");

                    InstructionType::Shift(left, ShiftType::RShift, right.parse().unwrap())
                } else {
                    InstructionType::WireValue(signal.to_string(), WireValueOp::Wire)
                };
            }
        };
    }
}

struct Circuit {
    wires: HashMap<String, u32>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            wires: HashMap::default(),
        }
    }

    pub fn get_wire_value(&self, wire: &str) -> &u32 {
        self.wires.get(wire).unwrap()
    }

    pub fn handle(&mut self, instructions: &mut Instructions) {
        self.reset_board();

        let mut current_instructions: HashMap<String, u32> = HashMap::new();

        // Delete from left instructions the ones we have already mapped in this iteration
        for (wire, signal) in current_instructions.iter() {
            self.wires.insert(wire.to_string(), signal.to_owned());
            instructions.remove_for(wire);
        }

        current_instructions.clear();

        while instructions.has_more() {
            for (wire, signal) in instructions.iter() {
                match InstructionType::from(signal) {
                    InstructionType::Value(value) => {
                        current_instructions.insert(wire.to_string(), value);
                    }
                    InstructionType::Logic(left, operand, right) => {
                        let (left_value, right_value) = self.get_values(&left, &right);

                        if left_value.is_some() && right_value.is_some() {
                            let left_value = left_value.unwrap();
                            let right_value = right_value.unwrap();

                            let result = match operand {
                                LogicGateOperand::Or => left_value | right_value,
                                LogicGateOperand::And => left_value & right_value,
                            };

                            current_instructions.insert(wire.to_string(), result);
                        }
                    }
                    InstructionType::Shift(source_wire, shift_type, additive) => {
                        let wire_value = self.wires.get(&source_wire);

                        if wire_value.is_some() {
                            let wire_value = wire_value.unwrap().to_owned();

                            let result = match shift_type {
                                ShiftType::RShift => wire_value >> additive,
                                ShiftType::LShift => wire_value << additive,
                            };

                            current_instructions.insert(wire.to_string(), result);
                        }
                    }
                    InstructionType::WireValue(source_wire, op_type) => {
                        let wire_value = self.wires.get(&source_wire);

                        if wire_value.is_some() {
                            let wire_value = wire_value.unwrap().to_owned();

                            let result = match op_type {
                                WireValueOp::Not => !wire_value,
                                WireValueOp::Wire => wire_value,
                            };

                            current_instructions.insert(wire.to_string(), result);
                        }
                    }
                };
            }

            // Delete from left instructions the ones we have already mapped in this iteration
            for (wire, value) in current_instructions.iter() {
                instructions.remove_for(wire);
                self.wires.insert(wire.to_string(), value.to_owned());
            }

            current_instructions.clear();
        }
    }

    fn reset_board(&mut self) {
        self.wires = HashMap::new();
    }

    fn get_values(&self, left: &String, right: &String) -> (Option<u32>, Option<u32>) {
        let rv = match right.parse::<u32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        let lv = match left.parse::<u32>() {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        let left_value = self.wires.get(left);
        let right_value = self.wires.get(right);

        return if (left_value.is_some() || lv.is_some()) && (right_value.is_some() || rv.is_some())
        {
            let left_value = lv.unwrap_or_else(|| left_value.unwrap().to_owned());
            let right_value = rv.unwrap_or_else(|| right_value.unwrap().to_owned());

            (Some(left_value), Some(right_value))
        } else {
            (None, None)
        };
    }
}

pub struct Runner {}

impl Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }
}

struct Instructions {
    instructions: HashMap<String, String>,
    instructions_left: HashMap<String, String>,
}

impl Instructions {
    pub fn new() -> Self {
        Self {
            instructions: HashMap::new(),
            instructions_left: HashMap::new(),
        }
    }

    pub fn override_signal(&mut self, wire: &str, value: &u32) {
        self.instructions
            .insert(wire.to_string(), value.to_string());

        self.instructions_left.clear();

        for (k, v) in self.instructions.iter() {
            self.instructions_left.insert(k.clone(), v.clone());
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, String> {
        self.instructions_left.iter()
    }

    pub fn parse(&mut self, instruction: &str) {
        let (wire, signal) = self.split_instruction(instruction);

        self.instructions.insert(wire.clone(), signal.clone());
        self.instructions_left.insert(wire, signal);
    }

    pub fn remove_for(&mut self, wire: &str) {
        self.instructions_left.remove(wire);
    }

    pub fn has_more(&self) -> bool {
        !self.instructions_left.is_empty()
    }

    fn split_instruction(&self, instruction_stmt: &str) -> (String, String) {
        let instructions: Vec<&str> = instruction_stmt.split(" -> ").collect();

        let signal = instructions.get(0).unwrap().to_string();
        let wire = instructions.get(1).unwrap().to_string();

        (wire, signal)
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let mut circuit = Circuit::new();
        let mut instructions = Instructions::new();

        input
            .lines()
            .for_each(|instruction| instructions.parse(instruction));

        circuit.handle(&mut instructions);
    }

    fn run_part_two(&self, input: &String) {
        let mut circuit = Circuit::new();
        let mut instructions = Instructions::new();

        input
            .lines()
            .for_each(|instruction| instructions.parse(instruction));

        circuit.handle(&mut instructions);

        instructions.override_signal("b", circuit.get_wire_value("a"));

        circuit.handle(&mut instructions);
    }
}
