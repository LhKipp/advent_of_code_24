use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Map = Vec<Vec<i32>>;

fn parse_file_to_list(filepath: &str) -> Result<Map, io::Error> {
    let path = Path::new(filepath);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut result: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line?; // Handle potential IO errors
        let numbers: Vec<i32> = line
            .chars() // Split by whitespace (space or tab)
            .filter_map(|s| s.to_digit(10).map(|v| v as i32)) // Parse each part as an i32, ignore invalid entries
            .collect();
        result.push(numbers);
    }

    Ok(result)
}

fn height_of(map: &Map, row: usize, col: usize) -> Option<i32> {
    map.get(row).and_then(|r| r.get(col)).cloned()
}

fn score_of_trailhead(map: &Map, trailhead_row: usize, trailhead_col: usize) -> i32 {
    let mut next_options = vec![(trailhead_row, trailhead_col)];

    println!("Trailhead {} {}", trailhead_row, trailhead_col);
    let mut found = Vec::<(usize, usize)>::new();
    while let Some((row, col)) = next_options.pop() {
        let cur_height = height_of(map, row, col).unwrap();
        if cur_height == 9 {
            found.push((row, col));
            continue;
        }

        let adjacent_cells = vec![
            (row + 1, col),
            (row, col + 1),
            (row.checked_add_signed(-1).unwrap_or(99999), col),
            (row, col.checked_add_signed(-1).unwrap_or(99999)),
        ];

        for (adj_row, adj_col) in adjacent_cells {
            if height_of(map, adj_row, adj_col)
                .is_some_and(|adj_height| adj_height - cur_height == 1)
            {
                next_options.push((adj_row, adj_col));
            }
        }
    }

    println!(
        "Trailhead {} {} -> {}",
        trailhead_row,
        trailhead_col,
        found.len()
    );
    found.len() as i32
}

fn sum_of_trailhead_scores(map: &Map) -> i32 {
    let mut result = 0_i32;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if map[row][col] == 0 {
                result += score_of_trailhead(map, row, col);
            }
        }
    }
    result
}

fn print_vec_without_separators(data: &Map) {
    data.iter().for_each(|row| {
        row.iter().for_each(|&num| print!("{}", num));
        println!();
    });
}

fn main() {
    let map = parse_file_to_list(&std::env::args().nth(1).unwrap()).unwrap();
    print_vec_without_separators(&map);
    println!("{}", sum_of_trailhead_scores(&map));
}
