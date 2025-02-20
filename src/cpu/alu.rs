use std::cell::RefCell;
use std::ops::{Add, Shl};
use std::rc::Rc;
use crate::cpu::carry_trait::CarryTrait;
use crate::cpu::register8::Register8;
use crate::cpu::register::Register;

const ZERO_FLAG: u8 = 7;        //Z
const SUB_FLAG: u8 = 6;         //N
const HALF_CARRY_FLAG: u8 = 5;  //H
const CARRY_FLAG: u8 = 4;       //C

struct ALU {
    flags: Rc<RefCell<Register8>>
}


impl ALU where  {

    pub fn new(flags: Rc<RefCell<Register8>>) -> Self {
        Self {
            flags
        }
    }

    pub fn add<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        let mut flags = self.flags.borrow_mut();

        flags.set_bit(SUB_FLAG, false);
        flags.set_bit(CARRY_FLAG, a.get_value().add_carries(b.get_value()));

        let shifted_a: T::ValueType = a.get_value().shl(4).into();
        let shifted_b: T::ValueType = b.get_value().shl(4).into();
        flags.set_bit(HALF_CARRY_FLAG, shifted_a.add_carries(shifted_b));

        a.wrapping_add(b);

        flags.set_bit(ZERO_FLAG, a.is_zero());
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_u8_no_flags () {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(a.get_value(), 0x03);
    }

    #[test]
    fn add_resets_sub_bit() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(SUB_FLAG, true);

        alu.add(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(SUB_FLAG), false);
    }

    #[test]
    fn add_sets_zero_bit_when_zero() {
        let mut a = Register8::new(0xFF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ZERO_FLAG));
    }

    #[test]
    fn add_resets_zero_bit_when_zero() {
        let mut a = Register8::new(0xEF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ZERO_FLAG));
    }

    #[test]
    fn add_sets_half_carry_flag_correctly() {
        let mut a = Register8::new(0x0F);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(HALF_CARRY_FLAG));
    }

    #[test]
    fn add_sets_carry_flag_correctly() {
        let mut a = Register8::new(0xFF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(CARRY_FLAG), true);
    }
}
