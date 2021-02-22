#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - NVMCON register"]
    pub nvmcon: crate::Reg<nvmcon::NVMCON_SPEC>,
    #[doc = "0x04 - NVMCONCLR register"]
    pub nvmconclr: crate::Reg<nvmconclr::NVMCONCLR_SPEC>,
    #[doc = "0x08 - NVMCONSET register"]
    pub nvmconset: crate::Reg<nvmconset::NVMCONSET_SPEC>,
    #[doc = "0x0c - NVMCONINV register"]
    pub nvmconinv: crate::Reg<nvmconinv::NVMCONINV_SPEC>,
    #[doc = "0x10 - NVMKEY register"]
    pub nvmkey: crate::Reg<nvmkey::NVMKEY_SPEC>,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - NVMADDR register"]
    pub nvmaddr: crate::Reg<nvmaddr::NVMADDR_SPEC>,
    #[doc = "0x24 - NVMADDRCLR register"]
    pub nvmaddrclr: crate::Reg<nvmaddrclr::NVMADDRCLR_SPEC>,
    #[doc = "0x28 - NVMADDRSET register"]
    pub nvmaddrset: crate::Reg<nvmaddrset::NVMADDRSET_SPEC>,
    #[doc = "0x2c - NVMADDRINV register"]
    pub nvmaddrinv: crate::Reg<nvmaddrinv::NVMADDRINV_SPEC>,
    #[doc = "0x30 - NVMDATA register"]
    pub nvmdata: crate::Reg<nvmdata::NVMDATA_SPEC>,
    _reserved10: [u8; 12usize],
    #[doc = "0x40 - NVMSRCADDR register"]
    pub nvmsrcaddr: crate::Reg<nvmsrcaddr::NVMSRCADDR_SPEC>,
}
#[doc = "NVMCON register accessor: an alias for `Reg<NVMCON_SPEC>`"]
pub type NVMCON = crate::Reg<nvmcon::NVMCON_SPEC>;
#[doc = "NVMCON register"]
pub mod nvmcon;
#[doc = "NVMCONCLR register accessor: an alias for `Reg<NVMCONCLR_SPEC>`"]
pub type NVMCONCLR = crate::Reg<nvmconclr::NVMCONCLR_SPEC>;
#[doc = "NVMCONCLR register"]
pub mod nvmconclr;
#[doc = "NVMCONSET register accessor: an alias for `Reg<NVMCONSET_SPEC>`"]
pub type NVMCONSET = crate::Reg<nvmconset::NVMCONSET_SPEC>;
#[doc = "NVMCONSET register"]
pub mod nvmconset;
#[doc = "NVMCONINV register accessor: an alias for `Reg<NVMCONINV_SPEC>`"]
pub type NVMCONINV = crate::Reg<nvmconinv::NVMCONINV_SPEC>;
#[doc = "NVMCONINV register"]
pub mod nvmconinv;
#[doc = "NVMKEY register accessor: an alias for `Reg<NVMKEY_SPEC>`"]
pub type NVMKEY = crate::Reg<nvmkey::NVMKEY_SPEC>;
#[doc = "NVMKEY register"]
pub mod nvmkey;
#[doc = "NVMADDR register accessor: an alias for `Reg<NVMADDR_SPEC>`"]
pub type NVMADDR = crate::Reg<nvmaddr::NVMADDR_SPEC>;
#[doc = "NVMADDR register"]
pub mod nvmaddr;
#[doc = "NVMADDRCLR register accessor: an alias for `Reg<NVMADDRCLR_SPEC>`"]
pub type NVMADDRCLR = crate::Reg<nvmaddrclr::NVMADDRCLR_SPEC>;
#[doc = "NVMADDRCLR register"]
pub mod nvmaddrclr;
#[doc = "NVMADDRSET register accessor: an alias for `Reg<NVMADDRSET_SPEC>`"]
pub type NVMADDRSET = crate::Reg<nvmaddrset::NVMADDRSET_SPEC>;
#[doc = "NVMADDRSET register"]
pub mod nvmaddrset;
#[doc = "NVMADDRINV register accessor: an alias for `Reg<NVMADDRINV_SPEC>`"]
pub type NVMADDRINV = crate::Reg<nvmaddrinv::NVMADDRINV_SPEC>;
#[doc = "NVMADDRINV register"]
pub mod nvmaddrinv;
#[doc = "NVMDATA register accessor: an alias for `Reg<NVMDATA_SPEC>`"]
pub type NVMDATA = crate::Reg<nvmdata::NVMDATA_SPEC>;
#[doc = "NVMDATA register"]
pub mod nvmdata;
#[doc = "NVMSRCADDR register accessor: an alias for `Reg<NVMSRCADDR_SPEC>`"]
pub type NVMSRCADDR = crate::Reg<nvmsrcaddr::NVMSRCADDR_SPEC>;
#[doc = "NVMSRCADDR register"]
pub mod nvmsrcaddr;
