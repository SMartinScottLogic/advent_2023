use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

use tracing::debug;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    history: Vec<Vec<ResultType>>,
}
impl Solution {
    fn add_history(&mut self, value: Vec<ResultType>) {
        self.history.push(value);
    }
    fn next_element(row: &Vec<ResultType>) -> ResultType {
        let mut diff = VecDeque::new();
        diff.push_back(row.to_owned());
        loop {
            if let Some(last) = diff.back() {
                if last.iter().all(|v| *v == 0 as ResultType) {
                    break;
                }
                let next = last
                    .iter()
                    .fold((Vec::new(), None), |(mut acc, last), v| {
                        if let Some(l) = last {
                            acc.push(v - l);
                        }
                        (acc, Some(v))
                    });
                debug!(last = debug(&last), next = debug(&next), "next");
                diff.push_back(next.0);
            } else {
                panic!();
            }
        }
        let mut prev: Option<Vec<ResultType>> = None;
        while let Some(n) = diff.pop_back() {
            let mut current = n;
            match prev {
                None => current.push(0),
                Some(p) => {
                    let new = p.last().unwrap() + current.last().unwrap();
                    current.push(new);
                }
            }
            prev = Some(current);
        }
        debug!(prev = debug(&prev), "prev");
        *prev.unwrap().last().unwrap()
    }
}
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (_idx, line) in reader.lines().flatten().enumerate() {
            let row = line.split(' ').map(|v| v.parse().unwrap()).collect();
            solution.add_history(row);
            // Implement for problem
        }
        Ok(solution)
    }
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let result = self.history.iter().map(Self::next_element).sum();
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let result = self
            .history
            .iter()
            .map(|row| {
                let mut row = row.clone();
                row.reverse();
                Self::next_element(&row)
            })
            .sum();
        Ok(result)
    }
}

#[cfg(test)]
mod test {
}
