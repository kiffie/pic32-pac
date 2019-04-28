#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH2CON register"]
    pub dch2con: DCH2CON,
    #[doc = "0x04 - DCH2CONCLR register"]
    pub dch2conclr: DCH2CONCLR,
    #[doc = "0x08 - DCH2CONSET register"]
    pub dch2conset: DCH2CONSET,
    #[doc = "0x0c - DCH2CONINV register"]
    pub dch2coninv: DCH2CONINV,
    #[doc = "0x10 - DCH2ECON register"]
    pub dch2econ: DCH2ECON,
    #[doc = "0x14 - DCH2ECONCLR register"]
    pub dch2econclr: DCH2ECONCLR,
    #[doc = "0x18 - DCH2ECONSET register"]
    pub dch2econset: DCH2ECONSET,
    #[doc = "0x1c - DCH2ECONINV register"]
    pub dch2econinv: DCH2ECONINV,
    #[doc = "0x20 - DCH2INT register"]
    pub dch2int: DCH2INT,
    #[doc = "0x24 - DCH2INTCLR register"]
    pub dch2intclr: DCH2INTCLR,
    #[doc = "0x28 - DCH2INTSET register"]
    pub dch2intset: DCH2INTSET,
    #[doc = "0x2c - DCH2INTINV register"]
    pub dch2intinv: DCH2INTINV,
    #[doc = "0x30 - DCH2SSA register"]
    pub dch2ssa: DCH2SSA,
    #[doc = "0x34 - DCH2SSACLR register"]
    pub dch2ssaclr: DCH2SSACLR,
    #[doc = "0x38 - DCH2SSASET register"]
    pub dch2ssaset: DCH2SSASET,
    #[doc = "0x3c - DCH2SSAINV register"]
    pub dch2ssainv: DCH2SSAINV,
    #[doc = "0x40 - DCH2DSA register"]
    pub dch2dsa: DCH2DSA,
    #[doc = "0x44 - DCH2DSACLR register"]
    pub dch2dsaclr: DCH2DSACLR,
    #[doc = "0x48 - DCH2DSASET register"]
    pub dch2dsaset: DCH2DSASET,
    #[doc = "0x4c - DCH2DSAINV register"]
    pub dch2dsainv: DCH2DSAINV,
    #[doc = "0x50 - DCH2SSIZ register"]
    pub dch2ssiz: DCH2SSIZ,
    #[doc = "0x54 - DCH2SSIZCLR register"]
    pub dch2ssizclr: DCH2SSIZCLR,
    #[doc = "0x58 - DCH2SSIZSET register"]
    pub dch2ssizset: DCH2SSIZSET,
    #[doc = "0x5c - DCH2SSIZINV register"]
    pub dch2ssizinv: DCH2SSIZINV,
    #[doc = "0x60 - DCH2DSIZ register"]
    pub dch2dsiz: DCH2DSIZ,
    #[doc = "0x64 - DCH2DSIZCLR register"]
    pub dch2dsizclr: DCH2DSIZCLR,
    #[doc = "0x68 - DCH2DSIZSET register"]
    pub dch2dsizset: DCH2DSIZSET,
    #[doc = "0x6c - DCH2DSIZINV register"]
    pub dch2dsizinv: DCH2DSIZINV,
    #[doc = "0x70 - DCH2SPTR register"]
    pub dch2sptr: DCH2SPTR,
    #[doc = "0x74 - DCH2SPTRCLR register"]
    pub dch2sptrclr: DCH2SPTRCLR,
    #[doc = "0x78 - DCH2SPTRSET register"]
    pub dch2sptrset: DCH2SPTRSET,
    #[doc = "0x7c - DCH2SPTRINV register"]
    pub dch2sptrinv: DCH2SPTRINV,
    #[doc = "0x80 - DCH2DPTR register"]
    pub dch2dptr: DCH2DPTR,
    #[doc = "0x84 - DCH2DPTRCLR register"]
    pub dch2dptrclr: DCH2DPTRCLR,
    #[doc = "0x88 - DCH2DPTRSET register"]
    pub dch2dptrset: DCH2DPTRSET,
    #[doc = "0x8c - DCH2DPTRINV register"]
    pub dch2dptrinv: DCH2DPTRINV,
    #[doc = "0x90 - DCH2CSIZ register"]
    pub dch2csiz: DCH2CSIZ,
    #[doc = "0x94 - DCH2CSIZCLR register"]
    pub dch2csizclr: DCH2CSIZCLR,
    #[doc = "0x98 - DCH2CSIZSET register"]
    pub dch2csizset: DCH2CSIZSET,
    #[doc = "0x9c - DCH2CSIZINV register"]
    pub dch2csizinv: DCH2CSIZINV,
    #[doc = "0xa0 - DCH2CPTR register"]
    pub dch2cptr: DCH2CPTR,
    #[doc = "0xa4 - DCH2CPTRCLR register"]
    pub dch2cptrclr: DCH2CPTRCLR,
    #[doc = "0xa8 - DCH2CPTRSET register"]
    pub dch2cptrset: DCH2CPTRSET,
    #[doc = "0xac - DCH2CPTRINV register"]
    pub dch2cptrinv: DCH2CPTRINV,
    #[doc = "0xb0 - DCH2DAT register"]
    pub dch2dat: DCH2DAT,
    #[doc = "0xb4 - DCH2DATCLR register"]
    pub dch2datclr: DCH2DATCLR,
    #[doc = "0xb8 - DCH2DATSET register"]
    pub dch2datset: DCH2DATSET,
    #[doc = "0xbc - DCH2DATINV register"]
    pub dch2datinv: DCH2DATINV,
}
#[doc = "DCH2CON register"]
pub struct DCH2CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CON register"]
pub mod dch2con;
#[doc = "DCH2CONCLR register"]
pub struct DCH2CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CONCLR register"]
pub mod dch2conclr;
#[doc = "DCH2CONSET register"]
pub struct DCH2CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CONSET register"]
pub mod dch2conset;
#[doc = "DCH2CONINV register"]
pub struct DCH2CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CONINV register"]
pub mod dch2coninv;
#[doc = "DCH2ECON register"]
pub struct DCH2ECON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2ECON register"]
pub mod dch2econ;
#[doc = "DCH2ECONCLR register"]
pub struct DCH2ECONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2ECONCLR register"]
pub mod dch2econclr;
#[doc = "DCH2ECONSET register"]
pub struct DCH2ECONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2ECONSET register"]
pub mod dch2econset;
#[doc = "DCH2ECONINV register"]
pub struct DCH2ECONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2ECONINV register"]
pub mod dch2econinv;
#[doc = "DCH2INT register"]
pub struct DCH2INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2INT register"]
pub mod dch2int;
#[doc = "DCH2INTCLR register"]
pub struct DCH2INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2INTCLR register"]
pub mod dch2intclr;
#[doc = "DCH2INTSET register"]
pub struct DCH2INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2INTSET register"]
pub mod dch2intset;
#[doc = "DCH2INTINV register"]
pub struct DCH2INTINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2INTINV register"]
pub mod dch2intinv;
#[doc = "DCH2SSA register"]
pub struct DCH2SSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SSA register"]
pub mod dch2ssa;
#[doc = "DCH2SSACLR register"]
pub struct DCH2SSACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SSACLR register"]
pub mod dch2ssaclr;
#[doc = "DCH2SSASET register"]
pub struct DCH2SSASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SSASET register"]
pub mod dch2ssaset;
#[doc = "DCH2SSAINV register"]
pub struct DCH2SSAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SSAINV register"]
pub mod dch2ssainv;
#[doc = "DCH2DSA register"]
pub struct DCH2DSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DSA register"]
pub mod dch2dsa;
#[doc = "DCH2DSACLR register"]
pub struct DCH2DSACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DSACLR register"]
pub mod dch2dsaclr;
#[doc = "DCH2DSASET register"]
pub struct DCH2DSASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DSASET register"]
pub mod dch2dsaset;
#[doc = "DCH2DSAINV register"]
pub struct DCH2DSAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DSAINV register"]
pub mod dch2dsainv;
#[doc = "DCH2SSIZ register"]
pub struct DCH2SSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SSIZ register"]
pub mod dch2ssiz;
#[doc = "DCH2SSIZCLR register"]
pub struct DCH2SSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SSIZCLR register"]
pub mod dch2ssizclr;
#[doc = "DCH2SSIZSET register"]
pub struct DCH2SSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SSIZSET register"]
pub mod dch2ssizset;
#[doc = "DCH2SSIZINV register"]
pub struct DCH2SSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SSIZINV register"]
pub mod dch2ssizinv;
#[doc = "DCH2DSIZ register"]
pub struct DCH2DSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DSIZ register"]
pub mod dch2dsiz;
#[doc = "DCH2DSIZCLR register"]
pub struct DCH2DSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DSIZCLR register"]
pub mod dch2dsizclr;
#[doc = "DCH2DSIZSET register"]
pub struct DCH2DSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DSIZSET register"]
pub mod dch2dsizset;
#[doc = "DCH2DSIZINV register"]
pub struct DCH2DSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DSIZINV register"]
pub mod dch2dsizinv;
#[doc = "DCH2SPTR register"]
pub struct DCH2SPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SPTR register"]
pub mod dch2sptr;
#[doc = "DCH2SPTRCLR register"]
pub struct DCH2SPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SPTRCLR register"]
pub mod dch2sptrclr;
#[doc = "DCH2SPTRSET register"]
pub struct DCH2SPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SPTRSET register"]
pub mod dch2sptrset;
#[doc = "DCH2SPTRINV register"]
pub struct DCH2SPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2SPTRINV register"]
pub mod dch2sptrinv;
#[doc = "DCH2DPTR register"]
pub struct DCH2DPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DPTR register"]
pub mod dch2dptr;
#[doc = "DCH2DPTRCLR register"]
pub struct DCH2DPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DPTRCLR register"]
pub mod dch2dptrclr;
#[doc = "DCH2DPTRSET register"]
pub struct DCH2DPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DPTRSET register"]
pub mod dch2dptrset;
#[doc = "DCH2DPTRINV register"]
pub struct DCH2DPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DPTRINV register"]
pub mod dch2dptrinv;
#[doc = "DCH2CSIZ register"]
pub struct DCH2CSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CSIZ register"]
pub mod dch2csiz;
#[doc = "DCH2CSIZCLR register"]
pub struct DCH2CSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CSIZCLR register"]
pub mod dch2csizclr;
#[doc = "DCH2CSIZSET register"]
pub struct DCH2CSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CSIZSET register"]
pub mod dch2csizset;
#[doc = "DCH2CSIZINV register"]
pub struct DCH2CSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CSIZINV register"]
pub mod dch2csizinv;
#[doc = "DCH2CPTR register"]
pub struct DCH2CPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CPTR register"]
pub mod dch2cptr;
#[doc = "DCH2CPTRCLR register"]
pub struct DCH2CPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CPTRCLR register"]
pub mod dch2cptrclr;
#[doc = "DCH2CPTRSET register"]
pub struct DCH2CPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CPTRSET register"]
pub mod dch2cptrset;
#[doc = "DCH2CPTRINV register"]
pub struct DCH2CPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2CPTRINV register"]
pub mod dch2cptrinv;
#[doc = "DCH2DAT register"]
pub struct DCH2DAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DAT register"]
pub mod dch2dat;
#[doc = "DCH2DATCLR register"]
pub struct DCH2DATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DATCLR register"]
pub mod dch2datclr;
#[doc = "DCH2DATSET register"]
pub struct DCH2DATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DATSET register"]
pub mod dch2datset;
#[doc = "DCH2DATINV register"]
pub struct DCH2DATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH2DATINV register"]
pub mod dch2datinv;
