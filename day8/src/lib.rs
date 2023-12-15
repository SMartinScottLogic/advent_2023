use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use anyhow::Context;
use tracing::debug;
use utils::math::lowest_common_multiple_many;

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
        self.network
            .insert(adjacency.name, (adjacency.left, adjacency.right));
    }
}
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().map_while(Result::ok).enumerate() {
            let line = line.trim();
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

impl Solution {
    fn next_z(&self, node: &str, steps: u64) -> (String, u64) {
        let num_instructions = self.instructions.len() as u64;
        let mut delta = 0;
        let mut node = node;
        loop {
            let instruction_num = ((steps + delta) % num_instructions) as usize;
            let instruction = self.instructions.chars().nth(instruction_num).unwrap();
            let (left, right) = self.network.get(node).unwrap();
            node = match instruction {
                'L' => left,
                'R' => right,
                _ => panic!(),
            };
            delta += 1;
            if node.ends_with('Z') {
                break;
            }
        }
        debug!(node, steps, delta, "jump");
        (node.to_string(), delta)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut node = "AAA";
        let mut steps = 0;
        loop {
            if node == "ZZZ" {
                break;
            }
            let instuction = self
                .instructions
                .chars()
                .nth(steps % self.instructions.len())
                .unwrap();
            let (left, right) = self.network.get(node).context("lookup node")?;
            node = match instuction {
                'L' => left,
                'R' => right,
                _ => panic!(),
            };
            steps += 1;
            debug!(node, steps, "step");
        }
        Ok(steps as u64)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let d = self
            .network
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|node| self.next_z(node, 0).1)
            .collect::<Vec<_>>();
        let result = lowest_common_multiple_many(&d[..]);
        Ok(result)
    }
}
#[cfg(test)]
mod test {}
