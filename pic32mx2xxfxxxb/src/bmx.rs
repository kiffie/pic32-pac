#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BMXCON register"]
    pub bmxcon: crate::Reg<bmxcon::BMXCON_SPEC>,
    #[doc = "0x04 - BMXCONCLR register"]
    pub bmxconclr: crate::Reg<bmxconclr::BMXCONCLR_SPEC>,
    #[doc = "0x08 - BMXCONSET register"]
    pub bmxconset: crate::Reg<bmxconset::BMXCONSET_SPEC>,
    #[doc = "0x0c - BMXCONINV register"]
    pub bmxconinv: crate::Reg<bmxconinv::BMXCONINV_SPEC>,
    #[doc = "0x10 - BMXDKPBA register"]
    pub bmxdkpba: crate::Reg<bmxdkpba::BMXDKPBA_SPEC>,
    #[doc = "0x14 - BMXDKPBACLR register"]
    pub bmxdkpbaclr: crate::Reg<bmxdkpbaclr::BMXDKPBACLR_SPEC>,
    #[doc = "0x18 - BMXDKPBASET register"]
    pub bmxdkpbaset: crate::Reg<bmxdkpbaset::BMXDKPBASET_SPEC>,
    #[doc = "0x1c - BMXDKPBAINV register"]
    pub bmxdkpbainv: crate::Reg<bmxdkpbainv::BMXDKPBAINV_SPEC>,
    #[doc = "0x20 - BMXDUDBA register"]
    pub bmxdudba: crate::Reg<bmxdudba::BMXDUDBA_SPEC>,
    #[doc = "0x24 - BMXDUDBACLR register"]
    pub bmxdudbaclr: crate::Reg<bmxdudbaclr::BMXDUDBACLR_SPEC>,
    #[doc = "0x28 - BMXDUDBASET register"]
    pub bmxdudbaset: crate::Reg<bmxdudbaset::BMXDUDBASET_SPEC>,
    #[doc = "0x2c - BMXDUDBAINV register"]
    pub bmxdudbainv: crate::Reg<bmxdudbainv::BMXDUDBAINV_SPEC>,
    #[doc = "0x30 - BMXDUPBA register"]
    pub bmxdupba: crate::Reg<bmxdupba::BMXDUPBA_SPEC>,
    #[doc = "0x34 - BMXDUPBACLR register"]
    pub bmxdupbaclr: crate::Reg<bmxdupbaclr::BMXDUPBACLR_SPEC>,
    #[doc = "0x38 - BMXDUPBASET register"]
    pub bmxdupbaset: crate::Reg<bmxdupbaset::BMXDUPBASET_SPEC>,
    #[doc = "0x3c - BMXDUPBAINV register"]
    pub bmxdupbainv: crate::Reg<bmxdupbainv::BMXDUPBAINV_SPEC>,
    #[doc = "0x40 - BMXDRMSZ register"]
    pub bmxdrmsz: crate::Reg<bmxdrmsz::BMXDRMSZ_SPEC>,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - BMXPUPBA register"]
    pub bmxpupba: crate::Reg<bmxpupba::BMXPUPBA_SPEC>,
    #[doc = "0x54 - BMXPUPBACLR register"]
    pub bmxpupbaclr: crate::Reg<bmxpupbaclr::BMXPUPBACLR_SPEC>,
    #[doc = "0x58 - BMXPUPBASET register"]
    pub bmxpupbaset: crate::Reg<bmxpupbaset::BMXPUPBASET_SPEC>,
    #[doc = "0x5c - BMXPUPBAINV register"]
    pub bmxpupbainv: crate::Reg<bmxpupbainv::BMXPUPBAINV_SPEC>,
    #[doc = "0x60 - BMXPFMSZ register"]
    pub bmxpfmsz: crate::Reg<bmxpfmsz::BMXPFMSZ_SPEC>,
    _reserved22: [u8; 12usize],
    #[doc = "0x70 - BMXBOOTSZ register"]
    pub bmxbootsz: crate::Reg<bmxbootsz::BMXBOOTSZ_SPEC>,
}
#[doc = "BMXCON register accessor: an alias for `Reg<BMXCON_SPEC>`"]
pub type BMXCON = crate::Reg<bmxcon::BMXCON_SPEC>;
#[doc = "BMXCON register"]
pub mod bmxcon;
#[doc = "BMXCONCLR register accessor: an alias for `Reg<BMXCONCLR_SPEC>`"]
pub type BMXCONCLR = crate::Reg<bmxconclr::BMXCONCLR_SPEC>;
#[doc = "BMXCONCLR register"]
pub mod bmxconclr;
#[doc = "BMXCONSET register accessor: an alias for `Reg<BMXCONSET_SPEC>`"]
pub type BMXCONSET = crate::Reg<bmxconset::BMXCONSET_SPEC>;
#[doc = "BMXCONSET register"]
pub mod bmxconset;
#[doc = "BMXCONINV register accessor: an alias for `Reg<BMXCONINV_SPEC>`"]
pub type BMXCONINV = crate::Reg<bmxconinv::BMXCONINV_SPEC>;
#[doc = "BMXCONINV register"]
pub mod bmxconinv;
#[doc = "BMXDKPBA register accessor: an alias for `Reg<BMXDKPBA_SPEC>`"]
pub type BMXDKPBA = crate::Reg<bmxdkpba::BMXDKPBA_SPEC>;
#[doc = "BMXDKPBA register"]
pub mod bmxdkpba;
#[doc = "BMXDKPBACLR register accessor: an alias for `Reg<BMXDKPBACLR_SPEC>`"]
pub type BMXDKPBACLR = crate::Reg<bmxdkpbaclr::BMXDKPBACLR_SPEC>;
#[doc = "BMXDKPBACLR register"]
pub mod bmxdkpbaclr;
#[doc = "BMXDKPBASET register accessor: an alias for `Reg<BMXDKPBASET_SPEC>`"]
pub type BMXDKPBASET = crate::Reg<bmxdkpbaset::BMXDKPBASET_SPEC>;
#[doc = "BMXDKPBASET register"]
pub mod bmxdkpbaset;
#[doc = "BMXDKPBAINV register accessor: an alias for `Reg<BMXDKPBAINV_SPEC>`"]
pub type BMXDKPBAINV = crate::Reg<bmxdkpbainv::BMXDKPBAINV_SPEC>;
#[doc = "BMXDKPBAINV register"]
pub mod bmxdkpbainv;
#[doc = "BMXDUDBA register accessor: an alias for `Reg<BMXDUDBA_SPEC>`"]
pub type BMXDUDBA = crate::Reg<bmxdudba::BMXDUDBA_SPEC>;
#[doc = "BMXDUDBA register"]
pub mod bmxdudba;
#[doc = "BMXDUDBACLR register accessor: an alias for `Reg<BMXDUDBACLR_SPEC>`"]
pub type BMXDUDBACLR = crate::Reg<bmxdudbaclr::BMXDUDBACLR_SPEC>;
#[doc = "BMXDUDBACLR register"]
pub mod bmxdudbaclr;
#[doc = "BMXDUDBASET register accessor: an alias for `Reg<BMXDUDBASET_SPEC>`"]
pub type BMXDUDBASET = crate::Reg<bmxdudbaset::BMXDUDBASET_SPEC>;
#[doc = "BMXDUDBASET register"]
pub mod bmxdudbaset;
#[doc = "BMXDUDBAINV register accessor: an alias for `Reg<BMXDUDBAINV_SPEC>`"]
pub type BMXDUDBAINV = crate::Reg<bmxdudbainv::BMXDUDBAINV_SPEC>;
#[doc = "BMXDUDBAINV register"]
pub mod bmxdudbainv;
#[doc = "BMXDUPBA register accessor: an alias for `Reg<BMXDUPBA_SPEC>`"]
pub type BMXDUPBA = crate::Reg<bmxdupba::BMXDUPBA_SPEC>;
#[doc = "BMXDUPBA register"]
pub mod bmxdupba;
#[doc = "BMXDUPBACLR register accessor: an alias for `Reg<BMXDUPBACLR_SPEC>`"]
pub type BMXDUPBACLR = crate::Reg<bmxdupbaclr::BMXDUPBACLR_SPEC>;
#[doc = "BMXDUPBACLR register"]
pub mod bmxdupbaclr;
#[doc = "BMXDUPBASET register accessor: an alias for `Reg<BMXDUPBASET_SPEC>`"]
pub type BMXDUPBASET = crate::Reg<bmxdupbaset::BMXDUPBASET_SPEC>;
#[doc = "BMXDUPBASET register"]
pub mod bmxdupbaset;
#[doc = "BMXDUPBAINV register accessor: an alias for `Reg<BMXDUPBAINV_SPEC>`"]
pub type BMXDUPBAINV = crate::Reg<bmxdupbainv::BMXDUPBAINV_SPEC>;
#[doc = "BMXDUPBAINV register"]
pub mod bmxdupbainv;
#[doc = "BMXDRMSZ register accessor: an alias for `Reg<BMXDRMSZ_SPEC>`"]
pub type BMXDRMSZ = crate::Reg<bmxdrmsz::BMXDRMSZ_SPEC>;
#[doc = "BMXDRMSZ register"]
pub mod bmxdrmsz;
#[doc = "BMXPUPBA register accessor: an alias for `Reg<BMXPUPBA_SPEC>`"]
pub type BMXPUPBA = crate::Reg<bmxpupba::BMXPUPBA_SPEC>;
#[doc = "BMXPUPBA register"]
pub mod bmxpupba;
#[doc = "BMXPUPBACLR register accessor: an alias for `Reg<BMXPUPBACLR_SPEC>`"]
pub type BMXPUPBACLR = crate::Reg<bmxpupbaclr::BMXPUPBACLR_SPEC>;
#[doc = "BMXPUPBACLR register"]
pub mod bmxpupbaclr;
#[doc = "BMXPUPBASET register accessor: an alias for `Reg<BMXPUPBASET_SPEC>`"]
pub type BMXPUPBASET = crate::Reg<bmxpupbaset::BMXPUPBASET_SPEC>;
#[doc = "BMXPUPBASET register"]
pub mod bmxpupbaset;
#[doc = "BMXPUPBAINV register accessor: an alias for `Reg<BMXPUPBAINV_SPEC>`"]
pub type BMXPUPBAINV = crate::Reg<bmxpupbainv::BMXPUPBAINV_SPEC>;
#[doc = "BMXPUPBAINV register"]
pub mod bmxpupbainv;
#[doc = "BMXPFMSZ register accessor: an alias for `Reg<BMXPFMSZ_SPEC>`"]
pub type BMXPFMSZ = crate::Reg<bmxpfmsz::BMXPFMSZ_SPEC>;
#[doc = "BMXPFMSZ register"]
pub mod bmxpfmsz;
#[doc = "BMXBOOTSZ register accessor: an alias for `Reg<BMXBOOTSZ_SPEC>`"]
pub type BMXBOOTSZ = crate::Reg<bmxbootsz::BMXBOOTSZ_SPEC>;
#[doc = "BMXBOOTSZ register"]
pub mod bmxbootsz;
