use std::io::{BufRead, BufReader};

use tracing::info;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    lines: Vec<String>,
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
                let c = {
                    if line[i..].starts_with("one") {
                        '1'
                    } else if line[i..].starts_with("two") {
                        '2'
                    } else if line[i..].starts_with("three") {
                        '3'
                    } else if line[i..].starts_with("four") {
                        '4'
                    } else if line[i..].starts_with("five") {
                        '5'
                    } else if line[i..].starts_with("six") {
                        '6'
                    } else if line[i..].starts_with("seven") {
                        '7'
                    } else if line[i..].starts_with("eight") {
                        '8'
                    } else if line[i..].starts_with("nine") {
                        '9'
                    } else {
                        line.chars().nth(i).unwrap()
                    }
                };
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
