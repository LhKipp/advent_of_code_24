use std::{env, fs, process};

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
    let xmases: Vec<[Option<char>; 5]> = vec![
        [Some('M'), Some('S'), Some('A'), Some('M'), Some('S')],
        [Some('S'), Some('M'), Some('A'), Some('S'), Some('M')],
        [Some('S'), Some('S'), Some('A'), Some('M'), Some('M')],
        [Some('M'), Some('M'), Some('A'), Some('S'), Some('S')],
    ];
    let mut c = 0;

    for row in 0..len_i(lines) {
        for col in 0..len_i(&lines[row as usize]) {
            if xmases.contains(&[
                get(lines, row - 1, col - 1),
                get(lines, row - 1, col + 1),
                get(lines, row, col),
                get(lines, row + 1, col - 1),
                get(lines, row + 1, col + 1),
            ]) {
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
