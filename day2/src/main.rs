use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn parse_file(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line?; // Handle any errors reading the line
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok()) // Parse integers, ignore invalid ones
            .collect();

        result.push(numbers);
    }

    Ok(result)
}

fn is_report_safe(report: &[i32]) -> bool {
    println!("{:?}", report);
    let is_sorted = {
        if report.len() <= 1 {
            true
        } else if report[0] < report[1] {
            report.iter().tuple_windows().all(|(a, b)| a < b)
        } else {
            report.iter().tuple_windows().all(|(a, b)| a > b)
        }
    };
    if !is_sorted {
        return false;
    }

    return report.iter().tuple_windows().all(|(a, b)| {
        let diff = a.abs_diff(*b);
        (1..=3).contains(&diff)
    });
}

fn count_safe_reports(reports: Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| {
            report
                .iter()
                .combinations(report.len() - 1)
                .any(|r| is_report_safe(&r.into_iter().copied().collect_vec()))
                || is_report_safe(report)
        })
        .count()
}

fn main() {
    let file_path = "data.txt";

    match parse_file(file_path) {
        Ok(parsed_data) => {
            println!("{}", count_safe_reports(parsed_data));
        }
        Err(e) => eprintln!("Error reading the file: {}", e),
    }
}
