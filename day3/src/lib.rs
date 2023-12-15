use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use tracing::debug;
use utils::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    data: Matrix<char>,
    found_numbers: Vec<(isize, isize, isize, ResultType)>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        let (maxx, maxy) = self.data.dimensions();

        self.found_numbers = Vec::new();
        for y in 0..=maxy {
            let mut in_number = false;
            let mut cur_number = 0;
            let mut start_number = 0;
            let mut end_number = 0;

            for x in 0..=maxx + 1 {
                let c = self.data.get(x, y).unwrap_or(&'.');
                match c {
                    '0'..='9' if in_number => {
                        cur_number *= 10;
                        cur_number += c.to_digit(10).unwrap();
                        end_number += 1;
                    }
                    '0'..='9' => {
                        cur_number = c.to_digit(10).unwrap();
                        start_number = x;
                        end_number = x;
                        in_number = true;
                    }
                    // Ignore symbols and gap
                    _ if in_number => {
                        in_number = false;
                        self.found_numbers.push((
                            start_number,
                            end_number,
                            y,
                            cur_number as ResultType,
                        ));
                    }
                    _ => {}
                };
            }
            assert!(!in_number);
        }
        debug!(found_numbers = debug(&self.found_numbers), "numbers");
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        fn is_symbol(c: char) -> bool {
            !(c.is_numeric() || c == '.')
        }

        let r: ResultType = self
            .found_numbers
            .iter()
            .filter(|(sx, ex, y, _)| {
                for y in y - 1..=y + 1 {
                    for x in sx - 1..=ex + 1 {
                        let c = self.data.get(x, y).unwrap_or(&'.');
                        if is_symbol(*c) {
                            return true;
                        }
                    }
                }
                false
            })
            .map(|(_, _, _, v)| *v as ResultType)
            .sum();
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut gears = HashMap::new();
        for (sx, ex, y, v) in self.found_numbers.iter() {
            for y in y - 1..=y + 1 {
                for x in sx - 1..=ex + 1 {
                    let c = *self.data.get(x, y).unwrap_or(&'.');
                    // Gear
                    if c == '*' {
                        gears.entry((x, y)).or_insert(Vec::new()).push(v);
                    }
                }
            }
        }

        debug!(gears = debug(&gears), "gears");
        let r = gears
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| {
                let mut total = 1;
                for value in v {
                    total *= (**value) as ResultType;
                }
                total
            })
            .sum();
        Ok(r)
    }
}

impl Solution {
    fn set(&mut self, x: usize, y: usize, c: char) {
        self.data.set(x as isize, y as isize, c);
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().flatten().enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.set(x, y, c);
            }
        }
        Ok(solution)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use utils::Solution as _;

    #[test]
    fn read() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .join("\n");
        let r = BufReader::new(input.as_bytes());
        let mut s = crate::Solution::try_from(r).unwrap();
        s.analyse(false);
        assert_eq!(4361 as ResultType, s.answer_part1(false).unwrap());
    }
}
