use itertools::{iproduct, Itertools};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum SchematicType {
    Key,
    Lock,
}
#[derive(Debug)]
struct Schematic {
    pub kind: SchematicType,
    pub heights: Vec<i32>,
    pub schematic: String,
}

fn parse_schematic(schematic: String) -> Schematic {
    // Split the schematic into rows
    let mut rows: Vec<String> = schematic.lines().map(String::from).collect();

    // Initialize a vector to hold the column heights
    let mut heights: Vec<i32> = vec![0; rows[0].len()];

    let kind = if rows[0].chars().next() == Some('#') {
        SchematicType::Lock
    } else {
        rows.reverse();
        SchematicType::Key
    };

    // Determine the height for each column (count of # from top for locks and from bottom for keys)
    for row in rows.iter().skip(1) {
        for (i, ch) in row.chars().enumerate() {
            if ch == '#' {
                heights[i] += 1;
            }
        }
    }

    Schematic {
        kind,
        heights,
        schematic,
    }
}

fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    // Parse the lock and key into height vectors
    let schemas = input
        .split("\n\n")
        .map(String::from)
        .map(parse_schematic)
        .collect_vec();

    let schematics = schemas.into_iter().into_group_map_by(|s| s.kind);
    let mut result = 0;
    for (key, lock) in iproduct!(
        schematics.get(&SchematicType::Key).unwrap(),
        schematics.get(&SchematicType::Lock).unwrap()
    ) {
        if (key.heights.iter().zip(&lock.heights)).all(|(k, l)| k + l < 6) {
            result += 1;
        }
    }

    println!("{result}");
}
