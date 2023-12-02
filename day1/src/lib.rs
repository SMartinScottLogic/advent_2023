use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use lazy_static::lazy_static;
use tracing::info;
use utils::map;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    lines: Vec<String>,
}

lazy_static! {
    static ref NAME_MAP: HashMap<&'static str, char> = map! {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9'
    };
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total = 0_u64;
        for line in &self.lines {
            let first = line
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap_or_default()
                .to_digit(10)
                .unwrap_or_default();
            let last = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .last()
                .unwrap_or_default()
                .to_digit(10)
                .unwrap_or_default();
            total += (first * 10 + last) as u64;
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut total = 0_u64;
        for line in &self.lines {
            let mut newline = String::new();
            let mut i = 0;
            loop {
                if i >= line.len() {
                    break;
                }
                let mut c = line.chars().nth(i).unwrap();

                for (name, new_c) in NAME_MAP.iter() {
                    if line[i..].starts_with(name) {
                        c = *new_c;
                    }
                }
                newline.push(c);
                i += 1;
            }
            let first = newline
                .chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap();
            let last = newline
                .chars()
                .filter(|c| c.is_ascii_digit())
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap();
            info!(line, newline, first, last, "part");
            total += (first * 10 + last) as u64;
        }
        // Implement for problem
        Ok(total)
    }
}

impl Solution {
    fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            solution.add_line(line);
        }
        Ok(solution)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn stub() {
        assert_eq!(1 + 1, 2);
    }
}
