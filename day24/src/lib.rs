use std::io::{BufRead, BufReader};
use tracing::{debug, event_enabled, info, Level};

pub type ResultType = f64;

#[derive(Debug, Default)]
pub struct Solution {
    hailstones: Vec<HailStone>,
}
impl Solution {
    fn add_hailstone(&mut self, hailstone: HailStone) {
        self.hailstones.push(hailstone);
    }
}
#[derive(Debug, Clone)]
struct HailStone {
    px: ResultType,
    py: ResultType,
    pz: ResultType,
    vx: ResultType,
    vy: ResultType,
    vz: ResultType,
}
impl From<String> for HailStone {
    fn from(value: String) -> Self {
        debug!(value);
        let r = regex::Regex::new(r"^(?<px>[-0-9]+),\s(?<py>[-0-9]+),\s+(?<pz>[-0-9]+)\s+@\s+(?<vx>[-0-9]+),\s+(?<vy>[-0-9]+),\s+(?<vz>[-0-9]+)$").unwrap();
        let c = r.captures(&value).unwrap();
        let px = c.name("px").unwrap().as_str().parse().unwrap();
        let py = c.name("py").unwrap().as_str().parse().unwrap();
        let pz = c.name("pz").unwrap().as_str().parse().unwrap();
        let vx = c.name("vx").unwrap().as_str().parse().unwrap();
        let vy = c.name("vy").unwrap().as_str().parse().unwrap();
        let vz = c.name("vz").unwrap().as_str().parse().unwrap();
        Self {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }
}
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().map_while(Result::ok) {
            // Implement for problem
            let hailstone = HailStone::from(line);
            solution.add_hailstone(hailstone);
        }
        Ok(solution)
    }
}
impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, is_full: bool) -> Self::Result {
        let (min, max) = if is_full {
            (200000000000000.0, 400000000000000.0)
        } else {
            (7.0, 27.0)
        };
        let mut tracks = Vec::new();
        for hailstone in &self.hailstones {
            let tx0 = (min - hailstone.px) / hailstone.vx;
            let tx1 = (max - hailstone.px) / hailstone.vx;
            let ty0 = (min - hailstone.py) / hailstone.vy;
            let ty1 = (max - hailstone.py) / hailstone.vy;
            let t = tx0.max(tx1).max(ty0).max(ty1);
            debug!(hailstone = debug(hailstone), tx0, tx1, ty0, ty1, t);
            let endx = hailstone.px + hailstone.vx * t;
            let endy = hailstone.py + hailstone.vy * t;
            tracks.push(((hailstone.px, hailstone.py), (endx, endy), t));
        }
        let mut count = 0.0;
        for (id1, (start1, end1, _)) in tracks.iter().enumerate() {
            for (id2, (start2, end2, _)) in tracks.iter().enumerate() {
                if id1 >= id2 {
                    continue;
                }
                let x1 = start1.0;
                let x2 = end1.0;
                let x3 = start2.0;
                let x4 = end2.0;
                let y1 = start1.1;
                let y2 = end1.1;
                let y3 = start2.1;
                let y4 = end2.1;
                let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
                if denom != 0.0 {
                    let nom_t = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
                    let nom_u = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);
                    let nom_x = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
                    let nom_y = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
                    let int_x = nom_x / denom;
                    let int_y = nom_y / denom;
                    //         count += 1.0;
                    let int_t = nom_t / denom;
                    let int_u = nom_u / denom;
                    if (min..=max).contains(&int_x)
                        && (min..=max).contains(&int_y)
                        && (0.0..=1.0).contains(&int_t)
                        && (0.0..=1.0).contains(&int_u)
                    {
                        debug!(
                            int_x,
                            int_y,
                            int_t,
                            int_u,
                            start1 = debug(start1),
                            end1 = debug(end1),
                            start2 = debug(start2),
                            end2 = debug(end2)
                        );
                        count += 1.0;
                    }
                } else {
                    // Assume don't intersect for now
                }
            }
        }
        // Implement for problem
        Ok(count)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        let (min, max) = if is_full {
            (200000000000000.0, 400000000000000.0)
        } else {
            (7.0, 27.0)
        };
        let mut tracks = Vec::new();
        for hailstone in &self.hailstones {
            let tx0 = (min - hailstone.px) / hailstone.vx;
            let tx1 = (max - hailstone.px) / hailstone.vx;
            let ty0 = (min - hailstone.py) / hailstone.vy;
            let ty1 = (max - hailstone.py) / hailstone.vy;
            let t = tx0.max(tx1).max(ty0).max(ty1);
            debug!(hailstone = debug(hailstone), tx0, tx1, ty0, ty1, t);
            let endx = hailstone.px + hailstone.vx * t;
            let endy = hailstone.py + hailstone.vy * t;
            tracks.push(((hailstone.px, hailstone.py), (endx, endy), t));
        }
        // Implement for problem
        Ok(0.0)
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
