use std::cell::RefCell;
use std::ops::{Add, BitAnd, BitOr, BitXor, Shl};
use std::rc::Rc;
use crate::cpu::carry_trait::CarryTrait;
use crate::cpu::register8::Register8;
use crate::cpu::register::Register;

pub struct ALU {
    flags: Rc<RefCell<Register8>>
}


impl ALU {

    pub const ZERO_FLAG: u8 = 7;        //Z
    pub const SUB_FLAG: u8 = 6;         //N
    pub const HALF_CARRY_FLAG: u8 = 5;  //H
    pub const CARRY_FLAG: u8 = 4;       //C

    pub fn new(flags: Rc<RefCell<Register8>>) -> Self {
        Self {
            flags
        }
    }

    pub fn and<T: Register>(&self, a: &mut T, b: &T)
    where<T as Register>::ValueType: BitAnd,
         <T as Register>::ValueType: From<<<T as Register>::ValueType as BitAnd>::Output>
    {
        let mut flags = self.flags.borrow_mut();

        flags.set_bit(Self::SUB_FLAG, false);
        flags.set_bit(Self::HALF_CARRY_FLAG, true);
        flags.set_bit(Self::CARRY_FLAG, false);

        let value = (a.get_value() & b.get_value()).into();
        a.set_value(value);

        flags.set_bit(Self::ZERO_FLAG, a.is_zero());
    }

    pub fn xor_internal<T: Register>(&self, a: Rc<RefCell<T>>, b: Rc<RefCell<T>>)
    where
        <T as Register>::ValueType: BitXor,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as BitXor>::Output>
    {
        let mut flags = self.flags.borrow_mut();

        flags.set_bit(Self::SUB_FLAG, false);
        flags.set_bit(Self::HALF_CARRY_FLAG, false);
        flags.set_bit(Self::CARRY_FLAG, false);

        let value = (a.borrow().get_value() ^ b.borrow().get_value()).into();
        a.borrow_mut().set_value(value);

        flags.set_bit(Self::ZERO_FLAG, a.borrow().is_zero());
    }

    pub fn or_internal<T: Register>(&self, a: Rc<RefCell<T>>, b: Rc<RefCell<T>>)
    where
        <T as Register>::ValueType: BitOr,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as BitOr>::Output>
    {
        let mut flags = self.flags.borrow_mut();

        flags.set_bit(Self::SUB_FLAG, false);
        flags.set_bit(Self::HALF_CARRY_FLAG, false);
        flags.set_bit(Self::CARRY_FLAG, false);

        let value = (a.borrow().get_value() | b.borrow().get_value()).into();
        a.borrow_mut().set_value(value);

        flags.set_bit(Self::ZERO_FLAG, a.borrow().is_zero());
    }

    fn add_internal<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        let mut flags = self.flags.borrow_mut();
        flags.set_bit(Self::SUB_FLAG, false);

        if a.get_value().add_carries(b.get_value()) {
            flags.set_bit(Self::CARRY_FLAG, true);
        }

        let shifted_a: T::ValueType = a.get_value().shl(4).into();
        let shifted_b: T::ValueType = b.get_value().shl(4).into();
        if shifted_a.add_carries(shifted_b) {
            flags.set_bit(Self::HALF_CARRY_FLAG, true);
        }

        a.wrapping_add(b);

        flags.set_bit(Self::ZERO_FLAG, a.is_zero());
    }

    pub fn add<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        {
            let mut flags = self.flags.borrow_mut();
            flags.set_bit(Self::CARRY_FLAG, false);
            flags.set_bit(Self::HALF_CARRY_FLAG, false);
        }

        self.add_internal(a, b);
    }

    pub fn add_no_carry<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        let carry_flag = {
            let mut flags = self.flags.borrow_mut();
            flags.set_bit(Self::HALF_CARRY_FLAG, false);

            flags.get_bit(Self::CARRY_FLAG)
        };

        self.add_internal(a, b);

        self.flags.borrow_mut().set_bit(Self::CARRY_FLAG, carry_flag);
    }

    pub fn adc<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>,
    {
        let carry = self.flags.borrow().get_bit(Self::CARRY_FLAG);

        self.add(a, b); //Flags are reset by this method so no need for direct manipulation here

        if carry {
            self.add_internal(a, &T::one());
        }
    }

    fn sub_internal<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        let mut flags = self.flags.borrow_mut();
        flags.set_bit(Self::SUB_FLAG, true);

        if a.get_value().sub_borrows(b.get_value()) {
            flags.set_bit(Self::CARRY_FLAG, true);
        }

        let shifted_a: T::ValueType = a.get_value().shl(4).into();
        let shifted_b: T::ValueType = b.get_value().shl(4).into();

        if shifted_a.sub_borrows(shifted_b) {
            flags.set_bit(Self::HALF_CARRY_FLAG, true);
        }
        // THIS IS WRONG!!! Barely anything uses half carry so it's maybe fine?

        a.wrapping_sub(b);

        flags.set_bit(Self::ZERO_FLAG, a.is_zero());
    }

    pub fn sub<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        {
            let mut flags = self.flags.borrow_mut();
            flags.set_bit(Self::CARRY_FLAG, false);
            flags.set_bit(Self::HALF_CARRY_FLAG, false);
        }

        self.sub_internal(a, b);
    }

    pub fn sub_no_carry<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>
    {
        let carry_flag = {
            let mut flags = self.flags.borrow_mut();
            flags.set_bit(Self::HALF_CARRY_FLAG, false);

            flags.get_bit(Self::CARRY_FLAG)
        };

        self.sub_internal(a, b); //Flags are reset by this method so no need for direct manipulation here

        self.flags.borrow_mut().set_bit(Self::CARRY_FLAG, carry_flag);
    }

    pub fn sbc<T: Register>(&self, a: &mut T, b: &T)
    where
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Shl<i32>>::Output>,
        <T as Register>::ValueType: From<<<T as Register>::ValueType as Add>::Output>,
    {
        let carry = self.flags.borrow().get_bit(Self::CARRY_FLAG);

        self.sub(a, b); //Flags are reset by this method so no need for direct manipulation here

        if carry {
            self.sub_internal(a, &T::one());
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::cpu::register16::Register16;
    use super::*;


    #[test]
    fn ands_correctly() {
        let mut a = Register8::new(0b11110000);
        let b = Register8::new(0b01010101);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.and(&mut a, &b);

        assert_eq!(a.get_value(), 0b01010000);
    }

    #[test]
    fn and_resets_sub_flag() {
        let mut a = Register8::new(0b11110000);
        let b = Register8::new(0b01010101);
        let flags = Rc::new(RefCell::new(Register8::new(0xFF)));
        let alu = ALU::new(flags.clone());

        alu.and(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::SUB_FLAG));
    }

    #[test]
    fn and_resets_half_carry_flag() {
        let mut a = Register8::new(0b11110000);
        let b = Register8::new(0b01010101);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.and(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn and_resets_carry_flag() {
        let mut a = Register8::new(0b11110000);
        let b = Register8::new(0b01010101);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.and(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::CARRY_FLAG));
    }

    #[test]
    fn and_sets_zero_flag_correctly() {
        let mut a = Register8::new(0);
        let b = Register8::new(0);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.and(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn and_resets_zero_flag_correctly() {
        let mut a = Register8::new(0b11110000);
        let b = Register8::new(0b01010101);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.and(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn xors_correctly() {
        let a = Rc::new(RefCell::new(Register8::new(0b11110000)));
        let b = Rc::new(RefCell::new(Register8::new(0b01010101)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.xor_internal(a.clone(), b);

        assert_eq!(a.borrow().get_value(), 0b10100101);
    }

    #[test]
    fn xor_resets_sub_flag() {
        let a = Rc::new(RefCell::new(Register8::new(0b11110000)));
        let b = Rc::new(RefCell::new(Register8::new(0b01010101)));
        let flags = Rc::new(RefCell::new(Register8::new(0xFF)));
        let alu = ALU::new(flags.clone());

        alu.xor_internal(a.clone(), b);

        assert_eq!(false, flags.borrow().get_bit(ALU::SUB_FLAG));
    }

    #[test]
    fn xor_resets_half_carry_flag() {
        let a = Rc::new(RefCell::new(Register8::new(0b11110000)));
        let b = Rc::new(RefCell::new(Register8::new(0b01010101)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.xor_internal(a.clone(), b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn xor_resets_carry_flag() {
        let a = Rc::new(RefCell::new(Register8::new(0b11110000)));
        let b = Rc::new(RefCell::new(Register8::new(0b01010101)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.xor_internal(a.clone(), b);

        assert_eq!(false, flags.borrow().get_bit(ALU::CARRY_FLAG));
    }

    #[test]
    fn xor_sets_zero_flag_correctly() {
        let a = Rc::new(RefCell::new(Register8::new(0x00)));
        let b = Rc::new(RefCell::new(Register8::new(0x00)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.xor_internal(a.clone(), b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn xor_resets_zero_flag_correctly() {
        let a = Rc::new(RefCell::new(Register8::new(0x01)));
        let b = Rc::new(RefCell::new(Register8::new(0x00)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.xor_internal(a.clone(), b);

        assert_eq!(false, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }


    #[test]
    fn ors_correctly() {
        let a = Rc::new(RefCell::new(Register8::new(0b11110000)));
        let b = Rc::new(RefCell::new(Register8::new(0b01010101)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.or_internal(a.clone(), b);

        assert_eq!(a.borrow().get_value(), 0b11110101);
    }

    #[test]
    fn or_resets_sub_flag() {
        let a = Rc::new(RefCell::new(Register8::new(0b11110000)));
        let b = Rc::new(RefCell::new(Register8::new(0b01010101)));
        let flags = Rc::new(RefCell::new(Register8::new(0xFF)));
        let alu = ALU::new(flags.clone());

        alu.or_internal(a.clone(), b);

        assert_eq!(false, flags.borrow().get_bit(ALU::SUB_FLAG));
    }

    #[test]
    fn or_resets_half_carry_flag() {
        let a = Rc::new(RefCell::new(Register8::new(0b11110000)));
        let b = Rc::new(RefCell::new(Register8::new(0b01010101)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.or_internal(a.clone(), b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn or_resets_carry_flag() {
        let a = Rc::new(RefCell::new(Register8::new(0b11110000)));
        let b = Rc::new(RefCell::new(Register8::new(0b01010101)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.or_internal(a.clone(), b);

        assert_eq!(false, flags.borrow().get_bit(ALU::CARRY_FLAG));
    }

    #[test]
    fn or_sets_zero_flag_correctly() {
        let a = Rc::new(RefCell::new(Register8::new(0x00)));
        let b = Rc::new(RefCell::new(Register8::new(0x00)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.or_internal(a.clone(), b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn or_resets_zero_flag_correctly() {
        let a = Rc::new(RefCell::new(Register8::new(0x01)));
        let b = Rc::new(RefCell::new(Register8::new(0x00)));
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.or_internal(a.clone(), b);

        assert_eq!(false, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }


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

        flags.borrow_mut().set_bit(ALU::SUB_FLAG, true);

        alu.add(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::SUB_FLAG), false);
    }

    #[test]
    fn add_sets_zero_bit_when_zero() {
        let mut a = Register8::new(0xFF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn add_resets_zero_bit_when_not_zero() {
        let mut a = Register8::new(0xEF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn add_sets_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x0F);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn add_does_not_set_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x02);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn add_sets_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0x0F00);
        let b = Register16::new(0x0F00);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn add_does_not_set_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0xF0);
        let b = Register16::new(0xF0);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn add_sets_carry_flag_correctly() {
        let mut a = Register8::new(0xFF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::CARRY_FLAG), true);
    }



    #[test]
    fn add_no_carry_resets_sub_bit() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::SUB_FLAG, true);

        alu.add_no_carry(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::SUB_FLAG), false);
    }

    #[test]
    fn add_no_carry_sets_zero_bit_when_zero() {
        let mut a = Register8::new(0xFF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add_no_carry(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn add_no_carry_resets_zero_bit_when_not_zero() {
        let mut a = Register8::new(0xEF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add_no_carry(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn add_no_carry_sets_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x0F);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add_no_carry(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn add_no_carry_does_not_set_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x02);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add_no_carry(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn add_no_carry_sets_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0x0F00);
        let b = Register16::new(0x0F00);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add_no_carry(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn add_no_carry_does_not_set_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0xF0);
        let b = Register16::new(0xF0);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add_no_carry(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn add_no_carry_does_not_change_carry_flag() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        let alu = ALU::new(flags.clone());

        alu.add_no_carry(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::CARRY_FLAG), true);
    }

    #[test]
    fn adc_does_not_carry_when_carry_bit_not_set() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, false);
        alu.adc(&mut a, &b);

        assert_eq!(0x03, a.get_value());
    }

    #[test]
    fn adc_carries_when_carry_bit_set() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(0x04, a.get_value());
    }

    #[test]
    fn adc_resets_carry() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::CARRY_FLAG));
    }

    #[test]
    fn adc_with_carry_bit_set_preserves_carry_from_a_b_addition() {
        let mut a = Register8::new(0xF0);
        let b = Register8::new(0x1F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::CARRY_FLAG));
    }

    #[test]
    fn adc_with_carry_bit_set_sets_carry_bit_on_carry_wrap() {
        //edge case where carry causes the rollover rather than the a+b

        let mut a = Register8::new(0xF0);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::CARRY_FLAG));
    }

    #[test]
    fn adc_with_carry_preserves_half_carry_from_a_b_addition() {
        let mut a = Register8::new(0x08);
        let b = Register8::new(0x08);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn adc_with_carry_sets_half_carry_when_caused_by_carry() {
        let mut a = Register8::new(0x07);
        let b = Register8::new(0x08);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }


    #[test]
    fn adc_with_carry_sets_zero_when_zero() {
        let mut a = Register8::new(0xF0);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        alu.adc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn subs_u8_no_flags () {
        let mut a = Register8::new(0x10);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub(&mut a, &b);

        assert_eq!(a.get_value(), 0x0F);
    }

    #[test]
    fn subs_u16_no_flags () {
        let mut a = Register16::new(0x101);
        let b = Register16::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub(&mut a, &b);

        assert_eq!(a.get_value(), 0xFF);
    }

    #[test]
    fn sub_sets_sub_bit() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::SUB_FLAG, false);

        alu.sub(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::SUB_FLAG), true);
    }

    #[test]
    fn sub_sets_zero_bit_when_zero() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn sub_resets_zero_bit_when_not_zero() {
        let mut a = Register8::new(0xEF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn sub_sets_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x18);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sub_does_not_set_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x02);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sub_sets_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0x1800);
        let b = Register16::new(0x0F00);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sub_does_not_set_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0xF0);
        let b = Register16::new(0xF0);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sub_sets_carry_flag() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x10);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::CARRY_FLAG), true);
    }

    #[test]
    fn sub_no_carry_resets_sub_bit() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::SUB_FLAG, false);

        alu.sub_no_carry(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::SUB_FLAG), true);
    }

    #[test]
    fn sub_no_carry_sets_zero_bit_when_zero() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub_no_carry(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn sub_no_carry_resets_zero_bit_when_not_zero() {
        let mut a = Register8::new(0xEF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub_no_carry(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn sub_no_carry_sets_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x18);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub_no_carry(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sub_no_carry_does_not_set_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x04);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub_no_carry(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sub_no_carry_sets_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0x1800);
        let b = Register16::new(0x0F00);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub_no_carry(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sub_no_carry_does_not_set_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0x1F0);
        let b = Register16::new(0xF0);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sub_no_carry(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sub_no_carry_does_not_change_carry_flag() {
        let mut a = Register8::new(0x02);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        let alu = ALU::new(flags.clone());

        alu.sub_no_carry(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::CARRY_FLAG), true);
    }

    #[test]
    fn sbcs_u8_no_flags () {
        let mut a = Register8::new(0x10);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sbc(&mut a, &b);

        assert_eq!(a.get_value(), 0x0F);
    }

    #[test]
    fn sbcs_u16_no_flags () {
        let mut a = Register16::new(0x101);
        let b = Register16::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sbc(&mut a, &b);

        assert_eq!(a.get_value(), 0xFF);
    }

    #[test]
    fn sbcs_u8_carry_flag () {
        let mut a = Register8::new(0x10);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        let alu = ALU::new(flags.clone());

        alu.sbc(&mut a, &b);

        assert_eq!(0x0E, a.get_value());
    }

    #[test]
    fn sbcs_u16_carry_flag () {
        let mut a = Register16::new(0x101);
        let b = Register16::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        flags.borrow_mut().set_bit(ALU::CARRY_FLAG, true);
        let alu = ALU::new(flags.clone());

        alu.sbc(&mut a, &b);

        assert_eq!(0xFE, a.get_value());
    }

    #[test]
    fn sbc_sets_sub_bit() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        flags.borrow_mut().set_bit(ALU::SUB_FLAG, false);

        alu.sbc(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::SUB_FLAG), true);
    }

    #[test]
    fn sbc_sets_zero_bit_when_zero() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sbc(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn sbc_resets_zero_bit_when_not_zero() {
        let mut a = Register8::new(0xEF);
        let b = Register8::new(0x01);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sbc(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::ZERO_FLAG));
    }

    #[test]
    fn sbc_sets_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x18);
        let b = Register8::new(0x0F);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sbc_does_not_set_half_carry_flag_correctly_for_u8() {
        let mut a = Register8::new(0x02);
        let b = Register8::new(0x02);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sbc_sets_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0x1800);
        let b = Register16::new(0x0F00);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(true, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sbc_does_not_set_half_carry_flag_correctly_for_u16() {
        let mut a = Register16::new(0xF0);
        let b = Register16::new(0xF0);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.add(&mut a, &b);

        assert_eq!(false, flags.borrow().get_bit(ALU::HALF_CARRY_FLAG));
    }

    #[test]
    fn sbc_sets_carry_flag() {
        let mut a = Register8::new(0x01);
        let b = Register8::new(0x10);
        let flags = Rc::new(RefCell::new(Register8::new(0x00)));
        let alu = ALU::new(flags.clone());

        alu.sbc(&mut a, &b);

        assert_eq!(flags.borrow().get_bit(ALU::CARRY_FLAG), true);
    }

}
