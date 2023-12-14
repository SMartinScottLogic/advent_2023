use std::{
    collections::{HashMap, VecDeque},
    io::{BufRead, BufReader},
};

use tracing::debug;
use utils::Matrix;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    data: Matrix<char>,
}
impl Solution {
    fn add_cell(&mut self, x: usize, y: usize, c: char) {
        self.data.set(x as isize, y as isize, c);
    }

    fn calculate_load_north(grid: &Matrix<char>) -> ResultType {
        let (maxx, maxy) = grid.dimensions();

        let mut total = 0;
        for y in 0..=maxy {
            for x in 0..=maxx {
                match grid.get(x, y) {
                    None | Some('.') => {}
                    Some('O') => {
                        total += maxy - y + 1;
                    }
                    Some('#') => {}
                    c => panic!("Unexpected: {:?}", c),
                }
            }
        }
        total as ResultType
    }

    fn roll_north(data: &mut Matrix<char>) {
        let (maxx, maxy) = data.dimensions();
        let mut vacant: HashMap<isize, isize> = HashMap::new();
        for y in 0..=maxy {
            for x in 0..=maxx {
                match data.get(x, y) {
                    None | Some('.') => {}
                    Some('#') => {
                        vacant.insert(x, y + 1);
                    }
                    Some('O') => {
                        let entry = vacant.entry(x).or_default();
                        data.set(x, y, '.');
                        data.set(x, *entry, 'O');
                        *entry += 1;
                    }
                    c => panic!("Unexpected: {:?}", c),
                }
            }
        }
    }
    fn roll_east(data: &mut Matrix<char>) {
        let (maxx, maxy) = data.dimensions();
        let mut vacant: HashMap<isize, isize> = HashMap::new();

        for ox in 0..=maxx {
            let x = maxx - ox;
            for y in 0..=maxy {
                match data.get(x, y) {
                    None | Some('.') => {}
                    Some('#') => {
                        vacant.insert(y, x - 1);
                    }
                    Some('O') => {
                        let entry = vacant.entry(y).or_insert(maxx);
                        data.set(x, y, '.');
                        data.set(*entry, y, 'O');
                        *entry -= 1;
                    }
                    c => panic!("Unexpected: {:?}", c),
                }
            }
        }
    }
    fn roll_south(data: &mut Matrix<char>) {
        let (maxx, maxy) = data.dimensions();
        let mut vacant: HashMap<isize, isize> = HashMap::new();
        for oy in 0..=maxy {
            let y = maxy - oy;
            for x in 0..=maxx {
                match data.get(x, y) {
                    None | Some('.') => {}
                    Some('#') => {
                        vacant.insert(x, y - 1);
                    }
                    Some('O') => {
                        let entry = vacant.entry(x).or_insert(maxy);
                        data.set(x, y, '.');
                        data.set(x, *entry, 'O');
                        *entry -= 1;
                    }
                    c => panic!("Unexpected: {:?}", c),
                }
            }
        }
    }
    fn roll_west(data: &mut Matrix<char>) {
        let (maxx, maxy) = data.dimensions();
        let mut vacant: HashMap<isize, isize> = HashMap::new();

        for x in 0..=maxx {
            for y in 0..=maxy {
                match data.get(x, y) {
                    None | Some('.') => {}
                    Some('#') => {
                        vacant.insert(y, x + 1);
                    }
                    Some('O') => {
                        let entry = vacant.entry(y).or_insert(0);
                        data.set(x, y, '.');
                        data.set(*entry, y, 'O');
                        *entry += 1;
                    }
                    c => panic!("Unexpected: {:?}", c),
                }
            }
        }
    }
    fn roll_cycle(data: &mut Matrix<char>) {
        Self::roll_north(data);
        Self::roll_west(data);
        Self::roll_south(data);
        Self::roll_east(data);
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().map_while(Result::ok).enumerate() {
            for (x, c) in line.chars().enumerate() {
                solution.add_cell(x, y, c);
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
        // Implement for problem
        let mut data = self.data.clone();
        Self::roll_north(&mut data);

        let total = Self::calculate_load_north(&data);

        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        let mut data = self.data.clone();

        let lookback = 1000;
        let mut window = VecDeque::new();

        let mut seen = HashMap::new();
        let mut i = 0;
        let (current ,cycle_len) = loop {
            Self::roll_cycle(&mut data);
            i += 1;
            let load = Self::calculate_load_north(&data);

            window.push_back(load);
            if window.len() > lookback {
                window.pop_front();
            }
            if window.len() == lookback {
            if let Some(last_seen) = seen.get(&window) {
                let cycle_len = i - last_seen;
                debug!(i, last_seen, cycle_len, "dupe");
                break (i, cycle_len);
            }
            seen.insert(window.clone(), i);
            }
        };

        let loops = (1000000000 - current) / cycle_len;
        let remainder = 1000000000 - current - (cycle_len * loops);
        debug!(loops, remainder, "loops?");

        for _ in 0..remainder {
            Self::roll_cycle(&mut data);
        }

        let total = Self::calculate_load_north(&data);
        Ok(total)
    }
}

