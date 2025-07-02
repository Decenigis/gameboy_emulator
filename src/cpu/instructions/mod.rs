#[cfg(test)]
mod nullable_instruction;
#[cfg(test)]
pub use nullable_instruction::*;

#[macro_use]
mod instruction;
mod bad_instruction;

mod nop; //0x00

mod ld_r_n;
mod ld_r_r;
mod ret_with_condition;
mod inc_r;
mod push_rr;
mod pop_rr;
mod add_a_r;
mod ld_rr_nn;

mod dec_bc; //0x0B
mod jr_nz; //0x20
mod ld_hl_nn; //0x21
mod inc_hl; //0x23
mod ld_sp_nn; //0x31
mod ld_hl_n; //0x36
mod add_a_a; //0x87
mod xor_a; //0xAF
mod or_b; //0xB0
mod jp_nn; //0xC3
mod ret; //0xC9
mod call_nn; //0xCD
mod add_a_n; //0xC6
mod sub_a_n; //0xD6
mod ldh_n_a; //0xE0
mod ldh_c_a; //0xE2
mod ld_nn_a; //0xEA
mod di;

pub use instruction::Instruction;
pub use instruction::decode_instruction;
use bad_instruction::BadInstruction;

pub use nop::Nop;       //0x00
use ld_r_n::*;
use ld_r_r::*;
use ret_with_condition::*;
use inc_r::*;
use push_rr::*;
use pop_rr::*;
use add_a_r::*;
use ld_rr_nn::*;

use dec_bc::DecBc;      //0x0B
use jr_nz::JrNz;        //0x20
use ld_hl_nn::LdHlNn;   //0x21
use inc_hl::IncHl;      //0x23
use ld_sp_nn::LdSpNn;   //0x31
use ld_hl_n::LdHlN;     //0x36
use add_a_a::AddAA;     //0x87
use xor_a::XorA;        //0xAF
use or_b::OrB;          //0xB0
use jp_nn::JpNn;        //0xC3
use ret::Ret;           //0xC9
use call_nn::CallNn;    //0xCD
use add_a_n::AddAN;     //0xD6
use sub_a_n::SubAN;     //0xD6
use ldh_n_a::LdhNA;     //0xE0
use ldh_c_a::LdhCA;     //0xE2
use ld_nn_a::LdNNA;     //0xEA
use di::Di;             //0xF3
