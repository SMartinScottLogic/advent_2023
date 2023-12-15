use std::io::{BufRead, BufReader};

use pathfinding::directed::dijkstra::dijkstra_all;
use tracing::debug;
use utils::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    grid: Matrix<char>,
}
impl Solution {
    fn set_grid(&mut self, x: usize, y: usize, c: char) {
        self.grid.set(x as isize, y as isize, c);
    }

    fn calculate_loop_distances(
        grid: &Matrix<char>,
    ) -> std::collections::HashMap<(isize, isize), ((isize, isize), i32)> {
        let ((start_x, start_y), _) = grid.sparse_iter().find(|(_, c)| *c == &'S').unwrap();
        let results = dijkstra_all(&(*start_x, *start_y), |(x, y)| {
            let x = *x;
            let y = *y;
            let mut r = Vec::new();
            let (north_valid, south_valid, east_valid, west_valid) = match grid.get(x, y).unwrap() {
                '|' => (true, true, false, false),
                '-' => (false, false, true, true),
                'L' => (true, false, true, false),
                'J' => (true, false, false, true),
                '7' => (false, true, false, true),
                'F' => (false, true, true, false),
                'S' => (true, true, true, true),
                _ => panic!(),
            };
            if north_valid {
                match grid.get(x, y - 1) {
                    Some(c) if *c == '|' => r.push(((x, y - 1), 1)),
                    Some(c) if *c == '7' => r.push(((x, y - 1), 1)),
                    Some(c) if *c == 'F' => r.push(((x, y - 1), 1)),
                    Some(c) if *c == 'S' => r.push(((x, y - 1), 1)),
                    _ => {}
                };
            }
            if south_valid {
                match grid.get(x, y + 1) {
                    Some(c) if *c == '|' => r.push(((x, y + 1), 1)),
                    Some(c) if *c == 'L' => r.push(((x, y + 1), 1)),
                    Some(c) if *c == 'J' => r.push(((x, y + 1), 1)),
                    Some(c) if *c == 'S' => r.push(((x, y + 1), 1)),
                    _ => {}
                };
            }
            if west_valid {
                match grid.get(x - 1, y) {
                    Some(c) if *c == '-' => r.push(((x - 1, y), 1)),
                    Some(c) if *c == 'F' => r.push(((x - 1, y), 1)),
                    Some(c) if *c == 'L' => r.push(((x - 1, y), 1)),
                    Some(c) if *c == 'S' => r.push(((x - 1, y), 1)),
                    _ => {}
                };
            }
            if east_valid {
                match grid.get(x + 1, y) {
                    Some(c) if *c == '-' => r.push(((x + 1, y), 1)),
                    Some(c) if *c == 'J' => r.push(((x + 1, y), 1)),
                    Some(c) if *c == '7' => r.push(((x + 1, y), 1)),
                    Some(c) if *c == 'S' => r.push(((x + 1, y), 1)),
                    _ => {}
                };
            }
            r
        });
        let mut result_matrix = Matrix::new();
        for (to, (via, cost)) in &results {
            debug!(to = debug(to), via = debug(via), cost, "cost");
            result_matrix.set(to.0, to.1, *cost);
        }
        results
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.set_grid(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let results = Self::calculate_loop_distances(&self.grid);
        let result = results.iter().max_by_key(|(_, (_, cost))| cost).unwrap();
        Ok(result.1 .1 as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut expanded_grid = Matrix::new();
        for ((x, y), c) in self.grid.sparse_iter() {
            expanded_grid.set(x * 2, y * 2, *c);
            match *c {
                '|' => expanded_grid.set(x * 2, y * 2 + 1, '|'),
                '-' => expanded_grid.set(x * 2 + 1, y * 2, '-'),
                'L' => expanded_grid.set(x * 2 + 1, y * 2, '-'),
                'J' => {}
                '7' => expanded_grid.set(x * 2, y * 2 + 1, '|'),
                'F' => {
                    expanded_grid.set(x * 2 + 1, y * 2, '-');
                    expanded_grid.set(x * 2, y * 2 + 1, '|')
                }
                'S' => {
                    expanded_grid.set(x * 2 + 1, y * 2, '-');
                    expanded_grid.set(x * 2, y * 2 + 1, '|')
                }
                _ => {}
            }
        }
        let results = Self::calculate_loop_distances(&expanded_grid);

        let mut loop_nodes = Matrix::new();
        let ((start_x, start_y), _) = expanded_grid
            .sparse_iter()
            .find(|(_, c)| *c == &'S')
            .unwrap();
        loop_nodes.set(*start_x, *start_y, 1);
        for ((x, y), _) in results {
            loop_nodes.set(x, y, 1);
        }
        let (maxx, maxy) = loop_nodes.dimensions();
        let mut probes = vec![(maxx, maxy + 1)];
        while let Some((probex, probey)) = probes.pop() {
            loop_nodes.set(probex, probey, 2);
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
                if nx >= -1
                    && ny >= -1
                    && nx <= maxx + 1
                    && ny <= maxy + 1
                    && loop_nodes.get(nx, ny).is_none()
                {
                    probes.push((nx, ny))
                }
            }
        }
        let mut reachable = Matrix::new();
        let mut count = 0;
        for y in -1..=maxy + 1 {
            for x in -1..=maxx + 1 {
                if x % 2 == 0 && y % 2 == 0 {
                    let score = match loop_nodes.get(x, y) {
                        None if x % 2 == 0 && y % 2 == 0 => 1,
                        _ => 0,
                    };
                    reachable.set(x / 2, y / 2, score);
                    count += score;
                }
            }
        }
        Ok(count)
    }
}

#[cfg(test)]
mod test {}
