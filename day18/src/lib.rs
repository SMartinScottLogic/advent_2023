use std::io::{BufRead, BufReader};
use itertools::Itertools;
use tracing::{debug, event_enabled, info, Level};
use utils::Matrix;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    trenches: Vec<Dig>,
}
impl Solution {
    fn add_trench(&mut self, trench: Dig) {
        self.trenches.push(trench);
    }
}

#[derive(Debug)]
enum Direction {
    Up, Down, Left, Right
}
#[derive(Debug)]
struct Dig {
    direction: Direction,
    length: usize,
    color: String,
}
impl From<String> for Dig {
    fn from(value: String) -> Self {
        let r = regex::Regex::new(r"^(?<dir>.)\s+(?<length>\d+)\s+\((?<color>#[0-9a-f]+)\)$").unwrap();
        let c = r.captures(&value).unwrap();
        let direction = match c.name("dir").unwrap().as_str() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!()
        };
        let length = c.name("length").unwrap().as_str().parse().unwrap();
        let color = c.name("color").unwrap().as_str().to_string();

        Dig { direction, length, color}
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
        let mut pos = (0, 0);
        let mut lagoon = Matrix::new();
        for trench in &self.trenches {
            let (dx, dy) = match trench.direction {
                Direction::Up => (0, -1),
                Direction::Down => (0, 1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };
            for i in 0..trench.length {
                pos.0 += dx;
                pos.1 += dy;
                lagoon.set(pos.0, pos.1, 1);
            }
        }
        if event_enabled!(Level::DEBUG) {
        lagoon.display_with_mapping(|v| match v {
            0 => '.',
            1 => '#',
            _ =>panic!()
        }.to_string());
        }

        let min_x = lagoon.min_x();
        let min_y = lagoon.min_y();
        let max_x = lagoon.max_x();
        let max_y = lagoon.max_y();

        // Flood outside
        let mut probes = vec![(max_x, max_y + 1)];
        while let Some((probex, probey)) = probes.pop() {
            lagoon.set(probex, probey, 3);
            for (dx, dy) in [
                (0, -1),
                (1, -1),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, 1),
            ] {
                let nx = probex + dx;
                let ny = probey + dy;
                if nx >= min_x - 1
                    && ny >= min_y - 1
                    && nx <= max_x + 1
                    && ny <= max_y + 1
                    && lagoon.get(nx, ny).is_none()
                {
                    probes.push((nx, ny))
                }
            }
        }


        for y in min_y..=max_y {
            let mut inside = false;
            let mut on_wall = false;
            for x in min_x..=max_x {
                let c = *lagoon.get(x, y).unwrap_or(&4);
                match lagoon.get(x, y) {
                    Some(3) => {inside = false; on_wall = false},
                    Some(2) => {inside = true; on_wall = false},
                    Some(1) if on_wall => {},
                    Some(1) if !on_wall => on_wall = true,
                    _ if on_wall => {
                        on_wall = false;
                        inside = !inside;
                        if inside {
                            lagoon.set(x, y, 2);
                        }
                    }
                    _ if inside => lagoon.set(x, y, 2),
                    _ if !inside => {}
                    _ => panic!()
                }
            }
        }
        for x in min_x..=max_x {
            let mut inside = false;
            let mut on_wall = false;
            for y in min_y..=max_y {
                let c = *lagoon.get(x, y).unwrap_or(&4);
                match lagoon.get(x, y) {
                    Some(3) => {inside = false; on_wall = false},
                    Some(2) => {inside = true; on_wall = false},
                    Some(1) if on_wall => {},
                    Some(1) if !on_wall => on_wall = true,
                    _ if on_wall => {
                        on_wall = false;
                        inside = !inside;
                        if inside {
                            lagoon.set(x, y, 2);
                        }
                    }
                    _ if inside => lagoon.set(x, y, 2),
                    _ if !inside => {}
                    _ => panic!()
                }
            }
        }
        if event_enabled!(Level::DEBUG) {
             lagoon.display_with_mapping(|v| match v {
             0 => '.',
             1 => '#',
             2 => 'X',
             3 => ' ',
             _ =>panic!()
         }.to_string());
        }
        // Implement for problem
        let r = lagoon.sparse_iter().filter(|(k, v)| **v != 3).count();
        Ok(r as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut pos: (ResultType, ResultType) = (0, 0);
        let mut perimeter = 0;
        let mut area = 0;

        // Pick's theorem

        for (direction, length) in self.trenches.iter().map(|trench| {
            let distance = trench.color.chars().skip(1).take(5).collect::<String>();
            let distance = ResultType::from_str_radix(&distance, 16).unwrap();
            let direction = match trench.color.chars().last().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!()
            };
            (direction, distance)
        }) {
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

        info!(perimeter, area, "r");

        let r = area + perimeter / 2 + 1;

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
