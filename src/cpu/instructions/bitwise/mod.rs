mod bitwise_bad_instruction;
mod bitwise;

mod rr_r;
mod bit;
mod res_b_r;
mod res_b_hl;
mod set_b_r;
mod swap_r;

use rr_r::*;
use bit::*;
use res_b_r::*;
use res_b_hl::*;
use set_b_r::*;
use swap_r::*;

pub use bitwise::Bitwise;
