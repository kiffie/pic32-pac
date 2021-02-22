#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C1CON register"]
    pub cont: crate::Reg<cont::CONT_SPEC>,
    #[doc = "0x04 - I2C1CONCLR register"]
    pub contclr: crate::Reg<contclr::CONTCLR_SPEC>,
    #[doc = "0x08 - I2C1CONSET register"]
    pub contset: crate::Reg<contset::CONTSET_SPEC>,
    #[doc = "0x0c - I2C1CONINV register"]
    pub continv: crate::Reg<continv::CONTINV_SPEC>,
    #[doc = "0x10 - I2C1STAT register"]
    pub stat: crate::Reg<stat::STAT_SPEC>,
    #[doc = "0x14 - I2C1STATCLR register"]
    pub statclr: crate::Reg<statclr::STATCLR_SPEC>,
    #[doc = "0x18 - I2C1STATSET register"]
    pub statset: crate::Reg<statset::STATSET_SPEC>,
    #[doc = "0x1c - I2C1STATINV register"]
    pub statinv: crate::Reg<statinv::STATINV_SPEC>,
    #[doc = "0x20 - I2C1ADD register"]
    pub add: crate::Reg<add::ADD_SPEC>,
    #[doc = "0x24 - I2C1ADDCLR register"]
    pub addclr: crate::Reg<addclr::ADDCLR_SPEC>,
    #[doc = "0x28 - I2C1ADDSET register"]
    pub addset: crate::Reg<addset::ADDSET_SPEC>,
    #[doc = "0x2c - I2C1ADDINV register"]
    pub addinv: crate::Reg<addinv::ADDINV_SPEC>,
    #[doc = "0x30 - I2C1MSK register"]
    pub msk: crate::Reg<msk::MSK_SPEC>,
    #[doc = "0x34 - I2C1MSKCLR register"]
    pub mskclr: crate::Reg<mskclr::MSKCLR_SPEC>,
    #[doc = "0x38 - I2C1MSKSET register"]
    pub mskset: crate::Reg<mskset::MSKSET_SPEC>,
    #[doc = "0x3c - I2C1MSKINV register"]
    pub mskinv: crate::Reg<mskinv::MSKINV_SPEC>,
    #[doc = "0x40 - I2C1BRG register"]
    pub brg: crate::Reg<brg::BRG_SPEC>,
    #[doc = "0x44 - I2C1BRGCLR register"]
    pub brgclr: crate::Reg<brgclr::BRGCLR_SPEC>,
    #[doc = "0x48 - I2C1BRGSET register"]
    pub brgset: crate::Reg<brgset::BRGSET_SPEC>,
    #[doc = "0x4c - I2C1BRGINV register"]
    pub brginv: crate::Reg<brginv::BRGINV_SPEC>,
    #[doc = "0x50 - I2C1TRN register"]
    pub trn: crate::Reg<trn::TRN_SPEC>,
    #[doc = "0x54 - I2C1TRNCLR register"]
    pub trnclr: crate::Reg<trnclr::TRNCLR_SPEC>,
    #[doc = "0x58 - I2C1TRNSET register"]
    pub trnset: crate::Reg<trnset::TRNSET_SPEC>,
    #[doc = "0x5c - I2C1TRNINV register"]
    pub trninv: crate::Reg<trninv::TRNINV_SPEC>,
    #[doc = "0x60 - I2C1RCV register"]
    pub rcv: crate::Reg<rcv::RCV_SPEC>,
}
#[doc = "CONT register accessor: an alias for `Reg<CONT_SPEC>`"]
pub type CONT = crate::Reg<cont::CONT_SPEC>;
#[doc = "I2C1CON register"]
pub mod cont;
#[doc = "CONTCLR register accessor: an alias for `Reg<CONTCLR_SPEC>`"]
pub type CONTCLR = crate::Reg<contclr::CONTCLR_SPEC>;
#[doc = "I2C1CONCLR register"]
pub mod contclr;
#[doc = "CONTSET register accessor: an alias for `Reg<CONTSET_SPEC>`"]
pub type CONTSET = crate::Reg<contset::CONTSET_SPEC>;
#[doc = "I2C1CONSET register"]
pub mod contset;
#[doc = "CONTINV register accessor: an alias for `Reg<CONTINV_SPEC>`"]
pub type CONTINV = crate::Reg<continv::CONTINV_SPEC>;
#[doc = "I2C1CONINV register"]
pub mod continv;
#[doc = "STAT register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "I2C1STAT register"]
pub mod stat;
#[doc = "STATCLR register accessor: an alias for `Reg<STATCLR_SPEC>`"]
pub type STATCLR = crate::Reg<statclr::STATCLR_SPEC>;
#[doc = "I2C1STATCLR register"]
pub mod statclr;
#[doc = "STATSET register accessor: an alias for `Reg<STATSET_SPEC>`"]
pub type STATSET = crate::Reg<statset::STATSET_SPEC>;
#[doc = "I2C1STATSET register"]
pub mod statset;
#[doc = "STATINV register accessor: an alias for `Reg<STATINV_SPEC>`"]
pub type STATINV = crate::Reg<statinv::STATINV_SPEC>;
#[doc = "I2C1STATINV register"]
pub mod statinv;
#[doc = "ADD register accessor: an alias for `Reg<ADD_SPEC>`"]
pub type ADD = crate::Reg<add::ADD_SPEC>;
#[doc = "I2C1ADD register"]
pub mod add;
#[doc = "ADDCLR register accessor: an alias for `Reg<ADDCLR_SPEC>`"]
pub type ADDCLR = crate::Reg<addclr::ADDCLR_SPEC>;
#[doc = "I2C1ADDCLR register"]
pub mod addclr;
#[doc = "ADDSET register accessor: an alias for `Reg<ADDSET_SPEC>`"]
pub type ADDSET = crate::Reg<addset::ADDSET_SPEC>;
#[doc = "I2C1ADDSET register"]
pub mod addset;
#[doc = "ADDINV register accessor: an alias for `Reg<ADDINV_SPEC>`"]
pub type ADDINV = crate::Reg<addinv::ADDINV_SPEC>;
#[doc = "I2C1ADDINV register"]
pub mod addinv;
#[doc = "MSK register accessor: an alias for `Reg<MSK_SPEC>`"]
pub type MSK = crate::Reg<msk::MSK_SPEC>;
#[doc = "I2C1MSK register"]
pub mod msk;
#[doc = "MSKCLR register accessor: an alias for `Reg<MSKCLR_SPEC>`"]
pub type MSKCLR = crate::Reg<mskclr::MSKCLR_SPEC>;
#[doc = "I2C1MSKCLR register"]
pub mod mskclr;
#[doc = "MSKSET register accessor: an alias for `Reg<MSKSET_SPEC>`"]
pub type MSKSET = crate::Reg<mskset::MSKSET_SPEC>;
#[doc = "I2C1MSKSET register"]
pub mod mskset;
#[doc = "MSKINV register accessor: an alias for `Reg<MSKINV_SPEC>`"]
pub type MSKINV = crate::Reg<mskinv::MSKINV_SPEC>;
#[doc = "I2C1MSKINV register"]
pub mod mskinv;
#[doc = "BRG register accessor: an alias for `Reg<BRG_SPEC>`"]
pub type BRG = crate::Reg<brg::BRG_SPEC>;
#[doc = "I2C1BRG register"]
pub mod brg;
#[doc = "BRGCLR register accessor: an alias for `Reg<BRGCLR_SPEC>`"]
pub type BRGCLR = crate::Reg<brgclr::BRGCLR_SPEC>;
#[doc = "I2C1BRGCLR register"]
pub mod brgclr;
#[doc = "BRGSET register accessor: an alias for `Reg<BRGSET_SPEC>`"]
pub type BRGSET = crate::Reg<brgset::BRGSET_SPEC>;
#[doc = "I2C1BRGSET register"]
pub mod brgset;
#[doc = "BRGINV register accessor: an alias for `Reg<BRGINV_SPEC>`"]
pub type BRGINV = crate::Reg<brginv::BRGINV_SPEC>;
#[doc = "I2C1BRGINV register"]
pub mod brginv;
#[doc = "TRN register accessor: an alias for `Reg<TRN_SPEC>`"]
pub type TRN = crate::Reg<trn::TRN_SPEC>;
#[doc = "I2C1TRN register"]
pub mod trn;
#[doc = "TRNCLR register accessor: an alias for `Reg<TRNCLR_SPEC>`"]
pub type TRNCLR = crate::Reg<trnclr::TRNCLR_SPEC>;
#[doc = "I2C1TRNCLR register"]
pub mod trnclr;
#[doc = "TRNSET register accessor: an alias for `Reg<TRNSET_SPEC>`"]
pub type TRNSET = crate::Reg<trnset::TRNSET_SPEC>;
#[doc = "I2C1TRNSET register"]
pub mod trnset;
#[doc = "TRNINV register accessor: an alias for `Reg<TRNINV_SPEC>`"]
pub type TRNINV = crate::Reg<trninv::TRNINV_SPEC>;
#[doc = "I2C1TRNINV register"]
pub mod trninv;
#[doc = "RCV register accessor: an alias for `Reg<RCV_SPEC>`"]
pub type RCV = crate::Reg<rcv::RCV_SPEC>;
#[doc = "I2C1RCV register"]
pub mod rcv;
