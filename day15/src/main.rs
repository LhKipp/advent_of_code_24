use std::fs::File;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;
type Commands = Vec<char>;

// Define a function that parses the file into the grid and commands
fn parse_input_file(file_path: &str) -> io::Result<(Grid, Commands)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut commands: Vec<char> = Vec::new();
    let mut is_command_part = false;

    // Read the lines from the file
    for line in reader.lines() {
        let line = line?; // Unwrap the line content

        if line.is_empty() {
            is_command_part = true;
            continue; // Skip empty lines
        }

        if is_command_part {
            commands.extend(line.chars()); // Add command characters to the list
        } else {
            grid.push(line.chars().collect());
        }
    }

    Ok((grid, commands)) // Return the grid and commands as a tuple
}

fn print_grid(grid: &Grid) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn index_of_robot(grid: &Grid) -> (usize, usize) {
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == '@' {
                return (r, c);
            }
        }
    }
    unreachable!()
}

fn sum_of_box_coordinates(grid: &Grid) -> usize {
    let mut total = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'O' {
                total += 100 * r + c;
            }
        }
    }
    return total;
}

fn pos_plus_command((r, c): (usize, usize), command: char) -> (usize, usize) {
    if command == '^' {
        return (r - 1, c);
    } else if command == '>' {
        return (r, c + 1);
    } else if command == '<' {
        return (r, c - 1);
    } else if command == 'v' {
        return (r + 1, c);
    }
    unreachable!()
}

fn move_robot(grid: &mut Grid, commands: &Commands) {
    let (mut r_row, mut r_col) = index_of_robot(grid);

    for command in commands {
        print_grid(grid);
        let (next_row, next_col) = pos_plus_command((r_row, r_col), *command);
        let elem_on_next = grid[next_row][next_col];
        if elem_on_next == '#' {
            continue;
        } else if elem_on_next == '.' {
            grid[r_row][r_col] = '.';
            grid[next_row][next_col] = '@';
            r_row = next_row;
            r_col = next_col;
        } else if elem_on_next == 'O' {
            let (mut tmp_row, mut tmp_col) = pos_plus_command((next_row, next_col), *command);
            let mut tmp_pos = vec![(tmp_row, tmp_col)];
            loop {
                let tmp_elem = grid[tmp_row][tmp_col];
                if tmp_elem == '#' {
                    break;
                } else if tmp_elem == '.' {
                    for (t_row, t_col) in tmp_pos {
                        grid[t_row][t_col] = 'O';
                    }
                    grid[r_row][r_col] = '.';
                    grid[next_row][next_col] = '@';
                    r_row = next_row;
                    r_col = next_col;
                    break;
                }
                (tmp_row, tmp_col) = pos_plus_command((tmp_row, tmp_col), *command);
                tmp_pos.push((tmp_row, tmp_col));
            }
        }
    }
}

fn main() -> io::Result<()> {
    let (mut grid, commands) = parse_input_file(&std::env::args().nth(1).unwrap())?;
    move_robot(&mut grid, &commands);
    print_grid(&grid);
    println!("Total GPS: {}", sum_of_box_coordinates(&grid));
    Ok(())
}
