#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI1CON register"]
    pub con1: crate::Reg<con1::CON1_SPEC>,
    #[doc = "0x04 - SPI1CONCLR register"]
    pub con1clr: crate::Reg<con1clr::CON1CLR_SPEC>,
    #[doc = "0x08 - SPI1CONSET register"]
    pub con1set: crate::Reg<con1set::CON1SET_SPEC>,
    #[doc = "0x0c - SPI1CONINV register"]
    pub con1inv: crate::Reg<con1inv::CON1INV_SPEC>,
    #[doc = "0x10 - SPI1STAT register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x14 - SPI1STATCLR register"]
    pub statclr: crate::Reg<statclr::STATCLR_SPEC>,
    #[doc = "0x18 - SPI1STATSET register"]
    pub statset: crate::Reg<statset::STATSET_SPEC>,
    #[doc = "0x1c - SPI1STATINV register"]
    pub statinv: crate::Reg<statinv::STATINV_SPEC>,
    #[doc = "0x20 - SPI1BUF register"]
    pub buf: crate::Reg<buf::BUF_SPEC>,
    _reserved9: [u8; 12usize],
    #[doc = "0x30 - SPI1BRG register"]
    pub brg: crate::Reg<brg::BRG_SPEC>,
    #[doc = "0x34 - SPI1BRGCLR register"]
    pub brgclr: crate::Reg<brgclr::BRGCLR_SPEC>,
    #[doc = "0x38 - SPI1BRGSET register"]
    pub brgset: crate::Reg<brgset::BRGSET_SPEC>,
    #[doc = "0x3c - SPI1BRGINV register"]
    pub brginv: crate::Reg<brginv::BRGINV_SPEC>,
    #[doc = "0x40 - SPI1CON2 register"]
    pub con2: crate::Reg<con2::CON2_SPEC>,
    #[doc = "0x44 - SPI1CON2CLR register"]
    pub con2clr: crate::Reg<con2clr::CON2CLR_SPEC>,
    #[doc = "0x48 - SPI1CON2SET register"]
    pub con2set: crate::Reg<con2set::CON2SET_SPEC>,
    #[doc = "0x4c - SPI1CON2INV register"]
    pub con2inv: crate::Reg<con2inv::CON2INV_SPEC>,
}
#[doc = "CON1 register accessor: an alias for `Reg<CON1_SPEC>`"]
pub type CON1 = crate::Reg<con1::CON1_SPEC>;
#[doc = "SPI1CON register"]
pub mod con1;
#[doc = "CON1CLR register accessor: an alias for `Reg<CON1CLR_SPEC>`"]
pub type CON1CLR = crate::Reg<con1clr::CON1CLR_SPEC>;
#[doc = "SPI1CONCLR register"]
pub mod con1clr;
#[doc = "CON1SET register accessor: an alias for `Reg<CON1SET_SPEC>`"]
pub type CON1SET = crate::Reg<con1set::CON1SET_SPEC>;
#[doc = "SPI1CONSET register"]
pub mod con1set;
#[doc = "CON1INV register accessor: an alias for `Reg<CON1INV_SPEC>`"]
pub type CON1INV = crate::Reg<con1inv::CON1INV_SPEC>;
#[doc = "SPI1CONINV register"]
pub mod con1inv;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPI1STAT register"]
pub mod stat;
#[doc = "STATCLR register accessor: an alias for `Reg<STATCLR_SPEC>`"]
pub type STATCLR = crate::Reg<statclr::STATCLR_SPEC>;
#[doc = "SPI1STATCLR register"]
pub mod statclr;
#[doc = "STATSET register accessor: an alias for `Reg<STATSET_SPEC>`"]
pub type STATSET = crate::Reg<statset::STATSET_SPEC>;
#[doc = "SPI1STATSET register"]
pub mod statset;
#[doc = "STATINV register accessor: an alias for `Reg<STATINV_SPEC>`"]
pub type STATINV = crate::Reg<statinv::STATINV_SPEC>;
#[doc = "SPI1STATINV register"]
pub mod statinv;
#[doc = "BUF register accessor: an alias for `Reg<BUF_SPEC>`"]
pub type BUF = crate::Reg<buf::BUF_SPEC>;
#[doc = "SPI1BUF register"]
pub mod buf;
#[doc = "BRG register accessor: an alias for `Reg<BRG_SPEC>`"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "SPI1BRG register"]
pub mod brg;
#[doc = "BRGCLR register accessor: an alias for `Reg<BRGCLR_SPEC>`"]
pub type BRGCLR = crate::Reg<brgclr::BRGCLR_SPEC>;
#[doc = "SPI1BRGCLR register"]
pub mod brgclr;
#[doc = "BRGSET register accessor: an alias for `Reg<BRGSET_SPEC>`"]
pub type BRGSET = crate::Reg<brgset::BRGSET_SPEC>;
#[doc = "SPI1BRGSET register"]
pub mod brgset;
#[doc = "BRGINV register accessor: an alias for `Reg<BRGINV_SPEC>`"]
pub type BRGINV = crate::Reg<brginv::BRGINV_SPEC>;
#[doc = "SPI1BRGINV register"]
pub mod brginv;
#[doc = "CON2 register accessor: an alias for `Reg<CON2_SPEC>`"]
pub type CON2 = crate::Reg<con2::CON2_SPEC>;
#[doc = "SPI1CON2 register"]
pub mod con2;
#[doc = "CON2CLR register accessor: an alias for `Reg<CON2CLR_SPEC>`"]
pub type CON2CLR = crate::Reg<con2clr::CON2CLR_SPEC>;
#[doc = "SPI1CON2CLR register"]
pub mod con2clr;
#[doc = "CON2SET register accessor: an alias for `Reg<CON2SET_SPEC>`"]
pub type CON2SET = crate::Reg<con2set::CON2SET_SPEC>;
#[doc = "SPI1CON2SET register"]
pub mod con2set;
#[doc = "CON2INV register accessor: an alias for `Reg<CON2INV_SPEC>`"]
pub type CON2INV = crate::Reg<con2inv::CON2INV_SPEC>;
#[doc = "SPI1CON2INV register"]
pub mod con2inv;
