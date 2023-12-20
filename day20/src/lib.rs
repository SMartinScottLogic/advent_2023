use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{BufRead, BufReader},
};
use tracing::{debug, info};
use utils::math::lowest_common_multiple_many;

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
            Some('%') => (source.chars().skip(1).collect::<String>(), Mode::FlipFlop),
            Some('&') => (
                source.chars().skip(1).collect::<String>(),
                Mode::Conjunction,
            ),
            _ => (source.to_string(), Mode::None),
        };
        self.modules.push((source.to_string(), mode, targets));
    }

    fn perform_step(
        &self,
        ends: &HashSet<String>,
        state: &mut HashMap<String, i32>,
        memory: &mut HashMap<String, HashMap<String, i32>>,
    ) -> (ResultType, ResultType, HashSet<String>) {
        let mut num_high = 0 as ResultType;
        let mut num_low = 0 as ResultType;
        let mut next = VecDeque::new();
        let mut encountered_ends = HashSet::new();
        next.push_back(("button".to_string(), "broadcaster".to_string(), -1));
        while let Some((s, t, v)) = next.pop_front() {
            debug!("{s} -{v}-> {t}");
            if v == 1 {
                num_high += 1 as ResultType;
            }
            if v == -1 {
                num_low += 1 as ResultType;
            }
            for (_, mode, targets) in self.modules.iter().filter(|(s, _, _)| s == &t) {
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
                    new_v = if mem.iter().all(|(_, v)| *v == 1) {
                        -1
                    } else {
                        1
                    };
                    debug!(s, t, v, mem = debug(mem), new_v, "c");
                }
                if mode == &Mode::None {
                    new_v = v;
                }
                for target in targets {
                    next.push_back((t.clone(), (*target).clone(), new_v));
                    if ends.contains(target) && new_v == -1 {
                        encountered_ends.insert(target.clone());
                    }
                }
            }
        }
        (num_high, num_low, encountered_ends)
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
        for (s, _, _) in self
            .modules
            .iter()
            .filter(|(_, m, _)| m == &Mode::Conjunction)
        {
            memory.insert((*s).clone(), HashMap::new());
        }
        for (src, _, targets) in &self.modules {
            for target in targets {
                match memory.get_mut(target) {
                    None => {}
                    Some(v) => {
                        v.insert(src.clone(), -1);
                    }
                }
            }
        }

        let mut num_low = 0;
        let mut num_high = 0;
        let mut state = HashMap::new();
        for i in 0..1000 {
            info!(i, "pass");
            let (s_high, s_low, _) = self.perform_step(&HashSet::new(), &mut state, &mut memory);
            num_high += s_high;
            num_low += s_low;
        }
        info!(num_high, num_low, "done?");
        // Implement for problem
        Ok(num_high * num_low)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        if !is_full {
            return Ok(0);
        }
        // Setup memory for conjunctions
        let mut memory = HashMap::new();
        for (s, _, _) in self
            .modules
            .iter()
            .filter(|(_, m, _)| m == &Mode::Conjunction)
        {
            memory.insert((*s).clone(), HashMap::new());
        }
        for (src, _, targets) in &self.modules {
            for target in targets {
                match memory.get_mut(target) {
                    None => {}
                    Some(v) => {
                        v.insert(src.clone(), -1);
                    }
                }
            }
        }
        // broadcaster has 4x outputs, rx is conjunction of 1 (zh), which has 4x inputs -> separable graphs?
        let ends = memory
            .get("zh")
            .unwrap()
            .iter()
            .map(|(k, v)| k)
            .cloned()
            .collect::<HashSet<_>>();

        let mut first_seen = HashMap::new();
        let mut state = HashMap::new();

        let mut i = 0;
        loop {
            let (_, _, encountered_ends) = self.perform_step(&ends, &mut state, &mut memory);
            i += 1;
            for encountered_end in encountered_ends {
                first_seen.entry(encountered_end).or_insert(i);
            }
            if first_seen.len() == ends.len() {
                break;
            }
            debug!(state = debug(&state), i, ends = debug(&ends), "s");
        }
        let r = lowest_common_multiple_many(&first_seen.values().cloned().collect::<Vec<_>>());
        info!(
            first_seen = debug(first_seen),
            ends = debug(ends),
            r,
            "done?"
        );
        Ok(r)
    }
}
