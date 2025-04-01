#[cfg(test)]
mod nullable_instruction;
#[cfg(test)]
pub use nullable_instruction::*;

mod instruction;
mod bad_instruction;

mod nop; //0x00
mod ld_bc_nn; //0x01
mod ld_hl_nn; //0x21
mod ld_sp_nn; //0x31
mod ld_hl_n; //0x36
mod xor_a; //0xAF
mod jp_nn; //0xC3
mod call_nn; //0xCD
mod ldh_n_a; //0xE0
mod di;//0xF3





pub use instruction::Instruction;
pub use instruction::decode_instruction;
use bad_instruction::BadInstruction;

pub use nop::Nop;       //0x00
use ld_bc_nn::LdBcNn;   //0x01
use ld_hl_nn::LdHlNn;   //0x21
use ld_sp_nn::LdSpNn;   //0x31
use ld_hl_n::LdHlN;     //0x36
use xor_a::XorA;        //0xAF
use jp_nn::JpNn;        //0xC3
use call_nn::CallNn;    //0xCD
use ldh_n_a::LdhNA;     //0xE0
use di::Di;             //0xF3
