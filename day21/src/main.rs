use std::collections::HashMap;

use itertools::Itertools;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Movement {
    UpLeft,
    DownLeft,
    UpRight,
    DownRight,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
    fn diff(&self, other: &Pos) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    fn movement_to(&self, to_pos: Pos) -> Movement {
        // go to left
        if self.x > to_pos.x {
            if self.y > to_pos.y {
                Movement::UpLeft
            } else {
                Movement::DownLeft
            }
        // its on the right
        } else if self.y > to_pos.y {
            Movement::UpRight
        } else {
            Movement::DownRight
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum PadKind {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    Numeric,
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    Directional,
}

#[derive(Clone, Debug)]
struct Pad {
    pub current: Pos,
    pub moves: String,
    pub kind: PadKind,
}
type Cache = HashMap<(String, usize), String>;

impl Pad {
    fn new(kind: PadKind) -> Self {
        Self {
            current: Self::to_pos_(kind, 'A'),
            moves: String::new(),
            kind,
        }
    }

    fn to_pos(&self, c: char) -> Pos {
        Self::to_pos_(self.kind, c)
    }

    fn to_pos_(kind: PadKind, c: char) -> Pos {
        if kind == PadKind::Directional {
            match c {
                ' ' => Pos::new(0, 0),
                '^' => Pos::new(1, 0),
                'A' => Pos::new(2, 0),
                '<' => Pos::new(0, 1),
                'v' => Pos::new(1, 1),
                '>' => Pos::new(2, 1),
                _ => unreachable!(),
            }
        } else {
            match c {
                '7' => Pos::new(0, 0),
                '8' => Pos::new(1, 0),
                '9' => Pos::new(2, 0),
                '4' => Pos::new(0, 1),
                '5' => Pos::new(1, 1),
                '6' => Pos::new(2, 1),
                '1' => Pos::new(0, 2),
                '2' => Pos::new(1, 2),
                '3' => Pos::new(2, 2),
                ' ' => Pos::new(0, 3),
                '0' => Pos::new(1, 3),
                'A' => Pos::new(2, 3),
                _ => unreachable!(),
            }
        }
    }

    fn move_to(&mut self, to: char) {
        let to_pos = self.to_pos(to);

        let diff = self.current.diff(&to_pos);

        let left_right =
            (if diff.0 < 0 { ">" } else { "<" }).repeat(diff.0.unsigned_abs() as usize);
        let up_down = (if diff.1 < 0 { "v" } else { "^" }).repeat(diff.1.unsigned_abs() as usize);

        let mut combined = (|| {
            if left_right.is_empty() || up_down.is_empty() {
                return left_right + &up_down;
            }
            // going to the left
            if self.kind == PadKind::Numeric && self.current.y == 3 && to_pos.x == 0 {
                return up_down + &left_right;
            }
            // going to the bottom
            if self.kind == PadKind::Numeric && self.current.x == 0 && to_pos.y == 3 {
                return left_right + &up_down;
            }
            // we are on the <
            if self.kind == PadKind::Directional && self.current.x == 0 {
                return left_right + &up_down;
            }
            // we go to the <
            if self.kind == PadKind::Directional && to_pos.x == 0 {
                return up_down + &left_right;
            }
            let movement = self.current.movement_to(to_pos);
            let (preferred, alternative) = match movement {
                Movement::UpLeft => (left_right.clone() + &up_down, up_down + &left_right),
                Movement::DownLeft => (left_right.clone() + &up_down, up_down + &left_right),
                Movement::UpRight => (up_down.clone() + &left_right, left_right + &up_down),
                Movement::DownRight => (up_down.clone() + &left_right, left_right + &up_down),
            };

            if !self.is_valid_move(&preferred) {
                return alternative;
            }

            preferred
        })();

        combined.push('A');

        self.current = to_pos;
        self.moves += &combined;
    }

    fn is_valid_move(&self, move_: &str) -> bool {
        let mut current = self.current;
        let gap_pos = self.to_pos(' ');
        for c in move_.chars() {
            match c {
                '>' => current.x += 1,
                '<' => current.x -= 1,
                '^' => current.y -= 1,
                'v' => current.y += 1,
                _ => {}
            }
            if current == gap_pos {
                return false;
            }
        }
        true
    }

    fn moves_for(mut self, input: &str) -> String {
        for c in input.chars() {
            self.move_to(c);
        }
        self.moves
        // let mut parts = input.split('A').collect::<Vec<_>>();
        // // remove last ""
        // parts.pop();
        // // println!("parts: {:?}", parts);
        //
        // let mut dir_pads = vec![Self::new()];
        //
        // for part in parts {
        //     let mut sub_inputs = if part.len() <= 1 {
        //         vec![part.chars().collect::<Vec<_>>()]
        //     } else {
        //         part.chars().permutations(part.len()).unique().collect()
        //     };
        //     if sub_inputs.contains(&vec!['^', '<']) && sub_inputs.contains(&vec!['<', '^']) {
        //         sub_inputs.retain(|v| v != &['^', '<']);
        //     }
        //     // let sub_inputs = part.chars().permutations(part.len()).unique().collect_vec();
        //     // sub_inputs.retain_mut(|v| v.sli);
        //
        //     println!("sub_inputs: {:?}", sub_inputs);
        //
        //     let mut next_pads = vec![];
        //     for dir_pad in dir_pads {
        //         let moved_pads = sub_inputs
        //             .iter()
        //             .filter_map(|sub_input| {
        //                 let mut dir_pad_temp = dir_pad.clone();
        //                 let added_move = dir_pad_temp.add_move_for(sub_input);
        //                 if !Self::is_valid(&added_move) {
        //                     println!(
        //                         "added_move {:?} to generate {:?} is not valid",
        //                         added_move, sub_input
        //                     );
        //                     return None;
        //                 }
        //                 Some((sub_input, added_move, dir_pad_temp))
        //             })
        //             .collect::<Vec<_>>();
        //         // println!("moved_pads {:?}", moved_pads);
        //         // println!("sub_inputs {:?}", sub_inputs);
        //         let mut min_pads = moved_pads
        //             .into_iter()
        //             .min_set_by(|a, b| Self::dist_of_move(&a.1).cmp(&Self::dist_of_move(&b.1)));
        //         for (_, _, pad) in &mut min_pads {
        //             pad.move_to('A');
        //         }
        //         next_pads.extend(min_pads.into_iter().map(|v| v.2));
        //     }
        //     dir_pads = next_pads;
        //
        //     println!(
        //         "dir_pads: {:?} after part {}",
        //         dir_pads.iter().map(|pad| pad.moves.concat()).collect_vec(),
        //         part
        //     );
        // }
        //
        // dir_pads
        //     .into_iter()
        //     .min_set_by(|a, b| a.moves.concat().len().cmp(&b.moves.concat().len()))
        //     .into_iter()
        //     .map(|pad| pad.moves.concat())
        //     .collect()
    }
}

fn command_to_parts(command: &str) -> Vec<String> {
    command
        .split_inclusive('A')
        .map(|p| p.to_string())
        .collect_vec()
}

// <>Av^A
//
// <>A,0 -> 3
// C D => update <>A with C+D

const ITERATIONS: usize = 25;
fn value_of_cache() -> HashMap<String, usize> {
    let mut move_once_cache: HashMap<String, (Vec<String>, usize)> = HashMap::new();
    let mut value_cache: HashMap<String, usize> = HashMap::new();

    let moves = vec![
        "A", ">A", ">>A", ">>^A", "^>A", ">^A", "^A", "<A", "vA", "v<A", "<vA", "^<A", "<^A",
        "<<A", "v<<A",
    ];
    for move_ in &moves {
        value_cache.insert(move_.to_string(), 0);
    }

    for move_ in &moves {
        let new_move = Pad::new(PadKind::Directional).moves_for(move_);
        let new_move_parts = command_to_parts(&new_move);
        move_once_cache.insert(move_.to_string(), (new_move_parts, new_move.len()));
    }

    for _ in 0..ITERATIONS {
    }

    value_cache
}

fn value_of(
    input: &str,
    after_iterations: usize,
    cache: &mut HashMap<(String, usize), String>,
) -> String {
    if after_iterations == 0 {
        return input.to_string();
    }
    let mut result = String::with_capacity(input.len() * 2);
    for sub_input in command_to_parts(input) {
        if let Some(new_commands) = cache.get(&(sub_input.to_string(), after_iterations)) {
            result += new_commands;
            continue;
        }
        // compute based on the value of the prior iteration
        let prior_sub_input = value_of(&sub_input, after_iterations - 1, cache);
        let new_commands = Pad::new(PadKind::Directional).moves_for(&prior_sub_input);
        cache.insert(
            (sub_input.to_string(), after_iterations),
            new_commands.clone(),
        );
        result += &new_commands;
    }
    return result;
}

fn main() {
    // let inputs = vec!["029A", "980A", "179A", "456A", "379A"]; // example
    // let inputs = vec!["179A"];
    let inputs = vec!["964A", "140A", "413A", "670A", "593A"];

    let inputs = std::env::args().skip(1).collect_vec();

    let mut cache = Cache::new();
    let total: usize = inputs
        .iter()
        .map(|input| {
            println!("handling {input}");
            let commands = Pad::new(PadKind::Numeric).moves_for(input);
            // println!("{}", commands);
            let mut commands = command_to_parts(&commands);
            // println!("-> {:?}", commands);

            let num = input[0..3].parse::<usize>().unwrap();
            let mut subtotal = 0;
            while let Some(sub_input) = commands.pop() {
                // println!(
                //     "{sub_input} - {iterations} - commands len {}",
                //     commands.len()
                // );
                subtotal += value_of(&sub_input, ITERATIONS, &mut cache).len();
            }
            println!("{} - {} * {}", input, subtotal, num);
            subtotal * num
        })
        .sum();

    println!("{total}")
}
