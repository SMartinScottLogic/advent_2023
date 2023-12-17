use std::io::{BufRead, BufReader};
use tracing::{debug, event_enabled, Level};
use utils::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    island: Matrix<isize>,
}
impl Solution {
    fn set_block(&mut self, x: usize, y: usize, cost: u32) {
        self.island.set(x as isize, y as isize, cost as isize);
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                let cost = c.to_digit(10).unwrap();
                solution.set_block(x, y, cost);
            }
        }
        Ok(solution)
    }
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}
type State = (isize, isize, Option<(Direction, usize)>);

impl Solution {
    fn success(&self, (x, y, _): &State) -> bool {
        let (maxx, maxy) = self.island.dimensions();
        *x == maxx && *y == maxy
    }
    fn heuristic(&self, (x, y, _): &State) -> isize {
        let (maxx, maxy) = self.island.dimensions();
        (maxx - x) + (maxy - y)
    }
    fn successors_part1(&self, (x, y, s): &State) -> Vec<(State, isize)> {
        let x = *x;
        let y = *y;

        let c = match s {
            None => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x + 1, y, Some((Direction::East, 1))),
                (x, y + 1, Some((Direction::South, 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::North, c)) if *c < 3 => vec![
                (x, y - 1, Some((Direction::North, c + 1))),
                (x + 1, y, Some((Direction::East, 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::North, _)) => vec![
                (x + 1, y, Some((Direction::East, 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::East, c)) if *c < 3 => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x + 1, y, Some((Direction::East, c + 1))),
                (x, y + 1, Some((Direction::South, 1))),
            ],
            Some((Direction::East, _)) => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x, y + 1, Some((Direction::South, 1))),
            ],
            Some((Direction::South, c)) if *c < 3 => vec![
                (x + 1, y, Some((Direction::East, 1))),
                (x, y + 1, Some((Direction::South, c + 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::South, _)) => vec![
                (x + 1, y, Some((Direction::East, 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::West, c)) if *c < 3 => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x, y + 1, Some((Direction::South, 1))),
                (x - 1, y, Some((Direction::West, c + 1))),
            ],
            Some((Direction::West, _)) => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x, y + 1, Some((Direction::South, 1))),
            ],
        };
        c.into_iter()
            .filter_map(|(x, y, s)| self.island.get(x, y).map(|c| ((x, y, s), *c)))
            .collect::<Vec<_>>()
    }

    fn successors_part2(&self, (x, y, s): &State) -> Vec<(State, isize)> {
        let (maxx, maxy) = self.island.dimensions();
        let x = *x;
        let y = *y;

        let c = match s {
            None => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x + 1, y, Some((Direction::East, 1))),
                (x, y + 1, Some((Direction::South, 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::North, c)) if *c < 4 => {
                vec![(x, y - 1, Some((Direction::North, c + 1)))]
            }
            Some((Direction::North, c)) if *c < 10 => vec![
                (x, y - 1, Some((Direction::North, c + 1))),
                (x + 1, y, Some((Direction::East, 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::North, _)) => vec![
                (x + 1, y, Some((Direction::East, 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::East, c)) if *c < 4 => {
                vec![(x + 1, y, Some((Direction::East, c + 1)))]
            }
            Some((Direction::East, c)) if *c < 10 => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x + 1, y, Some((Direction::East, c + 1))),
                (x, y + 1, Some((Direction::South, 1))),
            ],
            Some((Direction::East, _)) => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x, y + 1, Some((Direction::South, 1))),
            ],
            Some((Direction::South, c)) if *c < 4 => {
                vec![(x, y + 1, Some((Direction::South, c + 1)))]
            }
            Some((Direction::South, c)) if *c < 10 => vec![
                (x + 1, y, Some((Direction::East, 1))),
                (x, y + 1, Some((Direction::South, c + 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::South, _)) => vec![
                (x + 1, y, Some((Direction::East, 1))),
                (x - 1, y, Some((Direction::West, 1))),
            ],
            Some((Direction::West, c)) if *c < 4 => {
                vec![(x - 1, y, Some((Direction::West, c + 1)))]
            }
            Some((Direction::West, c)) if *c < 10 => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x, y + 1, Some((Direction::South, 1))),
                (x - 1, y, Some((Direction::West, c + 1))),
            ],
            Some((Direction::West, _)) => vec![
                (x, y - 1, Some((Direction::North, 1))),
                (x, y + 1, Some((Direction::South, 1))),
            ],
        };
        c.into_iter()
            .filter_map(|(x, y, s)| self.island.get(x, y).map(|c| ((x, y, s), *c)))
            .filter(|((x, y, s), _)| {
                if *x == maxx && *y == maxy {
                    s.unwrap().1 >= 4
                } else {
                    true
                }
            })
            .collect::<Vec<_>>()
    }
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let r = pathfinding::directed::astar::astar(
            &(0, 0, None),
            |s| self.successors_part1(s),
            |s| self.heuristic(s),
            |s| self.success(s),
        )
        .unwrap();
        if event_enabled!(Level::DEBUG) {
            debug!(r = debug(&r), "result");
            let p = r.0.iter().fold(Matrix::new(), |mut path, v| {
                path.set(v.0, v.1, 1);
                path
            });
            p.display_with_mapping(|v| {
                match v {
                    0 => ".",
                    1 => "#",
                    _ => panic!(),
                }
                .to_string()
            });
        }
        // Implement for problem
        Ok(r.1 as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let r = pathfinding::directed::astar::astar(
            &(0, 0, None),
            |s| self.successors_part2(s),
            |s| self.heuristic(s),
            |s| self.success(s),
        )
        .unwrap();
        if event_enabled!(Level::DEBUG) {
            debug!(r = debug(&r), "result");
            let p = r.0.iter().fold(Matrix::new(), |mut path, v| {
                path.set(v.0, v.1, 1);
                path
            });
            p.display_with_mapping(|v| {
                match v {
                    0 => ".",
                    1 => "#",
                    _ => panic!(),
                }
                .to_string()
            });
        }
        Ok(r.1 as ResultType)
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
