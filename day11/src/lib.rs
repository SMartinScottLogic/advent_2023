use std::{
    cmp::{max, min},
    collections::HashSet,
    io::{BufRead, BufReader},
};

use tracing::debug;
use utils::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    space: Matrix<char>,
}

impl Solution {
    fn set_space(&mut self, x: usize, y: usize, c: char) {
        self.space.set(x as isize, y as isize, c);
    }

    fn get_expansions(&self) -> (HashSet<isize>, HashSet<isize>) {
        let (maxx, maxy) = self.space.dimensions();

        let mut expand_columns = HashSet::new();
        let mut expand_rows = HashSet::new();

        debug!(maxx, maxy, "dimensions");
        for y in 0..=maxy {
            let mut expand_row = true;
            for x in 0..=maxx {
                if let Some('#') = self.space.get(x, y) {
                    debug!(x, y, "No expand row");
                    expand_row = false
                };
            }
            if expand_row {
                debug!(y, "Expand row");
                expand_rows.insert(y);
            }
        }
        debug!(
            expand_columns = debug(&expand_columns),
            expand_rows = debug(&expand_rows),
            "expand"
        );
        for x in 0..=maxx {
            let mut expand_column = true;
            for y in 0..=maxy {
                if let Some('#') = self.space.get(x, y) {
                    debug!(x, y, "No expand column");
                    expand_column = false
                };
            }
            if expand_column {
                debug!(x, "Expand column");
                expand_columns.insert(x);
            }
        }
        debug!(
            expand_columns = debug(&expand_columns),
            expand_rows = debug(&expand_rows),
            "expand"
        );
        (expand_columns, expand_rows)
    }

    fn expand(&self) -> Matrix<char> {
        let (maxx, maxy) = self.space.dimensions();
        let (expand_columns, expand_rows) = self.get_expansions();

        let mut new_space = Matrix::new();
        let mut dy = 0;
        for y in 0..=maxy {
            let mut dx = 0;
            for x in 0..=maxx {
                match self.space.get(x, y) {
                    None => {
                        self.space.display();
                        panic!("Nothing @ ({x}, {y})");
                    }
                    Some(c) => {
                        new_space.set(x + dx, y + dy, *c);
                        if expand_columns.contains(&x) {
                            dx += 1;
                            new_space.set(x + dx, y + dy, *c);
                        }
                    }
                }
            }
            if expand_rows.contains(&y) {
                dy += 1;
                let mut dx = 0;
                for x in 0..=maxx {
                    let c = *self.space.get(x, y).unwrap();
                    new_space.set(x + dx, y + dy, c);
                    if expand_columns.contains(&x) {
                        dx += 1;
                        new_space.set(x + dx, y + dy, c);
                    }
                }
            }
        }
        new_space
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().flatten().enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.set_space(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let space = self.expand();
        let galaxies = space
            .sparse_iter()
            .filter(|(_, c)| *c == &'#')
            .map(|(p, _)| p)
            .cloned()
            .collect::<Vec<_>>();
        let mut total = 0;
        for (a, ga) in galaxies.iter().enumerate() {
            for (b, gb) in galaxies.iter().enumerate() {
                if a > b {
                    let cost = (gb.0 - ga.0).unsigned_abs() as ResultType
                        + (gb.1 - ga.1).unsigned_abs() as ResultType;
                    debug!(ga = debug(ga), gb = debug(gb), cost, "cost");
                    total += cost;
                }
            }
        }
        Ok(total)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        let (expand_columns, expand_rows) = self.get_expansions();
        let expansion_factor = if is_full { 1000000 } else { 100 };

        let galaxies = self
            .space
            .sparse_iter()
            .filter(|(_, c)| *c == &'#')
            .map(|(p, _)| p)
            .cloned()
            .collect::<Vec<_>>();
        let mut total = 0;
        for (a, ga) in galaxies.iter().enumerate() {
            for (b, gb) in galaxies.iter().enumerate() {
                if a > b {
                    let mut cost = 0;
                    for x in min(gb.0, ga.0)..max(gb.0, ga.0) {
                        if expand_columns.contains(&x) {
                            cost += expansion_factor;
                        } else {
                            cost += 1;
                        }
                    }
                    for y in min(gb.1, ga.1)..max(gb.1, ga.1) {
                        if expand_rows.contains(&y) {
                            cost += expansion_factor;
                        } else {
                            cost += 1;
                        }
                    }
                    debug!(ga = debug(ga), gb = debug(gb), cost, "cost");
                    total += cost;
                }
            }
        }
        Ok(total)
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
    fn part1_sample() {
        let input = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ]
        .join("\n");
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(374 as ResultType, s.answer_part1(false).unwrap());
    }
    #[test]
    #[traced_test]
    fn part2_sample() {
        let input = [
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ]
        .join("\n");
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(8410 as ResultType, s.answer_part2(false).unwrap());
    }
}
