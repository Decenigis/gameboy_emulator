use crate::cpu::register::Register;

#[derive(Clone)]
pub struct Register8 {
    value: u8,
    is_flags: bool,
}

impl Register for Register8 {
    type ValueType = u8;

    fn zero() -> Self {
        Self { value: 0, is_flags: false }
    }

    fn one() -> Self {
        Self { value: 1, is_flags: false }
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn get_affects_zero() -> bool {
        true
    }


    fn set_value(&mut self, value: Self::ValueType) {
        self.value = value;

        if self.is_flags {
            self.value = self.value & 0xF0;
        }
    }

    fn get_value(&self) -> Self::ValueType {
        self.value
    }


    fn set_bit(&mut self, bit: u8, value: bool) { //use of wrapping_shl to prevent panics
        if value {
            self.value |= (1 as Self::ValueType).wrapping_shl(bit as u32);
        } else {
            self.value &= !(1 as Self::ValueType).wrapping_shl(bit as u32);
        }
    }

    fn get_bit(&self, bit: u8) -> bool {
        (self.value & (1 as Self::ValueType).wrapping_shl(bit as u32)) != 0
    }


    fn increment(&mut self) {
        self.value = self.value.wrapping_add(1);
    }

    fn decrement(&mut self) {
        self.value = self.value.wrapping_sub(1);
    }


    fn wrapping_add(&mut self, other: &Self) {
        self.value = self.value.wrapping_add(other.value);
    }

    fn wrapping_sub(&mut self, other: &Self) {
        self.value = self.value.wrapping_sub(other.value);
    }
}

impl Register8 {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            is_flags: false
        }
    }

    pub fn set_is_flags(&mut self) {
        self.is_flags = true;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_returns_zero() {
        assert_eq!(0, Register8::zero().get_value());
    }

    #[test]
    fn one_returns_one() {
        assert_eq!(1, Register8::one().get_value());
    }

    #[test]
    fn is_zero_true_when_zero() {
        assert_eq!(true, Register8::zero().is_zero())
    }

    #[test]
    fn is_zero_false_when_one() {
        assert_eq!(false, Register8::one().is_zero())
    }

    #[test]
    fn sets_and_gets_value() {
        let expected_value = 0x12;
        let mut register = Register8::zero();

        register.set_value(expected_value);

        assert_eq!(expected_value, register.get_value());
    }

    #[test]
    fn sets_bit() {
        let mut register = Register8::zero();

        register.set_bit(4, true);

        assert_eq!(0x10, register.get_value())
    }

    #[test]
    fn resets_bit() {
        let mut register = Register8::new(0x10);

        register.set_bit(4, false);

        assert_eq!(0x00, register.get_value())
    }

    #[test]
    fn gets_bit() {
        let register = Register8::new(0x10);

        assert_eq!(false, register.get_bit(0));
        assert_eq!(true, register.get_bit(4));
    }

    #[test]
    fn increments() {
        let mut register = Register8::new(0x0F);

        register.increment();

        assert_eq!(0x10, register.get_value());
    }

    #[test]
    fn increment_wraps() {
        let mut register = Register8::new(0xFF);

        register.increment();

        assert_eq!(0x00, register.get_value());
    }

    #[test]
    fn decrements() {
        let mut register = Register8::new(0x10);

        register.decrement();

        assert_eq!(0x0F, register.get_value());
    }

    #[test]
    fn decrement_wraps() {
        let mut register = Register8::new(0x00);

        register.decrement();

        assert_eq!(0xFF, register.get_value());
    }

    #[test]
    fn wrapping_add() {
        let mut a = Register8::new(0xF0);
        let b = Register8::new(0x1F);

        a.wrapping_add(&b);

        assert_eq!(0x0F, a.get_value());
    }

    #[test]
    fn wrapping_sub() {
        let mut a = Register8::new(0x0F);
        let b = Register8::new(0x1F);

        a.wrapping_sub(&b);

        assert_eq!(0xF0, a.get_value());
    }
    
    #[test]
    fn if_is_flags_ignore_lower_nibble() {
        let mut register = Register8::new(0x00);
        register.set_is_flags();
        
        register.set_value(0xFF);

        assert_eq!(0xF0, register.get_value());
    }
}
