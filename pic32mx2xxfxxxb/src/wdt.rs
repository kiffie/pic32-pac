#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDTCON register"]
    pub wdtcon: crate::Reg<wdtcon::WDTCON_SPEC>,
    #[doc = "0x04 - WDTCONCLR register"]
    pub wdtconclr: crate::Reg<wdtconclr::WDTCONCLR_SPEC>,
    #[doc = "0x08 - WDTCONSET register"]
    pub wdtconset: crate::Reg<wdtconset::WDTCONSET_SPEC>,
    #[doc = "0x0c - WDTCONINV register"]
    pub wdtconinv: crate::Reg<wdtconinv::WDTCONINV_SPEC>,
}
#[doc = "WDTCON register accessor: an alias for `Reg<WDTCON_SPEC>`"]
pub type WDTCON = crate::Reg<wdtcon::WDTCON_SPEC>;
#[doc = "WDTCON register"]
pub mod wdtcon;
#[doc = "WDTCONCLR register accessor: an alias for `Reg<WDTCONCLR_SPEC>`"]
pub type WDTCONCLR = crate::Reg<wdtconclr::WDTCONCLR_SPEC>;
#[doc = "WDTCONCLR register"]
pub mod wdtconclr;
#[doc = "WDTCONSET register accessor: an alias for `Reg<WDTCONSET_SPEC>`"]
pub type WDTCONSET = crate::Reg<wdtconset::WDTCONSET_SPEC>;
#[doc = "WDTCONSET register"]
pub mod wdtconset;
#[doc = "WDTCONINV register accessor: an alias for `Reg<WDTCONINV_SPEC>`"]
pub type WDTCONINV = crate::Reg<wdtconinv::WDTCONINV_SPEC>;
#[doc = "WDTCONINV register"]
pub mod wdtconinv;
