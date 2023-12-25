use std::{io::{BufRead, BufReader}, collections::{HashMap, HashSet}};
use rustworkx_core::{petgraph::graph::{UnGraph, NodeIndex}, connectivity::stoer_wagner_min_cut};
use tracing::debug;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    components: HashSet<String>,
    connections: HashMap<String, Vec<String>>,
}
impl Solution {
    fn add_connection(&mut self, a: &str, b: &str) {
        self.components.insert(a.to_string());
        self.components.insert(b.to_string());
        if a < b {
            self.connections.entry(a.to_string()).or_default().push(b.to_string());
        } else {
            self.connections.entry(b.to_string()).or_default().push(a.to_string());
        }
    }
}

#[allow(unused_variables, unused_mut)]
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines().map_while(Result::ok) {
            // Implement for problem
            let (a, r) = line.split_once(':').unwrap();
            let a = a.trim();
            for b in r.split(' ').filter(|b| !b.is_empty()) {
                solution.add_connection(a, b);
            }
        }
        Ok(solution)
    }
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut graph: UnGraph<(), ()> = UnGraph::new_undirected();
        let mut nodes = HashMap::new();
        for node in &self.components {
            let gn = graph.add_node(());
            nodes.insert((*node).clone(), gn);
        }
        let mut edges = Vec::new();
        for (a, others) in &self.connections {
            for b in others {
                edges.push((*nodes.get(a).unwrap(), *nodes.get(b).unwrap()));
            }
        }
        graph.extend_with_edges(edges);
        let min_cut_res: rustworkx_core::Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
        let (min_cut, partition) = min_cut_res.unwrap().unwrap();

        let lhs = HashSet::<NodeIndex>::from_iter(partition);
        let r = lhs.len() * (self.components.len() - lhs.len());
        debug!(min_cut, r, "done?");

        Ok(r as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        // Implement for problem
        Ok(0)
    }
}
