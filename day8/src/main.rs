use std::arch::x86_64::_mm256_and_pd;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn parse_file_to_lines(file_path: &str) -> Vec<Vec<char>> {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec()
}

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct Pos {
    row: i32,
    col: i32,
}
impl Pos {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    // oh god forgive me for not doing all the vector math here
    fn plus_difference(self, other: Pos, i: i32) -> Self {
        Pos::new(
            self.row + i * (self.row - other.row),
            self.col + i * (self.col - other.col),
        )
    }
}

type AntennaPositions = HashMap<char, Vec<Pos>>;

fn find_antennas(map: &Vec<Vec<char>>) -> AntennaPositions {
    let mut result = AntennaPositions::new();
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            let map_c = map[row][col];
            if map_c == '.' {
                continue;
            }
            let pos = Pos::new(row as i32, col as i32);
            match result.entry(map_c) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry.get_mut().push(pos);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(vec![pos]);
                }
            };
        }
    }
    result
}

fn is_valid(pos: Pos, map: &[Vec<char>]) -> bool {
    !(pos.row < 0
        || pos.col < 0
        || pos.row as usize >= map.len()
        || pos.col as usize >= map[0].len())
}

fn find_count_pos_with_antinode(
    antenna_positions: &AntennaPositions,
    map: &Vec<Vec<char>>,
) -> usize {
    let mut result = HashSet::<Pos>::new();
    // find
    for (_, frequency_positions) in antenna_positions {
        frequency_positions
            .iter()
            .combinations(2)
            .for_each(|positions| {
                result.insert(*positions[0]);
                result.insert(*positions[1]);
                (1..)
                    .map_while(|i| {
                        println!("{}", i);
                        let pos = positions[0].plus_difference(*positions[1], i);
                        is_valid(pos, map).then_some(pos)
                    })
                    .for_each(|pos| {
                        result.insert(pos);
                    });
                (-100..-1)
                    .rev()
                    .map_while(|i| {
                        println!("{}", i);
                        let pos = positions[0].plus_difference(*positions[1], i);
                        is_valid(pos, map).then_some(pos)
                    })
                    .for_each(|pos| {
                        result.insert(pos);
                    });
            })
    }

    result.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = parse_file_to_lines(file_path);
    let antenna_positions = find_antennas(&lines);

    println!(
        "{}",
        find_count_pos_with_antinode(&antenna_positions, &lines)
    );
}
