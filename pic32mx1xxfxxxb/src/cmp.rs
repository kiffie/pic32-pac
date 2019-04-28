#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMSTAT register"]
    pub cmstat: CMSTAT,
    #[doc = "0x04 - CMSTATCLR register"]
    pub cmstatclr: CMSTATCLR,
    #[doc = "0x08 - CMSTATSET register"]
    pub cmstatset: CMSTATSET,
    #[doc = "0x0c - CMSTATINV register"]
    pub cmstatinv: CMSTATINV,
}
#[doc = "CMSTAT register"]
pub struct CMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMSTAT register"]
pub mod cmstat;
#[doc = "CMSTATCLR register"]
pub struct CMSTATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMSTATCLR register"]
pub mod cmstatclr;
#[doc = "CMSTATSET register"]
pub struct CMSTATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMSTATSET register"]
pub mod cmstatset;
#[doc = "CMSTATINV register"]
pub struct CMSTATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CMSTATINV register"]
pub mod cmstatinv;
