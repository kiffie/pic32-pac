#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CFGCON register"]
    pub cfgcon: crate::Reg<cfgcon::CFGCON_SPEC>,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - DEVID register"]
    pub devid: crate::Reg<devid::DEVID_SPEC>,
    _reserved2: [u8; 12usize],
    #[doc = "0x30 - SYSKEY register"]
    pub syskey: crate::Reg<syskey::SYSKEY_SPEC>,
    #[doc = "0x34 - SYSKEYCLR register"]
    pub syskeyclr: crate::Reg<syskeyclr::SYSKEYCLR_SPEC>,
    #[doc = "0x38 - SYSKEYSET register"]
    pub syskeyset: crate::Reg<syskeyset::SYSKEYSET_SPEC>,
    #[doc = "0x3c - SYSKEYINV register"]
    pub syskeyinv: crate::Reg<syskeyinv::SYSKEYINV_SPEC>,
    #[doc = "0x40 - PMD1 register"]
    pub pmd1: crate::Reg<pmd1::PMD1_SPEC>,
    #[doc = "0x44 - PMD1CLR register"]
    pub pmd1clr: crate::Reg<pmd1clr::PMD1CLR_SPEC>,
    #[doc = "0x48 - PMD1SET register"]
    pub pmd1set: crate::Reg<pmd1set::PMD1SET_SPEC>,
    #[doc = "0x4c - PMD1INV register"]
    pub pmd1inv: crate::Reg<pmd1inv::PMD1INV_SPEC>,
    #[doc = "0x50 - PMD2 register"]
    pub pmd2: crate::Reg<pmd2::PMD2_SPEC>,
    #[doc = "0x54 - PMD2CLR register"]
    pub pmd2clr: crate::Reg<pmd2clr::PMD2CLR_SPEC>,
    #[doc = "0x58 - PMD2SET register"]
    pub pmd2set: crate::Reg<pmd2set::PMD2SET_SPEC>,
    #[doc = "0x5c - PMD2INV register"]
    pub pmd2inv: crate::Reg<pmd2inv::PMD2INV_SPEC>,
    #[doc = "0x60 - PMD3 register"]
    pub pmd3: crate::Reg<pmd3::PMD3_SPEC>,
    #[doc = "0x64 - PMD3CLR register"]
    pub pmd3clr: crate::Reg<pmd3clr::PMD3CLR_SPEC>,
    #[doc = "0x68 - PMD3SET register"]
    pub pmd3set: crate::Reg<pmd3set::PMD3SET_SPEC>,
    #[doc = "0x6c - PMD3INV register"]
    pub pmd3inv: crate::Reg<pmd3inv::PMD3INV_SPEC>,
    #[doc = "0x70 - PMD4 register"]
    pub pmd4: crate::Reg<pmd4::PMD4_SPEC>,
    #[doc = "0x74 - PMD4CLR register"]
    pub pmd4clr: crate::Reg<pmd4clr::PMD4CLR_SPEC>,
    #[doc = "0x78 - PMD4SET register"]
    pub pmd4set: crate::Reg<pmd4set::PMD4SET_SPEC>,
    #[doc = "0x7c - PMD4INV register"]
    pub pmd4inv: crate::Reg<pmd4inv::PMD4INV_SPEC>,
    #[doc = "0x80 - PMD5 register"]
    pub pmd5: crate::Reg<pmd5::PMD5_SPEC>,
    #[doc = "0x84 - PMD5CLR register"]
    pub pmd5clr: crate::Reg<pmd5clr::PMD5CLR_SPEC>,
    #[doc = "0x88 - PMD5SET register"]
    pub pmd5set: crate::Reg<pmd5set::PMD5SET_SPEC>,
    #[doc = "0x8c - PMD5INV register"]
    pub pmd5inv: crate::Reg<pmd5inv::PMD5INV_SPEC>,
    #[doc = "0x90 - PMD6 register"]
    pub pmd6: crate::Reg<pmd6::PMD6_SPEC>,
    #[doc = "0x94 - PMD6CLR register"]
    pub pmd6clr: crate::Reg<pmd6clr::PMD6CLR_SPEC>,
    #[doc = "0x98 - PMD6SET register"]
    pub pmd6set: crate::Reg<pmd6set::PMD6SET_SPEC>,
    #[doc = "0x9c - PMD6INV register"]
    pub pmd6inv: crate::Reg<pmd6inv::PMD6INV_SPEC>,
}
#[doc = "CFGCON register accessor: an alias for `Reg<CFGCON_SPEC>`"]
pub type CFGCON = crate::Reg<cfgcon::CFGCON_SPEC>;
#[doc = "CFGCON register"]
pub mod cfgcon;
#[doc = "DEVID register accessor: an alias for `Reg<DEVID_SPEC>`"]
pub type DEVID = crate::Reg<devid::DEVID_SPEC>;
#[doc = "DEVID register"]
pub mod devid;
#[doc = "SYSKEY register accessor: an alias for `Reg<SYSKEY_SPEC>`"]
pub type SYSKEY = crate::Reg<syskey::SYSKEY_SPEC>;
#[doc = "SYSKEY register"]
pub mod syskey;
#[doc = "SYSKEYCLR register accessor: an alias for `Reg<SYSKEYCLR_SPEC>`"]
pub type SYSKEYCLR = crate::Reg<syskeyclr::SYSKEYCLR_SPEC>;
#[doc = "SYSKEYCLR register"]
pub mod syskeyclr;
#[doc = "SYSKEYSET register accessor: an alias for `Reg<SYSKEYSET_SPEC>`"]
pub type SYSKEYSET = crate::Reg<syskeyset::SYSKEYSET_SPEC>;
#[doc = "SYSKEYSET register"]
pub mod syskeyset;
#[doc = "SYSKEYINV register accessor: an alias for `Reg<SYSKEYINV_SPEC>`"]
pub type SYSKEYINV = crate::Reg<syskeyinv::SYSKEYINV_SPEC>;
#[doc = "SYSKEYINV register"]
pub mod syskeyinv;
#[doc = "PMD1 register accessor: an alias for `Reg<PMD1_SPEC>`"]
pub type PMD1 = crate::Reg<pmd1::PMD1_SPEC>;
#[doc = "PMD1 register"]
pub mod pmd1;
#[doc = "PMD1CLR register accessor: an alias for `Reg<PMD1CLR_SPEC>`"]
pub type PMD1CLR = crate::Reg<pmd1clr::PMD1CLR_SPEC>;
#[doc = "PMD1CLR register"]
pub mod pmd1clr;
#[doc = "PMD1SET register accessor: an alias for `Reg<PMD1SET_SPEC>`"]
pub type PMD1SET = crate::Reg<pmd1set::PMD1SET_SPEC>;
#[doc = "PMD1SET register"]
pub mod pmd1set;
#[doc = "PMD1INV register accessor: an alias for `Reg<PMD1INV_SPEC>`"]
pub type PMD1INV = crate::Reg<pmd1inv::PMD1INV_SPEC>;
#[doc = "PMD1INV register"]
pub mod pmd1inv;
#[doc = "PMD2 register accessor: an alias for `Reg<PMD2_SPEC>`"]
pub type PMD2 = crate::Reg<pmd2::PMD2_SPEC>;
#[doc = "PMD2 register"]
pub mod pmd2;
#[doc = "PMD2CLR register accessor: an alias for `Reg<PMD2CLR_SPEC>`"]
pub type PMD2CLR = crate::Reg<pmd2clr::PMD2CLR_SPEC>;
#[doc = "PMD2CLR register"]
pub mod pmd2clr;
#[doc = "PMD2SET register accessor: an alias for `Reg<PMD2SET_SPEC>`"]
pub type PMD2SET = crate::Reg<pmd2set::PMD2SET_SPEC>;
#[doc = "PMD2SET register"]
pub mod pmd2set;
#[doc = "PMD2INV register accessor: an alias for `Reg<PMD2INV_SPEC>`"]
pub type PMD2INV = crate::Reg<pmd2inv::PMD2INV_SPEC>;
#[doc = "PMD2INV register"]
pub mod pmd2inv;
#[doc = "PMD3 register accessor: an alias for `Reg<PMD3_SPEC>`"]
pub type PMD3 = crate::Reg<pmd3::PMD3_SPEC>;
#[doc = "PMD3 register"]
pub mod pmd3;
#[doc = "PMD3CLR register accessor: an alias for `Reg<PMD3CLR_SPEC>`"]
pub type PMD3CLR = crate::Reg<pmd3clr::PMD3CLR_SPEC>;
#[doc = "PMD3CLR register"]
pub mod pmd3clr;
#[doc = "PMD3SET register accessor: an alias for `Reg<PMD3SET_SPEC>`"]
pub type PMD3SET = crate::Reg<pmd3set::PMD3SET_SPEC>;
#[doc = "PMD3SET register"]
pub mod pmd3set;
#[doc = "PMD3INV register accessor: an alias for `Reg<PMD3INV_SPEC>`"]
pub type PMD3INV = crate::Reg<pmd3inv::PMD3INV_SPEC>;
#[doc = "PMD3INV register"]
pub mod pmd3inv;
#[doc = "PMD4 register accessor: an alias for `Reg<PMD4_SPEC>`"]
pub type PMD4 = crate::Reg<pmd4::PMD4_SPEC>;
#[doc = "PMD4 register"]
pub mod pmd4;
#[doc = "PMD4CLR register accessor: an alias for `Reg<PMD4CLR_SPEC>`"]
pub type PMD4CLR = crate::Reg<pmd4clr::PMD4CLR_SPEC>;
#[doc = "PMD4CLR register"]
pub mod pmd4clr;
#[doc = "PMD4SET register accessor: an alias for `Reg<PMD4SET_SPEC>`"]
pub type PMD4SET = crate::Reg<pmd4set::PMD4SET_SPEC>;
#[doc = "PMD4SET register"]
pub mod pmd4set;
#[doc = "PMD4INV register accessor: an alias for `Reg<PMD4INV_SPEC>`"]
pub type PMD4INV = crate::Reg<pmd4inv::PMD4INV_SPEC>;
#[doc = "PMD4INV register"]
pub mod pmd4inv;
#[doc = "PMD5 register accessor: an alias for `Reg<PMD5_SPEC>`"]
pub type PMD5 = crate::Reg<pmd5::PMD5_SPEC>;
#[doc = "PMD5 register"]
pub mod pmd5;
#[doc = "PMD5CLR register accessor: an alias for `Reg<PMD5CLR_SPEC>`"]
pub type PMD5CLR = crate::Reg<pmd5clr::PMD5CLR_SPEC>;
#[doc = "PMD5CLR register"]
pub mod pmd5clr;
#[doc = "PMD5SET register accessor: an alias for `Reg<PMD5SET_SPEC>`"]
pub type PMD5SET = crate::Reg<pmd5set::PMD5SET_SPEC>;
#[doc = "PMD5SET register"]
pub mod pmd5set;
#[doc = "PMD5INV register accessor: an alias for `Reg<PMD5INV_SPEC>`"]
pub type PMD5INV = crate::Reg<pmd5inv::PMD5INV_SPEC>;
#[doc = "PMD5INV register"]
pub mod pmd5inv;
#[doc = "PMD6 register accessor: an alias for `Reg<PMD6_SPEC>`"]
pub type PMD6 = crate::Reg<pmd6::PMD6_SPEC>;
#[doc = "PMD6 register"]
pub mod pmd6;
#[doc = "PMD6CLR register accessor: an alias for `Reg<PMD6CLR_SPEC>`"]
pub type PMD6CLR = crate::Reg<pmd6clr::PMD6CLR_SPEC>;
#[doc = "PMD6CLR register"]
pub mod pmd6clr;
#[doc = "PMD6SET register accessor: an alias for `Reg<PMD6SET_SPEC>`"]
pub type PMD6SET = crate::Reg<pmd6set::PMD6SET_SPEC>;
#[doc = "PMD6SET register"]
pub mod pmd6set;
#[doc = "PMD6INV register accessor: an alias for `Reg<PMD6INV_SPEC>`"]
pub type PMD6INV = crate::Reg<pmd6inv::PMD6INV_SPEC>;
#[doc = "PMD6INV register"]
pub mod pmd6inv;
