#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC1CON register"]
    pub ic1con: IC1CON,
    #[doc = "0x04 - IC1CONCLR register"]
    pub ic1conclr: IC1CONCLR,
    #[doc = "0x08 - IC1CONSET register"]
    pub ic1conset: IC1CONSET,
    #[doc = "0x0c - IC1CONINV register"]
    pub ic1coninv: IC1CONINV,
    #[doc = "0x10 - IC1BUF register"]
    pub ic1buf: IC1BUF,
}
#[doc = "IC1CON register"]
pub struct IC1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC1CON register"]
pub mod ic1con;
#[doc = "IC1CONCLR register"]
pub struct IC1CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC1CONCLR register"]
pub mod ic1conclr;
#[doc = "IC1CONSET register"]
pub struct IC1CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC1CONSET register"]
pub mod ic1conset;
#[doc = "IC1CONINV register"]
pub struct IC1CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC1CONINV register"]
pub mod ic1coninv;
#[doc = "IC1BUF register"]
pub struct IC1BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC1BUF register"]
pub mod ic1buf;
