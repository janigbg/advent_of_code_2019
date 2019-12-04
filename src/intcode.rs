#[derive(Debug)]
pub enum Instruction {
    Sum { i1: usize, i2: usize, out: usize },
    Prod { i1: usize, i2: usize, out: usize },
    Halt,
}

pub fn process_instruction(program: &mut Vec<i32>, pc: &mut usize) -> bool {
    let instruction = match program[*pc] {
        1 => Instruction::Sum {
            i1: program[*pc + 1] as usize,
            i2: program[*pc + 2] as usize,
            out: program[*pc + 3] as usize,
        },
        2 => Instruction::Prod {
            i1: program[*pc + 1] as usize,
            i2: program[*pc + 2] as usize,
            out: program[*pc + 3] as usize,
        },
        99 => Instruction::Halt,
        _ => panic!(format!("Invalid instruction at {}: {}", *pc, program[*pc])),
    };

    println!("{:?}", instruction);

    match instruction {
        Instruction::Sum { i1, i2, out } => {
            program[out] = program[i1] + program[i2];
            *pc += 4;
            true
        }
        Instruction::Prod { i1, i2, out } => {
            program[out] = program[i1] * program[i2];
            *pc += 4;
            true
        }
        Instruction::Halt => {
            *pc += 1;
            false
        }
    }
}
