use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use tracing::{debug, info};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    games: HashMap<usize, Vec<Set>>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let red = 12;
        let green = 13;
        let blue = 14;
        let is_permitted = |game: &Vec<Set>| {
            game.iter().all(|set| {
                set.colors.iter().all(|(count, color)| {
                    let permitted = match color {
                        Color::Red => count <= &red,
                        Color::Green => count <= &green,
                        Color::Blue => count <= &blue,
                    };
                    debug!(color = debug(color), count, red, green, blue, "permitted?");
                    permitted
                })
            })
        };

        let r = self
            .games
            .iter()
            .filter(|(_, game)| is_permitted(game))
            .map(|(id, game)| {
                info!(id, game = debug(game), "permitted");
                (id, game)
            })
            .map(|(id, _)| *id as ResultType)
            .sum();
        Ok(r)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let r = self
            .games
            .values()
            .map(|game| {
                let mut min_red = 0;
                let mut min_green = 0;
                let mut min_blue = 0;
                for g in game {
                    for (count, color) in &g.colors {
                        match color {
                            Color::Red if *count > min_red => min_red = *count,
                            Color::Green if *count > min_green => min_green = *count,
                            Color::Blue if *count > min_blue => min_blue = *count,
                            _ => {}
                        };
                    }
                }
                (min_red, min_green, min_blue)
            })
            .map(|min| min.0 * min.1 * min.2)
            .sum();
        Ok(r)
    }
}

impl Solution {
    fn add_game(&mut self, id: usize, sets: Vec<Set>) {
        self.games.insert(id, sets);
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (id, line) in reader.lines().flatten().enumerate() {
            if let Some((_game, sets)) = line.split_once(':') {
                let sets = sets.split(';').map(Set::from).collect();
                solution.add_game(id + 1, sets);
            }
        }
        Ok(solution)
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}
impl From<&str> for Color {
    fn from(color: &str) -> Self {
        match color {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => panic!("illegal color {color}"),
        }
    }
}
#[derive(Debug)]
struct Set {
    colors: Vec<(u64, Color)>,
}
impl From<&str> for Set {
    fn from(value: &str) -> Self {
        debug!(value, "parse 1");
        let r = regex::Regex::new(r"^(?<count>\d+)\s+(?<color>\w+)").unwrap();
        let colors = value
            .split(',')
            .map(|s| {
                debug!(s, "parse 2");
                let c = r.captures(s.trim()).unwrap();
                let count = c.name("count").unwrap().as_str().parse().unwrap();
                let color = c.name("color").unwrap().as_str().into();
                (count, color)
            })
            .collect();
        debug!(value, colors = debug(&colors), "parse");
        Self { colors }
    }
}
