use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    data: Vec<(String, Vec<ResultType>)>,
}
impl Solution {
    fn add_springs(&mut self, known: &str, groups: Vec<u64>) {
        self.data.push((known.to_string(), groups));
    }

    fn possible_arrangements(puzzle: &str, groups: &Vec<ResultType>) -> ResultType {
        match puzzle.find('?') {
            None => {
                debug!(puzzle, groups = debug(&groups), "score");
                let observed = puzzle
                    .split('.')
                    .map(|g| g.len() as ResultType)
                    .filter(|l| l > &0)
                    .collect::<Vec<_>>();
                if observed.eq(groups) {
                    1
                } else {
                    0
                }
            }
            Some(i) => {
                let mut dot_puzzle = puzzle.to_string();
                dot_puzzle.replace_range(i..=i, ".");
                let mut hash_puzzle = puzzle.to_string();
                hash_puzzle.replace_range(i..=i, "#");
                Self::possible_arrangements(&dot_puzzle, groups)
                    + Self::possible_arrangements(&hash_puzzle, groups)
            }
        }
    }

    fn possible_arrangements_step2(puzzle: &str, groups: &Vec<ResultType>) -> ResultType {
        let mut puzzle = puzzle.to_string();
        puzzle.push('.');
        let mut counts = HashMap::new();
        counts.insert((0, 0, 0), 1 as ResultType);
        let num_groups = groups.len();
        for (pos, c) in puzzle.chars().enumerate() {
            for group_id in 0..=num_groups {
                let group = groups.get(group_id).unwrap_or(&0);
                debug!(pos, group_id, group, "grrr");
                for in_group_pos in 0..=(1 + group) {
                    let mult = counts.get(&(pos, group_id, in_group_pos));
                    if mult.is_none() {
                        continue;
                    }
                    let mult = *mult.unwrap_or(&0);
                    if mult == 0 {
                        continue;
                    }
                    debug!(
                        mult,
                        pos,
                        group_id,
                        in_group_pos,
                        c = debug(c),
                        counts = debug(&counts),
                        groups = debug(groups),
                        "get"
                    );
                    match c {
                        '.' if in_group_pos == 0 => {
                            *counts.entry((pos + 1, group_id, in_group_pos)).or_default() += mult;
                        }
                        '.' => {
                            let g = groups.get(group_id).unwrap_or(&0);
                            if *g == in_group_pos {
                                *counts.entry((pos + 1, group_id + 1, 0)).or_default() += mult;
                            }
                        }
                        '#' => {
                            let g = groups.get(group_id).unwrap_or(&0);
                            if *g >= in_group_pos {
                                *counts
                                    .entry((pos + 1, group_id, in_group_pos + 1))
                                    .or_default() += mult;
                            }
                        }
                        '?' => {
                            let g = groups.get(group_id).unwrap_or(&0);
                            // Can be a '.'
                            if in_group_pos == 0 {
                                *counts.entry((pos + 1, group_id, 0)).or_default() += mult;
                            }
                            if *g == in_group_pos {
                                *counts.entry((pos + 1, group_id + 1, 0)).or_default() += mult;
                            }
                            // Can be a '#'
                            if *g >= in_group_pos {
                                *counts
                                    .entry((pos + 1, group_id, in_group_pos + 1))
                                    .or_default() += mult;
                            }
                        }
                        _ => panic!("unexpected condition {c}"),
                    }
                    debug!(
                        state = debug((pos, group_id, in_group_pos)),
                        c = debug(c),
                        counts = debug(&counts),
                        "after"
                    );
                }
            }
        }
        debug!(count = debug(&counts), puzzle, "done");
        *counts.get(&(puzzle.len(), groups.len(), 0)).unwrap()
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (_, line) in reader.lines().flatten().enumerate() {
            let (lhs, rhs) = line.split_once(' ').unwrap();
            let groups = rhs.split(',').map(|v| v.parse().unwrap()).collect();
            solution.add_springs(lhs, groups);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let result = self
            .data
            .iter()
            .map(|(data, groups)| Self::possible_arrangements(data, groups))
            .sum();
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let data = self
            .data
            .iter()
            .map(|(conditions, groups)| {
                let mut n_conditions = Vec::new();
                let mut n_groups = Vec::new();
                for _ in 1..=5 {
                    n_conditions.push(conditions.to_owned());
                    let mut n_g = groups.clone();
                    n_groups.append(&mut n_g);
                }
                let n_conditions = n_conditions.join("?");
                (n_conditions, n_groups)
            })
            .collect::<Vec<_>>();

        let result = data
            .iter()
            .map(|(data, groups)| Self::possible_arrangements_step2(data, groups))
            .sum();
        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn t1() {
        assert_eq!(1, Solution::possible_arrangements_step2("#", &vec![1]));
    }

    #[test]
    #[traced_test]
    fn t2() {
        assert_eq!(1, Solution::possible_arrangements_step2("##", &vec![2]));
    }

    #[test]
    #[traced_test]
    fn t3() {
        assert_eq!(1, Solution::possible_arrangements_step2("#.#", &vec![1, 1]));
    }

    #[test]
    #[traced_test]
    fn t4() {
        assert_eq!(2, Solution::possible_arrangements_step2("?.?", &vec![1]));
    }
}
