use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

type Map = Vec<Vec<char>>;

fn read_file_to_map(file_path: &str) -> io::Result<Map> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut map: Map = Vec::new();

    for line in reader.lines() {
        let line = line?; // Get the line, propagating errors if any
        let chars = line.chars().collect::<Vec<_>>();
        map.push(chars);
    }

    Ok(map)
}

type Pos = (usize, usize); // row, col

fn map_at(map: &Map, r: i32, c: i32) -> Option<char> {
    let row = usize::try_from(r).ok()?;
    let col = usize::try_from(c).ok()?;
    map.get(row).and_then(|v| v.get(col)).cloned()
}

fn find_connected_components(map: &mut Map) -> (Vec<(i32, Vec<(i32, i32)>)>, Vec<Vec<i32>>) {
    let mut connected_components = vec![];
    let mut map_with_components = vec![vec![0; map[0].len()]; map.len()];
    let mut component_counter = 0;

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            // do nothing. current is not connected
            let r = row as i32;
            let c = col as i32;

            if connected_components
                .iter()
                .flat_map(|(_, v)| v)
                .any(|e: &(i32, i32)| *e == (r, c))
            {
                continue;
            }

            component_counter += 1;
            let current = map_at(map, r, c).unwrap();
            // println!("checking {},{} -> {}", r, c, current);

            let mut connnected = vec![];
            let mut to_search = vec![(r, c)];
            let mut visited = HashSet::<(i32, i32)>::new();
            while let Some((r_next, c_next)) = to_search.pop() {
                // println!("checking {},{}", r_next, c_next);
                if visited.contains(&(r_next, c_next)) {
                    continue;
                }
                visited.insert((r_next, c_next));
                let elem_next = map_at(map, r_next, c_next);
                if elem_next.is_some_and(|e| e == current) {
                    // println!("{},{} -> {} matches", r_next, c_next, elem_next.unwrap());
                    connnected.push((r_next, c_next));
                    map_with_components[r_next as usize][c_next as usize] = component_counter;
                } else {
                    continue;
                }
                to_search.append(&mut vec![
                    (r_next - 1, c_next),
                    (r_next + 1, c_next),
                    (r_next, c_next - 1),
                    (r_next, c_next + 1),
                ]);
            }
            // println!("connected {:?}", connnected);
            // println!("====================");
            connected_components.push((component_counter, connnected))
        }
    }

    (connected_components, map_with_components)
}

// Function to print the 2D grid
fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for id in row {
            print!("{}", id); // Print the formatted string "cn"
        }
        println!(); // Move to the next line after each row
    }
}

fn has_id_at(id: i32, map: &Vec<Vec<i32>>, r: i32, c: i32) -> Option<bool> {
    let row = usize::try_from(r).ok()?;
    let col = usize::try_from(c).ok()?;
    map.get(row).and_then(|v| v.get(col)).map(|v| *v == id)
}

fn cost_of((comp_id, comp): &(i32, Vec<(i32, i32)>), map: &Vec<Vec<i32>>) -> usize {
    println!("cost of {comp_id}");
    let count_corners: i32 = comp
        .iter()
        .map(|(row, col)| {
            let top = has_id_at(*comp_id, map, row - 1, *col).unwrap_or(false);
            let bot = has_id_at(*comp_id, map, row + 1, *col).unwrap_or(false);
            let left = has_id_at(*comp_id, map, *row, *col - 1).unwrap_or(false);
            let right = has_id_at(*comp_id, map, *row, *col + 1).unwrap_or(false);

            let mut count_corners = 0;

            if top && left && !has_id_at(*comp_id, map, row - 1, col - 1).is_some_and(|v| v) {
                count_corners += 1;
            }
            if top && right && !has_id_at(*comp_id, map, row - 1, col + 1).is_some_and(|v| v) {
                count_corners += 1;
            }
            if bot && left && !has_id_at(*comp_id, map, row + 1, col - 1).is_some_and(|v| v) {
                count_corners += 1;
            }
            if bot && right && !has_id_at(*comp_id, map, row + 1, col + 1).is_some_and(|v| v) {
                count_corners += 1;
            }
            // if (bot && left && !top && !right)
            // || (bot && right && !top && !left)
            // || ()
            // {
            //     count_corners += 1;
            // }

            let connected_com_count = [top, bot, left, right].iter().filter(|v| **v).count();
            if connected_com_count == 2 && (!(top && bot) && !(left && right)) {
                count_corners += 1;
            }
            if connected_com_count == 1 {
                count_corners += 2;
            } else if connected_com_count == 0 {
                count_corners += 4;
            }

            println!("got {count_corners} corners at {row} {col}");
            count_corners
        })
        .sum();

    println!(
        "{} * {} = {}",
        count_corners,
        comp.len(),
        count_corners as usize * comp.len()
    );
    count_corners as usize * comp.len()
}

fn main() {
    let mut map = read_file_to_map(&std::env::args().nth(1).unwrap()).unwrap();
    let (components, map_with_components) = find_connected_components(&mut map);
    print_grid(&map_with_components);
    println!(
        "{}",
        components
            .iter()
            .map(|c| cost_of(c, &map_with_components))
            .sum::<usize>()
    );
}
//  A
//  AA
//   x
//  AAx
//   A
//
//
//  A  AA  A
//         A
//
