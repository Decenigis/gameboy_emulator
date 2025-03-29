#[cfg(test)]
mod nullable_instruction;
#[cfg(test)]
pub use nullable_instruction::*;

mod instruction;
mod bad_instruction;

mod nop; //0x00
mod jp;  //0xC3
mod di; //0xF3

pub use instruction::Instruction;
pub use instruction::decode_instruction;

pub use nop::NOP;//0x00
use jp::JP;      //0xC3
use di::DI;      //0xF3
