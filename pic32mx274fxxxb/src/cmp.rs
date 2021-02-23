#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMSTAT register"]
    pub cmstat: crate::Reg<cmstat::CMSTAT_SPEC>,
    #[doc = "0x04 - CMSTATCLR register"]
    pub cmstatclr: crate::Reg<cmstatclr::CMSTATCLR_SPEC>,
    #[doc = "0x08 - CMSTATSET register"]
    pub cmstatset: crate::Reg<cmstatset::CMSTATSET_SPEC>,
    #[doc = "0x0c - CMSTATINV register"]
    pub cmstatinv: crate::Reg<cmstatinv::CMSTATINV_SPEC>,
}
#[doc = "CMSTAT register accessor: an alias for `Reg<CMSTAT_SPEC>`"]
pub type CMSTAT = crate::Reg<cmstat::CMSTAT_SPEC>;
#[doc = "CMSTAT register"]
pub mod cmstat;
#[doc = "CMSTATCLR register accessor: an alias for `Reg<CMSTATCLR_SPEC>`"]
pub type CMSTATCLR = crate::Reg<cmstatclr::CMSTATCLR_SPEC>;
#[doc = "CMSTATCLR register"]
pub mod cmstatclr;
#[doc = "CMSTATSET register accessor: an alias for `Reg<CMSTATSET_SPEC>`"]
pub type CMSTATSET = crate::Reg<cmstatset::CMSTATSET_SPEC>;
#[doc = "CMSTATSET register"]
pub mod cmstatset;
#[doc = "CMSTATINV register accessor: an alias for `Reg<CMSTATINV_SPEC>`"]
pub type CMSTATINV = crate::Reg<cmstatinv::CMSTATINV_SPEC>;
#[doc = "CMSTATINV register"]
pub mod cmstatinv;
