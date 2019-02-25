#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM1CON register"]
    pub cm1con: CM1CON,
    #[doc = "0x04 - CM1CONCLR register"]
    pub cm1conclr: CM1CONCLR,
    #[doc = "0x08 - CM1CONSET register"]
    pub cm1conset: CM1CONSET,
    #[doc = "0x0c - CM1CONINV register"]
    pub cm1coninv: CM1CONINV,
}
#[doc = "CM1CON register"]
pub struct CM1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM1CON register"]
pub mod cm1con;
#[doc = "CM1CONCLR register"]
pub struct CM1CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM1CONCLR register"]
pub mod cm1conclr;
#[doc = "CM1CONSET register"]
pub struct CM1CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM1CONSET register"]
pub mod cm1conset;
#[doc = "CM1CONINV register"]
pub struct CM1CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM1CONINV register"]
pub mod cm1coninv;
