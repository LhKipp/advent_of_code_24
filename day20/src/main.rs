use std::collections::HashMap;
use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

use petgraph::algo::{astar, dijkstra};
use petgraph::graph::{NodeIndex, UnGraph};

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

fn cityblock_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn find_positions_within_distance(x_t: i32, y_t: i32, max_distance: i32) -> Vec<(i32, i32)> {
    let mut positions = Vec::new();

    // Iterate through x values around the target
    for x in (x_t - max_distance)..=(x_t + max_distance) {
        // Calculate the maximum possible distance for y given the x value
        let max_y_distance = max_distance - (x_t - x).abs();

        // Iterate through y values that are within the allowable cityblock distance
        for y in (y_t - max_y_distance)..=(y_t + max_y_distance) {
            if cityblock_distance(x_t, y_t, x, y) <= max_distance {
                positions.push((x, y));
            }
        }
    }

    positions
}

fn shortcuts_from(
    n: NodeIndex<u32>,
    grid: &Vec<Vec<char>>,
    graph: &G,
    nidx: &HashMap<Pos, NodeIndex>,
    cost_to_nodes: &HashMap<NodeIndex, i32>,
) -> i32 {
    let cur_pos = graph.node_weight(n).unwrap();
    let cur_cost = cost_to_nodes[&n];

    if cur_cost < 100 {
        return 0;
    }

    let mut count_good_skips = 0;

    // todo set to 20
    for (x, y) in find_positions_within_distance(cur_pos.x as i32, cur_pos.y as i32, 20) {
        if x >= 0 && y >= 0 && y < grid.len() as i32 && x < grid[y as usize].len() as i32 {
            let skip_pos = Pos {
                x: x as usize,
                y: y as usize,
            };
            let dist = cityblock_distance(
                cur_pos.x as i32,
                cur_pos.y as i32,
                skip_pos.x as i32,
                skip_pos.y as i32,
            );
            if VISITABLE.contains(&grid[skip_pos.y][skip_pos.x]) {
                let cost_to_skip_node = cost_to_nodes[&nidx[&skip_pos]];
                println!("{cur_cost} - {cost_to_skip_node} >= 100",);
                // - dist as it costs to get to the skip pos
                if cur_cost - cost_to_skip_node - dist >= 100 {
                    println!("YES");
                    count_good_skips += 1;
                }
            }
        }
    }

    count_good_skips
}

fn main() {
    // Get the filename from the first positional argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let grid = parse(filename);
    let (graph, nidx) = graph_for(&grid);
    let (start, end) = start_and_end(&grid);

    let cost_to_nodes = dijkstra(&graph, nidx[&end], None, |_| 1);
    let (min_cost, shortest_path) = astar(
        &graph,
        nidx[&start],
        |finish| finish == nidx[&end],
        |_| 1,
        |_| 0,
    )
    .unwrap();
    println!("min_cost {}", min_cost);
    // println!("{:?}", cost_to_nodes);
    // println!("{:?}", shortest_path);

    let count_shortcuts: i32 = shortest_path
        .iter()
        .map(|n| shortcuts_from(*n, &grid, &graph, &nidx, &cost_to_nodes))
        .sum();

    println!("{:?}", count_shortcuts);
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
