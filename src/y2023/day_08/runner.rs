use std::collections::HashMap;

use rayon::prelude::*;

use crate::exercise::lcm_of;
pub use crate::exercise::DayExercise;

type Instructions = Vec<char>;
type Nodes = HashMap<String, (String, String)>;

pub struct Runner {}

impl<'a> Runner {
    pub fn boxed() -> Box<Self> {
        Box::new(Self {})
    }

    fn get_instructions(&self, input: &str) -> Instructions {
        return input.lines().next().unwrap().chars().collect();
    }

    fn build_nodes(&self, input: &str) -> Nodes {
        let mut nodes: Nodes = HashMap::from([]);

        for line in input.lines().skip(2) {
            let (origin, left, right) = self.get_node_values(line);

            nodes.insert(origin, (left, right));
        }

        return nodes;
    }

    pub fn get_node_values(&self, line: &str) -> (String, String, String) {
        let split_str: Vec<&str> = line.split(" = ").collect();

        let source = split_str.get(0).unwrap().to_string();

        let pairs: Vec<&str> = split_str.get(1).unwrap().split(", ").collect();
        let left = pairs.get(0).and_then(|v| Some(v.replace("(", ""))).unwrap();
        let right = pairs.get(1).and_then(|v| Some(v.replace(")", ""))).unwrap();

        return (source, left, right);
    }

    pub fn parse_input(&self, input: &str) -> (Nodes, Instructions) {
        return (self.build_nodes(input), self.get_instructions(input));
    }

    fn count_steps(
        &self,
        key: &str,
        target_key: &str,
        nodes: &Nodes,
        instructions: &Instructions,
    ) -> u64 {
        let mut total_steps = 0;
        let mut found = false;
        let mut next_key = key;

        while !found {
            'cyclic_loop: for instruction in instructions.iter().cycle() {
                total_steps += 1;

                let (left, right) = nodes.get(next_key).unwrap();

                next_key = if instruction == &'L' { left } else { right };

                if next_key.ends_with(target_key) {
                    found = true;
                    break 'cyclic_loop;
                }
            }
        }

        return total_steps;
    }
}

impl DayExercise for Runner {
    fn run_part_one(&self, input: &String) {
        let (nodes, instructions) = self.parse_input(input);

        let total_steps = self.count_steps("AAA", "ZZZ", &nodes, &instructions);

        println!("Total is {total_steps}");
    }

    fn run_part_two(&self, input: &String) {
        let (nodes, instructions) = self.parse_input(input);

        let starting_keys: Vec<&String> = nodes
            .par_iter()
            .filter(|(start, _)| start.ends_with("A"))
            .map(|(start, _)| start)
            .collect();

        let starting_keys_steps: Vec<u64> = starting_keys
            .par_iter()
            .map(|key| self.count_steps(key, "Z", &nodes, &instructions))
            .collect();

        let total_steps = lcm_of(starting_keys_steps);

        println!("Total is {total_steps}");
    }
}
