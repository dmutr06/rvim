use std::{collections::HashMap, fs::File, io::Read, str::FromStr};

use crate::{Instruction, Word};

#[derive(Debug, Clone, Copy)]
pub enum CompileError {
    BadInstruction(usize),
    BadOperand(usize),
    CannotWrite,
}

pub fn compile(src: &mut File, out: &mut File) -> Result<(), CompileError> {
    use Instruction::*;
    let mut program: Vec<Instruction> = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();

    let mut buf = String::new();
    src.read_to_string(&mut buf).unwrap();
    {
        let mut ip = 0;
        for line in buf.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match line {
                label if label.ends_with(":") => {
                    labels.insert(String::from(&label[0..label.len() - 1]), ip);
                },
                _ => ip += 1,

            }        
        }
    }

    for (idx, line) in buf.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut words = line.split_whitespace();

        let inst = words.next().unwrap();
        match inst {
            "push" => {
                let x: Word = try_parse(words.next(), idx + 1)?;
                require_no_more_ops(words, idx + 1)?;
                program.push(Push(x));
            },
            "add" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Add);
            }
            "sub" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Sub);
            }
            "mul" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Mul);
            }
            "div" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Div);
            }
            "halt" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Halt);
            },
            "jmp" => {
                let addr = parse_addr_or_label(words.next(), &labels, idx + 1)?;
                require_no_more_ops(words, idx + 1)?;
                program.push(Jmp(addr));
            },
            "jif" => {
                let addr = parse_addr_or_label(words.next(), &labels, idx + 1)?;
                require_no_more_ops(words, idx + 1)?;
                program.push(Jif(addr));
            },
            "jifz" => {
                let addr = parse_addr_or_label(words.next(), &labels, idx + 1)?;
                require_no_more_ops(words, idx + 1)?;
                program.push(Jifz(addr));
            },
            "eq" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Eq);
            },
            "dup" => {
                let dup_idx: usize = try_parse(words.next(), idx + 1)?;
                require_no_more_ops(words, idx + 1)?;
                program.push(Dup(dup_idx));
            },
            "pop" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Pop);
            },
            "swap" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Swap);
            },
            "rot" => {
                require_no_more_ops(words, idx + 1)?;
                program.push(Rot);
            }
            label if label.ends_with(":") =>  {
                continue;
            },
            _ => return Err(CompileError::BadInstruction(idx + 1)),
        }
    }

    match bincode::encode_into_std_write(&program, out, bincode::config::standard()) {
        Ok(_) => Ok(()),
        Err(_) => Err(CompileError::CannotWrite),
    }
}

fn try_parse<T: FromStr>(word: Option<&str>, line: usize) -> Result<T, CompileError> {
    match word {
        None => Err(CompileError::BadOperand(line)),
        Some(word) => {
            Ok(word.parse::<T>().map_err(|_| CompileError::BadOperand(line))?)
        }
    }
}

fn require_no_more_ops<'a>(mut words: impl Iterator<Item = &'a str>, line: usize) -> Result<(), CompileError> {
    if words.next().is_some() {
        Err(CompileError::BadOperand(line))
    } else {
        Ok(())
    }
}

fn parse_addr_or_label(word: Option<&str>, labels: &HashMap<String, usize>, line: usize) -> Result<usize, CompileError> {
    match word {
        None => Err(CompileError::BadOperand(line)),
        Some(word) => {
            if word.as_bytes()[0].is_ascii_digit() {
                try_parse(Some(word), line)
            } else {
                match labels.get(word) {
                    Some(addr) => Ok(*addr),
                    None => Err(CompileError::BadOperand(line)),
                }
            }
        }
    }
}
