use std::{env, fs, process};

use itertools::Itertools;

fn len_i<T>(v: &[T]) -> i32 {
    i32::try_from(v.len()).unwrap()
}

fn get(lines: &[Vec<char>], row: i32, col: i32) -> Option<char> {
    if 0 <= row && row < len_i(lines) && 0 <= col && col < len_i(&lines[row as usize]) {
        Some(lines[row as usize][col as usize])
    } else {
        None
    }
}

fn count_xmas(lines: &[Vec<char>]) -> usize {
    const XMAS: [Option<char>; 4] = [Some('X'), Some('M'), Some('A'), Some('S')];
    let mut c = 0;

    for row in 0..len_i(lines) {
        for col in 0..len_i(&lines[row as usize]) {
            println!("{} - {}", row, col);
            // left to right
            if (0..4).map(|i| get(lines, row, col + i)).collect_vec() == XMAS {
                c += 1;
            }
            // right to left
            if (0..4).map(|i| get(lines, row, col - i)).collect_vec() == XMAS {
                c += 1;
            }
            // top to bottom
            if (0..4).map(|i| get(lines, row + i, col)).collect_vec() == XMAS {
                c += 1;
            }
            // bottom to top
            if (0..4).map(|i| get(lines, row - i, col)).collect_vec() == XMAS {
                c += 1;
            }
            // ↘ p
            if (0..4).map(|i| get(lines, row + i, col + i)).collect_vec() == XMAS {
                c += 1;
            }
            if (0..4).map(|i| get(lines, row - i, col + i)).collect_vec() == XMAS {
                c += 1;
            }
            if (0..4).map(|i| get(lines, row + i, col - i)).collect_vec() == XMAS {
                c += 1;
            }
            if (0..4).map(|i| get(lines, row - i, col - i)).collect_vec() == XMAS {
                c += 1;
            }
        }
    }

    c
}

fn main() {
    let file_path = env::args().nth(1).expect("Usage: <file_path>");

    let contents: Vec<Vec<char>> = fs::read_to_string(&file_path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading file {}: {}", file_path, e);
            process::exit(1);
        })
        .split('\n')
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    println!("{}", count_xmas(&contents));
}

// 1  2  4  7  11 16 22
// 3  5  8  12 17
// 6  9  13 18
// 10 14 19
// 15 20
// 21
//
// (row, col) | v
// (0,0) | 1
