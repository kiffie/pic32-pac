#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CFGCON register"]
    pub cfgcon: CFGCON,
    _reserved0: [u8; 28usize],
    #[doc = "0x20 - DEVID register"]
    pub devid: DEVID,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - SYSKEY register"]
    pub syskey: SYSKEY,
    #[doc = "0x34 - SYSKEYCLR register"]
    pub syskeyclr: SYSKEYCLR,
    #[doc = "0x38 - SYSKEYSET register"]
    pub syskeyset: SYSKEYSET,
    #[doc = "0x3c - SYSKEYINV register"]
    pub syskeyinv: SYSKEYINV,
    #[doc = "0x40 - PMD1 register"]
    pub pmd1: PMD1,
    #[doc = "0x44 - PMD1CLR register"]
    pub pmd1clr: PMD1CLR,
    #[doc = "0x48 - PMD1SET register"]
    pub pmd1set: PMD1SET,
    #[doc = "0x4c - PMD1INV register"]
    pub pmd1inv: PMD1INV,
    #[doc = "0x50 - PMD2 register"]
    pub pmd2: PMD2,
    #[doc = "0x54 - PMD2CLR register"]
    pub pmd2clr: PMD2CLR,
    #[doc = "0x58 - PMD2SET register"]
    pub pmd2set: PMD2SET,
    #[doc = "0x5c - PMD2INV register"]
    pub pmd2inv: PMD2INV,
    #[doc = "0x60 - PMD3 register"]
    pub pmd3: PMD3,
    #[doc = "0x64 - PMD3CLR register"]
    pub pmd3clr: PMD3CLR,
    #[doc = "0x68 - PMD3SET register"]
    pub pmd3set: PMD3SET,
    #[doc = "0x6c - PMD3INV register"]
    pub pmd3inv: PMD3INV,
    #[doc = "0x70 - PMD4 register"]
    pub pmd4: PMD4,
    #[doc = "0x74 - PMD4CLR register"]
    pub pmd4clr: PMD4CLR,
    #[doc = "0x78 - PMD4SET register"]
    pub pmd4set: PMD4SET,
    #[doc = "0x7c - PMD4INV register"]
    pub pmd4inv: PMD4INV,
    #[doc = "0x80 - PMD5 register"]
    pub pmd5: PMD5,
    #[doc = "0x84 - PMD5CLR register"]
    pub pmd5clr: PMD5CLR,
    #[doc = "0x88 - PMD5SET register"]
    pub pmd5set: PMD5SET,
    #[doc = "0x8c - PMD5INV register"]
    pub pmd5inv: PMD5INV,
    #[doc = "0x90 - PMD6 register"]
    pub pmd6: PMD6,
    #[doc = "0x94 - PMD6CLR register"]
    pub pmd6clr: PMD6CLR,
    #[doc = "0x98 - PMD6SET register"]
    pub pmd6set: PMD6SET,
    #[doc = "0x9c - PMD6INV register"]
    pub pmd6inv: PMD6INV,
}
#[doc = "CFGCON register"]
pub struct CFGCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CFGCON register"]
pub mod cfgcon;
#[doc = "DEVID register"]
pub struct DEVID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DEVID register"]
pub mod devid;
#[doc = "SYSKEY register"]
pub struct SYSKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSKEY register"]
pub mod syskey;
#[doc = "SYSKEYCLR register"]
pub struct SYSKEYCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSKEYCLR register"]
pub mod syskeyclr;
#[doc = "SYSKEYSET register"]
pub struct SYSKEYSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSKEYSET register"]
pub mod syskeyset;
#[doc = "SYSKEYINV register"]
pub struct SYSKEYINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SYSKEYINV register"]
pub mod syskeyinv;
#[doc = "PMD1 register"]
pub struct PMD1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD1 register"]
pub mod pmd1;
#[doc = "PMD1CLR register"]
pub struct PMD1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD1CLR register"]
pub mod pmd1clr;
#[doc = "PMD1SET register"]
pub struct PMD1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD1SET register"]
pub mod pmd1set;
#[doc = "PMD1INV register"]
pub struct PMD1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD1INV register"]
pub mod pmd1inv;
#[doc = "PMD2 register"]
pub struct PMD2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD2 register"]
pub mod pmd2;
#[doc = "PMD2CLR register"]
pub struct PMD2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD2CLR register"]
pub mod pmd2clr;
#[doc = "PMD2SET register"]
pub struct PMD2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD2SET register"]
pub mod pmd2set;
#[doc = "PMD2INV register"]
pub struct PMD2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD2INV register"]
pub mod pmd2inv;
#[doc = "PMD3 register"]
pub struct PMD3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD3 register"]
pub mod pmd3;
#[doc = "PMD3CLR register"]
pub struct PMD3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD3CLR register"]
pub mod pmd3clr;
#[doc = "PMD3SET register"]
pub struct PMD3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD3SET register"]
pub mod pmd3set;
#[doc = "PMD3INV register"]
pub struct PMD3INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD3INV register"]
pub mod pmd3inv;
#[doc = "PMD4 register"]
pub struct PMD4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD4 register"]
pub mod pmd4;
#[doc = "PMD4CLR register"]
pub struct PMD4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD4CLR register"]
pub mod pmd4clr;
#[doc = "PMD4SET register"]
pub struct PMD4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD4SET register"]
pub mod pmd4set;
#[doc = "PMD4INV register"]
pub struct PMD4INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD4INV register"]
pub mod pmd4inv;
#[doc = "PMD5 register"]
pub struct PMD5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD5 register"]
pub mod pmd5;
#[doc = "PMD5CLR register"]
pub struct PMD5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD5CLR register"]
pub mod pmd5clr;
#[doc = "PMD5SET register"]
pub struct PMD5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD5SET register"]
pub mod pmd5set;
#[doc = "PMD5INV register"]
pub struct PMD5INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD5INV register"]
pub mod pmd5inv;
#[doc = "PMD6 register"]
pub struct PMD6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD6 register"]
pub mod pmd6;
#[doc = "PMD6CLR register"]
pub struct PMD6CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD6CLR register"]
pub mod pmd6clr;
#[doc = "PMD6SET register"]
pub struct PMD6SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD6SET register"]
pub mod pmd6set;
#[doc = "PMD6INV register"]
pub struct PMD6INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PMD6INV register"]
pub mod pmd6inv;
