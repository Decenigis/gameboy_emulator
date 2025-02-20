use std::ops::{Add, Shl, Sub};
use crate::cpu::carry_trait::CarryTrait;

pub trait Register: Clone {

    type ValueType: CarryTrait + PartialEq + Add + Sub + Shl<i32>;

    fn zero() -> Self;
    fn one() -> Self;

    fn is_zero(&self) -> bool;

    fn get_affects_zero() -> bool;

    fn set_value(&mut self, value: Self::ValueType);
    fn get_value(&self) -> Self::ValueType;

    fn set_bit(&mut self, bit: u8, value: bool);
    fn get_bit(&self, bit: u8) -> bool;

    fn increment(&mut self);
    fn decrement(&mut self);

    fn wrapping_add(&mut self, other: &Self);
    fn wrapping_sub(&mut self, other: &Self);

}
