use bincode::{Decode, Encode};
use crate::Word;

#[derive(Debug, Encode, Decode, Clone, Copy)]
pub enum Instruction {
    Push(Word),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Halt,
    Jmp(usize),
    Eq,
    Jif(usize),
    Dup(usize),
    Swap,
}
