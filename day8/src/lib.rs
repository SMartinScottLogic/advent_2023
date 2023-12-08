use std::{io::{BufRead, BufReader}, collections::HashMap};

use tracing::{info, debug};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    instructions: String,
    network: HashMap<String, (String, String)>,
}
impl Solution {
    fn set_instructions(&mut self, instructions: &str) {
        self.instructions = instructions.to_string();
    }
    fn add_adjacency(&mut self, adjacency: Adjacency) {
        self.network.insert(adjacency.name, (adjacency.left, adjacency.right));
    }
}
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().flatten().enumerate() {
            let line = line.trim();
            // Implement for problem
            if id == 0 {
                solution.set_instructions(line);
            } else if !line.is_empty() {
                let adjacency = Adjacency::from(line);
                solution.add_adjacency(adjacency);
            }
        }
        Ok(solution)
    }
}

#[derive(Debug)]
struct Adjacency {
    name: String,
    left: String,
    right: String,
}
impl From<&str> for Adjacency {
    fn from(value: &str) -> Self {
        let r = regex::Regex::new(r"(?<name>\w+)\s+=\s+\((?<left>\w+), (?<right>\w+)\)$").unwrap();
        let c = r.captures(value).unwrap();
        let name = c.name("name").unwrap().as_str().to_string();
        let left = c.name("left").unwrap().as_str().to_string();
        let right = c.name("right").unwrap().as_str().to_string();
        Self { name, left, right }
    }
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut steps = 0;
        let mut node = "AAA";
        loop {
            if node == "ZZZ" {
                break;
            }
            let instuction = self.instructions.chars().nth(steps % self.instructions.len()).unwrap();
            let (left, right) = self.network.get(node).unwrap();
            node = match instuction {
                'L' => {
                    left
                }
                'R' => {
                    right
                }
                _ => panic!()
            };
            steps += 1;
            debug!(node, steps, "step");
        }
        // Implement for problem
        Ok(steps as u64)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    use utils::Solution;

    #[test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
