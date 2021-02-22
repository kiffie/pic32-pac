#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CVRCON register"]
    pub cvrcon: crate::Reg<cvrcon::CVRCON_SPEC>,
    #[doc = "0x04 - CVRCONCLR register"]
    pub cvrconclr: crate::Reg<cvrconclr::CVRCONCLR_SPEC>,
    #[doc = "0x08 - CVRCONSET register"]
    pub cvrconset: crate::Reg<cvrconset::CVRCONSET_SPEC>,
    #[doc = "0x0c - CVRCONINV register"]
    pub cvrconinv: crate::Reg<cvrconinv::CVRCONINV_SPEC>,
}
#[doc = "CVRCON register accessor: an alias for `Reg<CVRCON_SPEC>`"]
pub type CVRCON = crate::Reg<cvrcon::CVRCON_SPEC>;
#[doc = "CVRCON register"]
pub mod cvrcon;
#[doc = "CVRCONCLR register accessor: an alias for `Reg<CVRCONCLR_SPEC>`"]
pub type CVRCONCLR = crate::Reg<cvrconclr::CVRCONCLR_SPEC>;
#[doc = "CVRCONCLR register"]
pub mod cvrconclr;
#[doc = "CVRCONSET register accessor: an alias for `Reg<CVRCONSET_SPEC>`"]
pub type CVRCONSET = crate::Reg<cvrconset::CVRCONSET_SPEC>;
#[doc = "CVRCONSET register"]
pub mod cvrconset;
#[doc = "CVRCONINV register accessor: an alias for `Reg<CVRCONINV_SPEC>`"]
pub type CVRCONINV = crate::Reg<cvrconinv::CVRCONINV_SPEC>;
#[doc = "CVRCONINV register"]
pub mod cvrconinv;
