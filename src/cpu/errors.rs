use std::{error::Error, fmt::Display};

use crate::cpu::instructions::Instruction;

#[derive(Debug, Clone, Copy)]
pub struct IllegalInstructionErr(Instruction);


impl Display for IllegalInstructionErr{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Illegal instruction, {:?}", self.0)
    }
}

impl Error for IllegalInstructionErr{}

impl From<Instruction> for IllegalInstructionErr{
    fn from(value: Instruction) -> Self {
        Self(value)
    }
}