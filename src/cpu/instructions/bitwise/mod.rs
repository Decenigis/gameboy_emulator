mod bitwise_bad_instruction;
mod bitwise;

mod rlc_r;
mod rrc_r;
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

mod rlc_hl; //0xCB, 0x06
mod rrc_hl; //0xCB, 0x0E
mod rl_hl; //0xCB, 0x16
mod rr_hl; //0xCB, 0x1E
mod sla_hl; //0xCB, 0x26
mod swap_hl; //0xCB, 0x36

use rlc_r::*;
use rr_r::*;
use rrc_r::*;
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

use rlc_hl::RlcHl;
use rrc_hl::RrcHl;
use rl_hl::RlHl;
use rr_hl::RrHl;
use sla_hl::SlaHl;
use swap_hl::SwapHl;

pub use bitwise::Bitwise;
