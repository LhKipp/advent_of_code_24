use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::atomic::{AtomicI32, Ordering};

use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::graph::{NodeIndex, UnGraph};
use rayon::iter::IntoParallelRefIterator;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: usize,
    pub y: usize,
}

fn parse(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().collect())
        .collect()
}

type G = UnGraph<Pos, i32>;
type NIdx = NodeIndex<u32>;

const VISITABLE: [char; 3] = ['S', 'E', '.'];
const DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // left
    (1, 0),  // right
    (0, -1), // up
    (0, 1),  // down
];

// fn is_visitable(nodeidx: &[Vec<Option<NIdx>>], x: usize, y: usize) -> bool {
//     return nodeidx
//         .get(x)
//         .and_then(|v| v.get(y).cloned())
//         .is_some_and(|v| v.is_some());
// }

fn graph_for(grid: &Vec<Vec<char>>) -> (G, HashMap<Pos, NIdx>) {
    let mut graph = G::new_undirected();
    let mut pos_map: HashMap<Pos, NIdx> = HashMap::new();

    // Iterate through the grid and create nodes for each dot (.)
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if VISITABLE.contains(&grid[y][x]) {
                let pos = Pos { x, y };

                let node_idx = graph.add_node(pos);
                pos_map.insert(pos, node_idx);

                // Add edges to neighboring dots
                for (dx, dy) in &DIRECTIONS {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && ny >= 0 && nx < grid[y].len() as isize && ny < grid.len() as isize
                    {
                        let neighbor_pos = Pos {
                            x: nx as usize,
                            y: ny as usize,
                        };
                        if VISITABLE.contains(&grid[ny as usize][nx as usize]) {
                            if let Some(&neighbor_idx) = pos_map.get(&neighbor_pos) {
                                graph.add_edge(node_idx, neighbor_idx, 1);
                            }
                        }
                    }
                }
            }
        }
    }

    (graph, pos_map)
}

fn start_and_end(grid: &Vec<Vec<char>>) -> (Pos, Pos) {
    let mut start = Pos { x: 0, y: 0 };
    let mut end = Pos { x: 0, y: 0 };
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start = Pos { x, y };
            } else if grid[y][x] == 'E' {
                end = Pos { x, y };
            }
        }
    }
    println!("start {:?}, end {:?}", start, end);
    (start, end)
}

fn shortest_path(
    mut graph: G,
    idx: &HashMap<Pos, NIdx>,
    start_end: (Pos, Pos),
    skip: Option<(Pos, Pos)>,
) -> i32 {
    if let Some(skip) = skip {
        graph.add_edge(idx[&skip.0], idx[&skip.1], 2);
    }

    let path = astar(
        &graph,
        idx[&start_end.0],
        |finish| finish == idx[&start_end.1],
        |e| *e.weight(),
        |_| 1,
    )
    .unwrap();
    path.0
}

fn find_skips(grid: &Vec<Vec<char>>) -> HashSet<(Pos, Pos)> {
    let mut results: HashSet<(Pos, Pos)> = HashSet::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if !VISITABLE.contains(&grid[y][x]) {
                continue;
            }

            let start = Pos { x, y };
            itertools::repeat_n(DIRECTIONS, 2)
                .multi_cartesian_product()
                .for_each(|comb| {
                    let (first, second) = (comb[0], comb[1]);
                    let dy = first.0 + second.0;
                    let dx = first.1 + second.1;
                    if dy == 0 && dx == 0 {
                        return;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0
                        && ny >= 0
                        && nx < grid[y].len() as isize
                        && ny < grid.len() as isize
                        && VISITABLE.contains(&grid[ny as usize][nx as usize])
                    {
                        let neighbour = Pos {
                            x: nx as usize,
                            y: ny as usize,
                        };
                        if results.contains(&(neighbour, start)) {
                            return;
                        }
                        results.insert((start, neighbour));
                    }
                });
        }
    }

    results
}

fn main() {
    // Get the filename from the first positional argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let grid = parse(filename);
    let (graph, nidx) = graph_for(&grid);
    let (start, end) = start_and_end(&grid);
    let skips = find_skips(&grid);

    println!("found {} skips", skips.len());

    let baseline = shortest_path(graph.clone(), &nidx, (start, end), None);

    println!("baseline {}", baseline);

    let i = AtomicI32::new(0);

    let good_skips = skips
        .par_iter()
        .filter(|skip| {
            let cost = shortest_path(graph.clone(), &nidx, (start, end), Some(**skip));
            // println!("with skip {:?} -> {}", skip, cost);
            let cur = i.fetch_add(1, Ordering::SeqCst);
            println!("{} -> skip cost: {}", cur, cost);
            // baseline > cost
            (baseline - cost) >= 100
        })
        .count();

    println!("{:?}", good_skips);
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
