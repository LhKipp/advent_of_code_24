use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

use petgraph::algo::astar;
use petgraph::graph::{NodeIndex, UnGraph};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: usize,
    pub y: usize,
}

fn parse_positions_from_file(filename: &str) -> Vec<Pos> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    let mut pos_list = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<i32> = line
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if parts.len() == 2 {
            pos_list.push(Pos {
                x: parts[0] as usize,
                y: parts[1] as usize,
            });
        }
    }

    pos_list
}

type G = UnGraph<Pos, ()>;
type NIdx = NodeIndex<u32>;

fn is_visitable(nodeidx: &[Vec<Option<NIdx>>], x: usize, y: usize) -> bool {
    return nodeidx
        .get(x)
        .and_then(|v| v.get(y).cloned())
        .is_some_and(|v| v.is_some());
}

fn graph_for(falling_bytes: &[Pos]) -> (G, Vec<Vec<Option<NIdx>>>) {
    let mut graph = G::new_undirected();
    let mut nodeidx: Vec<Vec<Option<NIdx>>> = vec![vec![None; 71]; 71];
    (0..=70).for_each(|x| {
        for y in 0..=70 {
            if !falling_bytes.contains(&Pos { x, y }) {
                // println!("assigning {},{}", x,y);
                nodeidx[x][y] = Some(graph.add_node(Pos { x, y }));
            }
        }
    });
    for x in 0..=70 {
        for y in 0..=70 {
            if !falling_bytes.contains(&Pos { x, y }) {
                if x < 70 && is_visitable(&nodeidx, x + 1, y) {
                    graph.add_edge(nodeidx[x][y].unwrap(), nodeidx[x + 1][y].unwrap(), ());
                }
                // if x > 0 {
                //     graph.add_edge(nodeidx[x][y], nodeidx[x - 1][y], ());
                // }
                if y < 70 && is_visitable(&nodeidx, x, y + 1) {
                    graph.add_edge(nodeidx[x][y].unwrap(), nodeidx[x][y + 1].unwrap(), ());
                }
                // if y > 0 {
                //     graph.add_edge(nodeidx[x][y], nodeidx[x][y - 1], ());
                // }
            }
        }
    }
    (graph, nodeidx)
}

fn shortest_path(falling_bytes: &[Pos]) -> Pos {
    let mut falling_bytes_i = 0;
    let (mut graph, mut nodeidx) = graph_for(&falling_bytes[0..falling_bytes_i]);

    while let Some(path) = astar(
        &graph,
        nodeidx[0][0].unwrap(),
        |finish| finish == nodeidx[70][70].unwrap(),
        |_| 1,
        |_| 0,
    ) {
        let path_positions: Vec<Pos> = path
            .1
            .iter()
            .map(|n| *graph.node_weight(*n).unwrap())
            .collect();
        println!("Found path: {:?}", path_positions);
        for _ in 0..99999 {
            let falling_byte = falling_bytes[falling_bytes_i];
            falling_bytes_i += 1;
            if path_positions.contains(&falling_byte) {
                println!("Falling byte {:?} blocks path", falling_byte);
                (graph, nodeidx) = graph_for(&falling_bytes[0..falling_bytes_i]);
                break;
            }
        }
    }

    falling_bytes[falling_bytes_i - 1]
}

fn main() {
    // Get the filename from the first positional argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Parse positions from the file and print them
    let pos_list = parse_positions_from_file(filename);
    let cost = shortest_path(&pos_list);

    println!("{:?}", cost);
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
