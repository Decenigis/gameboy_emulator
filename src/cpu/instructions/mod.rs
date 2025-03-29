#[cfg(test)]
mod nullable_instruction;
#[cfg(test)]
pub use nullable_instruction::*;

mod instruction;
mod bad_instruction;
mod nop;
mod di;


pub use instruction::Instruction;
pub use instruction::decode_instruction;
pub use nop::NOP;
use di::DI;
