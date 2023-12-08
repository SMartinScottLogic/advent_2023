use std::{io::{BufRead, BufReader}, collections::HashMap};

use anyhow::Context;
use tracing::{info, debug};

pub type ResultType = u64;
fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        nums[0]
    } else {
        let others = lcm(&nums[1..]);
        nums[0] * others / gcd(nums[0], others)
    }
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

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

impl Solution {
    fn next_z(&self, node: &str, steps: u64) -> (String, u64) {
        let num_instructions = self.instructions.len() as u64;
        let mut delta = 0;
        let mut node = node.clone();
        info!(node, steps, delta, "start");
        loop {
            let instruction_num = ((steps + delta) % num_instructions) as usize;
            let instruction = self.instructions.chars().nth(instruction_num).unwrap();
            let (left, right) = self.network.get(node).unwrap();
            node = match instruction {
                'L' => {
                    left
                }
                'R' => {
                    right
                }
                _ => panic!()
            };
            delta += 1;
            debug!(node, steps, delta, "step");
            if node.ends_with('Z') {
                break;
            }
        }
        info!(node, steps, delta, "jump");
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
            let instuction = self.instructions.chars().nth(steps % self.instructions.len()).unwrap();
            let (left, right) = self.network.get(node).context("lookup node")?;
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
        let nodes= self.network.keys().filter(|k| k.ends_with('A')).cloned().map(|s| (s, 0_u64)).collect::<Vec<_>>();
        info!(num = nodes.len(), "ghosts");

        let d = nodes.iter().map(|(node, steps)| self.next_z(node, *steps).1).collect::<Vec<_>>();
        let result = lcm(&d[..]);
        info!(result, "guess");

        Ok(result)
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
