#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTMUCON register"]
    pub ctmucon: crate::Reg<ctmucon::CTMUCON_SPEC>,
    #[doc = "0x04 - CTMUCONCLR register"]
    pub ctmuconclr: crate::Reg<ctmuconclr::CTMUCONCLR_SPEC>,
    #[doc = "0x08 - CTMUCONSET register"]
    pub ctmuconset: crate::Reg<ctmuconset::CTMUCONSET_SPEC>,
    #[doc = "0x0c - CTMUCONINV register"]
    pub ctmuconinv: crate::Reg<ctmuconinv::CTMUCONINV_SPEC>,
}
#[doc = "CTMUCON register accessor: an alias for `Reg<CTMUCON_SPEC>`"]
pub type CTMUCON = crate::Reg<ctmucon::CTMUCON_SPEC>;
#[doc = "CTMUCON register"]
pub mod ctmucon;
#[doc = "CTMUCONCLR register accessor: an alias for `Reg<CTMUCONCLR_SPEC>`"]
pub type CTMUCONCLR = crate::Reg<ctmuconclr::CTMUCONCLR_SPEC>;
#[doc = "CTMUCONCLR register"]
pub mod ctmuconclr;
#[doc = "CTMUCONSET register accessor: an alias for `Reg<CTMUCONSET_SPEC>`"]
pub type CTMUCONSET = crate::Reg<ctmuconset::CTMUCONSET_SPEC>;
#[doc = "CTMUCONSET register"]
pub mod ctmuconset;
#[doc = "CTMUCONINV register accessor: an alias for `Reg<CTMUCONINV_SPEC>`"]
pub type CTMUCONINV = crate::Reg<ctmuconinv::CTMUCONINV_SPEC>;
#[doc = "CTMUCONINV register"]
pub mod ctmuconinv;
