#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T3CON register"]
    pub t3con: T3CON,
    #[doc = "0x04 - T3CONCLR register"]
    pub t3conclr: T3CONCLR,
    #[doc = "0x08 - T3CONSET register"]
    pub t3conset: T3CONSET,
    #[doc = "0x0c - T3CONINV register"]
    pub t3coninv: T3CONINV,
    #[doc = "0x10 - TMR3 register"]
    pub tmr3: TMR3,
    #[doc = "0x14 - TMR3CLR register"]
    pub tmr3clr: TMR3CLR,
    #[doc = "0x18 - TMR3SET register"]
    pub tmr3set: TMR3SET,
    #[doc = "0x1c - TMR3INV register"]
    pub tmr3inv: TMR3INV,
    #[doc = "0x20 - PR3 register"]
    pub pr3: PR3,
    #[doc = "0x24 - PR3CLR register"]
    pub pr3clr: PR3CLR,
    #[doc = "0x28 - PR3SET register"]
    pub pr3set: PR3SET,
    #[doc = "0x2c - PR3INV register"]
    pub pr3inv: PR3INV,
}
#[doc = "T3CON register"]
pub struct T3CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T3CON register"]
pub mod t3con;
#[doc = "T3CONCLR register"]
pub struct T3CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T3CONCLR register"]
pub mod t3conclr;
#[doc = "T3CONSET register"]
pub struct T3CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T3CONSET register"]
pub mod t3conset;
#[doc = "T3CONINV register"]
pub struct T3CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T3CONINV register"]
pub mod t3coninv;
#[doc = "TMR3 register"]
pub struct TMR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR3 register"]
pub mod tmr3;
#[doc = "TMR3CLR register"]
pub struct TMR3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR3CLR register"]
pub mod tmr3clr;
#[doc = "TMR3SET register"]
pub struct TMR3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR3SET register"]
pub mod tmr3set;
#[doc = "TMR3INV register"]
pub struct TMR3INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR3INV register"]
pub mod tmr3inv;
#[doc = "PR3 register"]
pub struct PR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR3 register"]
pub mod pr3;
#[doc = "PR3CLR register"]
pub struct PR3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR3CLR register"]
pub mod pr3clr;
#[doc = "PR3SET register"]
pub struct PR3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR3SET register"]
pub mod pr3set;
#[doc = "PR3INV register"]
pub struct PR3INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR3INV register"]
pub mod pr3inv;
