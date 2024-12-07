use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Debug)]
struct Line {
    test_value: i64,
    numbers: Vec<i64>,
}

fn parse_file_to_lines(file_path: &str) -> Vec<Line> {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.replace(':', "").parse().unwrap())
                .collect();

            Line {
                test_value: parts[0],
                numbers: parts[1..].to_vec(),
            }
        })
        .collect()
}

fn check_is_valid(line: &Line) -> bool {
    if line.numbers.len() == 1 {
        return line.test_value == line.numbers[0];
    }

    (0..(line.numbers.len() - 1))
        .map(|_| vec!['*', '+', '|'])
        .multi_cartesian_product()
        .any(|ops| {
            let calculated = line.numbers[1..].iter().zip(ops).fold(
                line.numbers[0],
                |acc, (number, op)| match op {
                    '*' => acc * number,
                    '+' => acc + number,
                    '|' => (acc.to_string() + &number.to_string())
                        .parse::<i64>()
                        .unwrap(),
                    _ => panic!(),
                },
            );
            calculated == line.test_value
        })
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let lines = parse_file_to_lines(file_path);

    let total: i64 = lines
        .iter()
        .filter(|l| check_is_valid(l))
        .fold(0_i64, |acc, l| acc + l.test_value);
    println!("total {}", total);

    Ok(())
}
