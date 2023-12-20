use std::{io::{BufRead, BufReader}, collections::{VecDeque, HashMap, HashSet}};
use tracing::{debug, info};

pub type ResultType = u64;

#[derive(Debug, PartialEq)]
enum Mode {
    None,
    FlipFlop,
    Conjunction,
}
#[derive(Debug, Default)]
pub struct Solution {
    modules: Vec<(String, Mode, Vec<String>)>,
}
impl Solution {
    fn add(&mut self, source: &str, targets: &str) {
        let targets = targets.split(", ").map(|s| s.to_string()).collect();
        let (source, mode) = match source.chars().nth(0) {
            Some('%') => {
                (source.chars().skip(1).collect::<String>(), Mode::FlipFlop)
            }
            Some('&') => {
                (source.chars().skip(1).collect::<String>(),                 Mode::Conjunction)
            }
            _ => {
                (source.to_string(),                 Mode::None)
            }
        };
        self.modules.push((source.to_string(), mode, targets));
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().map_while(Result::ok) {
            // Implement for problem
            let (from, to) = line.split_once(" -> ").unwrap();
            solution.add(from, to);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        // Setup memory for conjunctions
        let mut memory = HashMap::new();
        for (s, _, _) in self.modules.iter().filter(|(_, m, _)| m == &Mode::Conjunction) {
            memory.insert((*s).clone(), HashMap::new());
        }
        for (src, _, targets) in &self.modules {
            for target in targets {
                match memory.get_mut(target) {
                    None => {},
                    Some(v) => {v.insert(src.clone(), -1);}
                }
            }
        }   
        
        let mut num_low = 0;
        let mut num_high = 0;
        let mut state = HashMap::new();
        let mut next = VecDeque::new();
        for i in 0..1000 {
            info!(i, "pass");
        next.push_back(("button".to_string(), "broadcaster".to_string(), -1));
        while let Some((s, t, v)) = next.pop_front() {
            debug!("{s} -{v}-> {t}");
            if v == 1 {
                num_high += 1 as ResultType;
            }
            if v == -1 {
                num_low += 1 as ResultType;
            }
        for (_, mode, targets) in self.modules.iter().filter(|(s, _, _)| s==&t) {
                let mut new_v = 0;
                if mode == &Mode::FlipFlop {
                    if v == 1 {
                        // Nothing happens
                        continue;
                    }
                    if v == -1 {
                        let s = state.entry(t.clone()).or_insert(-1);
                        // flip state
                        *s *= -1;
                        new_v = *s;
                    }
                }
                if mode == &Mode::Conjunction {
                    let mem = memory.get_mut(&t).unwrap();
                    mem.insert(s.clone(), v);
                    new_v = if mem.iter().all(|(_, v)| *v==1) { -1 } else { 1 };
                    debug!(s, t, v, mem = debug(mem), new_v, "c");
                }
                if mode == &Mode::None {
                    new_v = v;
                }
                for target in targets {
                next.push_back((t.clone(), (*target).clone(), new_v));
                }
            }
        }
        }
        info!(num_high, num_low, "done?");
        // Implement for problem
        Ok(num_high * num_low)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
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
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
