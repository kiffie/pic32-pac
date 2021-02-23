#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U1MODE register"]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x04 - U1MODECLR register"]
    pub modeclr: crate::Reg<modeclr::MODECLR_SPEC>,
    #[doc = "0x08 - U1MODESET register"]
    pub modeset: crate::Reg<modeset::MODESET_SPEC>,
    #[doc = "0x0c - U1MODEINV register"]
    pub modeinv: crate::Reg<modeinv::MODEINV_SPEC>,
    #[doc = "0x10 - U1STA register"]
    pub sta: crate::Reg<sta::STA_SPEC>,
    #[doc = "0x14 - U1STACLR register"]
    pub staclr: crate::Reg<staclr::STACLR_SPEC>,
    #[doc = "0x18 - U1STASET register"]
    pub staset: crate::Reg<staset::STASET_SPEC>,
    #[doc = "0x1c - U1STAINV register"]
    pub stainv: crate::Reg<stainv::STAINV_SPEC>,
    #[doc = "0x20 - U1TXREG register"]
    pub txreg: crate::Reg<txreg::TXREG_SPEC>,
    _reserved9: [u8; 12usize],
    #[doc = "0x30 - U1RXREG register"]
    pub rxreg: crate::Reg<rxreg::RXREG_SPEC>,
    _reserved10: [u8; 12usize],
    #[doc = "0x40 - U1BRG register"]
    pub brg: crate::Reg<brg::BRG_SPEC>,
    #[doc = "0x44 - U1BRGCLR register"]
    pub brgclr: crate::Reg<brgclr::BRGCLR_SPEC>,
    #[doc = "0x48 - U1BRGSET register"]
    pub brgset: crate::Reg<brgset::BRGSET_SPEC>,
    #[doc = "0x4c - U1BRGINV register"]
    pub brginv: crate::Reg<brginv::BRGINV_SPEC>,
}
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "U1MODE register"]
pub mod mode;
#[doc = "MODECLR register accessor: an alias for `Reg<MODECLR_SPEC>`"]
pub type MODECLR = crate::Reg<modeclr::MODECLR_SPEC>;
#[doc = "U1MODECLR register"]
pub mod modeclr;
#[doc = "MODESET register accessor: an alias for `Reg<MODESET_SPEC>`"]
pub type MODESET = crate::Reg<modeset::MODESET_SPEC>;
#[doc = "U1MODESET register"]
pub mod modeset;
#[doc = "MODEINV register accessor: an alias for `Reg<MODEINV_SPEC>`"]
pub type MODEINV = crate::Reg<modeinv::MODEINV_SPEC>;
#[doc = "U1MODEINV register"]
pub mod modeinv;
#[doc = "STA register accessor: an alias for `Reg<STA_SPEC>`"]
pub type STA = crate::Reg<sta::STA_SPEC>;
#[doc = "U1STA register"]
pub mod sta;
#[doc = "STACLR register accessor: an alias for `Reg<STACLR_SPEC>`"]
pub type STACLR = crate::Reg<staclr::STACLR_SPEC>;
#[doc = "U1STACLR register"]
pub mod staclr;
#[doc = "STASET register accessor: an alias for `Reg<STASET_SPEC>`"]
pub type STASET = crate::Reg<staset::STASET_SPEC>;
#[doc = "U1STASET register"]
pub mod staset;
#[doc = "STAINV register accessor: an alias for `Reg<STAINV_SPEC>`"]
pub type STAINV = crate::Reg<stainv::STAINV_SPEC>;
#[doc = "U1STAINV register"]
pub mod stainv;
#[doc = "TXREG register accessor: an alias for `Reg<TXREG_SPEC>`"]
pub type TXREG = crate::Reg<txreg::TXREG_SPEC>;
#[doc = "U1TXREG register"]
pub mod txreg;
#[doc = "RXREG register accessor: an alias for `Reg<RXREG_SPEC>`"]
pub type RXREG = crate::Reg<rxreg::RXREG_SPEC>;
#[doc = "U1RXREG register"]
pub mod rxreg;
#[doc = "BRG register accessor: an alias for `Reg<BRG_SPEC>`"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "U1BRG register"]
pub mod brg;
#[doc = "BRGCLR register accessor: an alias for `Reg<BRGCLR_SPEC>`"]
pub type BRGCLR = crate::Reg<brgclr::BRGCLR_SPEC>;
#[doc = "U1BRGCLR register"]
pub mod brgclr;
#[doc = "BRGSET register accessor: an alias for `Reg<BRGSET_SPEC>`"]
pub type BRGSET = crate::Reg<brgset::BRGSET_SPEC>;
#[doc = "U1BRGSET register"]
pub mod brgset;
#[doc = "BRGINV register accessor: an alias for `Reg<BRGINV_SPEC>`"]
pub type BRGINV = crate::Reg<brginv::BRGINV_SPEC>;
#[doc = "U1BRGINV register"]
pub mod brginv;
