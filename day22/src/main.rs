use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn read_numbers_from_file(file_path: &str) -> Vec<usize> {
    let file = File::open(file_path).unwrap();

    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().trim().parse::<usize>().unwrap())
        .collect()
}

const ITERATIONS: usize = 2000;

fn evolve(n: usize) -> usize {
    let n_64 = (n ^ (n * 64)) % 16777216;
    let n_32 = (n_64 ^ (n_64 / 32)) % 16777216;

    (n_32 ^ (n_32 * 2048)) % 16777216
}

fn find_all_subsequence_prices(
    prices: &Vec<i32>,
    sum_prices_for_sequence: &mut HashMap<Vec<i32>, i32>,
) {
    let mut encountered: HashSet<Vec<i32>> = HashSet::new();

    let mut diff_sequence = vec![
        0,
        prices[1] - prices[0],
        prices[2] - prices[1],
        prices[3] - prices[2],
    ];
    let mut prior_price = prices[3];

    for price in &prices[4..] {
        diff_sequence.rotate_left(1);
        diff_sequence[3] = price - prior_price;
        prior_price = *price;

        if encountered.contains(&diff_sequence) {
            continue;
        }
        encountered.insert(diff_sequence.clone());
        *sum_prices_for_sequence
            .entry(diff_sequence.clone())
            .or_insert(0) += price;
    }
}

fn price_of(secret: usize) -> i32 {
    (secret % 10) as i32
}

fn main() {
    // Get the file path from the command line argument
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Usage: {} <file_path>", args[0]);
    let file_path = &args[1];

    // Call the function to read numbers from the file
    let numbers = read_numbers_from_file(file_path);

    let mut sum_prices_for_sequence: HashMap<Vec<i32>, i32> = HashMap::new();

    numbers
        .into_iter()
        .map(|mut v| {
            let mut prices = vec![price_of(v)];
            for _ in 0..ITERATIONS {
                v = evolve(v);
                prices.push(price_of(v));
            }
            prices
        })
        .for_each(|prices| find_all_subsequence_prices(&prices, &mut sum_prices_for_sequence));

    let mut total_price = sum_prices_for_sequence.into_iter().collect::<Vec<_>>();
    total_price.sort_by(|a, b| a.1.cmp(&b.1).reverse());

    println!("{:?}", &total_price[0..4]);
    println!("{:?}", total_price[0].1);
}
