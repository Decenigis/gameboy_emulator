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

pub struct ALU {
    flags: Rc<RefCell<Register8>>
}


impl ALU {

    pub fn new(flags: Rc<RefCell<Register8>>) -> Self {
        Self {
            flags
        }
    }

    fn add_internal<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        let mut flags = self.flags.borrow_mut();
        flags.set_bit(SUB_FLAG, false);

        if a.get_value().add_carries(b.get_value()) {
            flags.set_bit(CARRY_FLAG, true);
        }

        let shifted_a: T::ValueType = a.get_value().shl(4).into();
        let shifted_b: T::ValueType = b.get_value().shl(4).into();
        if shifted_a.add_carries(shifted_b) {
            flags.set_bit(HALF_CARRY_FLAG, true);
        }

        a.wrapping_add(b);

        flags.set_bit(ZERO_FLAG, a.is_zero());
    }

    pub fn add<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        {
            let mut flags = self.flags.borrow_mut();
            flags.set_bit(CARRY_FLAG, false);
            flags.set_bit(HALF_CARRY_FLAG, false);
        }

        self.add_internal(a, b);
    }

    pub fn adc<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>,
    {
        let carry = self.flags.borrow().get_bit(CARRY_FLAG);

        self.add(a, b); //Flags are reset by this method so no need for direct manipulation here

        if carry {
            self.add_internal(a, &T::one());
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::cpu::register16::Register16;
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
    fn adds_u16_no_flags () {
        let mut a = Register16::new(0xFF);
        let b = Register16::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(a.get_value(), 0x101);
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
    fn add_resets_zero_bit_when_not_zero() {
        let mut a = Register8::new(0xEF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ZERO_FLAG));
    }

    #[test]
    fn add_sets_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x0F);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(HALF_CARRY_FLAG));
    }

    #[test]
    fn add_does_not_set_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x02);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(HALF_CARRY_FLAG));
    }

    #[test]
    fn add_sets_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0x0F00);
        let b = Register16::new(0x0F00);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(HALF_CARRY_FLAG));
    }

    #[test]
    fn add_does_not_set_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0xF0);
        let b = Register16::new(0xF0);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(HALF_CARRY_FLAG));
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

    #[test]
    fn adc_does_not_carry_when_carry_bit_not_set() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(CARRY_FLAG, false);
        alu.adc(&mut a, &b);

        assert_eq!(0x03, a.get_value());
    }

    #[test]
    fn adc_carries_when_carry_bit_set() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(0x04, a.get_value());
    }

    #[test]
    fn adc_resets_carry() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(CARRY_FLAG));
    }

    #[test]
    fn adc_with_carry_bit_set_preserves_carry_from_a_b_addition() {
        let mut a = Register8::new(0xF0);
        let b = Register8::new(0x1F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(CARRY_FLAG));
    }

    #[test]
    fn adc_with_carry_bit_set_sets_carry_bit_on_carry_wrap() {
        //edge case where carry causes the rollover rather than the a+b

        let mut a = Register8::new(0xF0);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(CARRY_FLAG));
    }

    #[test]
    fn adc_with_carry_preserves_half_carry_from_a_b_addition() {
        let mut a = Register8::new(0x08);
        let b = Register8::new(0x08);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(HALF_CARRY_FLAG));
    }

    #[test]
    fn adc_with_carry_sets_half_carry_when_caused_by_carry() {
        let mut a = Register8::new(0x07);
        let b = Register8::new(0x08);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(HALF_CARRY_FLAG));
    }


    #[test]
    fn adc_with_carry_sets_zero_when_zero() {
        let mut a = Register8::new(0xF0);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ZERO_FLAG));
    }
}
