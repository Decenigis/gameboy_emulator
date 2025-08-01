mod bitwise_bad_instruction;
mod bitwise;

mod rr_r;
mod rl_r;
mod sla_r;
mod sra_r;
mod srl_r;
mod bit_b_r;
mod bit_b_hl;
mod res_b_r;
mod res_b_hl;
mod set_b_r;
mod set_b_hl;
mod swap_r;

mod rrc_hl; //0xCB, 0x0E

use rr_r::*;
use rl_r::*;
use sla_r::*;
use sra_r::*;
use srl_r::*;
use bit_b_r::*;
use bit_b_hl::*;
use res_b_r::*;
use res_b_hl::*;
use set_b_r::*;
use set_b_hl::*;
use swap_r::*;

pub use bitwise::Bitwise;
