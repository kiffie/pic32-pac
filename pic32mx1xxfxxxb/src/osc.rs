#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSCCON register"]
    pub osccon: OSCCON,
    #[doc = "0x04 - OSCCONCLR register"]
    pub oscconclr: OSCCONCLR,
    #[doc = "0x08 - OSCCONSET register"]
    pub oscconset: OSCCONSET,
    #[doc = "0x0c - OSCCONINV register"]
    pub oscconinv: OSCCONINV,
    #[doc = "0x10 - OSCTUN register"]
    pub osctun: OSCTUN,
    #[doc = "0x14 - OSCTUNCLR register"]
    pub osctunclr: OSCTUNCLR,
    #[doc = "0x18 - OSCTUNSET register"]
    pub osctunset: OSCTUNSET,
    #[doc = "0x1c - OSCTUNINV register"]
    pub osctuninv: OSCTUNINV,
    #[doc = "0x20 - REFOCON register"]
    pub refocon: REFOCON,
    #[doc = "0x24 - REFOCONCLR register"]
    pub refoconclr: REFOCONCLR,
    #[doc = "0x28 - REFOCONSET register"]
    pub refoconset: REFOCONSET,
    #[doc = "0x2c - REFOCONINV register"]
    pub refoconinv: REFOCONINV,
    #[doc = "0x30 - REFOTRIM register"]
    pub refotrim: REFOTRIM,
    #[doc = "0x34 - REFOTRIMCLR register"]
    pub refotrimclr: REFOTRIMCLR,
    #[doc = "0x38 - REFOTRIMSET register"]
    pub refotrimset: REFOTRIMSET,
    #[doc = "0x3c - REFOTRIMINV register"]
    pub refotriminv: REFOTRIMINV,
}
#[doc = "OSCCON register"]
pub struct OSCCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSCCON register"]
pub mod osccon;
#[doc = "OSCCONCLR register"]
pub struct OSCCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSCCONCLR register"]
pub mod oscconclr;
#[doc = "OSCCONSET register"]
pub struct OSCCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSCCONSET register"]
pub mod oscconset;
#[doc = "OSCCONINV register"]
pub struct OSCCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSCCONINV register"]
pub mod oscconinv;
#[doc = "OSCTUN register"]
pub struct OSCTUN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSCTUN register"]
pub mod osctun;
#[doc = "OSCTUNCLR register"]
pub struct OSCTUNCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSCTUNCLR register"]
pub mod osctunclr;
#[doc = "OSCTUNSET register"]
pub struct OSCTUNSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSCTUNSET register"]
pub mod osctunset;
#[doc = "OSCTUNINV register"]
pub struct OSCTUNINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OSCTUNINV register"]
pub mod osctuninv;
#[doc = "REFOCON register"]
pub struct REFOCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFOCON register"]
pub mod refocon;
#[doc = "REFOCONCLR register"]
pub struct REFOCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFOCONCLR register"]
pub mod refoconclr;
#[doc = "REFOCONSET register"]
pub struct REFOCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFOCONSET register"]
pub mod refoconset;
#[doc = "REFOCONINV register"]
pub struct REFOCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFOCONINV register"]
pub mod refoconinv;
#[doc = "REFOTRIM register"]
pub struct REFOTRIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFOTRIM register"]
pub mod refotrim;
#[doc = "REFOTRIMCLR register"]
pub struct REFOTRIMCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFOTRIMCLR register"]
pub mod refotrimclr;
#[doc = "REFOTRIMSET register"]
pub struct REFOTRIMSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFOTRIMSET register"]
pub mod refotrimset;
#[doc = "REFOTRIMINV register"]
pub struct REFOTRIMINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFOTRIMINV register"]
pub mod refotriminv;
