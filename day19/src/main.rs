use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead};

use trie_rs::Trie;

#[derive(Debug)]
struct InputData {
    pub towels: Vec<String>,
    pub designs: Vec<String>,
}

// Function to parse the input file
fn parse_input(file_path: &str) -> Result<InputData, io::Error> {
    // Read the file content
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines();

    // Parse the first line (towels)
    let towels_line = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Missing towels line"))??;
    let towels: Vec<String> = towels_line
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Parse the remaining lines (designs)
    let designs: Vec<String> = lines
        .map(|line| line.unwrap_or_else(|_| String::new())) // Handle line reading errors gracefully
        .filter(|design| !design.is_empty())
        .collect();

    Ok(InputData { towels, designs })
}

fn group_by_first_char(towels: Vec<String>) -> Trie<u8> {
    Trie::from_iter(towels.iter())
}

fn main() {
    // Get the file path from command line argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];

    // Parse the input file
    let data = parse_input(file_path).unwrap();
    let towels_by_start_char = group_by_first_char(data.towels);
    let mut cache = HashMap::new();
    let pos_designs: usize = data
        .designs
        .iter()
        .map(|d| {
            println!("checking design {}", d);
            is_possible(&towels_by_start_char, d, &mut cache)
        })
        .sum();
    println!("{}", pos_designs)
}

fn is_possible(towels: &Trie<u8>, design: &str, cache: &mut HashMap<String, usize>) -> usize {
    if let Some(prior_result) = cache.get(design) {
        return *prior_result;
    }

    if design.is_empty() {
        cache.insert(design.to_string(), 1);
        return 1;
    }

    let mut possible_ways = 0_usize;
    let ts: Vec<String> = towels.common_prefix_search(design).collect();
    for t in ts {
        if t.len() > design.len() {
            continue;
        }
        possible_ways += is_possible(towels, &design[t.len()..], cache);
    }
    cache.insert(design.to_string(), possible_ways);
    possible_ways
}
