use std::io::{BufRead, BufReader};
use tracing::debug;
use utils::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    contraption: Matrix<char>,
}
impl Solution {
    fn set_cell(&mut self, x: isize, y: isize, c: char) {
        self.contraption.set(x, y, c);
    }

    fn count_energised(&self, start: (isize, isize, isize, isize)) -> ResultType {
        let (maxx, maxy) = self.contraption.dimensions();
        let r = pathfinding::directed::dijkstra::dijkstra_all(&start, |(x, y, dx, dy)| {
            let x = *x;
            let y = *y;
            let dx = *dx;
            let dy = *dy;
            match self.contraption.get(x, y) {
                Some('.') => vec![((x + dx, y + dy, dx, dy), 1)],
                Some('-') if dx == 1 && dy == 0 => vec![((x + dx, y + dy, dx, dy), 1)],
                Some('-') if dx == -1 && dy == 0 => vec![((x + dx, y + dy, dx, dy), 1)],
                Some('|') if dx == 0 && dy == 1 => vec![((x + dx, y + dy, dx, dy), 1)],
                Some('|') if dx == 0 && dy == -1 => vec![((x + dx, y + dy, dx, dy), 1)],

                // Splitters
                Some('|') if dx == 1 && dy == 0 => {
                    vec![((x, y + 1, 0, 1), 1), ((x, y - 1, 0, -1), 1)]
                }
                Some('|') if dx == -1 && dy == 0 => {
                    vec![((x, y + 1, 0, 1), 1), ((x, y - 1, 0, -1), 1)]
                }
                Some('-') if dx == 0 && dy == 1 => {
                    vec![((x + 1, y, 1, 0), 1), ((x - 1, y, -1, 0), 1)]
                }
                Some('-') if dx == 0 && dy == -1 => {
                    vec![((x + 1, y, 1, 0), 1), ((x - 1, y, -1, 0), 1)]
                }
                // Mirrors
                Some('/') if dx == 1 && dy == 0 => vec![((x, y - 1, 0, -1), 1)],
                Some('/') if dx == -1 && dy == 0 => vec![((x, y + 1, 0, 1), 1)],
                Some('/') if dx == 0 && dy == -1 => vec![((x + 1, y, 1, 0), 1)],
                Some('/') if dx == 0 && dy == 1 => vec![((x - 1, y, -1, 0), 1)],

                Some('\\') if dx == 1 && dy == 0 => vec![((x, y + 1, 0, 1), 1)],
                Some('\\') if dx == -1 && dy == 0 => vec![((x, y - 1, 0, -1), 1)],
                Some('\\') if dx == 0 && dy == -1 => vec![((x - 1, y, -1, 0), 1)],
                Some('\\') if dx == 0 && dy == 1 => vec![((x + 1, y, 1, 0), 1)],

                // Out of contraption
                None => vec![],
                Some(c) => panic!("unexpected {c:?} @ {x} {y}, dir {dx} {dy}"),
            }
            .into_iter()
            // Filter out beams which exit the contraption
            .filter(|(n, _cost)| n.0 >= 0 && n.0 <= maxx && n.1 >= 0 && n.1 <= maxy)
            .collect::<Vec<_>>()
        });

        debug!(r = debug(&r), "dijkstra");
        let mut g = r.iter().fold(Matrix::new(), |mut acc, (p, _)| {
            acc.set(p.0, p.1, 1);
            acc
        });
        g.set(start.0, start.1, 1);
        g.len() as ResultType
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.set_cell(x as isize, y as isize, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let result = self.count_energised((0, 0, 1, 0));
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let (maxx, maxy) = self.contraption.dimensions();
        let mut max = 0;
        for y in 0..=maxy {
            // Left edge
            let count = self.count_energised((0, y, 1, 0));
            if count > max {
                max = count;
            }
            // Right edge
            let count = self.count_energised((maxx, y, -1, 0));
            if count > max {
                max = count;
            }
        }
        for x in 0..=maxx {
            // Top edge
            let count = self.count_energised((x, 0, 0, 1));
            if count > max {
                max = count;
            }
            // Bottom edge
            let count = self.count_energised((x, maxy, 0, -1));
            if count > max {
                max = count;
            }
        }
        Ok(max)
    }
}
