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
mod ld_r_hl;
mod ld_hl_r;
mod ret_with_condition;
mod inc_r;
mod inc_rr;
mod dec_r;
mod dec_rr;
mod push_rr;
mod pop_rr;
mod or_a_r;
mod add_a_r;
mod adc_a_r;
mod and_a_r;
mod add_hl_rr;
mod sub_a_r;
mod sbc_a_r;
mod xor_a_r;
mod cp_a_r;
mod ld_rr_nn;
mod ld_rr_a;
mod ld_a_rr;
mod call_cc_nn;
mod jr_cc_n;
mod jp_cc_nn;
mod rst_nn;

mod rlca; //0x07
mod ld_nn_sp; //0x08
mod rrca; //0x0F
mod rla; //0x17
mod jr_n; //0x18
mod ld_hl_nn; //0x21
mod ldi_hl_a; //0x22
mod inc_hl; //0x23
mod ldi_a_hl; //0x2A
mod cpl_a; //0x2F
mod ld_sp_nn; //0x31
mod ldd_hl_a; //0x32
mod inc_hl_addr; //0x34
mod dec_hl_addr; //0x35
mod ld_hl_n; //0x36
mod ldd_a_hl; //0x3A
mod halt; //0x76
mod add_a_hl; //0x86
mod add_a_a; //0x87
mod and_a_hl; //0xA6
mod and_a_a; //0xA7
mod xor_a; //0xAF
mod jp_nn; //0xC3
mod ret; //0xC9
mod add_a_n; //0xC6
mod bitwise; //0xCB
mod adc_a_n; //0xCE
mod call_nn; //0xCD
mod sub_a_n; //0xD6
mod reti; //0xD9
mod sbc_a_n; //0xDE
mod ldh_n_a; //0xE0
mod ldh_c_a; //0xE2
mod and_a_n; //0xE6
mod jp_hl; //0xE9
mod ld_nn_a; //0xEA
mod ldh_a_n; //0xF0
mod di; //0xF3
mod ld_hl_sp_n; // 0xF8
mod ld_sp_hl; // 0xF9
mod ld_a_nn; //0xFA
mod ei; //0xFB
mod cp_a_n; //0xFE




pub use instruction::Instruction;
pub use instruction::decode_instruction;
use bad_instruction::BadInstruction;

pub use nop::Nop;       //0x00
use ld_r_n::*;
use ld_r_r::*;
use ld_r_hl::*;
use ld_hl_r::*;
use ret_with_condition::*;
use inc_r::*;
use inc_rr::*;
use dec_r::*;
use dec_rr::*;
use push_rr::*;
use pop_rr::*;
use or_a_r::*;
use add_a_r::*;
use adc_a_r::*;
use and_a_r::*;
use add_hl_rr::*;
use sub_a_r::*;
use sbc_a_r::*;
use xor_a_r::*;
use cp_a_r::*;
use ld_rr_nn::*;
use ld_rr_a::*;
use ld_a_rr::*;
use call_cc_nn::*;
use jr_cc_n::*;
use jp_cc_nn::*;
use rst_nn::*;

use rlca::RlcA;         //0x07
use ld_nn_sp::LdNnSp;   //0x08
use rrca::RrcA;         //0x0F
use rla::RlA;           //0x17
use jr_n::JrN;          //0x18
use ldi_hl_a::LdiHlA;   //0x22
use inc_hl::IncHl;      //0x23
use ldi_a_hl::LdiAHl;   //0x2A
use cpl_a::CplA;        //0x2F
use ld_sp_nn::LdSpNn;   //0x31
use ldd_hl_a::LddHlA;   //0x32
use inc_hl_addr::IncHlAddr; //0x34
use dec_hl_addr::DecHlAddr; //0x34
use ldd_a_hl::LddAHl;   //0x3A
use ld_hl_n::LdHlN;     //0x36
use halt::Halt;         //0x76
use add_a_hl::AddAHl;   //0x86
use add_a_a::AddAA;     //0x87
use and_a_hl::AndAHl;   //0xA6
use and_a_a::AndAA;     //0xA7
use xor_a::XorA;        //0xAF
use jp_nn::JpNn;        //0xC3
use add_a_n::AddAN;     //0xC6
use ret::Ret;           //0xC9
use adc_a_n::AdcAN;     //0xCE
use bitwise::Bitwise;   //0xCB
use call_nn::CallNn;    //0xCD
use sub_a_n::SubAN;     //0xD6
use reti::Reti;         //0xD9
use sbc_a_n::SbcAN;     //0xDE
use ldh_n_a::LdhNA;     //0xE0
use ldh_c_a::LdhCA;     //0xE2
use and_a_n::AndAN;     //0xE6
use jp_hl::JpHl;        //0xE9
use ld_nn_a::LdNNA;     //0xEA
use ldh_a_n::LdhAN;     //0xF0
use di::Di;             //0xF3
use ld_hl_sp_n::LdHlSpN;//0xF8
use ld_sp_hl::LdSpHl;   //0xF9
use ld_a_nn::LdANN;     //0xFA
use ei::Ei;             //0xFB
use cp_a_n::CpAN;       //0xFE
