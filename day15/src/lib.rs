#![feature(ascii_char)]
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    sequence: String,
}
impl Solution {
    fn set_sequence(&mut self, sequence: &str) {
        self.sequence = sequence.to_string();
    }

    fn hash(part: &str) -> ResultType {
        let r = part.chars().fold(0 as ResultType, |mut acc, v| {
            acc += v.as_ascii().unwrap().to_u8() as ResultType;
            acc *= 17;
            acc &= 0xff;
            acc
        });
        debug!(part, r, "hash");
        r
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().map_while(Result::ok) {
            solution.set_sequence(&line);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let result = self.sequence.split(',').map(Self::hash).sum();
        // Implement for problem
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut boxes: HashMap<u64, Vec<(&str, &str)>> = HashMap::new();
        for step in self.sequence.split(',') {
            if let Some((label, length)) = step.split_once('=') {
                let lens_box = Self::hash(label);
                let entry = boxes.entry(lens_box).or_default();
                if let Some((_, b)) = entry.iter_mut().find(|(lab, _)| *lab == label) {
                    *b = length;
                } else {
                    entry.push((label, length));
                }
            } else {
                let (label, _) = step.split_once('-').unwrap();
                let lens_box = Self::hash(label);
                let entry = boxes.entry(lens_box).or_default();
                if let Some(i) = entry
                    .iter_mut()
                    .enumerate()
                    .find(|(_, (lab, _))| *lab == label)
                    .map(|(i, _)| i)
                {
                    entry.remove(i);
                }
            }
        }
        debug!(boxes = debug(&boxes), "final");

        let mut result = 0 as ResultType;
        for (box_id, content) in boxes {
            for (pos, (label, length)) in content.iter().enumerate() {
                let score =
                    (box_id + 1) * (pos as ResultType + 1) * length.parse::<ResultType>().unwrap();
                debug!(label, score, "lens score");
                result += score;
            }
        }
        // Implement for problem
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn hash() {
        let input = "HASH";
        assert_eq!(52 as ResultType, super::Solution::hash(input));
    }
}
