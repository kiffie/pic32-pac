#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH3CON register"]
    pub dch3con: DCH3CON,
    #[doc = "0x04 - DCH3CONCLR register"]
    pub dch3conclr: DCH3CONCLR,
    #[doc = "0x08 - DCH3CONSET register"]
    pub dch3conset: DCH3CONSET,
    #[doc = "0x0c - DCH3CONINV register"]
    pub dch3coninv: DCH3CONINV,
    #[doc = "0x10 - DCH3ECON register"]
    pub dch3econ: DCH3ECON,
    #[doc = "0x14 - DCH3ECONCLR register"]
    pub dch3econclr: DCH3ECONCLR,
    #[doc = "0x18 - DCH3ECONSET register"]
    pub dch3econset: DCH3ECONSET,
    #[doc = "0x1c - DCH3ECONINV register"]
    pub dch3econinv: DCH3ECONINV,
    #[doc = "0x20 - DCH3INT register"]
    pub dch3int: DCH3INT,
    #[doc = "0x24 - DCH3INTCLR register"]
    pub dch3intclr: DCH3INTCLR,
    #[doc = "0x28 - DCH3INTSET register"]
    pub dch3intset: DCH3INTSET,
    #[doc = "0x2c - DCH3INTINV register"]
    pub dch3intinv: DCH3INTINV,
    #[doc = "0x30 - DCH3SSA register"]
    pub dch3ssa: DCH3SSA,
    #[doc = "0x34 - DCH3SSACLR register"]
    pub dch3ssaclr: DCH3SSACLR,
    #[doc = "0x38 - DCH3SSASET register"]
    pub dch3ssaset: DCH3SSASET,
    #[doc = "0x3c - DCH3SSAINV register"]
    pub dch3ssainv: DCH3SSAINV,
    #[doc = "0x40 - DCH3DSA register"]
    pub dch3dsa: DCH3DSA,
    #[doc = "0x44 - DCH3DSACLR register"]
    pub dch3dsaclr: DCH3DSACLR,
    #[doc = "0x48 - DCH3DSASET register"]
    pub dch3dsaset: DCH3DSASET,
    #[doc = "0x4c - DCH3DSAINV register"]
    pub dch3dsainv: DCH3DSAINV,
    #[doc = "0x50 - DCH3SSIZ register"]
    pub dch3ssiz: DCH3SSIZ,
    #[doc = "0x54 - DCH3SSIZCLR register"]
    pub dch3ssizclr: DCH3SSIZCLR,
    #[doc = "0x58 - DCH3SSIZSET register"]
    pub dch3ssizset: DCH3SSIZSET,
    #[doc = "0x5c - DCH3SSIZINV register"]
    pub dch3ssizinv: DCH3SSIZINV,
    #[doc = "0x60 - DCH3DSIZ register"]
    pub dch3dsiz: DCH3DSIZ,
    #[doc = "0x64 - DCH3DSIZCLR register"]
    pub dch3dsizclr: DCH3DSIZCLR,
    #[doc = "0x68 - DCH3DSIZSET register"]
    pub dch3dsizset: DCH3DSIZSET,
    #[doc = "0x6c - DCH3DSIZINV register"]
    pub dch3dsizinv: DCH3DSIZINV,
    #[doc = "0x70 - DCH3SPTR register"]
    pub dch3sptr: DCH3SPTR,
    #[doc = "0x74 - DCH3SPTRCLR register"]
    pub dch3sptrclr: DCH3SPTRCLR,
    #[doc = "0x78 - DCH3SPTRSET register"]
    pub dch3sptrset: DCH3SPTRSET,
    #[doc = "0x7c - DCH3SPTRINV register"]
    pub dch3sptrinv: DCH3SPTRINV,
    #[doc = "0x80 - DCH3DPTR register"]
    pub dch3dptr: DCH3DPTR,
    #[doc = "0x84 - DCH3DPTRCLR register"]
    pub dch3dptrclr: DCH3DPTRCLR,
    #[doc = "0x88 - DCH3DPTRSET register"]
    pub dch3dptrset: DCH3DPTRSET,
    #[doc = "0x8c - DCH3DPTRINV register"]
    pub dch3dptrinv: DCH3DPTRINV,
    #[doc = "0x90 - DCH3CSIZ register"]
    pub dch3csiz: DCH3CSIZ,
    #[doc = "0x94 - DCH3CSIZCLR register"]
    pub dch3csizclr: DCH3CSIZCLR,
    #[doc = "0x98 - DCH3CSIZSET register"]
    pub dch3csizset: DCH3CSIZSET,
    #[doc = "0x9c - DCH3CSIZINV register"]
    pub dch3csizinv: DCH3CSIZINV,
    #[doc = "0xa0 - DCH3CPTR register"]
    pub dch3cptr: DCH3CPTR,
    #[doc = "0xa4 - DCH3CPTRCLR register"]
    pub dch3cptrclr: DCH3CPTRCLR,
    #[doc = "0xa8 - DCH3CPTRSET register"]
    pub dch3cptrset: DCH3CPTRSET,
    #[doc = "0xac - DCH3CPTRINV register"]
    pub dch3cptrinv: DCH3CPTRINV,
    #[doc = "0xb0 - DCH3DAT register"]
    pub dch3dat: DCH3DAT,
    #[doc = "0xb4 - DCH3DATCLR register"]
    pub dch3datclr: DCH3DATCLR,
    #[doc = "0xb8 - DCH3DATSET register"]
    pub dch3datset: DCH3DATSET,
    #[doc = "0xbc - DCH3DATINV register"]
    pub dch3datinv: DCH3DATINV,
}
#[doc = "DCH3CON register"]
pub struct DCH3CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CON register"]
pub mod dch3con;
#[doc = "DCH3CONCLR register"]
pub struct DCH3CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CONCLR register"]
pub mod dch3conclr;
#[doc = "DCH3CONSET register"]
pub struct DCH3CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CONSET register"]
pub mod dch3conset;
#[doc = "DCH3CONINV register"]
pub struct DCH3CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CONINV register"]
pub mod dch3coninv;
#[doc = "DCH3ECON register"]
pub struct DCH3ECON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3ECON register"]
pub mod dch3econ;
#[doc = "DCH3ECONCLR register"]
pub struct DCH3ECONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3ECONCLR register"]
pub mod dch3econclr;
#[doc = "DCH3ECONSET register"]
pub struct DCH3ECONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3ECONSET register"]
pub mod dch3econset;
#[doc = "DCH3ECONINV register"]
pub struct DCH3ECONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3ECONINV register"]
pub mod dch3econinv;
#[doc = "DCH3INT register"]
pub struct DCH3INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3INT register"]
pub mod dch3int;
#[doc = "DCH3INTCLR register"]
pub struct DCH3INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3INTCLR register"]
pub mod dch3intclr;
#[doc = "DCH3INTSET register"]
pub struct DCH3INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3INTSET register"]
pub mod dch3intset;
#[doc = "DCH3INTINV register"]
pub struct DCH3INTINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3INTINV register"]
pub mod dch3intinv;
#[doc = "DCH3SSA register"]
pub struct DCH3SSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SSA register"]
pub mod dch3ssa;
#[doc = "DCH3SSACLR register"]
pub struct DCH3SSACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SSACLR register"]
pub mod dch3ssaclr;
#[doc = "DCH3SSASET register"]
pub struct DCH3SSASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SSASET register"]
pub mod dch3ssaset;
#[doc = "DCH3SSAINV register"]
pub struct DCH3SSAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SSAINV register"]
pub mod dch3ssainv;
#[doc = "DCH3DSA register"]
pub struct DCH3DSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DSA register"]
pub mod dch3dsa;
#[doc = "DCH3DSACLR register"]
pub struct DCH3DSACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DSACLR register"]
pub mod dch3dsaclr;
#[doc = "DCH3DSASET register"]
pub struct DCH3DSASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DSASET register"]
pub mod dch3dsaset;
#[doc = "DCH3DSAINV register"]
pub struct DCH3DSAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DSAINV register"]
pub mod dch3dsainv;
#[doc = "DCH3SSIZ register"]
pub struct DCH3SSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SSIZ register"]
pub mod dch3ssiz;
#[doc = "DCH3SSIZCLR register"]
pub struct DCH3SSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SSIZCLR register"]
pub mod dch3ssizclr;
#[doc = "DCH3SSIZSET register"]
pub struct DCH3SSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SSIZSET register"]
pub mod dch3ssizset;
#[doc = "DCH3SSIZINV register"]
pub struct DCH3SSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SSIZINV register"]
pub mod dch3ssizinv;
#[doc = "DCH3DSIZ register"]
pub struct DCH3DSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DSIZ register"]
pub mod dch3dsiz;
#[doc = "DCH3DSIZCLR register"]
pub struct DCH3DSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DSIZCLR register"]
pub mod dch3dsizclr;
#[doc = "DCH3DSIZSET register"]
pub struct DCH3DSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DSIZSET register"]
pub mod dch3dsizset;
#[doc = "DCH3DSIZINV register"]
pub struct DCH3DSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DSIZINV register"]
pub mod dch3dsizinv;
#[doc = "DCH3SPTR register"]
pub struct DCH3SPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SPTR register"]
pub mod dch3sptr;
#[doc = "DCH3SPTRCLR register"]
pub struct DCH3SPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SPTRCLR register"]
pub mod dch3sptrclr;
#[doc = "DCH3SPTRSET register"]
pub struct DCH3SPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SPTRSET register"]
pub mod dch3sptrset;
#[doc = "DCH3SPTRINV register"]
pub struct DCH3SPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3SPTRINV register"]
pub mod dch3sptrinv;
#[doc = "DCH3DPTR register"]
pub struct DCH3DPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DPTR register"]
pub mod dch3dptr;
#[doc = "DCH3DPTRCLR register"]
pub struct DCH3DPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DPTRCLR register"]
pub mod dch3dptrclr;
#[doc = "DCH3DPTRSET register"]
pub struct DCH3DPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DPTRSET register"]
pub mod dch3dptrset;
#[doc = "DCH3DPTRINV register"]
pub struct DCH3DPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DPTRINV register"]
pub mod dch3dptrinv;
#[doc = "DCH3CSIZ register"]
pub struct DCH3CSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CSIZ register"]
pub mod dch3csiz;
#[doc = "DCH3CSIZCLR register"]
pub struct DCH3CSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CSIZCLR register"]
pub mod dch3csizclr;
#[doc = "DCH3CSIZSET register"]
pub struct DCH3CSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CSIZSET register"]
pub mod dch3csizset;
#[doc = "DCH3CSIZINV register"]
pub struct DCH3CSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CSIZINV register"]
pub mod dch3csizinv;
#[doc = "DCH3CPTR register"]
pub struct DCH3CPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CPTR register"]
pub mod dch3cptr;
#[doc = "DCH3CPTRCLR register"]
pub struct DCH3CPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CPTRCLR register"]
pub mod dch3cptrclr;
#[doc = "DCH3CPTRSET register"]
pub struct DCH3CPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CPTRSET register"]
pub mod dch3cptrset;
#[doc = "DCH3CPTRINV register"]
pub struct DCH3CPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3CPTRINV register"]
pub mod dch3cptrinv;
#[doc = "DCH3DAT register"]
pub struct DCH3DAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DAT register"]
pub mod dch3dat;
#[doc = "DCH3DATCLR register"]
pub struct DCH3DATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DATCLR register"]
pub mod dch3datclr;
#[doc = "DCH3DATSET register"]
pub struct DCH3DATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DATSET register"]
pub mod dch3datset;
#[doc = "DCH3DATINV register"]
pub struct DCH3DATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH3DATINV register"]
pub mod dch3datinv;
