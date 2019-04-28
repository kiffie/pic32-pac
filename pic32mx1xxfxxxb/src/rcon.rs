#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCON register"]
    pub rcon: RCON,
    #[doc = "0x04 - RCONCLR register"]
    pub rconclr: RCONCLR,
    #[doc = "0x08 - RCONSET register"]
    pub rconset: RCONSET,
    #[doc = "0x0c - RCONINV register"]
    pub rconinv: RCONINV,
    #[doc = "0x10 - RSWRST register"]
    pub rswrst: RSWRST,
    #[doc = "0x14 - RSWRSTCLR register"]
    pub rswrstclr: RSWRSTCLR,
    #[doc = "0x18 - RSWRSTSET register"]
    pub rswrstset: RSWRSTSET,
    #[doc = "0x1c - RSWRSTINV register"]
    pub rswrstinv: RSWRSTINV,
}
#[doc = "RCON register"]
pub struct RCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCON register"]
pub mod rcon;
#[doc = "RCONCLR register"]
pub struct RCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCONCLR register"]
pub mod rconclr;
#[doc = "RCONSET register"]
pub struct RCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCONSET register"]
pub mod rconset;
#[doc = "RCONINV register"]
pub struct RCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RCONINV register"]
pub mod rconinv;
#[doc = "RSWRST register"]
pub struct RSWRST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSWRST register"]
pub mod rswrst;
#[doc = "RSWRSTCLR register"]
pub struct RSWRSTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSWRSTCLR register"]
pub mod rswrstclr;
#[doc = "RSWRSTSET register"]
pub struct RSWRSTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSWRSTSET register"]
pub mod rswrstset;
#[doc = "RSWRSTINV register"]
pub struct RSWRSTINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RSWRSTINV register"]
pub mod rswrstinv;
