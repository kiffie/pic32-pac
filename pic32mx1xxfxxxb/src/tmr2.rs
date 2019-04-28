#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T2CON register"]
    pub t2con: T2CON,
    #[doc = "0x04 - T2CONCLR register"]
    pub t2conclr: T2CONCLR,
    #[doc = "0x08 - T2CONSET register"]
    pub t2conset: T2CONSET,
    #[doc = "0x0c - T2CONINV register"]
    pub t2coninv: T2CONINV,
    #[doc = "0x10 - TMR2 register"]
    pub tmr2: TMR2,
    #[doc = "0x14 - TMR2CLR register"]
    pub tmr2clr: TMR2CLR,
    #[doc = "0x18 - TMR2SET register"]
    pub tmr2set: TMR2SET,
    #[doc = "0x1c - TMR2INV register"]
    pub tmr2inv: TMR2INV,
    #[doc = "0x20 - PR2 register"]
    pub pr2: PR2,
    #[doc = "0x24 - PR2CLR register"]
    pub pr2clr: PR2CLR,
    #[doc = "0x28 - PR2SET register"]
    pub pr2set: PR2SET,
    #[doc = "0x2c - PR2INV register"]
    pub pr2inv: PR2INV,
}
#[doc = "T2CON register"]
pub struct T2CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T2CON register"]
pub mod t2con;
#[doc = "T2CONCLR register"]
pub struct T2CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T2CONCLR register"]
pub mod t2conclr;
#[doc = "T2CONSET register"]
pub struct T2CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T2CONSET register"]
pub mod t2conset;
#[doc = "T2CONINV register"]
pub struct T2CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T2CONINV register"]
pub mod t2coninv;
#[doc = "TMR2 register"]
pub struct TMR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR2 register"]
pub mod tmr2;
#[doc = "TMR2CLR register"]
pub struct TMR2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR2CLR register"]
pub mod tmr2clr;
#[doc = "TMR2SET register"]
pub struct TMR2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR2SET register"]
pub mod tmr2set;
#[doc = "TMR2INV register"]
pub struct TMR2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR2INV register"]
pub mod tmr2inv;
#[doc = "PR2 register"]
pub struct PR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR2 register"]
pub mod pr2;
#[doc = "PR2CLR register"]
pub struct PR2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR2CLR register"]
pub mod pr2clr;
#[doc = "PR2SET register"]
pub struct PR2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR2SET register"]
pub mod pr2set;
#[doc = "PR2INV register"]
pub struct PR2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR2INV register"]
pub mod pr2inv;
