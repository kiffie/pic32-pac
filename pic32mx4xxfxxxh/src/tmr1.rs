#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T1CON register"]
    pub t1con: T1CON,
    #[doc = "0x04 - T1CONCLR register"]
    pub t1conclr: T1CONCLR,
    #[doc = "0x08 - T1CONSET register"]
    pub t1conset: T1CONSET,
    #[doc = "0x0c - T1CONINV register"]
    pub t1coninv: T1CONINV,
    #[doc = "0x10 - TMR1 register"]
    pub tmr1: TMR1,
    #[doc = "0x14 - TMR1CLR register"]
    pub tmr1clr: TMR1CLR,
    #[doc = "0x18 - TMR1SET register"]
    pub tmr1set: TMR1SET,
    #[doc = "0x1c - TMR1INV register"]
    pub tmr1inv: TMR1INV,
    #[doc = "0x20 - PR1 register"]
    pub pr1: PR1,
    #[doc = "0x24 - PR1CLR register"]
    pub pr1clr: PR1CLR,
    #[doc = "0x28 - PR1SET register"]
    pub pr1set: PR1SET,
    #[doc = "0x2c - PR1INV register"]
    pub pr1inv: PR1INV,
}
#[doc = "T1CON register"]
pub struct T1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T1CON register"]
pub mod t1con;
#[doc = "T1CONCLR register"]
pub struct T1CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T1CONCLR register"]
pub mod t1conclr;
#[doc = "T1CONSET register"]
pub struct T1CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T1CONSET register"]
pub mod t1conset;
#[doc = "T1CONINV register"]
pub struct T1CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T1CONINV register"]
pub mod t1coninv;
#[doc = "TMR1 register"]
pub struct TMR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR1 register"]
pub mod tmr1;
#[doc = "TMR1CLR register"]
pub struct TMR1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR1CLR register"]
pub mod tmr1clr;
#[doc = "TMR1SET register"]
pub struct TMR1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR1SET register"]
pub mod tmr1set;
#[doc = "TMR1INV register"]
pub struct TMR1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TMR1INV register"]
pub mod tmr1inv;
#[doc = "PR1 register"]
pub struct PR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR1 register"]
pub mod pr1;
#[doc = "PR1CLR register"]
pub struct PR1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR1CLR register"]
pub mod pr1clr;
#[doc = "PR1SET register"]
pub struct PR1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR1SET register"]
pub mod pr1set;
#[doc = "PR1INV register"]
pub struct PR1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PR1INV register"]
pub mod pr1inv;
