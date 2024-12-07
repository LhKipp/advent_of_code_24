use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::bail;
use itertools::Itertools;

type Map = Vec<Vec<HashSet<char>>>;

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_90_deg(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn short_char(&self) -> char {
        match self {
            Direction::North => 'N',
            Direction::East => 'E',
            Direction::South => 'S',
            Direction::West => 'W',
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    row: i32,
    col: i32,
    direction: Direction,
}

impl Guard {
    fn next_pos(&self) -> (i32, i32) {
        match self.direction {
            Direction::North => (self.row - 1, self.col),
            Direction::East => (self.row, self.col + 1),
            Direction::South => (self.row + 1, self.col),
            Direction::West => (self.row, self.col - 1),
        }
    }
}

fn parse_map(file_path: &str) -> Map {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| {
                    let mut s = HashSet::new();
                    s.insert(c);
                    s
                })
                .collect_vec()
        })
        .collect_vec()
}

fn pos_of_guard(map: &Map) -> anyhow::Result<(i32, i32)> {
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col].contains(&'^') {
                return Ok((row as i32, col as i32));
            }
        }
    }
    bail!("no ^")
}

fn is_valid_pos(row: i32, col: i32, map: &Map) -> bool {
    !(row < 0 || row >= map.len() as i32 || col < 0 || col >= map[row as usize].len() as i32)
}

// fn move_ghost(ghost: &mut Guard, map: &mut Map) -> bool {
//     loop {
//         let markers = &mut map[ghost.row as usize][ghost.col as usize];
//         if markers.len() == 1 && markers.contains(&'.') {
//             markers.clear();
//         }
//         markers.insert(ghost.direction.short_char());
//         markers.insert('X');
//
//         let (new_row, new_col) = ghost.next_pos();
//
//         if !is_valid_pos(new_row, new_col, map) {
//             return false;
//         }
//
//         if map[new_row as usize][new_col as usize].contains(&'#') {
//             ghost.direction = ghost.direction.turn_90_deg();
//             continue;
//         }
//
//         if map[new_row as usize][new_col as usize].contains(&ghost.direction.short_char()) {
//             return true;
//         }
//
//         // update ghost pos
//         ghost.row = new_row;
//         ghost.col = new_col;
//     }
// }

fn is_guard_trapped(guard: &mut Guard, map: &mut Map) -> bool {
    loop {
        if !map[guard.row as usize][guard.col as usize].insert(guard.direction.short_char()) {
            // println!("Found map");
            // print_char_matrix(map);
            return true;
        }

        let (new_row, new_col) = guard.next_pos();
        if !is_valid_pos(new_row, new_col, map) {
            return false;
        }

        if map[new_row as usize][new_col as usize].contains(&'#') {
            guard.direction = guard.direction.turn_90_deg();
            continue;
        }

        // update guard pos
        guard.row = new_row;
        guard.col = new_col;
    }
}

// fn count_visited_pos(map: &Map) -> usize {
//     return map.iter().flatten().filter(|c| **c == '%').count();
// }

// fn print_char_matrix(matrix: &Map) {
//     matrix.iter().for_each(|row| {
//         row.iter().for_each(|c| print!("{:5}", c.iter().join("")));
//         println!(); // Newline after each row
//     });
// }

fn main() {
    let map = parse_map(&std::env::args().nth(1).unwrap());
    let (row, col) = pos_of_guard(&map).unwrap();
    let guard = Guard {
        row,
        col,
        direction: Direction::North,
    };

    let total: i32 = (0..map.len())
        .collect_vec()
        .par_iter()
        .map(|row| {
            let mut trapped_count = 0;
            for col in 0..map[*row].len() {
                if *row == guard.row as usize && col == guard.col as usize {
                    continue;
                }
                let mut tryout = map.clone();
                tryout[*row][col].clear();
                tryout[*row][col].insert('#');
                println!("{},{}", row, col);
                if is_guard_trapped(&mut guard.clone(), &mut tryout) {
                    trapped_count += 1;
                }
            }
            trapped_count
        })
        .sum();
    println!("{}", total);

    // println!("{}", count_visited_pos(&map));
}
