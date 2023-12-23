use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};
use tracing::debug;
use utils::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    tiles: Matrix<char>,
}
impl Solution {
    fn set_tile(&mut self, x: usize, y: usize, c: char) {
        self.tiles.set(x as isize, y as isize, c);
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            // Implement for problem
            for (x, c) in line.chars().enumerate() {
                solution.set_tile(x, y, c);
            }
        }
        Ok(solution)
    }
}
impl Solution {
    fn longest_path(
        &self,
        sx: isize,
        sy: isize,
        ex: isize,
        ey: isize,
        ignore_slopes: bool,
    ) -> ResultType {
        let mut max = 0;
        let mut remaining = Vec::new();
        remaining.push((sx, sy, 0, HashSet::new()));
        while let Some((x, y, d, visited)) = remaining.pop() {
            if x == ex && y == ey {
                max = std::cmp::max(max, d);
                debug!(max, d, "end");
                continue;
            }
            // Which directions can walk?
            let directions = if ignore_slopes {
                vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
            } else {
                match self.tiles.get(x, y).unwrap_or(&'#') {
                    '.' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
                    '>' => vec![(1, 0)],
                    '<' => vec![(-1, 0)],
                    '^' => vec![(0, -1)],
                    'v' => vec![(0, 1)],
                    '#' => panic!("standing in a tree"),
                    c => panic!("unexpected {c}"),
                }
            };
            for (dx, dy) in directions {
                if !visited.contains(&(x + dx, y + dy)) {
                    match self.tiles.get(x + dx, y + dy).unwrap_or(&'#') {
                        '#' => {}
                        '.' | '>' | 'v' | '<' | '^' => {
                            let mut n_visited = visited.clone();
                            n_visited.insert((x + dx, y + dy));
                            remaining.push((x + dx, y + dy, d + 1, n_visited));
                        }
                        c => panic!("unexpected {c}"),
                    }
                }
            }
        }
        max
    }

    fn longest_path_part2(&self, sx: isize, sy: isize, ex: isize, ey: isize) -> ResultType {
        let mut adjacency = HashMap::new();
        let (max_x, max_y) = self.tiles.dimensions();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if !matches!(self.tiles.get(x, y).unwrap_or(&'#'), '#') {
                    let r = adjacency.entry((x, y)).or_insert_with(HashMap::new);
                    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        if !matches!(self.tiles.get(x + dx, y + dy).unwrap_or(&'#'), '#') {
                            r.insert((x + dx, y + dy), 1);
                        }
                    }
                }
            }
        }
        let mut num_compact = 0;
        let keys = adjacency.keys().cloned().collect::<Vec<_>>();
        for node in keys {
            let neighbours: HashMap<(isize, isize), i32> = adjacency.get(&node).unwrap().clone();
            if neighbours.len() != 2 {
                continue;
            }
            num_compact += 1;
            // remove 'node' from both neighbours
            for (neigh, d) in neighbours.iter() {
                let other = neighbours
                    .iter()
                    .find(|(other, _)| other.0 != neigh.0 || other.1 != neigh.1)
                    .unwrap();
                let n: &mut HashMap<(isize, isize), i32> =
                    adjacency.get_mut(&(neigh.0, neigh.1)).unwrap();
                n.remove(&node);
                n.insert(*other.0, other.1 + d);
            }
            adjacency.remove(&node);
        }
        debug!(adjacency = debug(&adjacency), num_compact, "adj");

        let mut max = 0;
        let mut remaining = Vec::new();
        remaining.push(((sx, sy), 0, HashSet::new()));
        while let Some(((x, y), d, visited)) = remaining.pop() {
            if x == ex && y == ey {
                if max < d {
                    debug!(max, d, "end");
                    max = d;
                }
                continue;
            }
            // Which directions can walk?
            let neighbours = adjacency.get(&(x, y)).unwrap();
            for (neighbour, len) in neighbours {
                if !visited.contains(&neighbour) {
                    let mut n_visited = visited.clone();
                    n_visited.insert(neighbour);
                    remaining.push((*neighbour, d + len, n_visited));
                }
            }
        }
        max as ResultType
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Find start & end
        let (max_x, max_y) = self.tiles.dimensions();
        let start = (0..=max_x)
            .map(|x| (x, self.tiles.get(x, 0).unwrap_or(&'#')))
            .find(|(_, c)| *c == &'.')
            .unwrap()
            .0;
        let end = (0..=max_x)
            .map(|x| (x, self.tiles.get(x, max_y).unwrap_or(&'#')))
            .find(|(_, c)| *c == &'.')
            .unwrap()
            .0;

        debug!(start, end, "s");

        let r = self.longest_path(start, 0, end, max_y, false);
        debug!(r, "done?");
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Find start & end
        let (max_x, max_y) = self.tiles.dimensions();
        let start = (0..=max_x)
            .map(|x| (x, self.tiles.get(x, 0).unwrap_or(&'#')))
            .find(|(_, c)| *c == &'.')
            .unwrap()
            .0;
        let end = (0..=max_x)
            .map(|x| (x, self.tiles.get(x, max_y).unwrap_or(&'#')))
            .find(|(_, c)| *c == &'.')
            .unwrap()
            .0;

        debug!(start, end, "s");

        let r = self.longest_path_part2(start, 0, end, max_y);
        debug!(r, "done?");
        // Implement for problem
        // TOO Low: 6282
        // Did not end, but answer = 6574
        Ok(r)
    }
}
