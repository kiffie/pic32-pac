#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMACON register"]
    pub dmacon: crate::Reg<dmacon::DMACON_SPEC>,
    #[doc = "0x04 - DMACONCLR register"]
    pub dmaconclr: crate::Reg<dmaconclr::DMACONCLR_SPEC>,
    #[doc = "0x08 - DMACONSET register"]
    pub dmaconset: crate::Reg<dmaconset::DMACONSET_SPEC>,
    #[doc = "0x0c - DMACONINV register"]
    pub dmaconinv: crate::Reg<dmaconinv::DMACONINV_SPEC>,
    #[doc = "0x10 - DMASTAT register"]
    pub dmastat: crate::Reg<dmastat::DMASTAT_SPEC>,
    #[doc = "0x14 - DMASTATCLR register"]
    pub dmastatclr: crate::Reg<dmastatclr::DMASTATCLR_SPEC>,
    #[doc = "0x18 - DMASTATSET register"]
    pub dmastatset: crate::Reg<dmastatset::DMASTATSET_SPEC>,
    #[doc = "0x1c - DMASTATINV register"]
    pub dmastatinv: crate::Reg<dmastatinv::DMASTATINV_SPEC>,
    #[doc = "0x20 - DMAADDR register"]
    pub dmaaddr: crate::Reg<dmaaddr::DMAADDR_SPEC>,
    #[doc = "0x24 - DMAADDRCLR register"]
    pub dmaaddrclr: crate::Reg<dmaaddrclr::DMAADDRCLR_SPEC>,
    #[doc = "0x28 - DMAADDRSET register"]
    pub dmaaddrset: crate::Reg<dmaaddrset::DMAADDRSET_SPEC>,
    #[doc = "0x2c - DMAADDRINV register"]
    pub dmaaddrinv: crate::Reg<dmaaddrinv::DMAADDRINV_SPEC>,
    #[doc = "0x30 - DCRCCON register"]
    pub dcrccon: crate::Reg<dcrccon::DCRCCON_SPEC>,
    #[doc = "0x34 - DCRCCONCLR register"]
    pub dcrcconclr: crate::Reg<dcrcconclr::DCRCCONCLR_SPEC>,
    #[doc = "0x38 - DCRCCONSET register"]
    pub dcrcconset: crate::Reg<dcrcconset::DCRCCONSET_SPEC>,
    #[doc = "0x3c - DCRCCONINV register"]
    pub dcrcconinv: crate::Reg<dcrcconinv::DCRCCONINV_SPEC>,
    #[doc = "0x40 - DCRCDATA register"]
    pub dcrcdata: crate::Reg<dcrcdata::DCRCDATA_SPEC>,
    #[doc = "0x44 - DCRCDATACLR register"]
    pub dcrcdataclr: crate::Reg<dcrcdataclr::DCRCDATACLR_SPEC>,
    #[doc = "0x48 - DCRCDATASET register"]
    pub dcrcdataset: crate::Reg<dcrcdataset::DCRCDATASET_SPEC>,
    #[doc = "0x4c - DCRCDATAINV register"]
    pub dcrcdatainv: crate::Reg<dcrcdatainv::DCRCDATAINV_SPEC>,
    #[doc = "0x50 - DCRCXOR register"]
    pub dcrcxor: crate::Reg<dcrcxor::DCRCXOR_SPEC>,
    #[doc = "0x54 - DCRCXORCLR register"]
    pub dcrcxorclr: crate::Reg<dcrcxorclr::DCRCXORCLR_SPEC>,
    #[doc = "0x58 - DCRCXORSET register"]
    pub dcrcxorset: crate::Reg<dcrcxorset::DCRCXORSET_SPEC>,
    #[doc = "0x5c - DCRCXORINV register"]
    pub dcrcxorinv: crate::Reg<dcrcxorinv::DCRCXORINV_SPEC>,
}
#[doc = "DMACON register accessor: an alias for `Reg<DMACON_SPEC>`"]
pub type DMACON = crate::Reg<dmacon::DMACON_SPEC>;
#[doc = "DMACON register"]
pub mod dmacon;
#[doc = "DMACONCLR register accessor: an alias for `Reg<DMACONCLR_SPEC>`"]
pub type DMACONCLR = crate::Reg<dmaconclr::DMACONCLR_SPEC>;
#[doc = "DMACONCLR register"]
pub mod dmaconclr;
#[doc = "DMACONSET register accessor: an alias for `Reg<DMACONSET_SPEC>`"]
pub type DMACONSET = crate::Reg<dmaconset::DMACONSET_SPEC>;
#[doc = "DMACONSET register"]
pub mod dmaconset;
#[doc = "DMACONINV register accessor: an alias for `Reg<DMACONINV_SPEC>`"]
pub type DMACONINV = crate::Reg<dmaconinv::DMACONINV_SPEC>;
#[doc = "DMACONINV register"]
pub mod dmaconinv;
#[doc = "DMASTAT register accessor: an alias for `Reg<DMASTAT_SPEC>`"]
pub type DMASTAT = crate::Reg<dmastat::DMASTAT_SPEC>;
#[doc = "DMASTAT register"]
pub mod dmastat;
#[doc = "DMASTATCLR register accessor: an alias for `Reg<DMASTATCLR_SPEC>`"]
pub type DMASTATCLR = crate::Reg<dmastatclr::DMASTATCLR_SPEC>;
#[doc = "DMASTATCLR register"]
pub mod dmastatclr;
#[doc = "DMASTATSET register accessor: an alias for `Reg<DMASTATSET_SPEC>`"]
pub type DMASTATSET = crate::Reg<dmastatset::DMASTATSET_SPEC>;
#[doc = "DMASTATSET register"]
pub mod dmastatset;
#[doc = "DMASTATINV register accessor: an alias for `Reg<DMASTATINV_SPEC>`"]
pub type DMASTATINV = crate::Reg<dmastatinv::DMASTATINV_SPEC>;
#[doc = "DMASTATINV register"]
pub mod dmastatinv;
#[doc = "DMAADDR register accessor: an alias for `Reg<DMAADDR_SPEC>`"]
pub type DMAADDR = crate::Reg<dmaaddr::DMAADDR_SPEC>;
#[doc = "DMAADDR register"]
pub mod dmaaddr;
#[doc = "DMAADDRCLR register accessor: an alias for `Reg<DMAADDRCLR_SPEC>`"]
pub type DMAADDRCLR = crate::Reg<dmaaddrclr::DMAADDRCLR_SPEC>;
#[doc = "DMAADDRCLR register"]
pub mod dmaaddrclr;
#[doc = "DMAADDRSET register accessor: an alias for `Reg<DMAADDRSET_SPEC>`"]
pub type DMAADDRSET = crate::Reg<dmaaddrset::DMAADDRSET_SPEC>;
#[doc = "DMAADDRSET register"]
pub mod dmaaddrset;
#[doc = "DMAADDRINV register accessor: an alias for `Reg<DMAADDRINV_SPEC>`"]
pub type DMAADDRINV = crate::Reg<dmaaddrinv::DMAADDRINV_SPEC>;
#[doc = "DMAADDRINV register"]
pub mod dmaaddrinv;
#[doc = "DCRCCON register accessor: an alias for `Reg<DCRCCON_SPEC>`"]
pub type DCRCCON = crate::Reg<dcrccon::DCRCCON_SPEC>;
#[doc = "DCRCCON register"]
pub mod dcrccon;
#[doc = "DCRCCONCLR register accessor: an alias for `Reg<DCRCCONCLR_SPEC>`"]
pub type DCRCCONCLR = crate::Reg<dcrcconclr::DCRCCONCLR_SPEC>;
#[doc = "DCRCCONCLR register"]
pub mod dcrcconclr;
#[doc = "DCRCCONSET register accessor: an alias for `Reg<DCRCCONSET_SPEC>`"]
pub type DCRCCONSET = crate::Reg<dcrcconset::DCRCCONSET_SPEC>;
#[doc = "DCRCCONSET register"]
pub mod dcrcconset;
#[doc = "DCRCCONINV register accessor: an alias for `Reg<DCRCCONINV_SPEC>`"]
pub type DCRCCONINV = crate::Reg<dcrcconinv::DCRCCONINV_SPEC>;
#[doc = "DCRCCONINV register"]
pub mod dcrcconinv;
#[doc = "DCRCDATA register accessor: an alias for `Reg<DCRCDATA_SPEC>`"]
pub type DCRCDATA = crate::Reg<dcrcdata::DCRCDATA_SPEC>;
#[doc = "DCRCDATA register"]
pub mod dcrcdata;
#[doc = "DCRCDATACLR register accessor: an alias for `Reg<DCRCDATACLR_SPEC>`"]
pub type DCRCDATACLR = crate::Reg<dcrcdataclr::DCRCDATACLR_SPEC>;
#[doc = "DCRCDATACLR register"]
pub mod dcrcdataclr;
#[doc = "DCRCDATASET register accessor: an alias for `Reg<DCRCDATASET_SPEC>`"]
pub type DCRCDATASET = crate::Reg<dcrcdataset::DCRCDATASET_SPEC>;
#[doc = "DCRCDATASET register"]
pub mod dcrcdataset;
#[doc = "DCRCDATAINV register accessor: an alias for `Reg<DCRCDATAINV_SPEC>`"]
pub type DCRCDATAINV = crate::Reg<dcrcdatainv::DCRCDATAINV_SPEC>;
#[doc = "DCRCDATAINV register"]
pub mod dcrcdatainv;
#[doc = "DCRCXOR register accessor: an alias for `Reg<DCRCXOR_SPEC>`"]
pub type DCRCXOR = crate::Reg<dcrcxor::DCRCXOR_SPEC>;
#[doc = "DCRCXOR register"]
pub mod dcrcxor;
#[doc = "DCRCXORCLR register accessor: an alias for `Reg<DCRCXORCLR_SPEC>`"]
pub type DCRCXORCLR = crate::Reg<dcrcxorclr::DCRCXORCLR_SPEC>;
#[doc = "DCRCXORCLR register"]
pub mod dcrcxorclr;
#[doc = "DCRCXORSET register accessor: an alias for `Reg<DCRCXORSET_SPEC>`"]
pub type DCRCXORSET = crate::Reg<dcrcxorset::DCRCXORSET_SPEC>;
#[doc = "DCRCXORSET register"]
pub mod dcrcxorset;
#[doc = "DCRCXORINV register accessor: an alias for `Reg<DCRCXORINV_SPEC>`"]
pub type DCRCXORINV = crate::Reg<dcrcxorinv::DCRCXORINV_SPEC>;
#[doc = "DCRCXORINV register"]
pub mod dcrcxorinv;
