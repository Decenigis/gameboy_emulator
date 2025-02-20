use std::cell::RefCell;
use std::rc::Rc;
use crate::cpu::register16::Register16;
use crate::cpu::register8::Register8;

pub struct Registers {
    pub a: Rc<RefCell<Register8>>,
    pub f: Rc<RefCell<Register8>>,
    pub af: Register16,

    pub b: Rc<RefCell<Register8>>,
    pub c: Rc<RefCell<Register8>>,
    pub bc: Register16,

    pub d: Rc<RefCell<Register8>>,
    pub e: Rc<RefCell<Register8>>,
    pub de: Register16,

    pub h: Rc<RefCell<Register8>>,
    pub l: Rc<RefCell<Register8>>,
    pub hl: Register16,

    pub pc: Register16,
    pub sp: Register16,
}

impl Registers {
    pub fn new(
        af_val: u16,
        bc_val: u16,
        de_val: u16,
        hl_val: u16,
        pc_val: u16,
        sp_val: u16
    ) -> Self {
        let af = Register16::new(af_val);
        let bc = Register16::new(bc_val);
        let de = Register16::new(de_val);
        let hl = Register16::new(hl_val);
        let pc = Register16::new(pc_val);
        let sp = Register16::new(sp_val);

        Self {
            a: af.h.clone(),
            f: af.l.clone(),
            af,

            b: bc.h.clone(),
            c: bc.l.clone(),
            bc,

            d: de.h.clone(),
            e: de.l.clone(),
            de,

            h: hl.h.clone(),
            l: hl.l.clone(),
            hl,

            pc,
            sp,
        }
    }
}
