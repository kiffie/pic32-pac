#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CHECON register"]
    pub checon: crate::Reg<checon::CHECON_SPEC>,
    #[doc = "0x04 - CHECONCLR register"]
    pub checonclr: crate::Reg<checonclr::CHECONCLR_SPEC>,
    #[doc = "0x08 - CHECONSET register"]
    pub checonset: crate::Reg<checonset::CHECONSET_SPEC>,
    #[doc = "0x0c - CHECONINV register"]
    pub checoninv: crate::Reg<checoninv::CHECONINV_SPEC>,
    #[doc = "0x10 - CHEACC register"]
    pub cheacc: crate::Reg<cheacc::CHEACC_SPEC>,
    #[doc = "0x14 - CHEACCCLR register"]
    pub cheaccclr: crate::Reg<cheaccclr::CHEACCCLR_SPEC>,
    #[doc = "0x18 - CHEACCSET register"]
    pub cheaccset: crate::Reg<cheaccset::CHEACCSET_SPEC>,
    #[doc = "0x1c - CHEACCINV register"]
    pub cheaccinv: crate::Reg<cheaccinv::CHEACCINV_SPEC>,
    #[doc = "0x20 - CHETAG register"]
    pub chetag: crate::Reg<chetag::CHETAG_SPEC>,
    #[doc = "0x24 - CHETAGCLR register"]
    pub chetagclr: crate::Reg<chetagclr::CHETAGCLR_SPEC>,
    #[doc = "0x28 - CHETAGSET register"]
    pub chetagset: crate::Reg<chetagset::CHETAGSET_SPEC>,
    #[doc = "0x2c - CHETAGINV register"]
    pub chetaginv: crate::Reg<chetaginv::CHETAGINV_SPEC>,
    #[doc = "0x30 - CHEMSK register"]
    pub chemsk: crate::Reg<chemsk::CHEMSK_SPEC>,
    #[doc = "0x34 - CHEMSKCLR register"]
    pub chemskclr: crate::Reg<chemskclr::CHEMSKCLR_SPEC>,
    #[doc = "0x38 - CHEMSKSET register"]
    pub chemskset: crate::Reg<chemskset::CHEMSKSET_SPEC>,
    #[doc = "0x3c - CHEMSKINV register"]
    pub chemskinv: crate::Reg<chemskinv::CHEMSKINV_SPEC>,
    #[doc = "0x40 - CHEW0 register"]
    pub chew0: crate::Reg<chew0::CHEW0_SPEC>,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - CHEW1 register"]
    pub chew1: crate::Reg<chew1::CHEW1_SPEC>,
    _reserved18: [u8; 12usize],
    #[doc = "0x60 - CHEW2 register"]
    pub chew2: crate::Reg<chew2::CHEW2_SPEC>,
    _reserved19: [u8; 12usize],
    #[doc = "0x70 - CHEW3 register"]
    pub chew3: crate::Reg<chew3::CHEW3_SPEC>,
    _reserved20: [u8; 12usize],
    #[doc = "0x80 - CHELRU register"]
    pub chelru: crate::Reg<chelru::CHELRU_SPEC>,
    _reserved21: [u8; 12usize],
    #[doc = "0x90 - CHEHIT register"]
    pub chehit: crate::Reg<chehit::CHEHIT_SPEC>,
    _reserved22: [u8; 12usize],
    #[doc = "0xa0 - CHEMIS register"]
    pub chemis: crate::Reg<chemis::CHEMIS_SPEC>,
    _reserved23: [u8; 12usize],
    #[doc = "0xb0 - RESERVED1 register"]
    pub reserved1: crate::Reg<reserved1::RESERVED1_SPEC>,
    _reserved24: [u8; 12usize],
    #[doc = "0xc0 - CHEPFABT register"]
    pub chepfabt: crate::Reg<chepfabt::CHEPFABT_SPEC>,
}
#[doc = "CHECON register accessor: an alias for `Reg<CHECON_SPEC>`"]
pub type CHECON = crate::Reg<checon::CHECON_SPEC>;
#[doc = "CHECON register"]
pub mod checon;
#[doc = "CHECONCLR register accessor: an alias for `Reg<CHECONCLR_SPEC>`"]
pub type CHECONCLR = crate::Reg<checonclr::CHECONCLR_SPEC>;
#[doc = "CHECONCLR register"]
pub mod checonclr;
#[doc = "CHECONSET register accessor: an alias for `Reg<CHECONSET_SPEC>`"]
pub type CHECONSET = crate::Reg<checonset::CHECONSET_SPEC>;
#[doc = "CHECONSET register"]
pub mod checonset;
#[doc = "CHECONINV register accessor: an alias for `Reg<CHECONINV_SPEC>`"]
pub type CHECONINV = crate::Reg<checoninv::CHECONINV_SPEC>;
#[doc = "CHECONINV register"]
pub mod checoninv;
#[doc = "CHEACC register accessor: an alias for `Reg<CHEACC_SPEC>`"]
pub type CHEACC = crate::Reg<cheacc::CHEACC_SPEC>;
#[doc = "CHEACC register"]
pub mod cheacc;
#[doc = "CHEACCCLR register accessor: an alias for `Reg<CHEACCCLR_SPEC>`"]
pub type CHEACCCLR = crate::Reg<cheaccclr::CHEACCCLR_SPEC>;
#[doc = "CHEACCCLR register"]
pub mod cheaccclr;
#[doc = "CHEACCSET register accessor: an alias for `Reg<CHEACCSET_SPEC>`"]
pub type CHEACCSET = crate::Reg<cheaccset::CHEACCSET_SPEC>;
#[doc = "CHEACCSET register"]
pub mod cheaccset;
#[doc = "CHEACCINV register accessor: an alias for `Reg<CHEACCINV_SPEC>`"]
pub type CHEACCINV = crate::Reg<cheaccinv::CHEACCINV_SPEC>;
#[doc = "CHEACCINV register"]
pub mod cheaccinv;
#[doc = "CHETAG register accessor: an alias for `Reg<CHETAG_SPEC>`"]
pub type CHETAG = crate::Reg<chetag::CHETAG_SPEC>;
#[doc = "CHETAG register"]
pub mod chetag;
#[doc = "CHETAGCLR register accessor: an alias for `Reg<CHETAGCLR_SPEC>`"]
pub type CHETAGCLR = crate::Reg<chetagclr::CHETAGCLR_SPEC>;
#[doc = "CHETAGCLR register"]
pub mod chetagclr;
#[doc = "CHETAGSET register accessor: an alias for `Reg<CHETAGSET_SPEC>`"]
pub type CHETAGSET = crate::Reg<chetagset::CHETAGSET_SPEC>;
#[doc = "CHETAGSET register"]
pub mod chetagset;
#[doc = "CHETAGINV register accessor: an alias for `Reg<CHETAGINV_SPEC>`"]
pub type CHETAGINV = crate::Reg<chetaginv::CHETAGINV_SPEC>;
#[doc = "CHETAGINV register"]
pub mod chetaginv;
#[doc = "CHEMSK register accessor: an alias for `Reg<CHEMSK_SPEC>`"]
pub type CHEMSK = crate::Reg<chemsk::CHEMSK_SPEC>;
#[doc = "CHEMSK register"]
pub mod chemsk;
#[doc = "CHEMSKCLR register accessor: an alias for `Reg<CHEMSKCLR_SPEC>`"]
pub type CHEMSKCLR = crate::Reg<chemskclr::CHEMSKCLR_SPEC>;
#[doc = "CHEMSKCLR register"]
pub mod chemskclr;
#[doc = "CHEMSKSET register accessor: an alias for `Reg<CHEMSKSET_SPEC>`"]
pub type CHEMSKSET = crate::Reg<chemskset::CHEMSKSET_SPEC>;
#[doc = "CHEMSKSET register"]
pub mod chemskset;
#[doc = "CHEMSKINV register accessor: an alias for `Reg<CHEMSKINV_SPEC>`"]
pub type CHEMSKINV = crate::Reg<chemskinv::CHEMSKINV_SPEC>;
#[doc = "CHEMSKINV register"]
pub mod chemskinv;
#[doc = "CHEW0 register accessor: an alias for `Reg<CHEW0_SPEC>`"]
pub type CHEW0 = crate::Reg<chew0::CHEW0_SPEC>;
#[doc = "CHEW0 register"]
pub mod chew0;
#[doc = "CHEW1 register accessor: an alias for `Reg<CHEW1_SPEC>`"]
pub type CHEW1 = crate::Reg<chew1::CHEW1_SPEC>;
#[doc = "CHEW1 register"]
pub mod chew1;
#[doc = "CHEW2 register accessor: an alias for `Reg<CHEW2_SPEC>`"]
pub type CHEW2 = crate::Reg<chew2::CHEW2_SPEC>;
#[doc = "CHEW2 register"]
pub mod chew2;
#[doc = "CHEW3 register accessor: an alias for `Reg<CHEW3_SPEC>`"]
pub type CHEW3 = crate::Reg<chew3::CHEW3_SPEC>;
#[doc = "CHEW3 register"]
pub mod chew3;
#[doc = "CHELRU register accessor: an alias for `Reg<CHELRU_SPEC>`"]
pub type CHELRU = crate::Reg<chelru::CHELRU_SPEC>;
#[doc = "CHELRU register"]
pub mod chelru;
#[doc = "CHEHIT register accessor: an alias for `Reg<CHEHIT_SPEC>`"]
pub type CHEHIT = crate::Reg<chehit::CHEHIT_SPEC>;
#[doc = "CHEHIT register"]
pub mod chehit;
#[doc = "CHEMIS register accessor: an alias for `Reg<CHEMIS_SPEC>`"]
pub type CHEMIS = crate::Reg<chemis::CHEMIS_SPEC>;
#[doc = "CHEMIS register"]
pub mod chemis;
#[doc = "RESERVED1 register accessor: an alias for `Reg<RESERVED1_SPEC>`"]
pub type RESERVED1 = crate::Reg<reserved1::RESERVED1_SPEC>;
#[doc = "RESERVED1 register"]
pub mod reserved1;
#[doc = "CHEPFABT register accessor: an alias for `Reg<CHEPFABT_SPEC>`"]
pub type CHEPFABT = crate::Reg<chepfabt::CHEPFABT_SPEC>;
#[doc = "CHEPFABT register"]
pub mod chepfabt;
