use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

fn find_connected_components(map: &mut Map) -> Vec<Vec<(i32, i32)>> {
    let mut connected_components = vec![];

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            // do nothing. current is not connected
            let r = row as i32;
            let c = col as i32;

            if connected_components
                .iter()
                .flatten()
                .any(|e: &(i32, i32)| *e == (r, c))
            {
                continue;
            }

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
                } else {
                    continue;
                }
                to_search.append(&mut vec![(r_next - 1, c_next), (r_next + 1, c_next), (r_next, c_next - 1), (r_next, c_next + 1)]);
            }
            // println!("connected {:?}", connnected);
            // println!("====================");
            connected_components.push(connnected)
        }
    }

    connected_components
}

// Function to print the 2D grid
// fn print_grid(grid: &Vec<Vec<()>>) {
//     for row in grid {
//         for id in row {
//             print!("{}{}", id.c, id.n); // Print the formatted string "cn"
//         }
//         println!(); // Move to the next line after each row
//     }
// }

fn cost_of(comp: &Vec<(i32, i32)>) -> usize {
    let total_perimeter: usize = comp
        .iter()
        .map(|(r, c)| {
            let neighbouring = [(r - 1, *c), (r + 1, *c), (*r, c - 1), (*r, c + 1)];
            let neighbouring_same_components =
                neighbouring.iter().filter(|p| comp.contains(p)).count();
            4 - neighbouring_same_components
        })
        .sum();

    println!(
        "{} * {} = {}",
        total_perimeter,
        comp.len(),
        total_perimeter * comp.len()
    );
    total_perimeter * comp.len()
}

fn main() {
    let mut map = read_file_to_map(&std::env::args().nth(1).unwrap()).unwrap();
    let components = find_connected_components(&mut map);
    // print_grid(&map);
    println!("{}", components.iter().map(cost_of).sum::<usize>());
}
