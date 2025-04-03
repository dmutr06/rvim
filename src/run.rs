use std::fs::File;

use crate::{Instruction, Rvim, RvimError};

pub fn run_file(file: &mut File) -> Result<Rvim, RvimError> {
    let program: Vec<Instruction> = match bincode::decode_from_std_read(file, bincode::config::standard()) {
        Ok(program) => program,
        Err(why) => {
            eprintln!("Error reading program: {}", why);
            return Err(RvimError::BadByteCode);
        }
    };

    let mut vm = Rvim::new(program);

    vm.run()?;

    Ok(vm)
}
