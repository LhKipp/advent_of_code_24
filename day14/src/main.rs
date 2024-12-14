use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct PointVelocity {
    p: (i32, i32), // Point (x, y)
    v: (i32, i32), // Velocity (vx, vy)
}

fn parse_input(file_path: &str) -> io::Result<Vec<PointVelocity>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Regex pattern for matching p and v
    let re = Regex::new(r"p=([+-]?\d+),([+-]?\d+) v=([+-]?\d+),([+-]?\d+)").unwrap();

    let mut result = Vec::new();

    // Process each line in the file
    for line in reader.lines() {
        let line = line?;

        // Check if the line matches the pattern
        if let Some(caps) = re.captures(&line) {
            let p_x: i32 = caps[1].parse().unwrap();
            let p_y: i32 = caps[2].parse().unwrap();
            let v_x: i32 = caps[3].parse().unwrap();
            let v_y: i32 = caps[4].parse().unwrap();

            // Create PointVelocity struct and push to the result vector
            result.push(PointVelocity {
                p: (p_x, p_y),
                v: (v_x, v_y),
            });
        }
    }

    Ok(result)
}

const WIDE: i32 = 101;
const TALL: i32 = 103;
// const WIDE: i32 = 11;
// const TALL: i32 = 7;

fn move_robot(r: &PointVelocity, sec: i32) -> PointVelocity {
    PointVelocity {
        p: (
            (r.p.0 + r.v.0 * sec).rem_euclid(WIDE),
            (r.p.1 + r.v.1 * sec).rem_euclid(TALL),
        ),
        v: r.v,
    }
}

fn get_quadrant(r: &PointVelocity) -> Option<i32> {
    if r.p.0 == (WIDE / 2) || r.p.1 == (TALL / 2) {
        return None;
    }
    Some(match (r.p.0 > (WIDE / 2), r.p.1 > (TALL / 2)) {
        (true, true) => 4,
        (true, false) => 3,
        (false, true) => 2,
        (false, false) => 1,
    })
}

// Function to print the points on a 2D plane
fn print_points_on_plane(points: &Vec<PointVelocity>) {
    let mut grid = vec![vec!['.'; WIDE as usize]; TALL as usize];

    // Step 3: Plot the points on the grid
    for point in points {
        grid[point.p.1 as usize][point.p.0 as usize] = '#'; // Mark the point on the grid with a '#'
    }

    // Step 4: Print the grid
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn main() {
    // Example file path, replace it with the actual path
    let file_path = std::env::args().nth(1).unwrap();

    let robots = parse_input(&file_path).unwrap();

    let mut cur_min = 100000000000_usize;
    for x in 1..10000 {
        let moved = robots.iter().map(|r| move_robot(r, x)).collect_vec();
        let danger = moved
            .iter()
            .filter_map(get_quadrant)
            .fold(HashMap::new(), |mut acc, item| {
                *acc.entry(item).or_insert(0) += 1;
                acc
            })
            .values()
            .product::<i32>();
        if (danger as usize) < cur_min {
            cur_min = danger as usize;
            println!("AFTER {} seconds", x);
            print_points_on_plane(&moved);
        }
    }
}

// :)
// ###############################
// #.............................#
// #.............................#
// #.............................#
// #.............................#
// #..............#..............#
// #.............###.............#
// #............#####............#
// #...........#######...........#
// #..........#########..........#
// #............#####............#
// #...........#######...........#
// #..........#########..........#
// #.........###########.........#
// #........#############........#
// #..........#########..........#
// #.........###########.........#
// #........#############........#
// #.......###############.......#
// #......#################......#
// #........#############........#
// #.......###############.......#
// #......#################......#
// #.....###################.....#
// #....#####################....#
// #.............###.............#
// #.............###.............#
// #.............###.............#
// #.............................#
// #.............................#
// #.............................#
// #.............................#
// ###############################
