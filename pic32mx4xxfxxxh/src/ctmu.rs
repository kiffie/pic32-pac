#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTMUCON register"]
    pub ctmucon: CTMUCON,
    #[doc = "0x04 - CTMUCONCLR register"]
    pub ctmuconclr: CTMUCONCLR,
    #[doc = "0x08 - CTMUCONSET register"]
    pub ctmuconset: CTMUCONSET,
    #[doc = "0x0c - CTMUCONINV register"]
    pub ctmuconinv: CTMUCONINV,
}
#[doc = "CTMUCON register"]
pub struct CTMUCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTMUCON register"]
pub mod ctmucon;
#[doc = "CTMUCONCLR register"]
pub struct CTMUCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTMUCONCLR register"]
pub mod ctmuconclr;
#[doc = "CTMUCONSET register"]
pub struct CTMUCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTMUCONSET register"]
pub mod ctmuconset;
#[doc = "CTMUCONINV register"]
pub struct CTMUCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CTMUCONINV register"]
pub mod ctmuconinv;
