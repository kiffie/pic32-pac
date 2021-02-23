#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSCCON register"]
    pub osccon: crate::Reg<osccon::OSCCON_SPEC>,
    #[doc = "0x04 - OSCCONCLR register"]
    pub oscconclr: crate::Reg<oscconclr::OSCCONCLR_SPEC>,
    #[doc = "0x08 - OSCCONSET register"]
    pub oscconset: crate::Reg<oscconset::OSCCONSET_SPEC>,
    #[doc = "0x0c - OSCCONINV register"]
    pub oscconinv: crate::Reg<oscconinv::OSCCONINV_SPEC>,
    #[doc = "0x10 - OSCTUN register"]
    pub osctun: crate::Reg<osctun::OSCTUN_SPEC>,
    #[doc = "0x14 - OSCTUNCLR register"]
    pub osctunclr: crate::Reg<osctunclr::OSCTUNCLR_SPEC>,
    #[doc = "0x18 - OSCTUNSET register"]
    pub osctunset: crate::Reg<osctunset::OSCTUNSET_SPEC>,
    #[doc = "0x1c - OSCTUNINV register"]
    pub osctuninv: crate::Reg<osctuninv::OSCTUNINV_SPEC>,
    #[doc = "0x20 - REFOCON register"]
    pub refocon: crate::Reg<refocon::REFOCON_SPEC>,
    #[doc = "0x24 - REFOCONCLR register"]
    pub refoconclr: crate::Reg<refoconclr::REFOCONCLR_SPEC>,
    #[doc = "0x28 - REFOCONSET register"]
    pub refoconset: crate::Reg<refoconset::REFOCONSET_SPEC>,
    #[doc = "0x2c - REFOCONINV register"]
    pub refoconinv: crate::Reg<refoconinv::REFOCONINV_SPEC>,
    #[doc = "0x30 - REFOTRIM register"]
    pub refotrim: crate::Reg<refotrim::REFOTRIM_SPEC>,
    #[doc = "0x34 - REFOTRIMCLR register"]
    pub refotrimclr: crate::Reg<refotrimclr::REFOTRIMCLR_SPEC>,
    #[doc = "0x38 - REFOTRIMSET register"]
    pub refotrimset: crate::Reg<refotrimset::REFOTRIMSET_SPEC>,
    #[doc = "0x3c - REFOTRIMINV register"]
    pub refotriminv: crate::Reg<refotriminv::REFOTRIMINV_SPEC>,
}
#[doc = "OSCCON register accessor: an alias for `Reg<OSCCON_SPEC>`"]
pub type OSCCON = crate::Reg<osccon::OSCCON_SPEC>;
#[doc = "OSCCON register"]
pub mod osccon;
#[doc = "OSCCONCLR register accessor: an alias for `Reg<OSCCONCLR_SPEC>`"]
pub type OSCCONCLR = crate::Reg<oscconclr::OSCCONCLR_SPEC>;
#[doc = "OSCCONCLR register"]
pub mod oscconclr;
#[doc = "OSCCONSET register accessor: an alias for `Reg<OSCCONSET_SPEC>`"]
pub type OSCCONSET = crate::Reg<oscconset::OSCCONSET_SPEC>;
#[doc = "OSCCONSET register"]
pub mod oscconset;
#[doc = "OSCCONINV register accessor: an alias for `Reg<OSCCONINV_SPEC>`"]
pub type OSCCONINV = crate::Reg<oscconinv::OSCCONINV_SPEC>;
#[doc = "OSCCONINV register"]
pub mod oscconinv;
#[doc = "OSCTUN register accessor: an alias for `Reg<OSCTUN_SPEC>`"]
pub type OSCTUN = crate::Reg<osctun::OSCTUN_SPEC>;
#[doc = "OSCTUN register"]
pub mod osctun;
#[doc = "OSCTUNCLR register accessor: an alias for `Reg<OSCTUNCLR_SPEC>`"]
pub type OSCTUNCLR = crate::Reg<osctunclr::OSCTUNCLR_SPEC>;
#[doc = "OSCTUNCLR register"]
pub mod osctunclr;
#[doc = "OSCTUNSET register accessor: an alias for `Reg<OSCTUNSET_SPEC>`"]
pub type OSCTUNSET = crate::Reg<osctunset::OSCTUNSET_SPEC>;
#[doc = "OSCTUNSET register"]
pub mod osctunset;
#[doc = "OSCTUNINV register accessor: an alias for `Reg<OSCTUNINV_SPEC>`"]
pub type OSCTUNINV = crate::Reg<osctuninv::OSCTUNINV_SPEC>;
#[doc = "OSCTUNINV register"]
pub mod osctuninv;
#[doc = "REFOCON register accessor: an alias for `Reg<REFOCON_SPEC>`"]
pub type REFOCON = crate::Reg<refocon::REFOCON_SPEC>;
#[doc = "REFOCON register"]
pub mod refocon;
#[doc = "REFOCONCLR register accessor: an alias for `Reg<REFOCONCLR_SPEC>`"]
pub type REFOCONCLR = crate::Reg<refoconclr::REFOCONCLR_SPEC>;
#[doc = "REFOCONCLR register"]
pub mod refoconclr;
#[doc = "REFOCONSET register accessor: an alias for `Reg<REFOCONSET_SPEC>`"]
pub type REFOCONSET = crate::Reg<refoconset::REFOCONSET_SPEC>;
#[doc = "REFOCONSET register"]
pub mod refoconset;
#[doc = "REFOCONINV register accessor: an alias for `Reg<REFOCONINV_SPEC>`"]
pub type REFOCONINV = crate::Reg<refoconinv::REFOCONINV_SPEC>;
#[doc = "REFOCONINV register"]
pub mod refoconinv;
#[doc = "REFOTRIM register accessor: an alias for `Reg<REFOTRIM_SPEC>`"]
pub type REFOTRIM = crate::Reg<refotrim::REFOTRIM_SPEC>;
#[doc = "REFOTRIM register"]
pub mod refotrim;
#[doc = "REFOTRIMCLR register accessor: an alias for `Reg<REFOTRIMCLR_SPEC>`"]
pub type REFOTRIMCLR = crate::Reg<refotrimclr::REFOTRIMCLR_SPEC>;
#[doc = "REFOTRIMCLR register"]
pub mod refotrimclr;
#[doc = "REFOTRIMSET register accessor: an alias for `Reg<REFOTRIMSET_SPEC>`"]
pub type REFOTRIMSET = crate::Reg<refotrimset::REFOTRIMSET_SPEC>;
#[doc = "REFOTRIMSET register"]
pub mod refotrimset;
#[doc = "REFOTRIMINV register accessor: an alias for `Reg<REFOTRIMINV_SPEC>`"]
pub type REFOTRIMINV = crate::Reg<refotriminv::REFOTRIMINV_SPEC>;
#[doc = "REFOTRIMINV register"]
pub mod refotriminv;
