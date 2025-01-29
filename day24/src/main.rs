use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    And,
    Xor,
    Or,
}

impl Op {
    fn apply(&self, left: bool, right: bool) -> bool {
        match self {
            Op::And => left && right,
            Op::Xor => left != right,
            Op::Or => left || right,
        }
    }
}

#[derive(Debug, Clone)]
struct Gate {
    pub left_input: String,
    pub right_input: String,
    pub op: Op,
    pub output: String,
    pub handled: bool,
}

fn parse_input(filepath: &str) -> (HashMap<String, bool>, Vec<Gate>) {
    let file = File::open(filepath).unwrap();
    let reader = io::BufReader::new(file);

    let mut wire_values = HashMap::new();
    let mut gates = Vec::new();

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let mut parsing_wires = true;

    for line in lines {
        if line.is_empty() {
            parsing_wires = false;
            continue;
        }

        if parsing_wires {
            // Parse wire initial values (e.g., x00: 1)
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let wire = parts[0].trim().to_string();
                let value: bool = parts[1].trim() == "1";
                wire_values.insert(wire, value);
            }
        } else {
            // Parse gate operations (e.g., x00 AND y00 -> z00)
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 5 {
                let input1 = parts[0].to_string();
                let gate = match parts[1] {
                    "AND" => Op::And,
                    "XOR" => Op::Xor,
                    "OR" => Op::Or,
                    _ => panic!("Unknown gate"),
                };
                let input2 = parts[2].to_string();
                let output = parts[4].to_string();

                gates.push(Gate {
                    left_input: input1,
                    right_input: input2,
                    op: gate,
                    output,
                    handled: false,
                });
            }
        }
    }

    (wire_values, gates)
}

#[allow(non_camel_case_types)]
enum Sub {
    x_xor_y(i32),
    x_and_y(i32),
    c(i32),
    c_and_x_xor_y(i32),
    Sum(i32),
}

fn compute_gate(gate: &Gate, wire_values: &HashMap<String, bool>) -> Option<bool> {
    if let Some(computed) = wire_values.get(&gate.output) {
        return Some(*computed);
    }
    let input1_value = wire_values.get(&gate.left_input)?;
    let input2_value = wire_values.get(&gate.right_input)?;

    let result = gate.op.apply(*input1_value, *input2_value);
    return Some(result);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];

    let (mut wire_values, mut gates) = parse_input(filepath);
    // gates.sort_by(|a, b| a.output.cmp(&b.output));

    let mut subs: HashMap<String, Sub> = HashMap::new();
    subs.insert("qkm".into(), Sub::c(0));
    // subs.insert("c0".into(), "qkm".into());
    let mut falsy_assignments: Vec<String> = vec![];

    // find all x_xor_y and x_and_y
    for gate in &mut gates {
        if gate.left_input.starts_with('x') || gate.right_input.starts_with('x') {
            assert!(gate.right_input.starts_with('y') || gate.left_input.starts_with('y'));
            let num_left = gate.left_input[1..].parse::<i32>().unwrap();
            let num_right = gate.right_input[1..].parse::<i32>().unwrap();
            assert!(num_left == num_right);
            if gate.op == Op::Xor {
                subs.insert(gate.output.clone(), Sub::x_xor_y(num_left));
            } else if gate.op == Op::And {
                subs.insert(gate.output.clone(), Sub::x_and_y(num_left));
            } else {
                unreachable!()
            }
            gate.handled = true
        }
    }

    loop {
        for gate in &mut gates {
            if gate.left_input.starts_with('x') || gate.right_input.starts_with('x') {
                // handled above
                continue;
            }
            if gate.output == "z00" {
                // checked manually
                continue;
            }
            let left = subs.get(&gate.left_input);
            let right = subs.get(&gate.right_input);
            if let (Some(left), Some(right)) = (left, right) {
                match (left, right, gate.op) {
                    (Sub::x_xor_y(num_xor), Sub::c(num_c), Op::Xor)
                    | (Sub::c(num_c), Sub::x_xor_y(num_xor), Op::Xor) => {
                        if *num_c == 99 {
                            if let Sub::c(_) = left {
                                println!("Deduced {} to be c{}", gate.left_input, num_xor - 1);
                                subs.insert(gate.left_input.clone(), Sub::c(num_xor - 1));
                            } else {
                                println!("Deduced {} to be c{}", gate.right_input, num_xor - 1);
                                subs.insert(gate.right_input.clone(), Sub::c(num_xor - 1));
                            }
                            subs.insert(gate.output, Sub::Sum(*num_xor));
                        }
                        else if *num_c == num_xor - 1 {
                            // all good
                            subs.insert(gate.output, Sub::Sum(*num_xor));
                        } else {
                            println!("Sum XOR ({}) not having the right inputs. One xor for {} but c is for {}", gate.output ,num_xor, num_c);
                            println!("Continuing with Unknown");
                            subs.insert(gate.output, Sub::Sum(99));
                        }
                    },
                    (Sub::c(num_c), Sub::x_xor_y(num_xor), Op::And) 
                    | (Sub::c(num_c), Sub::x_xor_y(num_xor), Op::And) => {
                        if *num_c == 99 {
                            if let Sub::c(_) = left {
                                println!("Deduced {} to be c{}", gate.left_input, num_xor - 1);
                                subs.insert(gate.left_input.clone(), Sub::c(num_xor - 1));
                            } else {
                                println!("Deduced {} to be c{}", gate.right_input, num_xor - 1);
                                subs.insert(gate.right_input.clone(), Sub::c(num_xor - 1));
                            }
                            subs.insert(gate.output, Sub::c_and_x_xor_y(*num_xor));
                        }
                        else if *num_c == num_xor - 1 {
                            // all good
                            subs.insert(gate.output, Sub::c_and_x_xor_y(*num_xor));
                        } else {
                            println!("c_and_x_xor_y ({}) not having the right inputs. One xor for {} but c is for {}", gate.output ,num_xor, num_c);
                            println!("Continuing with Unknown");
                            subs.insert(gate.output, Sub::Sum(99));
                        }
                    }
                }
            }
        }
        if gates.iter().all(|g| g.handled) {
            break;
        }
    }
}

// for gate in gates.iter().skip(1) {
//     // z00 manually verified
//     if !gate.output.starts_with('z') {
//         break;
//     }
//     println!("checking {}", gate.output);
//     let num = gate.output[1..].to_string();
//     let num_min_1 = num.parse::<i32>().unwrap().sub(1).to_string();
//     // one side should be: x{num} XOR y{num}
//     // one side should be: c{num-1}
//     if gate.op != Op::Xor {
//         falsy_assignments.push(gate.output);
//         continue;
//     }
//     let c = if subs.get(&gate.left_input).cloned() == Some("A".to_string() + &num) {
//         Some(gate.right_input.clone())
//     } else if subs.get(&gate.right_input).cloned() == Some("A".to_string() + &num) {
//         Some(gate.left_input.clone())
//     } else {
//         let left_gate = gates.iter().find(|g| g.output == gate.left_input).unwrap().clone();
//         if left_gate.op ==
//         falsy_assignments.push(gate.output.clone());
//         None
//     };
//     if let Some(c) = c {
//         if subs.get(&c) == Some(&"C".to_string() + &num_min_1) {
//             continue;
//         }
//
//         let gate = gates.iter().find(|g| g.output == c).unwrap().clone();
//         if gate.op != Op::Or {
//             falsy_assignments.push(gate.output);
//             continue;
//         }
//     }
// }
// // Apply the gates and update the wire values
// loop {
//     let mut any_gate_uncomputed = false;
//     for gate in &gates {
//         if let Some(output) = compute_gate(gate, &wire_values) {
//             wire_values.insert(gate.output.clone(), output);
//         } else {
//             any_gate_uncomputed = true;
//         }
//     }
//     if !any_gate_uncomputed {
//         break;
//     }
// }
//
// let result = wire_values
//     .iter()
//     .filter(|e| e.0.starts_with('z'))
//     .sorted_by(|e1, e2| e1.0.cmp(e2.0).reverse())
//     .map(|e| if *e.1 { "1" } else { "0" })
//     .collect::<String>();
// println!("{}", result);
// println!("{}", isize::from_str_radix(&result, 2).unwrap());
// // // Print the final values of the wires
// // for (wire, value) in wire_values {
// //     println!("{}: {}", wire, if value { 1 } else { 0 });
// // }
