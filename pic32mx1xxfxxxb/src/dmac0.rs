#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH0CON register"]
    pub dch0con: DCH0CON,
    #[doc = "0x04 - DCH0CONCLR register"]
    pub dch0conclr: DCH0CONCLR,
    #[doc = "0x08 - DCH0CONSET register"]
    pub dch0conset: DCH0CONSET,
    #[doc = "0x0c - DCH0CONINV register"]
    pub dch0coninv: DCH0CONINV,
    #[doc = "0x10 - DCH0ECON register"]
    pub dch0econ: DCH0ECON,
    #[doc = "0x14 - DCH0ECONCLR register"]
    pub dch0econclr: DCH0ECONCLR,
    #[doc = "0x18 - DCH0ECONSET register"]
    pub dch0econset: DCH0ECONSET,
    #[doc = "0x1c - DCH0ECONINV register"]
    pub dch0econinv: DCH0ECONINV,
    #[doc = "0x20 - DCH0INT register"]
    pub dch0int: DCH0INT,
    #[doc = "0x24 - DCH0INTCLR register"]
    pub dch0intclr: DCH0INTCLR,
    #[doc = "0x28 - DCH0INTSET register"]
    pub dch0intset: DCH0INTSET,
    #[doc = "0x2c - DCH0INTINV register"]
    pub dch0intinv: DCH0INTINV,
    #[doc = "0x30 - DCH0SSA register"]
    pub dch0ssa: DCH0SSA,
    #[doc = "0x34 - DCH0SSACLR register"]
    pub dch0ssaclr: DCH0SSACLR,
    #[doc = "0x38 - DCH0SSASET register"]
    pub dch0ssaset: DCH0SSASET,
    #[doc = "0x3c - DCH0SSAINV register"]
    pub dch0ssainv: DCH0SSAINV,
    #[doc = "0x40 - DCH0DSA register"]
    pub dch0dsa: DCH0DSA,
    #[doc = "0x44 - DCH0DSACLR register"]
    pub dch0dsaclr: DCH0DSACLR,
    #[doc = "0x48 - DCH0DSASET register"]
    pub dch0dsaset: DCH0DSASET,
    #[doc = "0x4c - DCH0DSAINV register"]
    pub dch0dsainv: DCH0DSAINV,
    #[doc = "0x50 - DCH0SSIZ register"]
    pub dch0ssiz: DCH0SSIZ,
    #[doc = "0x54 - DCH0SSIZCLR register"]
    pub dch0ssizclr: DCH0SSIZCLR,
    #[doc = "0x58 - DCH0SSIZSET register"]
    pub dch0ssizset: DCH0SSIZSET,
    #[doc = "0x5c - DCH0SSIZINV register"]
    pub dch0ssizinv: DCH0SSIZINV,
    #[doc = "0x60 - DCH0DSIZ register"]
    pub dch0dsiz: DCH0DSIZ,
    #[doc = "0x64 - DCH0DSIZCLR register"]
    pub dch0dsizclr: DCH0DSIZCLR,
    #[doc = "0x68 - DCH0DSIZSET register"]
    pub dch0dsizset: DCH0DSIZSET,
    #[doc = "0x6c - DCH0DSIZINV register"]
    pub dch0dsizinv: DCH0DSIZINV,
    #[doc = "0x70 - DCH0SPTR register"]
    pub dch0sptr: DCH0SPTR,
    #[doc = "0x74 - DCH0SPTRCLR register"]
    pub dch0sptrclr: DCH0SPTRCLR,
    #[doc = "0x78 - DCH0SPTRSET register"]
    pub dch0sptrset: DCH0SPTRSET,
    #[doc = "0x7c - DCH0SPTRINV register"]
    pub dch0sptrinv: DCH0SPTRINV,
    #[doc = "0x80 - DCH0DPTR register"]
    pub dch0dptr: DCH0DPTR,
    #[doc = "0x84 - DCH0DPTRCLR register"]
    pub dch0dptrclr: DCH0DPTRCLR,
    #[doc = "0x88 - DCH0DPTRSET register"]
    pub dch0dptrset: DCH0DPTRSET,
    #[doc = "0x8c - DCH0DPTRINV register"]
    pub dch0dptrinv: DCH0DPTRINV,
    #[doc = "0x90 - DCH0CSIZ register"]
    pub dch0csiz: DCH0CSIZ,
    #[doc = "0x94 - DCH0CSIZCLR register"]
    pub dch0csizclr: DCH0CSIZCLR,
    #[doc = "0x98 - DCH0CSIZSET register"]
    pub dch0csizset: DCH0CSIZSET,
    #[doc = "0x9c - DCH0CSIZINV register"]
    pub dch0csizinv: DCH0CSIZINV,
    #[doc = "0xa0 - DCH0CPTR register"]
    pub dch0cptr: DCH0CPTR,
    #[doc = "0xa4 - DCH0CPTRCLR register"]
    pub dch0cptrclr: DCH0CPTRCLR,
    #[doc = "0xa8 - DCH0CPTRSET register"]
    pub dch0cptrset: DCH0CPTRSET,
    #[doc = "0xac - DCH0CPTRINV register"]
    pub dch0cptrinv: DCH0CPTRINV,
    #[doc = "0xb0 - DCH0DAT register"]
    pub dch0dat: DCH0DAT,
    #[doc = "0xb4 - DCH0DATCLR register"]
    pub dch0datclr: DCH0DATCLR,
    #[doc = "0xb8 - DCH0DATSET register"]
    pub dch0datset: DCH0DATSET,
    #[doc = "0xbc - DCH0DATINV register"]
    pub dch0datinv: DCH0DATINV,
}
#[doc = "DCH0CON register"]
pub struct DCH0CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CON register"]
pub mod dch0con;
#[doc = "DCH0CONCLR register"]
pub struct DCH0CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CONCLR register"]
pub mod dch0conclr;
#[doc = "DCH0CONSET register"]
pub struct DCH0CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CONSET register"]
pub mod dch0conset;
#[doc = "DCH0CONINV register"]
pub struct DCH0CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CONINV register"]
pub mod dch0coninv;
#[doc = "DCH0ECON register"]
pub struct DCH0ECON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0ECON register"]
pub mod dch0econ;
#[doc = "DCH0ECONCLR register"]
pub struct DCH0ECONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0ECONCLR register"]
pub mod dch0econclr;
#[doc = "DCH0ECONSET register"]
pub struct DCH0ECONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0ECONSET register"]
pub mod dch0econset;
#[doc = "DCH0ECONINV register"]
pub struct DCH0ECONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0ECONINV register"]
pub mod dch0econinv;
#[doc = "DCH0INT register"]
pub struct DCH0INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0INT register"]
pub mod dch0int;
#[doc = "DCH0INTCLR register"]
pub struct DCH0INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0INTCLR register"]
pub mod dch0intclr;
#[doc = "DCH0INTSET register"]
pub struct DCH0INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0INTSET register"]
pub mod dch0intset;
#[doc = "DCH0INTINV register"]
pub struct DCH0INTINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0INTINV register"]
pub mod dch0intinv;
#[doc = "DCH0SSA register"]
pub struct DCH0SSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SSA register"]
pub mod dch0ssa;
#[doc = "DCH0SSACLR register"]
pub struct DCH0SSACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SSACLR register"]
pub mod dch0ssaclr;
#[doc = "DCH0SSASET register"]
pub struct DCH0SSASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SSASET register"]
pub mod dch0ssaset;
#[doc = "DCH0SSAINV register"]
pub struct DCH0SSAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SSAINV register"]
pub mod dch0ssainv;
#[doc = "DCH0DSA register"]
pub struct DCH0DSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DSA register"]
pub mod dch0dsa;
#[doc = "DCH0DSACLR register"]
pub struct DCH0DSACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DSACLR register"]
pub mod dch0dsaclr;
#[doc = "DCH0DSASET register"]
pub struct DCH0DSASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DSASET register"]
pub mod dch0dsaset;
#[doc = "DCH0DSAINV register"]
pub struct DCH0DSAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DSAINV register"]
pub mod dch0dsainv;
#[doc = "DCH0SSIZ register"]
pub struct DCH0SSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SSIZ register"]
pub mod dch0ssiz;
#[doc = "DCH0SSIZCLR register"]
pub struct DCH0SSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SSIZCLR register"]
pub mod dch0ssizclr;
#[doc = "DCH0SSIZSET register"]
pub struct DCH0SSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SSIZSET register"]
pub mod dch0ssizset;
#[doc = "DCH0SSIZINV register"]
pub struct DCH0SSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SSIZINV register"]
pub mod dch0ssizinv;
#[doc = "DCH0DSIZ register"]
pub struct DCH0DSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DSIZ register"]
pub mod dch0dsiz;
#[doc = "DCH0DSIZCLR register"]
pub struct DCH0DSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DSIZCLR register"]
pub mod dch0dsizclr;
#[doc = "DCH0DSIZSET register"]
pub struct DCH0DSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DSIZSET register"]
pub mod dch0dsizset;
#[doc = "DCH0DSIZINV register"]
pub struct DCH0DSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DSIZINV register"]
pub mod dch0dsizinv;
#[doc = "DCH0SPTR register"]
pub struct DCH0SPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SPTR register"]
pub mod dch0sptr;
#[doc = "DCH0SPTRCLR register"]
pub struct DCH0SPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SPTRCLR register"]
pub mod dch0sptrclr;
#[doc = "DCH0SPTRSET register"]
pub struct DCH0SPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SPTRSET register"]
pub mod dch0sptrset;
#[doc = "DCH0SPTRINV register"]
pub struct DCH0SPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0SPTRINV register"]
pub mod dch0sptrinv;
#[doc = "DCH0DPTR register"]
pub struct DCH0DPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DPTR register"]
pub mod dch0dptr;
#[doc = "DCH0DPTRCLR register"]
pub struct DCH0DPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DPTRCLR register"]
pub mod dch0dptrclr;
#[doc = "DCH0DPTRSET register"]
pub struct DCH0DPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DPTRSET register"]
pub mod dch0dptrset;
#[doc = "DCH0DPTRINV register"]
pub struct DCH0DPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DPTRINV register"]
pub mod dch0dptrinv;
#[doc = "DCH0CSIZ register"]
pub struct DCH0CSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CSIZ register"]
pub mod dch0csiz;
#[doc = "DCH0CSIZCLR register"]
pub struct DCH0CSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CSIZCLR register"]
pub mod dch0csizclr;
#[doc = "DCH0CSIZSET register"]
pub struct DCH0CSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CSIZSET register"]
pub mod dch0csizset;
#[doc = "DCH0CSIZINV register"]
pub struct DCH0CSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CSIZINV register"]
pub mod dch0csizinv;
#[doc = "DCH0CPTR register"]
pub struct DCH0CPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CPTR register"]
pub mod dch0cptr;
#[doc = "DCH0CPTRCLR register"]
pub struct DCH0CPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CPTRCLR register"]
pub mod dch0cptrclr;
#[doc = "DCH0CPTRSET register"]
pub struct DCH0CPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CPTRSET register"]
pub mod dch0cptrset;
#[doc = "DCH0CPTRINV register"]
pub struct DCH0CPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0CPTRINV register"]
pub mod dch0cptrinv;
#[doc = "DCH0DAT register"]
pub struct DCH0DAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DAT register"]
pub mod dch0dat;
#[doc = "DCH0DATCLR register"]
pub struct DCH0DATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DATCLR register"]
pub mod dch0datclr;
#[doc = "DCH0DATSET register"]
pub struct DCH0DATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DATSET register"]
pub mod dch0datset;
#[doc = "DCH0DATINV register"]
pub struct DCH0DATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH0DATINV register"]
pub mod dch0datinv;
