use std::io::{BufRead, BufReader};

use tracing::{debug, info};

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
        let mut wins = Vec::new();
        for (race, max_hold) in self.times.iter().enumerate() {
            let target_distance = self.distances[race];
            let mut num_wins = 0;
            for hold in 0..=*max_hold {
                let speed = hold;
                let distance = speed * (max_hold - hold);
                debug!(hold, distance, target_distance, "travel");
                if distance > target_distance {
                    num_wins += 1;
                }
            }
            wins.push(num_wins);
        }
        info!(wins = debug(&wins), "wins");
        let result = wins.iter().fold(1, |mut acc, v| {
            acc *= v;
            acc
        });
        // Implement for problem
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let target_distance = self.distance;
        let max_hold = self.time;
        let mut num_wins = 0;

        for hold in 0..=max_hold {
            let speed = hold;
            let distance = speed * (max_hold - hold);
            debug!(hold, distance, target_distance, "travel");
            if distance > target_distance {
                num_wins += 1;
            }
        }
        // Implement for problem
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
mod test {
    use super::*;
    use std::io::BufReader;

    use utils::Solution;

    #[test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
