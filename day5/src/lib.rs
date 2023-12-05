use std::{io::{BufRead, BufReader}, collections::HashMap};

use tracing::debug;

pub type ResultType = u64;

#[derive(Debug)]
struct RangeMap {
    source_start: ResultType,
    dest_start: ResultType,
    range_length: ResultType,
}
impl RangeMap {
    fn includes(&self, seed: ResultType) -> bool {
        seed >= self.source_start && seed < self.source_start + self.range_length
    }
    fn convert(&self, seed: ResultType) -> ResultType {
        let delta = self.dest_start as i64 - self.source_start as i64;
        (seed as i64 + delta).try_into().unwrap()
    }
}
#[derive(Debug, Default)]
pub struct Solution {
    seeds: Vec<ResultType>,
    maps: HashMap<String, Vec<RangeMap>>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let result = self.seeds.iter().map(|seed| {
            self.get_location(*seed)
        }).min().unwrap();
        // Implement for problem
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut min_location = None;
        for (start, len) in self.seeds.chunks(2).map(|v| (v[0], v[1])) {
            for delta in 0..len {
                let seed = start + delta;
                let location = self.get_location(seed);
                match min_location {
                    None => min_location = Some(location),
                    Some(v) if v > location => min_location = Some(location),
                    _ => {}
                }
            }
        }
        // Implement for problem
        Ok(min_location.unwrap())
    }
}

impl Solution {
    fn set_seeds(&mut self, seeds: Vec<ResultType>) {
        self.seeds = seeds;
    }
    fn add_map(&mut self, name: &str, source_start: ResultType, dest_start: ResultType, range_length: ResultType) {
        let map = self.maps.entry(name.to_owned()).or_insert_with(Vec::new);
        map.push(RangeMap {source_start, dest_start, range_length});
    }
    fn map(&self, name: &str, source: ResultType) -> ResultType {
        let maps = self.maps.get(name).unwrap();
        for map in maps {
            if map.includes(source) {
                return map.convert(source);
            }
        }
        return source;
    }
    fn get_location(&self, seed: ResultType) -> ResultType {
        let soil = self.map("seed-to-soil", seed);
        let fertilizer = self.map("soil-to-fertilizer", soil);
        let water = self.map("fertilizer-to-water", fertilizer);
        let light = self.map("water-to-light", water);
        let temperature = self.map("light-to-temperature", light);
        let humidity = self.map("temperature-to-humidity", temperature);
        let location = self.map("humidity-to-location", humidity);
        debug!(seed, soil, fertilizer, water, light, temperature, humidity, location, "seed");
        location
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let r = regex::Regex::new(r"^(?<dest_start>\d+)\s+(?<source_start>\d+)\s+(?<range_len>\d+)$").unwrap();

        let mut solution = Self::default();
        let mut stage = 0;
        let mut has_header = false;
        for line in reader.lines().flatten() {
            if line.trim().is_empty() {
                stage += 1;
                has_header = false;
                continue;
            }
            // Implement for problem
            match stage {
                // seeds
                0 => {
                    let (_, seeds) = line.split_once(": ").unwrap();
                    let seeds = seeds.split(" ").map(|v| v.parse().unwrap()).collect();
                    solution.set_seeds(seeds);
                }
                _ if !has_header => {
                    has_header = true;
                }
                1 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map("seed-to-soil", source_start, dest_start, range_length);
                }
                2 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map("soil-to-fertilizer", source_start, dest_start, range_length);
                }
                3 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map("fertilizer-to-water", source_start, dest_start, range_length);
                }
                4 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map("water-to-light", source_start, dest_start, range_length);
                }
                5 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map("light-to-temperature", source_start, dest_start, range_length);
                }
                6 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map("temperature-to-humidity", source_start, dest_start, range_length);
                }
                7 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map("humidity-to-location", source_start, dest_start, range_length);
                }
                _ => panic!("unknown parse stage {stage}: {line}")
            }
            if stage == 0 {

            }
        }
        Ok(solution)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::io::BufReader;

    use utils::Solution;

    #[test]
    fn read() {
        let input = "replace for problem";
        let r = BufReader::new(input.as_bytes());
        let s = crate::Solution::try_from(r).unwrap();
        assert_eq!(0 as ResultType, s.answer_part1(false).unwrap());
    }
}
