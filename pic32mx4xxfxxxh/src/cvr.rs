#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CVRCON register"]
    pub cvrcon: CVRCON,
    #[doc = "0x04 - CVRCONCLR register"]
    pub cvrconclr: CVRCONCLR,
    #[doc = "0x08 - CVRCONSET register"]
    pub cvrconset: CVRCONSET,
    #[doc = "0x0c - CVRCONINV register"]
    pub cvrconinv: CVRCONINV,
}
#[doc = "CVRCON register"]
pub struct CVRCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CVRCON register"]
pub mod cvrcon;
#[doc = "CVRCONCLR register"]
pub struct CVRCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CVRCONCLR register"]
pub mod cvrconclr;
#[doc = "CVRCONSET register"]
pub struct CVRCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CVRCONSET register"]
pub mod cvrconset;
#[doc = "CVRCONINV register"]
pub struct CVRCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CVRCONINV register"]
pub mod cvrconinv;
