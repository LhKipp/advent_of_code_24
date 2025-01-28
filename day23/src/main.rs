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

fn bron_kerbosch_v2(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    g: &HashMap<String, HashSet<String>>,
    cliques: &mut Vec<Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            let mut clique: Vec<String> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = p
        .union(x)
        .max_by_key(|v| g.get(*v).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<String> = p.difference(&neighbors).cloned().collect();

        for v in candidates {
            // New R is R ∪ {v}
            let mut new_r = r.clone();
            new_r.insert(v.clone());

            // New P is P ∩ N(v)
            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p
                .intersection(&neighbors_v)
                .cloned()
                .collect::<HashSet<String>>();

            // New X is X ∩ N(v)
            let mut new_x = x
                .intersection(&neighbors_v)
                .cloned()
                .collect::<HashSet<String>>();

            // Recursive call
            bron_kerbosch_v2(&new_r, &mut new_p, &mut new_x, g, cliques);

            // Move v from P to X
            p.remove(&v);
            x.insert(v);
        }
    }
}

fn main() {
    // Get the filepath from the first argument
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let edges = parse_edges(filepath);

    // Build the graph as an adjacency list
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for edge in edges.iter() {
        graph
            .entry(edge.from.to_string())
            .or_insert_with(HashSet::new)
            .insert(edge.to.to_string());
    }

    // Initialize R, P, X
    let r: HashSet<String> = HashSet::new();
    let mut p: HashSet<String> = graph.keys().cloned().collect();
    let mut x: HashSet<String> = HashSet::new();

    // Collect cliques
    let mut cliques: Vec<Vec<String>> = Vec::new();
    bron_kerbosch_v2(&r, &mut p, &mut x, &graph, &mut cliques);

    // Sort the cliques for consistent output
    let mut sorted_cliques = cliques.clone();
    sorted_cliques.sort();

    // Print each clique
    for clique in sorted_cliques {
        println!("{}", clique.join(", "));
    }
}
