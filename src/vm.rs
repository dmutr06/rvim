use crate::{Instruction, Word};


#[derive(Debug, Clone)]
pub enum RvimError {
    StackUnderflow,
    StackOverflow,
    InvalidOperand,
    BadByteCode,
    DivByZero,
}

pub struct Rvim {
    stack: [Word; 1024],
    program: Vec<Instruction>,
    ip: usize,
    stack_len: usize,
    halt: bool,
}

impl Rvim {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            stack: [0; 1024],
            program,
            ip: 0,
            stack_len: 0,
            halt: false,
        }
    }

    pub fn exec_inst(&mut self) -> Result<(), RvimError> {
        let inst = self.program[self.ip];

        use Instruction::*;
        let move_ip = match inst {
            Push(x) => self.inst_push(x),
            Add => self.inst_add(),
            Sub => self.inst_sub(),
            Mul => self.inst_mul(),
            Div => self.inst_div(),
            Halt => self.inst_halt(),
            Jmp(ip) => self.inst_jmp(ip),
            Eq => self.inst_eq(),
            Jif(ip) => self.inst_jif(ip),
            Dup(idx) => self.inst_dup(idx),
            Pop => self.inst_pop(),
            Swap => self.inst_swap(),
        }?;

        if move_ip {
            self.ip += 1;
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), RvimError> {
        while self.ip < self.program.len() && !self.halt {
            self.exec_inst()?;
        }

        Ok(())
    }

    pub fn stack_dump(&self) {
        for i in 0..self.stack_len {
            println!("{}", self.stack[i]);
        }
    }

    fn inst_push(&mut self, x: Word) -> Result<bool, RvimError> {
        self.check_for_stack_overflow()?;
        self.stack[self.stack_len] = x;
        self.stack_len += 1;
        Ok(true)
    }

    fn inst_add(&mut self) -> Result<bool, RvimError> {
        self.require_stack_len(2)?;

        self.stack_len -= 1;
        self.stack[self.stack_len - 1] += self.stack[self.stack_len];
        Ok(true)
    }

    fn inst_sub(&mut self) -> Result<bool, RvimError> {
        self.require_stack_len(2)?;

        self.stack_len -= 1;
        self.stack[self.stack_len - 1] -= self.stack[self.stack_len];
        Ok(true)
    }
    
    fn inst_mul(&mut self) -> Result<bool, RvimError> {
        self.require_stack_len(2)?;

        self.stack_len -= 1;
        self.stack[self.stack_len - 1] *= self.stack[self.stack_len];
        Ok(true)
    }

    fn inst_div(&mut self) -> Result<bool, RvimError> {
        self.require_stack_len(2)?;

        self.stack_len -= 1;
        let div = self.stack[self.stack_len];
        if div == 0 {
            return Err(RvimError::DivByZero);
        }

        self.stack[self.stack_len - 1] /= self.stack[self.stack_len];
        Ok(true)
    }

    fn inst_jmp(&mut self, ip: usize) -> Result<bool, RvimError> {
        if ip >= self.program.len() {
            return Err(RvimError::InvalidOperand);
        }
        self.ip = ip;
        Ok(false)
    }

    fn inst_eq(&mut self) -> Result<bool, RvimError> {
        self.require_stack_len(2)?;

        self.stack_len -= 1;
        self.stack[self.stack_len - 1] =
        (self.stack[self.stack_len] == self.stack[self.stack_len - 1]) as Word;
        Ok(true)
    }

    fn inst_jif(&mut self, ip: usize) -> Result<bool, RvimError> {
        self.require_stack_len(1)?;

        if ip >= self.program.len() {
            return Err(RvimError::InvalidOperand);
        }

        self.stack_len -= 1;

        if self.stack[self.stack_len] != 0 {
            self.ip = ip;
            Ok(false)
        } else {
            Ok(true)
        }
    }

    fn inst_dup(&mut self, idx: usize) -> Result<bool, RvimError> {
        self.require_stack_len(idx)?;
        self.check_for_stack_overflow()?;

        self.stack[self.stack_len] = self.stack[self.stack_len - 1 - idx];
        self.stack_len += 1;
        Ok(true)
    }

    fn inst_halt(&mut self) -> Result<bool, RvimError> {
        self.halt = true;
        Ok(false)
    }

    fn inst_pop(&mut self) -> Result<bool, RvimError> {
        self.require_stack_len(1)?;
        self.stack_len -= 1;
        Ok(true)
    }

    fn inst_swap(&mut self) -> Result<bool, RvimError> {
        self.require_stack_len(2)?;
        let temp = self.stack[self.stack_len - 1];
        self.stack[self.stack_len - 1] = self.stack[self.stack_len - 2];
        self.stack[self.stack_len - 2] = temp;
        Ok(true)
    }
    
    fn require_stack_len(&self, min_len: usize) -> Result<(), RvimError> {
        if self.stack_len < min_len {
            Err(RvimError::StackUnderflow)
        } else {
            Ok(())
        }
    }

    fn check_for_stack_overflow(&self) -> Result<(), RvimError> {
        if self.stack_len >= self.stack.len() {
            Err(RvimError::StackOverflow)
        } else {
            Ok(())
        }
    }
}
