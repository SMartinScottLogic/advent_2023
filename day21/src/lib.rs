use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};
use tracing::debug;
use utils::Matrix;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    plots: Matrix<char>,
}
impl Solution {
    fn set_plot(&mut self, x: usize, y: usize, c: char) {
        self.plots.set(x as isize, y as isize, c);
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.set_plot(x, y, c);
            }
            // Implement for problem
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Find start position
        let ((x, y), _) = self
            .plots
            .sparse_iter()
            .find(|(_, c)| *c == &'S')
            .unwrap();

        let mut positions = HashSet::new();
        positions.insert((*x, *y));
        for i in 0..64 {
            let mut next_positions = HashSet::new();
            debug!(
                positions = debug(&positions),
                next_positions = debug(&next_positions),
                i,
                "step start"
            );
            for (x, y) in positions.iter() {
                for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    match self.plots.get(x + dx, y + dy).unwrap_or(&'#') {
                        '.' | 'S' => {
                            next_positions.insert((x + dx, y + dy));
                        }
                        _ => {}
                    }
                }
            }
            debug!(
                positions = debug(positions),
                next_positions = debug(&next_positions),
                i,
                "step end"
            );
            positions = next_positions;
        }
        debug!(plots = positions.len(), "done?");
        // Implement for problem
        Ok(positions.len() as ResultType)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        if !is_full {
            return Ok(0);
        }
        let (max_x, max_y) = self.plots.dimensions();
        debug!(max_x, max_y, "dimensions");
        let ((sx, sy), _) = self
            .plots
            .sparse_iter()
            .find(|(_, c)| *c == &'S')
            .unwrap();

        let mut positions = HashSet::new();
        positions.insert((*sx, *sy));
        // let mut totals = Vec::new();
        // let mut deltas = Vec::new();
        let mut stats = Vec::new();
        let mut stats_idx = Vec::new();

        let mut i = 0;
        let double_loop_size = (1 + max_x) * 2;
        loop {
            i += 1;
            let mut tmp2 = HashSet::new();
            for (x, y) in positions.iter() {
                for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let mut newx = x + dx;
                    let mut newy = y + dy;

                    while newx < 0 {
                        newx += max_x + 1;
                    }
                    while newx > max_x {
                        newx -= max_x + 1;
                    }
                    while newy < 0 {
                        newy += max_y + 1;
                    }
                    while newy > max_y {
                        newy -= max_y + 1;
                    }

                    match self.plots.get(newx, newy).unwrap() {
                        '.' | 'S' => {
                            tmp2.insert((x + dx, y + dy));
                        }
                        _ => {}
                    }
                }
            }
            if i % double_loop_size == *sx {
                stats.push(tmp2.len());
                stats_idx.push(i);
                debug!(stats = debug(&stats), i, "s");
            }

            positions = tmp2;
            if i > 1000 {
                break;
            }
        }
        let delta = stats.iter().fold((Vec::new(), 0), |(mut acc, last), v| {
            acc.push(v - last);
            (acc, *v)
        });
        let deltadelta = delta.0.iter().fold((Vec::new(), 0), |(mut acc, last), v| {
            acc.push(v - last);
            (acc, *v)
        });
        let loops_required = (26501365 - *sx) / double_loop_size;
        let loops_remaining = loops_required - (stats_idx.len() - 1) as isize;
        let mut current = *stats.last().unwrap();
        let mut delta = *delta.0.last().unwrap();
        let last_delta_delta = *deltadelta.0.last().unwrap();
        for _ in 1..=loops_remaining {
            delta += last_delta_delta;
            current += delta;
        }
        // TOO low: 351,594,054, 90,192,836,242
        // Wrong:
        // 90193727915
        // 584205647727006
        // Right: 584211423220706
        let answer = current;
        Ok(answer as ResultType)
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
