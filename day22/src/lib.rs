use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader},
};

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    bricks: Vec<Brick>,
    occupancy: HashMap<(ResultType, ResultType, ResultType), usize>,
}
impl Solution {
    fn add_brick(&mut self, brick: Brick) {
        self.bricks.push(brick);
    }

    fn get_falling(
        falling: &mut HashSet<usize>,
        above: &HashMap<usize, HashSet<usize>>,
        below: &HashMap<usize, HashSet<usize>>,
        brick_id: usize,
    ) {
        if !falling.insert(brick_id) {
            return;
        }
        if let Some(parents) = above.get(&brick_id) {
            for &parent in parents {
                if below[&parent].iter().all(|x| falling.contains(x)) {
                    Self::get_falling(falling, above, below, parent);
                }
            }
        }
    }
    fn gen_above_below(
        &self,
    ) -> (
        std::collections::HashMap<usize, std::collections::HashSet<usize>>,
        std::collections::HashMap<usize, std::collections::HashSet<usize>>,
    ) {
        let mut above = HashMap::<_, HashSet<_>>::new();
        let mut below = HashMap::<_, HashSet<_>>::new();
        for (i, brick) in self.bricks.iter().enumerate() {
            let x1 = brick.start.x;
            let y1 = brick.start.y;
            let z1 = brick.start.z;
            let x2 = brick.end.x;
            let y2 = brick.end.y;
            for (x, y) in (x1..=x2).cartesian_product(y1..=y2) {
                if let Some(&j) = self.occupancy.get(&(x, y, z1 - 1)) {
                    above.entry(j).or_default().insert(i);
                    below.entry(i).or_default().insert(j);
                }
            }
        }
        (above, below)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().map_while(Result::ok) {
            // Implement for problem
            let brick = Brick::from(line.as_str());
            solution.add_brick(brick);
        }
        Ok(solution)
    }
}
#[derive(Debug, Clone)]
struct Point3 {
    x: ResultType,
    y: ResultType,
    z: ResultType,
}
impl From<&str> for Point3 {
    fn from(value: &str) -> Self {
        let r = regex::Regex::new(r"^(?<x>\d+),(?<y>\d+),(?<z>\d+)$").unwrap();
        let c = r.captures(value).unwrap();
        let x = c.name("x").unwrap().as_str().parse().unwrap();
        let y = c.name("y").unwrap().as_str().parse().unwrap();
        let z = c.name("z").unwrap().as_str().parse().unwrap();
        Self { x, y, z }
    }
}
#[derive(Debug, Clone)]
struct Brick {
    start: Point3,
    end: Point3,
}
impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (lhs, rhs) = value.split_once('~').unwrap();
        let start = Point3::from(lhs);
        let end = Point3::from(rhs);
        Self { start, end }
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        for (i, brick) in self.bricks.iter().enumerate() {
            for x in brick.start.x..=brick.end.x {
                for y in brick.start.y..=brick.end.y {
                    for z in brick.start.z..=brick.end.z {
                        self.occupancy.insert((x, y, z), i);
                    }
                }
            }
        }
        let mut done = false;
        while !done {
            done = true;
            for (i, brick) in self.bricks.iter_mut().enumerate() {
                loop {
                    let min_x = brick.start.x;
                    let min_y = brick.start.y;
                    let min_z = brick.start.z;
                    let max_x = brick.end.x;
                    let max_y = brick.end.y;
                    let max_z = brick.end.z;
                    if min_z == 1 {
                        break;
                    }
                    if (min_x..=max_x)
                        .cartesian_product(min_y..=max_y)
                        .any(|(x, y)| self.occupancy.contains_key(&(x, y, min_z - 1)))
                    {
                        break;
                    }
                    // Brick drops
                    *brick = Brick {
                        start: Point3 {
                            x: min_x,
                            y: min_y,
                            z: min_z - 1,
                        },
                        end: Point3 {
                            x: max_x,
                            y: max_y,
                            z: max_z - 1,
                        },
                    };
                    for (x, y) in (min_x..=max_x).cartesian_product(min_y..=max_y) {
                        self.occupancy.remove(&(x, y, max_z));
                        self.occupancy.insert((x, y, min_z - 1), i);
                    }
                    done = false;
                }
            }
        }
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let (above, below) = self.gen_above_below();
        let mut answer = 0 as ResultType;
        for id in 0..self.bricks.len() {
            let mut falling = HashSet::new();
            Self::get_falling(&mut falling, &above, &below, id);
            answer += if falling.len() == 1 { 1 } else { 0 };
        }
        // Implement for problem
        // Too high: 416
        Ok(answer as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let (above, below) = self.gen_above_below();
        let mut answer = 0 as ResultType;
        for id in 0..self.bricks.len() {
            let mut falling = HashSet::new();
            Self::get_falling(&mut falling, &above, &below, id);
            answer += falling.len() as ResultType - 1;
        }
        // 70727?
        Ok(answer as ResultType)
    }
}
