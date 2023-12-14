use std::io::{BufRead, BufReader};

use tracing::debug;
use utils::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grids: Vec<Matrix<char>>,
}
impl Solution {
    fn add_grid(&mut self, grid: Matrix<char>) {
        self.grids.push(grid);
    }

    fn difference(a: &str, b: &str) -> usize {
        a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
    }

    fn test_vertical_mirror(grid: &Matrix<char>, mirror_after: isize, smudges: usize) -> bool {
        let (maxx, maxy) = grid.dimensions();
        let mut differences = 0;
        for y in 0..=maxy {
            let mut left = String::new();
            let mut right = String::new();
            let range = std::cmp::min(mirror_after + 1, maxx - mirror_after);
            debug!(mirror_after, range, maxx, "range");
            for i in 0..range {
                left.push(*grid.get(mirror_after - i, y).unwrap());
                right.push(*grid.get(mirror_after + 1 + i, y).unwrap());
                debug!(mirror_after, i, left, right, "step");
            }
            differences += Self::difference(&left, &right);
            debug!(mirror_after, range, maxx, left, right, "range");
        }
        differences == smudges
    }

    fn find_vertical_mirror(grid: &Matrix<char>, smudges: usize) -> Vec<isize> {
        let (maxx, _) = grid.dimensions();
        let mut mirrors = Vec::new();
        // Vertical mirror
        for x in 0..maxx {
            if Self::test_vertical_mirror(grid, x, smudges) {
                debug!(x, "vertical");
                mirrors.push(x);
            }
        }
        mirrors
    }
    fn test_horizontal_mirror(grid: &Matrix<char>, mirror_after: isize, smudges: usize) -> bool {
        let (maxx, maxy) = grid.dimensions();
        let mut differences = 0;
        for x in 0..=maxx {
            let mut top = String::new();
            let mut bottom = String::new();
            let range = std::cmp::min(mirror_after + 1, maxy - mirror_after);
            debug!(mirror_after, range, maxy, "range");
            for i in 0..range {
                top.push(*grid.get(x, mirror_after - i).unwrap());
                bottom.push(*grid.get(x, mirror_after + 1 + i).unwrap());
                debug!(mirror_after, i, top, bottom, "step");
            }
            differences += Self::difference(&top, &bottom);
            debug!(mirror_after, range, maxx, top, bottom, "range");
        }
        differences == smudges
    }

    fn find_horizontal_mirror(grid: &Matrix<char>, smudges: usize) -> Vec<isize> {
        let (_, maxy) = grid.dimensions();
        // Vertical mirror
        let mut mirrors = Vec::new();
        for y in 0..maxy {
            if Self::test_horizontal_mirror(grid, y, smudges) {
                debug!(y, "horizontal");
                mirrors.push(y);
            }
        }
        mirrors
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let mut grid = Matrix::new();
        let mut y = 0;
        for line in reader.lines().map_while(Result::ok) {
            // Implement for problem
            if line.trim().is_empty() {
                if !grid.is_empty() {
                    solution.add_grid(grid);
                }
                grid = Matrix::new();
                y = 0;
            } else {
                for (x, c) in line.chars().enumerate() {
                    grid.set(x as isize, y, c);
                }
                y += 1;
            }
        }
        if !grid.is_empty() {
            solution.add_grid(grid);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut horizontal_mirrors = Vec::new();
        let mut vertical_mirrors = Vec::new();
        for grid in &self.grids {
            let mut mirrors = Self::find_vertical_mirror(grid, 0);
            vertical_mirrors.append(&mut mirrors);
            let mut mirrors = Self::find_horizontal_mirror(grid, 0);
            horizontal_mirrors.append(&mut mirrors);
        }
        debug!(
            vertical_mirrors = debug(&vertical_mirrors),
            horizontal_mirrors = debug(&horizontal_mirrors),
            "mirrors"
        );
        let h_score = horizontal_mirrors
            .iter()
            .map(|s| 100 * (*s as ResultType + 1))
            .sum::<ResultType>();
        let v_score = vertical_mirrors
            .iter()
            .map(|s| *s as ResultType + 1)
            .sum::<ResultType>();
        let score = h_score + v_score;
        // Implement for problem
        Ok(score)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut horizontal_mirrors = Vec::new();
        let mut vertical_mirrors = Vec::new();
        for grid in &self.grids {
            let mut mirrors = Self::find_vertical_mirror(grid, 1);
            vertical_mirrors.append(&mut mirrors);
            let mut mirrors = Self::find_horizontal_mirror(grid, 1);
            horizontal_mirrors.append(&mut mirrors);
        }
        debug!(
            vertical_mirrors = debug(&vertical_mirrors),
            horizontal_mirrors = debug(&horizontal_mirrors),
            "mirrors"
        );
        let h_score = horizontal_mirrors
            .iter()
            .map(|s| 100 * (*s as ResultType + 1))
            .sum::<ResultType>();
        let v_score = vertical_mirrors
            .iter()
            .map(|s| *s as ResultType + 1)
            .sum::<ResultType>();
        let score = h_score + v_score;
        // Implement for problem
        Ok(score)
    }
}
