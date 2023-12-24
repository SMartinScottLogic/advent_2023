use std::io::{BufRead, BufReader};
use tracing::debug;
use z3::{
    ast::{self, Ast},
    Config, Context, SatResult, Solver,
};

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
        let ctx = Context::new(&Config::default());
        let solver = Solver::new(&ctx);
        let zero = ast::Int::from_i64(&ctx, 0);

        let rock_x = ast::Int::new_const(&ctx, "rock_x");
        let rock_y = ast::Int::new_const(&ctx, "rock_y");
        let rock_z = ast::Int::new_const(&ctx, "rock_z");
        let rock_vx = ast::Int::new_const(&ctx, "rock_vx");
        let rock_vy = ast::Int::new_const(&ctx, "rock_vy");
        let rock_vz = ast::Int::new_const(&ctx, "rock_vz");

        for (i, hailstone) in self.hailstones.iter().enumerate() {
            if i > 4 {
                continue;
            }
            let t_name = format!("t{i}");
            let t = ast::Int::new_const(&ctx, t_name);
            solver.assert(&t.ge(&zero));
            let x = ast::Int::from_i64(&ctx, hailstone.px as i64);
            let vx = ast::Int::from_i64(&ctx, hailstone.vx as i64);
            let px = &x + &vx * &t;
            let rxt = &rock_x + &rock_vx * &t;
            solver.assert(&px._eq(&rxt));
            let y = ast::Int::from_i64(&ctx, hailstone.py as i64);
            let vy = ast::Int::from_i64(&ctx, hailstone.vy as i64);
            let py = &y + &vy * &t;
            let ryt = &rock_y + &rock_vy * &t;
            solver.assert(&py._eq(&ryt));
            let z = ast::Int::from_i64(&ctx, hailstone.pz as i64);
            let vz = ast::Int::from_i64(&ctx, hailstone.vz as i64);
            let pz = &z + &vz * &t;
            let rzt = &rock_z + &rock_vz * &t;
            solver.assert(&pz._eq(&rzt));
        }

        let r = match solver.check() {
            SatResult::Sat => {
                let model = solver.get_model().unwrap();
                debug!(model = debug(&model));
                let x = model.get_const_interp(&rock_x).unwrap().as_i64().unwrap();
                let y = model.get_const_interp(&rock_y).unwrap().as_i64().unwrap();
                let z = model.get_const_interp(&rock_z).unwrap().as_i64().unwrap();
                x + y + z
            }
            _ => todo!(),
        };
        Ok(r as ResultType)
    }
}
