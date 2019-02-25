#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC5CON register"]
    pub ic5con: IC5CON,
    #[doc = "0x04 - IC5CONCLR register"]
    pub ic5conclr: IC5CONCLR,
    #[doc = "0x08 - IC5CONSET register"]
    pub ic5conset: IC5CONSET,
    #[doc = "0x0c - IC5CONINV register"]
    pub ic5coninv: IC5CONINV,
    #[doc = "0x10 - IC5BUF register"]
    pub ic5buf: IC5BUF,
}
#[doc = "IC5CON register"]
pub struct IC5CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC5CON register"]
pub mod ic5con;
#[doc = "IC5CONCLR register"]
pub struct IC5CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC5CONCLR register"]
pub mod ic5conclr;
#[doc = "IC5CONSET register"]
pub struct IC5CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC5CONSET register"]
pub mod ic5conset;
#[doc = "IC5CONINV register"]
pub struct IC5CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC5CONINV register"]
pub mod ic5coninv;
#[doc = "IC5BUF register"]
pub struct IC5BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC5BUF register"]
pub mod ic5buf;
