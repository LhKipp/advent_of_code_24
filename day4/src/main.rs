use std::{env, fs, process};

use itertools::Itertools;

enum IterMode {
    LeftToRight,         // →
    TopToBottom,         // ↓
    LeftToRightDiagonal, // ↘
    RightToLeftDiagonal, // ↙
}

struct TextIter<'a> {
    col: i32,
    row: i32,
    diagonal_last_start: i32,
    r_to_l_diagonal_go_down: bool,
    ended: bool,
    send_break: bool,
    lines: &'a Vec<Vec<char>>,
    mode: IterMode,
}

fn len_i<T>(v: &[T]) -> i32 {
    i32::try_from(v.len()).unwrap()
}

fn i_to_u(v: i32) -> usize {
    usize::try_from(v).unwrap()
}

impl<'a> TextIter<'a> {
    pub fn rowu(&self) -> usize {
        i_to_u(self.row)
    }
    pub fn colu(&self) -> usize {
        i_to_u(self.col)
    }
    pub fn new(mode: IterMode, lines: &'a Vec<Vec<char>>) -> Self {
        let (row, col, diagonal_last_start) = match mode {
            IterMode::LeftToRight => (0, 0, 0),
            IterMode::TopToBottom => (0, 0, 0),
            IterMode::LeftToRightDiagonal => (len_i(lines) - 1, 0, len_i(lines) - 1),
            IterMode::RightToLeftDiagonal => (0, 0, 0),
        };
        TextIter {
            send_break: false,
            diagonal_last_start,
            r_to_l_diagonal_go_down: false,
            col,
            row,
            ended: false,
            lines,
            mode,
        }
    }
}

impl<'a> Iterator for TextIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.send_break {
            self.send_break = false;
            return Some('\n');
        }
        if self.ended {
            return None;
        }

        let ret_val = self.lines[self.rowu()][self.colu()];

        // update row & col
        match self.mode {
            IterMode::LeftToRight => {
                self.col += 1;
                if self.colu() >= self.lines[self.rowu()].len() {
                    self.col = 0;
                    self.row += 1;
                    self.send_break = true;
                }
                if self.rowu() >= self.lines.len() {
                    self.ended = true;
                }
            }
            IterMode::TopToBottom => {
                self.row += 1;
                if self.rowu() >= self.lines.len() {
                    println!("top to bottom: col {}", self.col);
                    self.col += 1;
                    self.row = 0;
                    self.send_break = true;
                    println!("top to bottom: col {}", self.col);
                }
                if self.colu() >= self.lines[0].len() {
                    self.ended = true;
                }
            }
            IterMode::LeftToRightDiagonal => {
                self.col += 1;
                self.row += 1;
                if self.rowu() >= self.lines.len() || self.colu() >= self.lines[self.rowu()].len() {
                    self.diagonal_last_start -= 1;
                    if self.diagonal_last_start < 0 {
                        self.ended = true;
                    }
                    self.row = self.diagonal_last_start;
                    self.col = 0;
                }
            }
            IterMode::RightToLeftDiagonal => {
                self.col -= 1;
                self.row += 1;
                if self.col < 0 || self.rowu() >= self.lines.len() {
                    self.diagonal_last_start += 1;
                    if self.diagonal_last_start >= len_i(&self.lines[0])
                        && !self.r_to_l_diagonal_go_down
                    {
                        self.r_to_l_diagonal_go_down = true;
                        self.diagonal_last_start = 1;
                        self.row = self.diagonal_last_start;
                        self.col = len_i(&self.lines[self.rowu()]) - 1;
                    } else if self.diagonal_last_start >= len_i(self.lines)
                        && self.r_to_l_diagonal_go_down
                    {
                        self.ended = true;
                    } else if !self.r_to_l_diagonal_go_down {
                        self.col = self.diagonal_last_start;
                        self.row = 0;
                    } else if self.r_to_l_diagonal_go_down {
                        self.row = self.diagonal_last_start;
                        self.col = len_i(&self.lines[self.rowu()]) - 1;
                    }
                }
            }
        }

        Some(ret_val)
    }
}

fn count_xmas_in_array(chars: &[char]) -> usize {
    chars
        .windows(4)
        .filter(|c| *c == ['X', 'M', 'A', 'S'])
        .count()
}

fn count_xmas(lines: &Vec<Vec<char>>) -> usize {
    let left_to_right = TextIter::new(IterMode::LeftToRight, lines).collect_vec();
    let top_to_bottom = TextIter::new(IterMode::TopToBottom, lines).collect_vec();
    let l_to_r_diagonal = TextIter::new(IterMode::LeftToRightDiagonal, lines).collect_vec();
    let r_to_l_diagonal = TextIter::new(IterMode::RightToLeftDiagonal, lines).collect_vec();

    println!(
        "{:?}\n{:?}\n{:?}\n{:?}",
        left_to_right.clone().into_iter().join(""),
        top_to_bottom.clone().into_iter().join(""),
        l_to_r_diagonal.clone().into_iter().join(""),
        r_to_l_diagonal.clone().into_iter().join("")
    );

    count_xmas_in_array(&left_to_right)
        + count_xmas_in_array(&left_to_right.iter().rev().cloned().collect_vec())
        + count_xmas_in_array(&top_to_bottom)
        + count_xmas_in_array(&top_to_bottom.iter().rev().cloned().collect_vec())
        + count_xmas_in_array(&l_to_r_diagonal)
        + count_xmas_in_array(&l_to_r_diagonal.iter().rev().cloned().collect_vec())
        + count_xmas_in_array(&r_to_l_diagonal)
        + count_xmas_in_array(&r_to_l_diagonal.iter().rev().cloned().collect_vec())
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
