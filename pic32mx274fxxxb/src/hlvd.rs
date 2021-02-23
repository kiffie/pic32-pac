#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HLVDCON register"]
    pub hlvdcon: crate::Reg<hlvdcon::HLVDCON_SPEC>,
    #[doc = "0x04 - HLVDCONCLR register"]
    pub hlvdconclr: crate::Reg<hlvdconclr::HLVDCONCLR_SPEC>,
    #[doc = "0x08 - HLVDCONSET register"]
    pub hlvdconset: crate::Reg<hlvdconset::HLVDCONSET_SPEC>,
    #[doc = "0x0c - HLVDCONINV register"]
    pub hlvdconinv: crate::Reg<hlvdconinv::HLVDCONINV_SPEC>,
}
#[doc = "HLVDCON register accessor: an alias for `Reg<HLVDCON_SPEC>`"]
pub type HLVDCON = crate::Reg<hlvdcon::HLVDCON_SPEC>;
#[doc = "HLVDCON register"]
pub mod hlvdcon;
#[doc = "HLVDCONCLR register accessor: an alias for `Reg<HLVDCONCLR_SPEC>`"]
pub type HLVDCONCLR = crate::Reg<hlvdconclr::HLVDCONCLR_SPEC>;
#[doc = "HLVDCONCLR register"]
pub mod hlvdconclr;
#[doc = "HLVDCONSET register accessor: an alias for `Reg<HLVDCONSET_SPEC>`"]
pub type HLVDCONSET = crate::Reg<hlvdconset::HLVDCONSET_SPEC>;
#[doc = "HLVDCONSET register"]
pub mod hlvdconset;
#[doc = "HLVDCONINV register accessor: an alias for `Reg<HLVDCONINV_SPEC>`"]
pub type HLVDCONINV = crate::Reg<hlvdconinv::HLVDCONINV_SPEC>;
#[doc = "HLVDCONINV register"]
pub mod hlvdconinv;
