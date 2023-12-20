use std::io::{BufRead, BufReader};
use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    workflows: Vec<Workflow>,
    parts: Vec<Part>,
}
impl Solution {
    fn add_workflow(&mut self, workflow: Workflow) {
        self.workflows.push(workflow);
    }
    fn add_part(&mut self, part: Part) {
        self.parts.push(part);
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let mut mode = 0;
        for line in reader.lines().map_while(Result::ok) {
            // Implement for problem
            if line.trim().is_empty() {
                mode += 1;
                continue;
            }
            match mode {
                0 => {
                    let workflow = Workflow::from(line);
                    solution.add_workflow(workflow);
                }
                1 => {
                    let part = Part::from(line);
                    solution.add_part(part);
                }
                _ => panic!(),
            }
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut r = 0;
        for part in &self.parts {
            let mut cur_workflow_name = "in".to_string();
            loop {
                if cur_workflow_name == "A" || cur_workflow_name == "R" {
                    break;
                }
                let workflow = self
                    .workflows
                    .iter()
                    .find(|w| w.name == cur_workflow_name)
                    .unwrap();
                for condition in &workflow.conditions {
                    if condition.0.matches(part) {
                        cur_workflow_name = condition.1.to_owned();
                        break;
                    }
                }
            }
            debug!(part = debug(part), cur_workflow_name, "step");
            if cur_workflow_name == "A" {
                r += part.x + part.m + part.a + part.s;
            }
        }
        // Implement for problem
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut status = Vec::new();
        let mut accept = Vec::new();
        status.push(("in", (1, 4000), (1, 4000), (1, 4000), (1, 4000)));
        while let Some((name, mut x_range, mut m_range, mut a_range, mut s_range)) = status.pop() {
            // Accept terminal
            if name == "A" {
                accept.push((x_range, m_range, a_range, s_range));
                continue;
            }
            // Reject terminal
            if name == "R" {
                continue;
            }
            let workflow = self.workflows.iter().find(|w| w.name == name).unwrap();
            debug!(
                workflow = debug(workflow),
                x_range = debug(x_range),
                m_range = debug(m_range),
                a_range = debug(a_range),
                s_range = debug(s_range),
                "compute out range(s)"
            );
            for (condition, output) in &workflow.conditions {
                match condition {
                    Condition::None => {
                        status.push((output, x_range, m_range, a_range, s_range));
                    }
                    Condition::Less(var, val) if var == "x" => {
                        if x_range.0 < *val {
                            let bottom = x_range.0;
                            let top = std::cmp::min(val - 1, x_range.1);
                            status.push((output, (bottom, top), m_range, a_range, s_range));
                            if top != x_range.1 {
                                x_range.0 = *val;
                            } else {
                                panic!();
                            }
                        }
                    }
                    Condition::Less(var, val) if var == "m" => {
                        if m_range.0 < *val {
                            let bottom = m_range.0;
                            let top = std::cmp::min(val - 1, m_range.1);
                            status.push((output, x_range, (bottom, top), a_range, s_range));
                            if top != m_range.1 {
                                m_range.0 = *val;
                            } else {
                                panic!();
                            }
                        }
                    }
                    Condition::Less(var, val) if var == "a" => {
                        if a_range.0 < *val {
                            let bottom = a_range.0;
                            let top = std::cmp::min(val - 1, a_range.1);
                            status.push((output, x_range, m_range, (bottom, top), s_range));
                            if top != a_range.1 {
                                a_range.0 = *val;
                            } else {
                                panic!();
                            }
                        }
                    }
                    Condition::Less(var, val) if var == "s" => {
                        if s_range.0 < *val {
                            let bottom = s_range.0;
                            let top = std::cmp::min(val - 1, s_range.1);
                            status.push((output, x_range, m_range, a_range, (bottom, top)));
                            if top != s_range.1 {
                                s_range.0 = *val;
                            } else {
                                panic!();
                            }
                        }
                    }
                    Condition::Greater(var, val) if var == "x" => {
                        if x_range.1 > *val {
                            let bottom = std::cmp::max(val + 1, x_range.0);
                            let top = x_range.1;
                            status.push((output, (bottom, top), m_range, a_range, s_range));
                            if bottom != x_range.0 {
                                x_range.1 = *val;
                            } else {
                                panic!();
                            }
                        }
                    }
                    Condition::Greater(var, val) if var == "m" => {
                        if m_range.1 > *val {
                            let bottom = std::cmp::max(val + 1, m_range.0);
                            let top = m_range.1;
                            status.push((output, x_range, (bottom, top), a_range, s_range));
                            if bottom != m_range.0 {
                                m_range.1 = *val;
                            } else {
                                panic!();
                            }
                        }
                    }
                    Condition::Greater(var, val) if var == "a" => {
                        if a_range.1 > *val {
                            let bottom = std::cmp::max(val + 1, a_range.0);
                            let top = a_range.1;
                            status.push((output, x_range, m_range, (bottom, top), s_range));
                            if bottom != a_range.0 {
                                a_range.1 = *val;
                            } else {
                                panic!();
                            }
                        }
                    }
                    Condition::Greater(var, val) if var == "s" => {
                        if s_range.1 > *val {
                            let bottom = std::cmp::max(val + 1, s_range.0);
                            let top = s_range.1;
                            status.push((output, x_range, m_range, a_range, (bottom, top)));
                            if bottom != s_range.0 {
                                s_range.1 = *val;
                            } else {
                                panic!();
                            }
                        }
                    }

                    _ => panic!(),
                }
            }
        }
        debug!(accept = debug(&accept), count = accept.len(), "done?");
        let r = accept
            .into_iter()
            .map(|(x_range, m_range, a_range, s_range)| {
                (x_range.1 - x_range.0 + 1)
                    * (m_range.1 - m_range.0 + 1)
                    * (a_range.1 - a_range.0 + 1)
                    * (s_range.1 - s_range.0 + 1)
            })
            .sum();
        // Implement for problem
        Ok(r)
    }
}
#[derive(Debug)]
enum Condition {
    None,
    Less(String, ResultType),
    Greater(String, ResultType),
}
impl Condition {
    fn matches(&self, part: &Part) -> bool {
        match self {
            Self::None => true,
            Self::Less(var, value) if var == "x" => part.x < *value,
            Self::Less(var, value) if var == "m" => part.m < *value,
            Self::Less(var, value) if var == "a" => part.a < *value,
            Self::Less(var, value) if var == "s" => part.s < *value,
            Self::Greater(var, value) if var == "x" => part.x > *value,
            Self::Greater(var, value) if var == "m" => part.m > *value,
            Self::Greater(var, value) if var == "a" => part.a > *value,
            Self::Greater(var, value) if var == "s" => part.s > *value,
            _ => panic!("{self:?} vs {part:?}"),
        }
    }
}
impl From<&str> for Condition {
    fn from(value: &str) -> Self {
        let r = regex::Regex::new(r"^(?<var>.)(?<condition>.)(?<value>\d+)$").unwrap();
        let c = r.captures(value).unwrap();
        let var = c.name("var").unwrap().as_str().to_string();
        let value = c.name("value").unwrap().as_str().parse().unwrap();
        match c.name("condition").unwrap().as_str() {
            "<" => Self::Less(var, value),
            ">" => Self::Greater(var, value),
            _ => panic!("failed to parse: {value}"),
        }
    }
}
#[derive(Debug)]
struct Workflow {
    name: String,
    conditions: Vec<(Condition, String)>,
}
impl From<String> for Workflow {
    fn from(value: String) -> Self {
        let (name, output) = value.split_once('{').unwrap();
        let output = output.trim_end_matches('}');
        let conditions = output
            .split(',')
            .map(|s| match s.split_once(':') {
                Some((a, b)) => (Condition::from(a), b.to_string()),
                None => (Condition::None, s.to_string()),
            })
            .collect();
        Self {
            name: name.to_string(),
            conditions,
        }
    }
}
#[derive(Debug)]
struct Part {
    x: ResultType,
    m: ResultType,
    a: ResultType,
    s: ResultType,
}
impl From<String> for Part {
    fn from(value: String) -> Self {
        debug!(value, "part");
        let r = regex::Regex::new(r"^\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
        let c = r.captures(&value).unwrap();
        let x = c.name("x").unwrap().as_str().parse().unwrap();
        let m = c.name("m").unwrap().as_str().parse().unwrap();
        let a = c.name("a").unwrap().as_str().parse().unwrap();
        let s = c.name("s").unwrap().as_str().parse().unwrap();
        Self { x, m, a, s }
    }
}
