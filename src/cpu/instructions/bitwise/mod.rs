mod bitwise_bad_instruction;
mod bitwise;

mod rr_r;
mod bit_b_r;
mod res_b_r;
mod res_b_hl;
mod set_b_r;
mod set_b_hl;
mod swap_r;

use rr_r::*;
use bit_b_r::*;
use res_b_r::*;
use res_b_hl::*;
use set_b_r::*;
use set_b_hl::*;
use swap_r::*;

pub use bitwise::Bitwise;
