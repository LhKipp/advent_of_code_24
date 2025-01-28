use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Debug)]
struct Edge {
    pub from: String,
    pub to: String,
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

    let edges = parse_edges(filepath);

    let parents = union_find(&edges);

    println!("{:?}", parents);
    let mut groups = vec![];
    for (_, chunk) in &parents.into_iter().chunk_by(|p| p.1.clone()) {
        groups.push(chunk.collect_vec());
    }

    let count = groups
        .iter()
        .filter(|group| group.len() == 3 && group.iter().any(|(n, _)| n.starts_with('t')))
        .inspect(|g| println!("{:?}", g))
        .count();

    println!("{}", count);
}
