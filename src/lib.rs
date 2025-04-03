mod vm;
mod instruction;
mod run;
mod compile;

pub type Word = i32;
pub use instruction::Instruction;
pub use vm::{Rvim, RvimError};

pub use run::run_file;
pub use compile::compile;
