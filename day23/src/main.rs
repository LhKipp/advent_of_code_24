use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Edge {
    pub from: String,
    pub to: String,
}
impl Edge {
    fn reverse(&self) -> Edge {
        Self {
            to: self.from.clone(),
            from: self.to.clone(),
        }
    }
}

fn parse_edges(filepath: &str) -> Vec<Edge> {
    let file = File::open(filepath).unwrap();
    let reader = io::BufReader::new(file);

    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split('-').collect();

        if parts.len() == 2 {
            let edge = Edge {
                from: parts[0].to_string(),
                to: parts[1].to_string(),
            };
            edges.push(edge.reverse());
            edges.push(edge);
        }
    }

    edges
}

type Parents = HashMap<String, String>;
fn find(node: &str, parents: &Parents) -> String {
    if parents[node] != node {
        return find(&parents[node], parents);
    }
    node.to_string()
}

fn union_find(edges: &[Edge]) -> Parents {
    let mut parents = Parents::new();

    for edge in edges {
        parents.insert(edge.to.clone(), edge.to.clone());
        parents.insert(edge.from.clone(), edge.from.clone());
    }

    for edge in edges {
        parents.insert(find(&edge.to, &parents), find(&edge.from, &parents));
    }

    parents
}

fn main() {
    // Get the filepath from the first argument
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let edges = parse_edges(filepath)
        .into_iter()
        .into_group_map_by(|e| e.from.clone());
    let edges_set: HashSet<Edge> = edges.iter().flat_map(|e| e.1.clone()).collect();

    let mut result = vec![];
    for (from, edges) in edges {
        if from.starts_with('t') {
            for comb in edges.iter().combinations(2) {
                if edges_set.contains(&Edge {
                    from: comb[0].to.clone(),
                    to: comb[1].to.clone(),
                }) {
                    let mut cc = vec![from.clone(), comb[0].to.clone(), comb[1].to.clone()];
                    cc.sort();
                    result.push(cc);
                }
            }
        }
    }
    result = result.iter().unique().cloned().collect_vec();
    println!("{:?}", result);
    println!("{}", result.len());
}
