use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu::register8::Register8;
use crate::cpu::register::Register;

#[derive(Clone)]
pub struct Register16 {
    pub h: Rc<RefCell<Register8>>, //Public as these can be treated as seperate registers
    pub l: Rc<RefCell<Register8>>
}


impl Register for Register16 {
    type ValueType = u16;


    fn zero() -> Self {
        Self::new(0)
    }

    fn one() -> Self {
        Self::new(1)
    }

    fn is_zero(&self) -> bool {
        self.h.borrow().is_zero() && self.l.borrow().is_zero()
    }

    fn get_affects_zero() -> bool {
        false
    }

    fn set_value(&mut self, value: Self::ValueType) {
        self.h.borrow_mut().set_value(((value & 0xFF00) >> 8) as u8);
        self.l.borrow_mut().set_value((value & 0x00FF) as u8);
    }

    fn get_value(&self) -> Self::ValueType {
        ((self.h.borrow().get_value() as u16) << 8) | self.l.borrow().get_value() as u16
    }

    fn set_bit(&mut self, bit: u8, value: bool) { //use of wrapping_shl to prevent panics
        if value {
            self.set_value(self.get_value() | (1 as Self::ValueType).wrapping_shl(bit as u32));
        } else {
            self.set_value(self.get_value() & !(1 as Self::ValueType).wrapping_shl(bit as u32));
        }
    }

    fn get_bit(&self, bit: u8) -> bool {
        (self.get_value() & (1 as Self::ValueType).wrapping_shl(bit as u32)) != 0
    }

    fn increment(&mut self) {
        self.set_value(self.get_value().wrapping_add(1));
    }

    fn decrement(&mut self) {
        self.set_value(self.get_value().wrapping_sub(1));
    }


    fn wrapping_add(&mut self, other: &Self) {
        self.set_value(self.get_value().wrapping_add(other.get_value()));
    }

    fn wrapping_sub(&mut self, other: &Self) {
        self.set_value(self.get_value().wrapping_sub(other.get_value()));
    }
}

impl Register16 {
    pub fn new(value: u16) -> Self {
        Self {
            h: Rc::new(RefCell::new(Register8::new(((value & 0xFF00) >> 8) as u8))),
            l: Rc::new(RefCell::new(Register8::new((value & 0x00FF) as u8)))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_returns_zero() {
        assert_eq!(0, Register16::zero().get_value());
    }

    #[test]
    fn one_returns_one() {
        assert_eq!(1, Register16::one().get_value());
    }

    #[test]
    fn is_zero_true_when_zero() {
        assert_eq!(true, Register16::zero().is_zero())
    }

    #[test]
    fn is_zero_false_when_one() {
        assert_eq!(false, Register16::one().is_zero())
    }

    #[test]
    fn sets_and_gets_value() {
        let expected_value = 0x1234;
        let mut register = Register16::zero();

        register.set_value(expected_value);

        assert_eq!(expected_value, register.get_value());
    }

    #[test]
    fn sets_bit() {
        let mut register = Register16::zero();

        register.set_bit(4, true);
        register.set_bit(8, true);

        assert_eq!(0x110, register.get_value())
    }

    #[test]
    fn resets_bit() {
        let mut register = Register16::new(0x110);

        register.set_bit(8, false);
        register.set_bit(4, false);

        assert_eq!(0x00, register.get_value())
    }

    #[test]
    fn gets_bit() {
        let register = Register16::new(0x110);

        assert_eq!(false, register.get_bit(0));
        assert_eq!(false, register.get_bit(12));
        assert_eq!(true, register.get_bit(4));
        assert_eq!(true, register.get_bit(8));
    }

    #[test]
    fn increments() {
        let mut register = Register16::new(0xFF);

        register.increment();

        assert_eq!(0x100, register.get_value());
    }

    #[test]
    fn increment_wraps() {
        let mut register = Register16::new(0xFFFF);

        register.increment();

        assert_eq!(0x00, register.get_value());
    }

    #[test]
    fn decrements() {
        let mut register = Register16::new(0x100);

        register.decrement();

        assert_eq!(0xFF, register.get_value());
    }

    #[test]
    fn decrement_wraps() {
        let mut register = Register16::new(0x00);

        register.decrement();

        assert_eq!(0xFFFF, register.get_value());
    }

    #[test]
    fn wrapping_add() {
        let mut a = Register16::new(0xFF00);
        let b = Register16::new(0x1FF);

        a.wrapping_add(&b);

        assert_eq!(0xFF, a.get_value());
    }

    #[test]
    fn wrapping_sub() {
        let mut a = Register16::new(0xFF);
        let b = Register16::new(0x1FF);

        a.wrapping_sub(&b);

        assert_eq!(0xFF00, a.get_value());
    }
}
