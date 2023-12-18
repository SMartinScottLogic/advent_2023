use std::io::{BufRead, BufReader};
use tracing::debug;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    trenches: Vec<Dig>,
}
impl Solution {
    fn add_trench(&mut self, trench: Dig) {
        self.trenches.push(trench);
    }

    fn solve_directions(directions: &Vec<(Direction, ResultType)>) -> ResultType {
        let mut pos: (ResultType, ResultType) = (0, 0);
        let mut perimeter = 0;
        let mut area = 0;

        // TODO: Why does this work?
        for (direction, length) in directions {
            let (dx, dy) = match direction {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };
            pos.0 += dx * length;
            pos.1 += dy * length;
            perimeter += length;
            area += pos.0 * (dy * length);
        }

        debug!(perimeter, area, "r");

        area + perimeter / 2 + 1
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Dig {
    direction: Direction,
    length: usize,
    color: String,
}
impl From<String> for Dig {
    fn from(value: String) -> Self {
        let r =
            regex::Regex::new(r"^(?<dir>.)\s+(?<length>\d+)\s+\((?<color>#[0-9a-f]+)\)$").unwrap();
        let c = r.captures(&value).unwrap();
        let direction = match c.name("dir").unwrap().as_str() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };
        let length = c.name("length").unwrap().as_str().parse().unwrap();
        let color = c.name("color").unwrap().as_str().to_string();

        Dig {
            direction,
            length,
            color,
        }
    }
}
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().map_while(Result::ok) {
            let dig = Dig::from(line);
            solution.add_trench(dig);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let directions = self
            .trenches
            .iter()
            .map(|trench| (trench.direction.clone(), trench.length as ResultType))
            .collect();
        let r = Self::solve_directions(&directions);
        Ok(r as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let directions = self
            .trenches
            .iter()
            .map(|trench| {
                let distance = trench.color.chars().skip(1).take(5).collect::<String>();
                let distance = ResultType::from_str_radix(&distance, 16).unwrap();
                let direction = match trench.color.chars().last().unwrap() {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    _ => panic!(),
                };
                (direction, distance)
            })
            .collect();

        let r = Self::solve_directions(&directions);

        Ok(r as ResultType)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    use tracing_test::traced_test;
    use utils::Solution;

    #[test]
    #[traced_test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
