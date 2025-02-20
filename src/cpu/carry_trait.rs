pub trait CarryTrait {
    fn add_carries(&self, other: Self) -> bool;
    fn sub_borrows(&self, other: Self) -> bool;

}

impl CarryTrait for u8 {
    fn add_carries(&self, other: Self) -> bool {
        self.checked_add(other).is_none()
    }

    fn sub_borrows(&self, other: Self) -> bool {
        self.checked_sub(other).is_none()
    }
}

impl CarryTrait for u16 {
    fn add_carries(&self, other: Self) -> bool {
        self.checked_add(other).is_none()
    }

    fn sub_borrows(&self, other: Self) -> bool {
        self.checked_sub(other).is_none()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_returns_true_when_add_carries() {
        let a = 0xFFu8;
        let b = 0x10u8;

        assert_eq!(true, a.add_carries(b))
    }

    #[test]
    fn u8_returns_false_when_add_carries() {
        let a = 0x00u8;
        let b = 0x10u8;

        assert_eq!(false, a.add_carries(b))
    }
    #[test]
    fn u16_returns_true_when_add_carries() {
        let a = 0x0000u16;
        let b = 0x100u16;

        assert_eq!(true, a.sub_borrows(b))
    }

    #[test]
    fn u16_returns_false_when_add_carries() {
        let a = 0xFFFFu16;
        let b = 0x100u16;

        assert_eq!(false, a.sub_borrows(b))
    }
}
