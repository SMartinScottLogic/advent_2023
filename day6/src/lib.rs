use std::io::{BufRead, BufReader};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    times: Vec<u64>,
    distances: Vec<u64>,
    time: u64,
    distance: u64,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let result = self
            .times
            .iter()
            .enumerate()
            .map(|(race, max_hold)| Self::calculate_wins(self.distances[race], *max_hold))
            .fold(1, |mut acc, v| {
                acc *= v;
                acc
            });
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let num_wins = Self::calculate_wins(self.distance, self.time);
        Ok(num_wins)
    }
}

impl Solution {
    fn set_times(&mut self, times: Vec<u64>) {
        self.times = times;
    }
    fn set_distances(&mut self, distances: Vec<u64>) {
        self.distances = distances;
    }
    fn set_time(&mut self, time: u64) {
        self.time = time;
    }
    fn set_distance(&mut self, distance: u64) {
        self.distance = distance;
    }
    fn calculate_wins(target_distance: u64, max_hold: u64) -> ResultType {
        let mut num_wins = 0;
        for hold in 0..=max_hold {
            let speed = hold;
            let distance = speed * (max_hold - hold);
            debug!(hold, distance, target_distance, "travel");
            if distance > target_distance {
                num_wins += 1;
            }
        }
        num_wins
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().flatten().enumerate() {
            // Implement for problem
            let (_, values) = line.split_once(':').unwrap();
            let int_values = values
                .trim()
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            let fat_value = values
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse()
                .unwrap();
            match id {
                0 => {
                    solution.set_times(int_values);
                    solution.set_time(fat_value);
                }
                1 => {
                    solution.set_distances(int_values);
                    solution.set_distance(fat_value);
                }
                _ => panic!(),
            }
        }
        Ok(solution)
    }
}
#[cfg(test)]
mod test {}
