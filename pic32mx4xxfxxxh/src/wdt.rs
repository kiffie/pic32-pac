#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDTCON register"]
    pub wdtcon: WDTCON,
    #[doc = "0x04 - WDTCONCLR register"]
    pub wdtconclr: WDTCONCLR,
    #[doc = "0x08 - WDTCONSET register"]
    pub wdtconset: WDTCONSET,
    #[doc = "0x0c - WDTCONINV register"]
    pub wdtconinv: WDTCONINV,
}
#[doc = "WDTCON register"]
pub struct WDTCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDTCON register"]
pub mod wdtcon;
#[doc = "WDTCONCLR register"]
pub struct WDTCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDTCONCLR register"]
pub mod wdtconclr;
#[doc = "WDTCONSET register"]
pub struct WDTCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDTCONSET register"]
pub mod wdtconset;
#[doc = "WDTCONINV register"]
pub struct WDTCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "WDTCONINV register"]
pub mod wdtconinv;
