use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Button {
    name: String,
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Prize {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct InputBlock {
    a: Button,
    b: Button,
    p: Prize,
}

fn parse_input(file_path: &str) -> io::Result<Vec<InputBlock>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut blocks = Vec::new();

    // Regex patterns for parsing button and prize
    let button_re = Regex::new(r"Button (\w): X([+-]?\d+), Y([+-]?\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=([+-]?\d+), Y=([+-]?\d+)").unwrap();

    let mut current_block = InputBlock {
        a: Button {
            name: String::new(),
            x: 0,
            y: 0,
        },
        b: Button {
            name: String::new(),
            x: 0,
            y: 0,
        },
        p: Prize { x: 0, y: 0 },
    };

    // Process each line in the file
    for line in reader.lines() {
        let line = line?;

        // Skip empty lines (this will separate blocks)
        if line.trim().is_empty() {
            // If the current block is not empty, save it and reset
            if !current_block.a.name.is_empty()
                && !current_block.b.name.is_empty()
                && current_block.p.x != 0
                && current_block.p.y != 0
            {
                blocks.push(current_block);
            }

            // Reset the current block for the next set of data
            current_block = InputBlock {
                a: Button {
                    name: String::new(),
                    x: 0,
                    y: 0,
                },
                b: Button {
                    name: String::new(),
                    x: 0,
                    y: 0,
                },
                p: Prize { x: 0, y: 0 },
            };

            continue; // Skip processing the empty line
        }

        // Check for Button match
        if let Some(caps) = button_re.captures(&line) {
            let name = caps[1].to_string();
            let x: isize = caps[2].parse().unwrap();
            let y: isize = caps[3].parse().unwrap();

            if name == "A" {
                current_block.a = Button { name, x, y };
            } else if name == "B" {
                current_block.b = Button { name, x, y };
            }
        }
        // Check for Prize match
        else if let Some(caps) = prize_re.captures(&line) {
            let x: isize = caps[1].parse().unwrap();
            let y: isize = caps[2].parse().unwrap();
            current_block.p = Prize { x, y };
        }
    }

    // Add the last block if not empty
    if !current_block.a.name.is_empty()
        && !current_block.b.name.is_empty()
        && current_block.p.x != 0
        && current_block.p.y != 0
    {
        blocks.push(current_block);
    }

    Ok(blocks)
}

// fn get_token_cost_to_win(block: &InputBlock) -> i32 {
//     (0..=100)
//         .cartesian_product(0..=100)
//         .filter(|(a, b)| {
//             let x = block.a.x * a + block.b.x * b;
//             let y = block.a.y * a + block.b.y * b;
//             x == block.p.x && y == block.p.y
//         })
//         .map(|(a, b)| a * 3 + b)
//         .min()
//         .unwrap_or(0)
// }

fn get_token_cost_to_win_with_math(block: &mut InputBlock) -> isize {
    block.p.x += 10000000000000;
    block.p.y += 10000000000000;

    // - ax*by*B + ay*bx*B = px*ay-ax*py
    let b_divisor = -block.a.x * block.b.y + block.a.y * block.b.x;
    let b_divident = block.p.x * block.a.y - block.a.x * block.p.y;
    if b_divident % b_divisor != 0 {
        return 0;
    }
    let b = b_divident / b_divisor;

    // A = (py - by*B)/ay
    if (block.p.y - block.b.y * b) % block.a.y != 0 {
        return 0;
    }
    let a = (block.p.y - block.b.y * b) / block.a.y;
    println!("solution: A={}, B={}", a, b);
    a * 3 + b
}

fn main() {
    // Example file path, replace it with the actual path
    let mut blocks = parse_input(&std::env::args().nth(1).unwrap()).unwrap();
    println!(
        "{}",
        blocks
            .iter_mut()
            .map(get_token_cost_to_win_with_math)
            .sum::<isize>()
    );
}
