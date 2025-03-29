#[cfg(test)]
mod nullable_instruction;
#[cfg(test)]
pub use nullable_instruction::*;

mod instruction;
mod bad_instruction;

mod nop; //0x00
mod jp_nn;//0xC3
mod di;  //0xF3

pub use instruction::Instruction;
pub use instruction::decode_instruction;
use bad_instruction::BadInstruction;     

pub use nop::Nop;//0x00
use jp_nn::JpNn;      //0xC3
use di::Di;      //0xF3
