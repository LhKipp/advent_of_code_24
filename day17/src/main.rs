use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct ProgramState {
    a: i64,
    b: i64,
    c: i64,
    i: usize,
    out: Vec<i64>,
    instructions: Vec<i64>,
}

impl ProgramState {
    pub fn literal_op_at(&self, i: usize) -> i64 {
        self.instructions[i]
    }
    pub fn combo_op_at(&self, i: usize) -> i64 {
        match self.instructions[i] {
            v @ 0..=3 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

fn parse_input(file_path: &str) -> io::Result<ProgramState> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut program = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            a = line.split(':').nth(1).unwrap().trim().parse().unwrap();
        } else if i == 1 {
            b = line.split(':').nth(1).unwrap().trim().parse().unwrap();
        } else if i == 2 {
            c = line.split(':').nth(1).unwrap().trim().parse().unwrap();
        } else if i == 4 {
            let full_program: String = line.split(':').nth(1).unwrap().trim().parse().unwrap();
            program = full_program
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
        }
    }

    Ok(ProgramState {
        a,
        b,
        c,
        i: 0,
        out: vec![],
        instructions: program,
    })
}

fn bxl(program: &mut ProgramState) {
    let operand = program.literal_op_at(program.i + 1);
    program.b ^= operand;
    program.i += 2;
}

fn bst(program: &mut ProgramState) {
    let operand = program.combo_op_at(program.i + 1);
    program.b = operand % 8;
    program.i += 2;
}

fn jnz(program: &mut ProgramState) {
    if program.a == 0 {
        program.i += 2;
        return;
    }

    let operand = program.literal_op_at(program.i + 1);
    program.i = operand as usize;
}

fn bxc(program: &mut ProgramState) {
    program.b ^= program.c;
    program.i += 2;
}

fn out(program: &mut ProgramState) {
    let operand = program.combo_op_at(program.i + 1);
    program.out.push(operand % 8);
    program.i += 2;
}

fn division(program: &mut ProgramState) -> i64 {
    let operand = program.combo_op_at(program.i + 1);
    let bottom = 1 << operand;
    let vvv = program.a / bottom;
    program.i += 2;
    // println!("{} = {} / {}<<{} ({})", vvv, program.a, 2, operand, 2 << operand);
    vvv
}

fn execute_program(program: &mut ProgramState) {
    while (program.i + 1) < program.instructions.len() {
        println!("{:?}", program);
        match program.instructions[program.i] {
            0 => {
                program.a = division(program);
            }
            1 => bxl(program),
            2 => bst(program),
            3 => jnz(program),
            4 => bxc(program),
            5 => out(program),
            6 => program.b = division(program),
            7 => program.c = division(program),
            _ => unreachable!("found non opcode"),
        }
    }
}

fn main() -> io::Result<()> {
    let mut program_state = parse_input(&std::env::args().nth(1).unwrap())?;
    execute_program(&mut program_state);

    println!("program finished");
    println!("{:?}", program_state);
    let result = program_state
        .out
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("{}", result);

    Ok(())
}
