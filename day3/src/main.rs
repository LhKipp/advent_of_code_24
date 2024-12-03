use regex::Regex;
use std::{env, fs, process};

fn multiply_and_add(input: &str) -> i32 {
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))|(don't\(\))|(do\(\))").unwrap();
    let mut enabled = 1;
    return regex
        .find_iter(input)
        .map(|m| {
            let str_match = m.as_str();
            println!("{} - {}", str_match, enabled);
            if str_match.starts_with("don't") {
                enabled = 0;
                0
            } else if str_match.starts_with("do") {
                enabled = 1;
                0
            } else {
                let comma_index = str_match.find(',').unwrap();
                let n1: i32 = str_match[4..comma_index].parse().unwrap();
                let n2: i32 = str_match[(comma_index + 1)..(str_match.len() - 1)]
                    .parse()
                    .unwrap();
                n1 * n2 * enabled
            }
        })
        .reduce(|acc, elem| acc + elem)
        .unwrap();
    // return 0;
}

fn main() {
    let file_path = env::args().nth(1).expect("Usage: <file_path>");

    let contents = fs::read_to_string(&file_path).unwrap_or_else(|e| {
        eprintln!("Error reading file {}: {}", file_path, e);
        process::exit(1);
    });

    println!("{}", multiply_and_add(&contents));
}
