#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM2CON register"]
    pub cm2con: CM2CON,
    #[doc = "0x04 - CM2CONCLR register"]
    pub cm2conclr: CM2CONCLR,
    #[doc = "0x08 - CM2CONSET register"]
    pub cm2conset: CM2CONSET,
    #[doc = "0x0c - CM2CONINV register"]
    pub cm2coninv: CM2CONINV,
}
#[doc = "CM2CON register"]
pub struct CM2CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM2CON register"]
pub mod cm2con;
#[doc = "CM2CONCLR register"]
pub struct CM2CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM2CONCLR register"]
pub mod cm2conclr;
#[doc = "CM2CONSET register"]
pub struct CM2CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM2CONSET register"]
pub mod cm2conset;
#[doc = "CM2CONINV register"]
pub struct CM2CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM2CONINV register"]
pub mod cm2coninv;
