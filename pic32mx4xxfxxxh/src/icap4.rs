#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC4CON register"]
    pub ic4con: IC4CON,
    #[doc = "0x04 - IC4CONCLR register"]
    pub ic4conclr: IC4CONCLR,
    #[doc = "0x08 - IC4CONSET register"]
    pub ic4conset: IC4CONSET,
    #[doc = "0x0c - IC4CONINV register"]
    pub ic4coninv: IC4CONINV,
    #[doc = "0x10 - IC4BUF register"]
    pub ic4buf: IC4BUF,
}
#[doc = "IC4CON register"]
pub struct IC4CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC4CON register"]
pub mod ic4con;
#[doc = "IC4CONCLR register"]
pub struct IC4CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC4CONCLR register"]
pub mod ic4conclr;
#[doc = "IC4CONSET register"]
pub struct IC4CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC4CONSET register"]
pub mod ic4conset;
#[doc = "IC4CONINV register"]
pub struct IC4CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC4CONINV register"]
pub mod ic4coninv;
#[doc = "IC4BUF register"]
pub struct IC4BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC4BUF register"]
pub mod ic4buf;
