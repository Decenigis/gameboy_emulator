#[cfg(test)]
mod nullable_instruction;
#[cfg(test)]
pub use nullable_instruction::*;

mod instruction;
mod bad_instruction;

mod nop; //0x00
mod ld_bc_nn; //0x01
mod dec_bc; //0x0B
mod ld_hl_nn; //0x21
mod inc_hl; //0x23
mod ld_sp_nn; //0x31
mod ld_hl_n; //0x36
mod ld_a_c; //0x79
mod xor_a; //0xAF
mod or_b; //0xB0
mod jp_nn; //0xC3
mod call_nn; //0xCD
mod ldh_n_a; //0xE0
mod di; //0xF3






pub use instruction::Instruction;
pub use instruction::decode_instruction;
use bad_instruction::BadInstruction;

pub use nop::Nop;       //0x00
use ld_bc_nn::LdBcNn;   //0x01
use dec_bc::DecBc;      //0x0B
use ld_hl_nn::LdHlNn;   //0x21
use inc_hl::IncHl;      //0x23
use ld_sp_nn::LdSpNn;   //0x31
use ld_hl_n::LdHlN;     //0x36
use ld_a_c::LdAC;       //0x79
use xor_a::XorA;        //0xAF
use or_b::OrB;          //0xB0
use jp_nn::JpNn;        //0xC3
use call_nn::CallNn;    //0xCD
use ldh_n_a::LdhNA;     //0xE0
use di::Di;             //0xF3
