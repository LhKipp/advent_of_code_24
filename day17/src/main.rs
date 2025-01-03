use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct ProgramState {
    a: i64,
    b: i64,
    c: i64,
    i: usize,
    out: Vec<i64>,
    instructions: Vec<i64>,
}

impl ProgramState {
    pub fn reset(&mut self) {
        self.b = 0;
        self.c = 0;
        self.i = 0;
    }

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
}

fn bst(program: &mut ProgramState) {
    let operand = program.combo_op_at(program.i + 1);
    program.b = operand % 8;
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
}

fn out(program: &mut ProgramState) {
    let operand = program.combo_op_at(program.i + 1);
    program.out.push(operand % 8);
}

fn execute_program(program: &mut ProgramState) -> Vec<i64> {
    while (program.i + 1) < program.instructions.len() {
        let op = program.instructions[program.i];
        match op {
            0 => program.a >>= program.combo_op_at(program.i + 1),
            1 => bxl(program),
            2 => bst(program),
            3 => jnz(program),
            4 => bxc(program),
            5 => out(program),
            6 => program.b = program.a >> program.combo_op_at(program.i + 1),
            7 => program.c = program.a >> program.combo_op_at(program.i + 1),
            _ => unreachable!("found non opcode"),
        }
        if op != 3 {
            program.i += 2;
        }
    }
    std::mem::take(&mut program.out)
}

fn main() -> io::Result<()> {
    let mut program_state = parse_input(&std::env::args().nth(1).unwrap())?;

    let mut a = 0;
    for i in (0..program_state.instructions.len()).rev() {
        a <<= 3;
        let instructions: Vec<i64> = program_state.instructions[i..].to_vec();
        loop {
            program_state.reset();
            program_state.a = a;
            let out_test = execute_program(&mut program_state);
            println!("A {}: testing {:?} == {:?}", a, out_test, instructions);
            if out_test == instructions {
                break;
            }
            a += 1;
        }
    }

    println!("{}", a);

    Ok(())
}
