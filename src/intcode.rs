use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug, Copy, Clone)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl TryFrom<i32> for ParameterMode {
    type Error = String;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == ParameterMode::Position as i32 => Ok(ParameterMode::Position),
            x if x == ParameterMode::Immediate as i32 => Ok(ParameterMode::Immediate),
            _ => Err(format!("{} is not a ParameterMode", v)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Parameter {
    pub value: i32,
    pub mode: ParameterMode,
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Sum {
        i1: Parameter,
        i2: Parameter,
        out: Parameter,
    },
    Prod {
        i1: Parameter,
        i2: Parameter,
        out: Parameter,
    },
    Input {
        out: Parameter,
    },
    Output {
        i1: Parameter,
    },
    JumpIfTrue {
        i1: Parameter,
        pc: Parameter,
    },
    JumpIfFalse {
        i1: Parameter,
        pc: Parameter,
    },
    LessThan {
        i1: Parameter,
        i2: Parameter,
        out: Parameter,
    },
    Equals {
        i1: Parameter,
        i2: Parameter,
        out: Parameter,
    },
    Halt,
}

#[derive(Debug)]
pub struct IntCodeError {
    details: String,
}

impl IntCodeError {
    fn new<'a, T>(msg: T) -> Self
    where
        T: AsRef<str> + 'a,
    {
        IntCodeError {
            details: msg.as_ref().to_string(),
        }
    }
}

impl fmt::Display for IntCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for IntCodeError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<String> for IntCodeError {
    fn from(err: String) -> Self {
        IntCodeError::new(err)
    }
}

impl From<io::Error> for IntCodeError {
    fn from(err: io::Error) -> Self {
        IntCodeError::new(err.description())
    }
}

pub fn err_input() -> Result<i32, Box<dyn Error>> {
    Err(Box::new(IntCodeError::from(format!("No input"))))
}

pub fn process_instruction(
    program: &mut Vec<i32>,
    pc: &mut usize,
    input_fn: fn() -> Result<i32, Box<dyn Error>>,
) -> Result<bool, Box<dyn Error>> {
    let op = program[*pc];
    let opcode = op % 100;
    let mode1 = ParameterMode::try_from((op / 100) % 10)?;
    let mode2 = ParameterMode::try_from((op / 1000) % 10)?;
    let mode3 = ParameterMode::try_from((op / 10000) % 10)?;
    let instruction = match (opcode, mode1, mode2, mode3) {
        (1, _, _, ParameterMode::Position) => Instruction::Sum {
            i1: Parameter {
                value: program[*pc + 1],
                mode: mode1,
            },
            i2: Parameter {
                value: program[*pc + 2],
                mode: mode2,
            },
            out: Parameter {
                value: program[*pc + 3],
                mode: mode3,
            },
        },
        (2, _, _, ParameterMode::Position) => Instruction::Prod {
            i1: Parameter {
                value: program[*pc + 1],
                mode: mode1,
            },
            i2: Parameter {
                value: program[*pc + 2],
                mode: mode2,
            },
            out: Parameter {
                value: program[*pc + 3],
                mode: mode3,
            },
        },
        (3, ParameterMode::Position, _, _) => Instruction::Input {
            out: Parameter {
                value: program[*pc + 1],
                mode: mode1,
            },
        },
        (4, _, _, _) => Instruction::Output {
            i1: Parameter {
                value: program[*pc + 1],
                mode: mode1,
            },
        },
        (5, _, _, _) => Instruction::JumpIfTrue {
            i1: Parameter {
                value: program[*pc + 1],
                mode: mode1,
            },
            pc: Parameter {
                value: program[*pc + 2],
                mode: mode2,
            },
        },
        (6, _, _, _) => Instruction::JumpIfFalse {
            i1: Parameter {
                value: program[*pc + 1],
                mode: mode1,
            },
            pc: Parameter {
                value: program[*pc + 2],
                mode: mode2,
            },
        },
        (7, _, _, ParameterMode::Position) => Instruction::LessThan {
            i1: Parameter {
                value: program[*pc + 1],
                mode: mode1,
            },
            i2: Parameter {
                value: program[*pc + 2],
                mode: mode2,
            },
            out: Parameter {
                value: program[*pc + 3],
                mode: mode3,
            },
        },
        (8, _, _, ParameterMode::Position) => Instruction::Equals {
            i1: Parameter {
                value: program[*pc + 1],
                mode: mode1,
            },
            i2: Parameter {
                value: program[*pc + 2],
                mode: mode2,
            },
            out: Parameter {
                value: program[*pc + 3],
                mode: mode3,
            },
        },
        (99, _, _, _) => Instruction::Halt,
        _ => {
            return Err(Box::new(IntCodeError::new(format!(
                "Invalid instruction at {}: {}",
                *pc, program[*pc]
            ))))
        }
    };

    println!("{:?}", instruction);

    match instruction {
        Instruction::Sum { i1, i2, out } => {
            program[out.value as usize] =
                try_get_param(&program, i1)? + try_get_param(&program, i2)?;
            *pc += 4;
            Ok(true)
        }
        Instruction::Prod { i1, i2, out } => {
            program[out.value as usize] =
                try_get_param(&program, i1)? * try_get_param(&program, i2)?;
            *pc += 4;
            Ok(true)
        }
        Instruction::Input { out } => {
            program[out.value as usize] = input_fn()?;
            *pc += 2;
            Ok(true)
        }
        Instruction::Output { i1 } => {
            println!("OUTPUT: {}", try_get_param(&program, i1)?);
            *pc += 2;
            Ok(true)
        }
        Instruction::JumpIfTrue { i1, pc: pc_new } => {
            match try_get_param(&program, i1)? {
                0 => *pc += 3,
                _ => *pc = try_get_param(&program, pc_new)? as usize,
            }

            Ok(true)
        }
        Instruction::JumpIfFalse { i1, pc: pc_new } => {
            match try_get_param(&program, i1)? {
                0 => *pc = try_get_param(&program, pc_new)? as usize,
                _ => *pc += 3,
            }

            Ok(true)
        }
        Instruction::LessThan { i1, i2, out } => {
            program[out.value as usize] =
                match try_get_param(&program, i1)? < try_get_param(&program, i2)? {
                    true => 1,
                    false => 0,
                };

            *pc += 4;
            Ok(true)
        }
        Instruction::Equals { i1, i2, out } => {
            program[out.value as usize] =
                match try_get_param(&program, i1)? == try_get_param(&program, i2)? {
                    true => 1,
                    false => 0,
                };

            *pc += 4;
            Ok(true)
        }
        Instruction::Halt => {
            *pc += 1;
            Ok(false)
        }
    }
}

fn try_get_param(program: &Vec<i32>, param: Parameter) -> Result<i32, String> {
    match (param.mode, param.value) {
        (ParameterMode::Position, p) if p >= 0 && p < program.len() as i32 => {
            Ok(program[param.value as usize])
        }
        (ParameterMode::Immediate, _) => Ok(param.value),
        _ => Err(format!("Invalid parameter value: {:?}", param)),
    }
}
