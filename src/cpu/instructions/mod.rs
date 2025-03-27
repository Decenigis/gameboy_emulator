mod instruction;
mod bad_instruction;
mod nop;

#[cfg(test)]
mod nullable_instruction;
#[cfg(test)]
pub use nullable_instruction::*;


pub use instruction::Instruction;
pub use instruction::decode_instruction;
pub use nop::NOP;
