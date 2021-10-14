use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

impl TryFrom<i32> for ParameterMode {
    type Error = String;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == ParameterMode::Position as i32 => Ok(ParameterMode::Position),
            x if x == ParameterMode::Immediate as i32 => Ok(ParameterMode::Immediate),
            x if x == ParameterMode::Relative as i32 => Ok(ParameterMode::Relative),
            _ => Err(format!("{} is not a ParameterMode", v)),
        }
    }
}

impl TryFrom<i64> for ParameterMode {
    type Error = String;

    fn try_from(v: i64) -> Result<Self, Self::Error> {
        match v {
            x if x == ParameterMode::Position as i64 => Ok(ParameterMode::Position),
            x if x == ParameterMode::Immediate as i64 => Ok(ParameterMode::Immediate),
            x if x == ParameterMode::Relative as i64 => Ok(ParameterMode::Relative),
            _ => Err(format!("{} is not a ParameterMode", v)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Parameter {
    pub value: i64,
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
    RelativeBaseOffset {
        i1: Parameter,
    },
    Halt,
}

#[derive(Debug)]
pub struct IntCodeError {
    details: String,
}

impl IntCodeError {
    pub fn new<'a, T>(msg: T) -> Self
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
        IntCodeError::new(err.to_string())
    }
}

pub fn err_input() -> Result<i64, Box<dyn Error>> {
    Err(Box::new(IntCodeError::from(format!("No input"))))
}

pub fn stdout_output(out: i64) -> () {
    println!("OUTPUT: {}", out);
} 

pub fn process_instruction<FIn, FOut>(
    program: &mut Vec<i64>,
    pc: &mut usize,
    relative_base: &mut i64,
    input_fn: &mut FIn,
    output_fn: &mut FOut,
) -> Result<bool, Box<dyn Error>>
where
    FIn: FnMut() -> Result<i64, Box<dyn Error>>,
    FOut: FnMut(i64) -> ()
{
    let op = program[*pc];
    let opcode = op % 100;
    let mode1 = ParameterMode::try_from((op / 100) % 10)?;
    let mode2 = ParameterMode::try_from((op / 1000) % 10)?;
    let mode3 = ParameterMode::try_from((op / 10000) % 10)?;
    let instruction = match (opcode, mode1, mode2, mode3) {
        (1, _, _, m) if m != ParameterMode::Immediate => Instruction::Sum {
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
        (2, _, _, m) if m != ParameterMode::Immediate => Instruction::Prod {
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
        (3, m, _, _) if m != ParameterMode::Immediate => Instruction::Input {
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
        (7, _, _, m) if m != ParameterMode::Immediate => Instruction::LessThan {
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
        (8, _, _, m) if m != ParameterMode::Immediate => Instruction::Equals {
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
        (9, _, _, _) => Instruction::RelativeBaseOffset {
            i1: Parameter {
                value: program[*pc + 1],
                mode: mode1,
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

    //println!("{:?}", instruction);

    match instruction {
        Instruction::Sum { i1, i2, out } => {
            let mem = try_get_address(&program, &relative_base, out)?;
            program[mem] =
                try_get_param(&program, &relative_base, i1)? + try_get_param(&program, &relative_base, i2)?;
            *pc += 4;
            Ok(true)
        }
        Instruction::Prod { i1, i2, out } => {
            let mem = try_get_address(&program, &relative_base, out)?;
            program[mem] =
                try_get_param(&program, &relative_base, i1)? * try_get_param(&program, &relative_base, i2)?;
            *pc += 4;
            Ok(true)
        }
        Instruction::Input { out } => {
            let mem = try_get_address(&program, &relative_base, out)?;
            program[mem] = input_fn()?;
            *pc += 2;
            Ok(true)
        }
        Instruction::Output { i1 } => {
            output_fn(try_get_param(&program, &relative_base, i1)?);
            *pc += 2;
            Ok(true)
        }
        Instruction::JumpIfTrue { i1, pc: pc_new } => {
            match try_get_param(&program, &relative_base, i1)? {
                0 => *pc += 3,
                _ => *pc = try_get_param(&program, &relative_base, pc_new)? as usize,
            }

            Ok(true)
        }
        Instruction::JumpIfFalse { i1, pc: pc_new } => {
            match try_get_param(&program, &relative_base, i1)? {
                0 => *pc = try_get_param(&program, &relative_base, pc_new)? as usize,
                _ => *pc += 3,
            }

            Ok(true)
        }
        Instruction::LessThan { i1, i2, out } => {
            let mem = try_get_address(&program, &relative_base, out)?;
            program[mem] =
                match try_get_param(&program, &relative_base, i1)? < try_get_param(&program, &relative_base, i2)? {
                    true => 1,
                    false => 0,
                };

            *pc += 4;
            Ok(true)
        }
        Instruction::Equals { i1, i2, out } => {
            let mem = try_get_address(&program, &relative_base, out)?;
            program[mem] =
                match try_get_param(&program, &relative_base, i1)? == try_get_param(&program, &relative_base, i2)? {
                    true => 1,
                    false => 0,
                };

            *pc += 4;
            Ok(true)
        }
        Instruction::RelativeBaseOffset { i1 } => {
            *relative_base += try_get_param(&program, &relative_base, i1)?;
            *pc += 2;
            //println!("Relative offset: {}", *relative_base);
            Ok(true)
        }
        Instruction::Halt => {
            *pc += 1;
            Ok(false)
        }
    }
}

fn try_get_address(program: &Vec<i64>, relative_base: &i64, param: Parameter) -> Result<usize, String> {
    match (param.mode, param.value) {
        (ParameterMode::Position, p) if p >= 0 && p < program.len() as i64 => {
            Ok(param.value as usize)
        }
        (ParameterMode::Relative, p) if *relative_base + p >= 0 => {
            Ok((*relative_base + param.value) as usize)
        }
        _ => Err(format!("Invalid parameter value: {:?}", param)),
    }
}

fn try_get_param(program: &Vec<i64>, relative_base: &i64, param: Parameter) -> Result<i64, String> {
    match (param.mode, param.value) {
        (ParameterMode::Position, p) if p >= 0 && p < program.len() as i64 => {
            Ok(program[param.value as usize])
        }
        (ParameterMode::Immediate, _) => Ok(param.value),
        (ParameterMode::Relative, p) if *relative_base + p >= 0 => {
            Ok(program[(*relative_base + param.value) as usize])
        }
        _ => Err(format!("Invalid parameter value: {:?}", param)),
    }
}
