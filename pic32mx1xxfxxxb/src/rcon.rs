#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCON register"]
    pub rcon: crate::Reg<rcon::RCON_SPEC>,
    #[doc = "0x04 - RCONCLR register"]
    pub rconclr: crate::Reg<rconclr::RCONCLR_SPEC>,
    #[doc = "0x08 - RCONSET register"]
    pub rconset: crate::Reg<rconset::RCONSET_SPEC>,
    #[doc = "0x0c - RCONINV register"]
    pub rconinv: crate::Reg<rconinv::RCONINV_SPEC>,
    #[doc = "0x10 - RSWRST register"]
    pub rswrst: crate::Reg<rswrst::RSWRST_SPEC>,
    #[doc = "0x14 - RSWRSTCLR register"]
    pub rswrstclr: crate::Reg<rswrstclr::RSWRSTCLR_SPEC>,
    #[doc = "0x18 - RSWRSTSET register"]
    pub rswrstset: crate::Reg<rswrstset::RSWRSTSET_SPEC>,
    #[doc = "0x1c - RSWRSTINV register"]
    pub rswrstinv: crate::Reg<rswrstinv::RSWRSTINV_SPEC>,
}
#[doc = "RCON register accessor: an alias for `Reg<RCON_SPEC>`"]
pub type RCON = crate::Reg<rcon::RCON_SPEC>;
#[doc = "RCON register"]
pub mod rcon;
#[doc = "RCONCLR register accessor: an alias for `Reg<RCONCLR_SPEC>`"]
pub type RCONCLR = crate::Reg<rconclr::RCONCLR_SPEC>;
#[doc = "RCONCLR register"]
pub mod rconclr;
#[doc = "RCONSET register accessor: an alias for `Reg<RCONSET_SPEC>`"]
pub type RCONSET = crate::Reg<rconset::RCONSET_SPEC>;
#[doc = "RCONSET register"]
pub mod rconset;
#[doc = "RCONINV register accessor: an alias for `Reg<RCONINV_SPEC>`"]
pub type RCONINV = crate::Reg<rconinv::RCONINV_SPEC>;
#[doc = "RCONINV register"]
pub mod rconinv;
#[doc = "RSWRST register accessor: an alias for `Reg<RSWRST_SPEC>`"]
pub type RSWRST = crate::Reg<rswrst::RSWRST_SPEC>;
#[doc = "RSWRST register"]
pub mod rswrst;
#[doc = "RSWRSTCLR register accessor: an alias for `Reg<RSWRSTCLR_SPEC>`"]
pub type RSWRSTCLR = crate::Reg<rswrstclr::RSWRSTCLR_SPEC>;
#[doc = "RSWRSTCLR register"]
pub mod rswrstclr;
#[doc = "RSWRSTSET register accessor: an alias for `Reg<RSWRSTSET_SPEC>`"]
pub type RSWRSTSET = crate::Reg<rswrstset::RSWRSTSET_SPEC>;
#[doc = "RSWRSTSET register"]
pub mod rswrstset;
#[doc = "RSWRSTINV register accessor: an alias for `Reg<RSWRSTINV_SPEC>`"]
pub type RSWRSTINV = crate::Reg<rswrstinv::RSWRSTINV_SPEC>;
#[doc = "RSWRSTINV register"]
pub mod rswrstinv;
