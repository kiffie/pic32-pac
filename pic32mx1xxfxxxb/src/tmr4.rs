#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T4CON register"]
    pub t4con: T4CON,
    #[doc = "0x04 - T4CONCLR register"]
    pub t4conclr: T4CONCLR,
    #[doc = "0x08 - T4CONSET register"]
    pub t4conset: T4CONSET,
    #[doc = "0x0c - T4CONINV register"]
    pub t4coninv: T4CONINV,
    #[doc = "0x10 - TMR4 register"]
    pub tmr4: TMR4,
    #[doc = "0x14 - TMR4CLR register"]
    pub tmr4clr: TMR4CLR,
    #[doc = "0x18 - TMR4SET register"]
    pub tmr4set: TMR4SET,
    #[doc = "0x1c - TMR4INV register"]
    pub tmr4inv: TMR4INV,
    #[doc = "0x20 - PR4 register"]
    pub pr4: PR4,
    #[doc = "0x24 - PR4CLR register"]
    pub pr4clr: PR4CLR,
    #[doc = "0x28 - PR4SET register"]
    pub pr4set: PR4SET,
    #[doc = "0x2c - PR4INV register"]
    pub pr4inv: PR4INV,
}
#[doc = "T4CON register"]
pub struct T4CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T4CON register"]
pub mod t4con;
#[doc = "T4CONCLR register"]
pub struct T4CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T4CONCLR register"]
pub mod t4conclr;
#[doc = "T4CONSET register"]
pub struct T4CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T4CONSET register"]
pub mod t4conset;
#[doc = "T4CONINV register"]
pub struct T4CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T4CONINV register"]
pub mod t4coninv;
#[doc = "TMR4 register"]
pub struct TMR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR4 register"]
pub mod tmr4;
#[doc = "TMR4CLR register"]
pub struct TMR4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR4CLR register"]
pub mod tmr4clr;
#[doc = "TMR4SET register"]
pub struct TMR4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR4SET register"]
pub mod tmr4set;
#[doc = "TMR4INV register"]
pub struct TMR4INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR4INV register"]
pub mod tmr4inv;
#[doc = "PR4 register"]
pub struct PR4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR4 register"]
pub mod pr4;
#[doc = "PR4CLR register"]
pub struct PR4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR4CLR register"]
pub mod pr4clr;
#[doc = "PR4SET register"]
pub struct PR4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR4SET register"]
pub mod pr4set;
#[doc = "PR4INV register"]
pub struct PR4INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR4INV register"]
pub mod pr4inv;
