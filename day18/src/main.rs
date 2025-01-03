use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    pub x: i32,
    pub y: i32,
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
                x: parts[0],
                y: parts[1],
            });
        }
    }

    pos_list
}

fn grid_after(falling_bytes: &[Pos], steps: usize) -> Vec<Vec<char>> {
    let mut result = vec![vec!['.'; 71]; 71];
    for falling_byte in &falling_bytes[0..steps] {
        result[falling_byte.x as usize][falling_byte.y as usize] = '#';
    }
    result
}

type G = UnGraph<(usize, usize), ()>;
type NIdx = NodeIndex<u32>;

fn is_visitable(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    return grid
        .get(x)
        .and_then(|v| v.get(y))
        .is_some_and(|v| *v == '.');
}

fn shortest_path(grid: &[Vec<char>]) -> i32 {
    let mut graph = G::new_undirected();
    let mut nodeidx: Vec<Vec<Option<NIdx>>> = vec![vec![None; 71]; 71];

    for x in 0..=70 {
        for y in 0..=70 {
            if grid[x][y] != '#' {
                nodeidx[x][y] = Some(graph.add_node((x, y)));
            }
        }
    }
    for x in 0..=70 {
        for y in 0..=70 {
            if grid[x][y] != '#' {
                if is_visitable(grid, x + 1, y) {
                    graph.add_edge(nodeidx[x][y].unwrap(), nodeidx[x + 1][y].unwrap(), ());
                }
                if is_visitable(grid, x.wrapping_sub(1), y) {
                    graph.add_edge(nodeidx[x][y].unwrap(), nodeidx[x - 1][y].unwrap(), ());
                }
                if is_visitable(grid, x, y + 1) {
                    graph.add_edge(nodeidx[x][y].unwrap(), nodeidx[x][y + 1].unwrap(), ());
                }
                if is_visitable(grid, x, y.wrapping_sub(1)) {
                    graph.add_edge(nodeidx[x][y].unwrap(), nodeidx[x][y - 1].unwrap(), ());
                }
            }
        }
    }

    let costs = dijkstra(&graph, nodeidx[0][0].unwrap(), nodeidx[70][70], |_| 1);
    costs[&nodeidx[70][70].unwrap()]
}

fn main() {
    // Get the filename from the first positional argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Parse positions from the file and print them
    let pos_list = parse_positions_from_file(filename);
    let grid = grid_after(&pos_list, 1024);
    let cost = shortest_path(&grid);

    println!("{}", cost);
}
