#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMCON register"]
    pub pmcon: PMCON,
    #[doc = "0x04 - PMCONCLR register"]
    pub pmconclr: PMCONCLR,
    #[doc = "0x08 - PMCONSET register"]
    pub pmconset: PMCONSET,
    #[doc = "0x0c - PMCONINV register"]
    pub pmconinv: PMCONINV,
    #[doc = "0x10 - PMMODE register"]
    pub pmmode: PMMODE,
    #[doc = "0x14 - PMMODECLR register"]
    pub pmmodeclr: PMMODECLR,
    #[doc = "0x18 - PMMODESET register"]
    pub pmmodeset: PMMODESET,
    #[doc = "0x1c - PMMODEINV register"]
    pub pmmodeinv: PMMODEINV,
    #[doc = "0x20 - PMADDR register"]
    pub pmaddr: PMADDR,
    #[doc = "0x24 - PMADDRCLR register"]
    pub pmaddrclr: PMADDRCLR,
    #[doc = "0x28 - PMADDRSET register"]
    pub pmaddrset: PMADDRSET,
    #[doc = "0x2c - PMADDRINV register"]
    pub pmaddrinv: PMADDRINV,
    #[doc = "0x30 - PMDOUT register"]
    pub pmdout: PMDOUT,
    #[doc = "0x34 - PMDOUTCLR register"]
    pub pmdoutclr: PMDOUTCLR,
    #[doc = "0x38 - PMDOUTSET register"]
    pub pmdoutset: PMDOUTSET,
    #[doc = "0x3c - PMDOUTINV register"]
    pub pmdoutinv: PMDOUTINV,
    #[doc = "0x40 - PMDIN register"]
    pub pmdin: PMDIN,
    #[doc = "0x44 - PMDINCLR register"]
    pub pmdinclr: PMDINCLR,
    #[doc = "0x48 - PMDINSET register"]
    pub pmdinset: PMDINSET,
    #[doc = "0x4c - PMDININV register"]
    pub pmdininv: PMDININV,
    #[doc = "0x50 - PMAEN register"]
    pub pmaen: PMAEN,
    #[doc = "0x54 - PMAENCLR register"]
    pub pmaenclr: PMAENCLR,
    #[doc = "0x58 - PMAENSET register"]
    pub pmaenset: PMAENSET,
    #[doc = "0x5c - PMAENINV register"]
    pub pmaeninv: PMAENINV,
    #[doc = "0x60 - PMSTAT register"]
    pub pmstat: PMSTAT,
    #[doc = "0x64 - PMSTATCLR register"]
    pub pmstatclr: PMSTATCLR,
    #[doc = "0x68 - PMSTATSET register"]
    pub pmstatset: PMSTATSET,
    #[doc = "0x6c - PMSTATINV register"]
    pub pmstatinv: PMSTATINV,
}
#[doc = "PMCON register"]
pub struct PMCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMCON register"]
pub mod pmcon;
#[doc = "PMCONCLR register"]
pub struct PMCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMCONCLR register"]
pub mod pmconclr;
#[doc = "PMCONSET register"]
pub struct PMCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMCONSET register"]
pub mod pmconset;
#[doc = "PMCONINV register"]
pub struct PMCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMCONINV register"]
pub mod pmconinv;
#[doc = "PMMODE register"]
pub struct PMMODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMMODE register"]
pub mod pmmode;
#[doc = "PMMODECLR register"]
pub struct PMMODECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMMODECLR register"]
pub mod pmmodeclr;
#[doc = "PMMODESET register"]
pub struct PMMODESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMMODESET register"]
pub mod pmmodeset;
#[doc = "PMMODEINV register"]
pub struct PMMODEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMMODEINV register"]
pub mod pmmodeinv;
#[doc = "PMADDR register"]
pub struct PMADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMADDR register"]
pub mod pmaddr;
#[doc = "PMADDRCLR register"]
pub struct PMADDRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMADDRCLR register"]
pub mod pmaddrclr;
#[doc = "PMADDRSET register"]
pub struct PMADDRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMADDRSET register"]
pub mod pmaddrset;
#[doc = "PMADDRINV register"]
pub struct PMADDRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMADDRINV register"]
pub mod pmaddrinv;
#[doc = "PMDOUT register"]
pub struct PMDOUT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMDOUT register"]
pub mod pmdout;
#[doc = "PMDOUTCLR register"]
pub struct PMDOUTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMDOUTCLR register"]
pub mod pmdoutclr;
#[doc = "PMDOUTSET register"]
pub struct PMDOUTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMDOUTSET register"]
pub mod pmdoutset;
#[doc = "PMDOUTINV register"]
pub struct PMDOUTINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMDOUTINV register"]
pub mod pmdoutinv;
#[doc = "PMDIN register"]
pub struct PMDIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMDIN register"]
pub mod pmdin;
#[doc = "PMDINCLR register"]
pub struct PMDINCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMDINCLR register"]
pub mod pmdinclr;
#[doc = "PMDINSET register"]
pub struct PMDINSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMDINSET register"]
pub mod pmdinset;
#[doc = "PMDININV register"]
pub struct PMDININV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMDININV register"]
pub mod pmdininv;
#[doc = "PMAEN register"]
pub struct PMAEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMAEN register"]
pub mod pmaen;
#[doc = "PMAENCLR register"]
pub struct PMAENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMAENCLR register"]
pub mod pmaenclr;
#[doc = "PMAENSET register"]
pub struct PMAENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMAENSET register"]
pub mod pmaenset;
#[doc = "PMAENINV register"]
pub struct PMAENINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMAENINV register"]
pub mod pmaeninv;
#[doc = "PMSTAT register"]
pub struct PMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMSTAT register"]
pub mod pmstat;
#[doc = "PMSTATCLR register"]
pub struct PMSTATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMSTATCLR register"]
pub mod pmstatclr;
#[doc = "PMSTATSET register"]
pub struct PMSTATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMSTATSET register"]
pub mod pmstatset;
#[doc = "PMSTATINV register"]
pub struct PMSTATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMSTATINV register"]
pub mod pmstatinv;
