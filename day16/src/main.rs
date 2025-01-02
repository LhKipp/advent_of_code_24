use petgraph::graph::UnGraph;


// Define a function that parses the file into the grid and commands
fn parse_input_file(file_path: &str) -> io::Result<(UnGraph<>)> {
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

fn main() {
    let mut grid = parse_input_file(&std::env::args().nth(1).unwrap())?;
}
