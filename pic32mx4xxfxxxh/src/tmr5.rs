#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T5CON register"]
    pub t5con: T5CON,
    #[doc = "0x04 - T5CONCLR register"]
    pub t5conclr: T5CONCLR,
    #[doc = "0x08 - T5CONSET register"]
    pub t5conset: T5CONSET,
    #[doc = "0x0c - T5CONINV register"]
    pub t5coninv: T5CONINV,
    #[doc = "0x10 - TMR5 register"]
    pub tmr5: TMR5,
    #[doc = "0x14 - TMR5CLR register"]
    pub tmr5clr: TMR5CLR,
    #[doc = "0x18 - TMR5SET register"]
    pub tmr5set: TMR5SET,
    #[doc = "0x1c - TMR5INV register"]
    pub tmr5inv: TMR5INV,
    #[doc = "0x20 - PR5 register"]
    pub pr5: PR5,
    #[doc = "0x24 - PR5CLR register"]
    pub pr5clr: PR5CLR,
    #[doc = "0x28 - PR5SET register"]
    pub pr5set: PR5SET,
    #[doc = "0x2c - PR5INV register"]
    pub pr5inv: PR5INV,
}
#[doc = "T5CON register"]
pub struct T5CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T5CON register"]
pub mod t5con;
#[doc = "T5CONCLR register"]
pub struct T5CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T5CONCLR register"]
pub mod t5conclr;
#[doc = "T5CONSET register"]
pub struct T5CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T5CONSET register"]
pub mod t5conset;
#[doc = "T5CONINV register"]
pub struct T5CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T5CONINV register"]
pub mod t5coninv;
#[doc = "TMR5 register"]
pub struct TMR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR5 register"]
pub mod tmr5;
#[doc = "TMR5CLR register"]
pub struct TMR5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR5CLR register"]
pub mod tmr5clr;
#[doc = "TMR5SET register"]
pub struct TMR5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR5SET register"]
pub mod tmr5set;
#[doc = "TMR5INV register"]
pub struct TMR5INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR5INV register"]
pub mod tmr5inv;
#[doc = "PR5 register"]
pub struct PR5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR5 register"]
pub mod pr5;
#[doc = "PR5CLR register"]
pub struct PR5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR5CLR register"]
pub mod pr5clr;
#[doc = "PR5SET register"]
pub struct PR5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR5SET register"]
pub mod pr5set;
#[doc = "PR5INV register"]
pub struct PR5INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR5INV register"]
pub mod pr5inv;
