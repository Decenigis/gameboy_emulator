use std::ops::DerefMut;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::cpu::alu::ALU;
use crate::cpu::instructions::Instruction;
use crate::cpu::instructions::reusable::ReusableLdRN;
use crate::cpu::registers::Registers;
use crate::memory::MemoryController;

pub struct LdAN {
    reusable_ld_r_n: ReusableLdRN,
}

impl Instruction for LdAN {

    #[inline]
    fn from_opcode(opcode: &u8) -> Option<Box<dyn Instruction>> {
        if *opcode == 0x3E {
            return Some(Box::new(LdAN{
                reusable_ld_r_n: ReusableLdRN::new()
            }))
        }
        None
    }

    fn get_opcode(&self) -> u8 {
        0x3E
    }

    fn act(&mut self, registers: &mut Registers, _alu: &mut ALU, memory_controller: Arc<Mutex<MemoryController>>, _enable_interrupts: &mut bool) -> bool {
        self.reusable_ld_r_n.act(registers.a.borrow_mut().deref_mut(), &mut registers.pc, memory_controller)
    }
}



#[cfg(test)]
mod tests {
    use crate::reusable_testing_macro;
    use super::*;
    
    reusable_testing_macro!(0x3E, LdAN);
}
