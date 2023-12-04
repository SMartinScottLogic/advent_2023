use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    cards: Vec<(HashSet<u64>, Vec<u64>)>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut total = 0;
        for (winning, have) in &self.cards {
            let mut score = 0;
            for num in have {
                if winning.contains(num) {
                    if score == 0 {
                        score = 1;
                    } else {
                        score *= 2;
                    }
                }
            }
            total += score;
        }
        // Implement for problem
        Ok(total)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut count = HashMap::new();
        for (id, _) in self.cards.iter().enumerate() {
            count.insert(id, 1);
        }
        debug!(count = debug(&count), "start");
        for (id, (winning, have)) in self.cards.iter().enumerate() {
            let mut instance_count = 0;
            for num in have {
                if winning.contains(num) {
                    instance_count += 1;
                }
            }
            let unused_count = *count.get(&id).unwrap();
            if instance_count > 0 {
                for i in 1..=instance_count {
                    if let Some(v) = count.get_mut(&(i + id)) {
                        *v += unused_count
                    }
                }
            }
        }
        debug!(count = debug(&count), "pass");
        let result = count.values().sum();
        // Implement for problem
        Ok(result)
    }
}

impl Solution {
    fn add_cards(&mut self, winning: HashSet<u64>, have: Vec<u64>) {
        self.cards.push((winning, have));
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().flatten() {
            let (_, data) = line.split_once(':').unwrap();
            let (lhs, rhs) = data.split_once('|').unwrap();
            let winning = lhs
                .trim()
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.trim().parse().unwrap())
                .collect();
            let have = rhs
                .trim()
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|v| v.trim().parse().unwrap())
                .collect();
            solution.add_cards(winning, have);
            // Implement for problem
        }
        Ok(solution)
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn stub() {
        assert_eq!(1 + 1, 2);
    }
}
