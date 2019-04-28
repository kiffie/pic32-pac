#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC3CON register"]
    pub ic3con: IC3CON,
    #[doc = "0x04 - IC3CONCLR register"]
    pub ic3conclr: IC3CONCLR,
    #[doc = "0x08 - IC3CONSET register"]
    pub ic3conset: IC3CONSET,
    #[doc = "0x0c - IC3CONINV register"]
    pub ic3coninv: IC3CONINV,
    #[doc = "0x10 - IC3BUF register"]
    pub ic3buf: IC3BUF,
}
#[doc = "IC3CON register"]
pub struct IC3CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC3CON register"]
pub mod ic3con;
#[doc = "IC3CONCLR register"]
pub struct IC3CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC3CONCLR register"]
pub mod ic3conclr;
#[doc = "IC3CONSET register"]
pub struct IC3CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC3CONSET register"]
pub mod ic3conset;
#[doc = "IC3CONINV register"]
pub struct IC3CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC3CONINV register"]
pub mod ic3coninv;
#[doc = "IC3BUF register"]
pub struct IC3BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC3BUF register"]
pub mod ic3buf;
