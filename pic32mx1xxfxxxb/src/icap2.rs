#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC2CON register"]
    pub ic2con: IC2CON,
    #[doc = "0x04 - IC2CONCLR register"]
    pub ic2conclr: IC2CONCLR,
    #[doc = "0x08 - IC2CONSET register"]
    pub ic2conset: IC2CONSET,
    #[doc = "0x0c - IC2CONINV register"]
    pub ic2coninv: IC2CONINV,
    #[doc = "0x10 - IC2BUF register"]
    pub ic2buf: IC2BUF,
}
#[doc = "IC2CON register"]
pub struct IC2CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC2CON register"]
pub mod ic2con;
#[doc = "IC2CONCLR register"]
pub struct IC2CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC2CONCLR register"]
pub mod ic2conclr;
#[doc = "IC2CONSET register"]
pub struct IC2CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC2CONSET register"]
pub mod ic2conset;
#[doc = "IC2CONINV register"]
pub struct IC2CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC2CONINV register"]
pub mod ic2coninv;
#[doc = "IC2BUF register"]
pub struct IC2BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC2BUF register"]
pub mod ic2buf;
