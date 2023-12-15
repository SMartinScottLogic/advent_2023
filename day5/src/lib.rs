use std::{
    cmp::min,
    collections::HashMap,
    io::{BufRead, BufReader},
};

use tracing::debug;

pub type ResultType = i64;

#[derive(Clone, Debug)]
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
        let delta = self.dest_start - self.source_start;
        seed + delta
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
        let result = self
            .seeds
            .iter()
            .map(|seed| self.get_location(*seed))
            .min()
            .unwrap();
        Ok(result)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let v = self
            .seeds
            .chunks(2)
            .map(|v| (v[0], v[1]))
            .flat_map(|v| Self::transform(v, self.maps.get("seed-to-soil").unwrap()))
            .flat_map(|v| Self::transform(v, self.maps.get("soil-to-fertilizer").unwrap()))
            .flat_map(|v| Self::transform(v, self.maps.get("fertilizer-to-water").unwrap()))
            .flat_map(|v| Self::transform(v, self.maps.get("water-to-light").unwrap()))
            .flat_map(|v| Self::transform(v, self.maps.get("light-to-temperature").unwrap()))
            .flat_map(|v| Self::transform(v, self.maps.get("temperature-to-humidity").unwrap()))
            .flat_map(|v| Self::transform(v, self.maps.get("humidity-to-location").unwrap()))
            .collect::<Vec<_>>();
        debug!(v = debug(&v), "v");

        let result = v.iter().map(|v| v.0).min().unwrap();

        Ok(result)
    }
}

impl Solution {
    fn set_seeds(&mut self, seeds: Vec<ResultType>) {
        self.seeds = seeds;
    }
    fn add_map(
        &mut self,
        name: &str,
        source_start: ResultType,
        dest_start: ResultType,
        range_length: ResultType,
    ) {
        let map = self.maps.entry(name.to_owned()).or_default();
        map.push(RangeMap {
            source_start,
            dest_start,
            range_length,
        });
    }
    fn map(&self, name: &str, source: ResultType) -> ResultType {
        let maps = self.maps.get(name).unwrap();
        for map in maps {
            if map.includes(source) {
                return map.convert(source);
            }
        }
        source
    }
    fn get_location(&self, seed: ResultType) -> ResultType {
        let soil = self.map("seed-to-soil", seed);
        let fertilizer = self.map("soil-to-fertilizer", soil);
        let water = self.map("fertilizer-to-water", fertilizer);
        let light = self.map("water-to-light", water);
        let temperature = self.map("light-to-temperature", light);
        let humidity = self.map("temperature-to-humidity", temperature);
        let location = self.map("humidity-to-location", humidity);
        debug!(
            seed,
            soil, fertilizer, water, light, temperature, humidity, location, "seed"
        );
        location
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let r =
            regex::Regex::new(r"^(?<dest_start>\d+)\s+(?<source_start>\d+)\s+(?<range_len>\d+)$")
                .unwrap();

        let mut solution = Self::default();
        let mut stage = 0;
        let mut has_header = false;
        for line in reader.lines().map_while(Result::ok) {
            if line.trim().is_empty() {
                stage += 1;
                has_header = false;
                continue;
            }
            match stage {
                // seeds
                0 => {
                    let (_, seeds) = line.split_once(": ").unwrap();
                    let seeds = seeds.split(' ').map(|v| v.parse().unwrap()).collect();
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
                    solution.add_map(
                        "fertilizer-to-water",
                        source_start,
                        dest_start,
                        range_length,
                    );
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
                    solution.add_map(
                        "light-to-temperature",
                        source_start,
                        dest_start,
                        range_length,
                    );
                }
                6 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map(
                        "temperature-to-humidity",
                        source_start,
                        dest_start,
                        range_length,
                    );
                }
                7 => {
                    let c = r.captures(&line).unwrap();
                    let dest_start = c.name("dest_start").unwrap().as_str().parse().unwrap();
                    let source_start = c.name("source_start").unwrap().as_str().parse().unwrap();
                    let range_length = c.name("range_len").unwrap().as_str().parse().unwrap();
                    solution.add_map(
                        "humidity-to-location",
                        source_start,
                        dest_start,
                        range_length,
                    );
                }
                _ => panic!("unknown parse stage {stage}: {line}"),
            }
            if stage == 0 {}
        }
        Ok(solution)
    }
}

impl Solution {
    fn transform(
        (start, len): (ResultType, ResultType),
        map: &[RangeMap],
    ) -> Vec<(ResultType, ResultType)> {
        let mut start = start;
        let end = start + len;
        let mut map = map.to_vec();
        map.sort_by_cached_key(|v| v.source_start);

        let mut result = Vec::new();
        for range in map {
            // Entirely before current position
            if range.source_start + range.range_length < start {
                continue;
            }
            // Entirely after range
            if range.source_start > end {
                continue;
            }
            // Overlapping

            // Untransformed (before mapping)
            if range.source_start > start {
                debug!(start, len = (range.source_start - start), "Untransformed");
                result.push((start, range.source_start - start));
                start = range.source_start;
            }
            // Transformed section
            let section_len = min(range.range_length - start + range.source_start, end - start);
            let section_start = start + range.dest_start - range.source_start;
            result.push((section_start, section_len));
            debug!(
                start,
                new_start = section_start,
                len = section_len,
                "Transformed"
            );
            start += section_len;
        }
        // Unconsumed end
        if start < end {
            result.push((start, end - start));
            debug!(start, len = (end - start), "Unconsumed");
        }
        result
    }
}
#[cfg(test)]
mod test {
    use tracing_test::traced_test;

    use super::*;

    #[test]
    #[traced_test]
    fn transform_a() {
        let input = (100, 50);
        let next = [RangeMap {
            source_start: 0,
            dest_start: 10,
            range_length: 50,
        }];
        let result = Solution::transform(input, next.as_ref());
        assert_eq!(vec![(100, 50)], result);
    }

    #[test]
    #[traced_test]
    fn transform_b() {
        let input = (100, 50);
        let next = [
            RangeMap {
                source_start: 0,
                dest_start: 10,
                range_length: 50,
            },
            RangeMap {
                source_start: 75,
                dest_start: 20,
                range_length: 30,
            },
            RangeMap {
                source_start: 125,
                dest_start: 90,
                range_length: 50,
            },
        ];
        let result = Solution::transform(input, next.as_ref());
        assert_eq!(vec![(45, 5), (105, 20), (90, 25)], result);
    }
}
