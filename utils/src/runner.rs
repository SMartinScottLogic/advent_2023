use std::fmt::Display;

use anyhow::{Context, Result};
use log::{error, info};
use yansi::Paint;

use crate::{load, Solution};

pub trait BaseName {
    fn base_name(&self) -> Self;
}

impl BaseName for &str {
    fn base_name(&self) -> Self {
        self.rfind('.').map_or(self, |n| &self[..n])
    }
}

pub fn run<S, R>(samples: &[&str], full: &[&str]) -> Result<()>
where
    S: Solution
        + TryFrom<std::io::BufReader<std::fs::File>, Error = std::io::Error>
        + std::fmt::Debug,
    S::Result: Context<R, anyhow::Error>,
    R: Display,
{
    let basename = std::env::current_exe()
        .ok()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .map(|s| s.base_name())
        .unwrap()
        .to_owned();

    for suffix in samples.iter() {
        let filename = format!("input/{basename}.{suffix}");
        if let Err(e) = run_solution_file::<S, R>(&filename, false) {
            error!("Failed running against '{filename}': {e:?}");
        }
    }
    for suffix in full.iter() {
        let filename = format!("input/{basename}.{suffix}");
        if let Err(e) = run_solution_file::<S, R>(&filename, true) {
            error!("Failed running against '{filename}': {e:?}");
        }
    }
    Ok(())
}

fn run_solution_file<S, R>(filename: &str, is_full: bool) -> Result<()>
where
    S: Solution
        + TryFrom<std::io::BufReader<std::fs::File>, Error = std::io::Error>
        + std::fmt::Debug,
    S::Result: Context<R, anyhow::Error>,
    R: Display,
{
    let mut solution = load::<S>(filename)?;
    info!(
        "{}{} {}: {:?}",
        Paint::mask("🎄 "),
        Paint::bold(&Paint::green(filename)),
        Paint::bold(&Paint::yellow("solution")),
        solution
    );
    solution.analyse(is_full);
    info!(
        "{}part1 answer is {}",
        Paint::mask("🎅 "),
        Paint::bold(&Paint::red(
            &solution.answer_part1(is_full).context("part1 failed")?
        ))
    );
    info!(
        "{}part2 answer is {}",
        Paint::mask("🎅 "),
        Paint::bold(&Paint::red(
            &solution.answer_part2(is_full).context("part2 failed")?
        ))
    );

    Ok(())
}
