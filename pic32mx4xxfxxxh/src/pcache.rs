#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CHECON register"]
    pub checon: CHECON,
    #[doc = "0x04 - CHECONCLR register"]
    pub checonclr: CHECONCLR,
    #[doc = "0x08 - CHECONSET register"]
    pub checonset: CHECONSET,
    #[doc = "0x0c - CHECONINV register"]
    pub checoninv: CHECONINV,
    #[doc = "0x10 - CHEACC register"]
    pub cheacc: CHEACC,
    #[doc = "0x14 - CHEACCCLR register"]
    pub cheaccclr: CHEACCCLR,
    #[doc = "0x18 - CHEACCSET register"]
    pub cheaccset: CHEACCSET,
    #[doc = "0x1c - CHEACCINV register"]
    pub cheaccinv: CHEACCINV,
    #[doc = "0x20 - CHETAG register"]
    pub chetag: CHETAG,
    #[doc = "0x24 - CHETAGCLR register"]
    pub chetagclr: CHETAGCLR,
    #[doc = "0x28 - CHETAGSET register"]
    pub chetagset: CHETAGSET,
    #[doc = "0x2c - CHETAGINV register"]
    pub chetaginv: CHETAGINV,
    #[doc = "0x30 - CHEMSK register"]
    pub chemsk: CHEMSK,
    #[doc = "0x34 - CHEMSKCLR register"]
    pub chemskclr: CHEMSKCLR,
    #[doc = "0x38 - CHEMSKSET register"]
    pub chemskset: CHEMSKSET,
    #[doc = "0x3c - CHEMSKINV register"]
    pub chemskinv: CHEMSKINV,
    #[doc = "0x40 - CHEW0 register"]
    pub chew0: CHEW0,
    _reserved0: [u8; 12usize],
    #[doc = "0x50 - CHEW1 register"]
    pub chew1: CHEW1,
    _reserved1: [u8; 12usize],
    #[doc = "0x60 - CHEW2 register"]
    pub chew2: CHEW2,
    _reserved2: [u8; 12usize],
    #[doc = "0x70 - CHEW3 register"]
    pub chew3: CHEW3,
    _reserved3: [u8; 12usize],
    #[doc = "0x80 - CHELRU register"]
    pub chelru: CHELRU,
    _reserved4: [u8; 12usize],
    #[doc = "0x90 - CHEHIT register"]
    pub chehit: CHEHIT,
    _reserved5: [u8; 12usize],
    #[doc = "0xa0 - CHEMIS register"]
    pub chemis: CHEMIS,
    _reserved6: [u8; 12usize],
    #[doc = "0xb0 - RESERVED1 register"]
    pub reserved1: RESERVED1,
    _reserved7: [u8; 12usize],
    #[doc = "0xc0 - CHEPFABT register"]
    pub chepfabt: CHEPFABT,
}
#[doc = "CHECON register"]
pub struct CHECON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHECON register"]
pub mod checon;
#[doc = "CHECONCLR register"]
pub struct CHECONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHECONCLR register"]
pub mod checonclr;
#[doc = "CHECONSET register"]
pub struct CHECONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHECONSET register"]
pub mod checonset;
#[doc = "CHECONINV register"]
pub struct CHECONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHECONINV register"]
pub mod checoninv;
#[doc = "CHEACC register"]
pub struct CHEACC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEACC register"]
pub mod cheacc;
#[doc = "CHEACCCLR register"]
pub struct CHEACCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEACCCLR register"]
pub mod cheaccclr;
#[doc = "CHEACCSET register"]
pub struct CHEACCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEACCSET register"]
pub mod cheaccset;
#[doc = "CHEACCINV register"]
pub struct CHEACCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEACCINV register"]
pub mod cheaccinv;
#[doc = "CHETAG register"]
pub struct CHETAG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHETAG register"]
pub mod chetag;
#[doc = "CHETAGCLR register"]
pub struct CHETAGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHETAGCLR register"]
pub mod chetagclr;
#[doc = "CHETAGSET register"]
pub struct CHETAGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHETAGSET register"]
pub mod chetagset;
#[doc = "CHETAGINV register"]
pub struct CHETAGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHETAGINV register"]
pub mod chetaginv;
#[doc = "CHEMSK register"]
pub struct CHEMSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEMSK register"]
pub mod chemsk;
#[doc = "CHEMSKCLR register"]
pub struct CHEMSKCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEMSKCLR register"]
pub mod chemskclr;
#[doc = "CHEMSKSET register"]
pub struct CHEMSKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEMSKSET register"]
pub mod chemskset;
#[doc = "CHEMSKINV register"]
pub struct CHEMSKINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEMSKINV register"]
pub mod chemskinv;
#[doc = "CHEW0 register"]
pub struct CHEW0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEW0 register"]
pub mod chew0;
#[doc = "CHEW1 register"]
pub struct CHEW1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEW1 register"]
pub mod chew1;
#[doc = "CHEW2 register"]
pub struct CHEW2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEW2 register"]
pub mod chew2;
#[doc = "CHEW3 register"]
pub struct CHEW3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEW3 register"]
pub mod chew3;
#[doc = "CHELRU register"]
pub struct CHELRU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHELRU register"]
pub mod chelru;
#[doc = "CHEHIT register"]
pub struct CHEHIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEHIT register"]
pub mod chehit;
#[doc = "CHEMIS register"]
pub struct CHEMIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEMIS register"]
pub mod chemis;
#[doc = "RESERVED1 register"]
pub struct RESERVED1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RESERVED1 register"]
pub mod reserved1;
#[doc = "CHEPFABT register"]
pub struct CHEPFABT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CHEPFABT register"]
pub mod chepfabt;
