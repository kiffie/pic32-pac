#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH1CON register"]
    pub dch1con: DCH1CON,
    #[doc = "0x04 - DCH1CONCLR register"]
    pub dch1conclr: DCH1CONCLR,
    #[doc = "0x08 - DCH1CONSET register"]
    pub dch1conset: DCH1CONSET,
    #[doc = "0x0c - DCH1CONINV register"]
    pub dch1coninv: DCH1CONINV,
    #[doc = "0x10 - DCH1ECON register"]
    pub dch1econ: DCH1ECON,
    #[doc = "0x14 - DCH1ECONCLR register"]
    pub dch1econclr: DCH1ECONCLR,
    #[doc = "0x18 - DCH1ECONSET register"]
    pub dch1econset: DCH1ECONSET,
    #[doc = "0x1c - DCH1ECONINV register"]
    pub dch1econinv: DCH1ECONINV,
    #[doc = "0x20 - DCH1INT register"]
    pub dch1int: DCH1INT,
    #[doc = "0x24 - DCH1INTCLR register"]
    pub dch1intclr: DCH1INTCLR,
    #[doc = "0x28 - DCH1INTSET register"]
    pub dch1intset: DCH1INTSET,
    #[doc = "0x2c - DCH1INTINV register"]
    pub dch1intinv: DCH1INTINV,
    #[doc = "0x30 - DCH1SSA register"]
    pub dch1ssa: DCH1SSA,
    #[doc = "0x34 - DCH1SSACLR register"]
    pub dch1ssaclr: DCH1SSACLR,
    #[doc = "0x38 - DCH1SSASET register"]
    pub dch1ssaset: DCH1SSASET,
    #[doc = "0x3c - DCH1SSAINV register"]
    pub dch1ssainv: DCH1SSAINV,
    #[doc = "0x40 - DCH1DSA register"]
    pub dch1dsa: DCH1DSA,
    #[doc = "0x44 - DCH1DSACLR register"]
    pub dch1dsaclr: DCH1DSACLR,
    #[doc = "0x48 - DCH1DSASET register"]
    pub dch1dsaset: DCH1DSASET,
    #[doc = "0x4c - DCH1DSAINV register"]
    pub dch1dsainv: DCH1DSAINV,
    #[doc = "0x50 - DCH1SSIZ register"]
    pub dch1ssiz: DCH1SSIZ,
    #[doc = "0x54 - DCH1SSIZCLR register"]
    pub dch1ssizclr: DCH1SSIZCLR,
    #[doc = "0x58 - DCH1SSIZSET register"]
    pub dch1ssizset: DCH1SSIZSET,
    #[doc = "0x5c - DCH1SSIZINV register"]
    pub dch1ssizinv: DCH1SSIZINV,
    #[doc = "0x60 - DCH1DSIZ register"]
    pub dch1dsiz: DCH1DSIZ,
    #[doc = "0x64 - DCH1DSIZCLR register"]
    pub dch1dsizclr: DCH1DSIZCLR,
    #[doc = "0x68 - DCH1DSIZSET register"]
    pub dch1dsizset: DCH1DSIZSET,
    #[doc = "0x6c - DCH1DSIZINV register"]
    pub dch1dsizinv: DCH1DSIZINV,
    #[doc = "0x70 - DCH1SPTR register"]
    pub dch1sptr: DCH1SPTR,
    #[doc = "0x74 - DCH1SPTRCLR register"]
    pub dch1sptrclr: DCH1SPTRCLR,
    #[doc = "0x78 - DCH1SPTRSET register"]
    pub dch1sptrset: DCH1SPTRSET,
    #[doc = "0x7c - DCH1SPTRINV register"]
    pub dch1sptrinv: DCH1SPTRINV,
    #[doc = "0x80 - DCH1DPTR register"]
    pub dch1dptr: DCH1DPTR,
    #[doc = "0x84 - DCH1DPTRCLR register"]
    pub dch1dptrclr: DCH1DPTRCLR,
    #[doc = "0x88 - DCH1DPTRSET register"]
    pub dch1dptrset: DCH1DPTRSET,
    #[doc = "0x8c - DCH1DPTRINV register"]
    pub dch1dptrinv: DCH1DPTRINV,
    #[doc = "0x90 - DCH1CSIZ register"]
    pub dch1csiz: DCH1CSIZ,
    #[doc = "0x94 - DCH1CSIZCLR register"]
    pub dch1csizclr: DCH1CSIZCLR,
    #[doc = "0x98 - DCH1CSIZSET register"]
    pub dch1csizset: DCH1CSIZSET,
    #[doc = "0x9c - DCH1CSIZINV register"]
    pub dch1csizinv: DCH1CSIZINV,
    #[doc = "0xa0 - DCH1CPTR register"]
    pub dch1cptr: DCH1CPTR,
    #[doc = "0xa4 - DCH1CPTRCLR register"]
    pub dch1cptrclr: DCH1CPTRCLR,
    #[doc = "0xa8 - DCH1CPTRSET register"]
    pub dch1cptrset: DCH1CPTRSET,
    #[doc = "0xac - DCH1CPTRINV register"]
    pub dch1cptrinv: DCH1CPTRINV,
    #[doc = "0xb0 - DCH1DAT register"]
    pub dch1dat: DCH1DAT,
    #[doc = "0xb4 - DCH1DATCLR register"]
    pub dch1datclr: DCH1DATCLR,
    #[doc = "0xb8 - DCH1DATSET register"]
    pub dch1datset: DCH1DATSET,
    #[doc = "0xbc - DCH1DATINV register"]
    pub dch1datinv: DCH1DATINV,
}
#[doc = "DCH1CON register"]
pub struct DCH1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CON register"]
pub mod dch1con;
#[doc = "DCH1CONCLR register"]
pub struct DCH1CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CONCLR register"]
pub mod dch1conclr;
#[doc = "DCH1CONSET register"]
pub struct DCH1CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CONSET register"]
pub mod dch1conset;
#[doc = "DCH1CONINV register"]
pub struct DCH1CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CONINV register"]
pub mod dch1coninv;
#[doc = "DCH1ECON register"]
pub struct DCH1ECON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1ECON register"]
pub mod dch1econ;
#[doc = "DCH1ECONCLR register"]
pub struct DCH1ECONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1ECONCLR register"]
pub mod dch1econclr;
#[doc = "DCH1ECONSET register"]
pub struct DCH1ECONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1ECONSET register"]
pub mod dch1econset;
#[doc = "DCH1ECONINV register"]
pub struct DCH1ECONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1ECONINV register"]
pub mod dch1econinv;
#[doc = "DCH1INT register"]
pub struct DCH1INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1INT register"]
pub mod dch1int;
#[doc = "DCH1INTCLR register"]
pub struct DCH1INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1INTCLR register"]
pub mod dch1intclr;
#[doc = "DCH1INTSET register"]
pub struct DCH1INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1INTSET register"]
pub mod dch1intset;
#[doc = "DCH1INTINV register"]
pub struct DCH1INTINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1INTINV register"]
pub mod dch1intinv;
#[doc = "DCH1SSA register"]
pub struct DCH1SSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SSA register"]
pub mod dch1ssa;
#[doc = "DCH1SSACLR register"]
pub struct DCH1SSACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SSACLR register"]
pub mod dch1ssaclr;
#[doc = "DCH1SSASET register"]
pub struct DCH1SSASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SSASET register"]
pub mod dch1ssaset;
#[doc = "DCH1SSAINV register"]
pub struct DCH1SSAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SSAINV register"]
pub mod dch1ssainv;
#[doc = "DCH1DSA register"]
pub struct DCH1DSA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DSA register"]
pub mod dch1dsa;
#[doc = "DCH1DSACLR register"]
pub struct DCH1DSACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DSACLR register"]
pub mod dch1dsaclr;
#[doc = "DCH1DSASET register"]
pub struct DCH1DSASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DSASET register"]
pub mod dch1dsaset;
#[doc = "DCH1DSAINV register"]
pub struct DCH1DSAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DSAINV register"]
pub mod dch1dsainv;
#[doc = "DCH1SSIZ register"]
pub struct DCH1SSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SSIZ register"]
pub mod dch1ssiz;
#[doc = "DCH1SSIZCLR register"]
pub struct DCH1SSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SSIZCLR register"]
pub mod dch1ssizclr;
#[doc = "DCH1SSIZSET register"]
pub struct DCH1SSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SSIZSET register"]
pub mod dch1ssizset;
#[doc = "DCH1SSIZINV register"]
pub struct DCH1SSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SSIZINV register"]
pub mod dch1ssizinv;
#[doc = "DCH1DSIZ register"]
pub struct DCH1DSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DSIZ register"]
pub mod dch1dsiz;
#[doc = "DCH1DSIZCLR register"]
pub struct DCH1DSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DSIZCLR register"]
pub mod dch1dsizclr;
#[doc = "DCH1DSIZSET register"]
pub struct DCH1DSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DSIZSET register"]
pub mod dch1dsizset;
#[doc = "DCH1DSIZINV register"]
pub struct DCH1DSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DSIZINV register"]
pub mod dch1dsizinv;
#[doc = "DCH1SPTR register"]
pub struct DCH1SPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SPTR register"]
pub mod dch1sptr;
#[doc = "DCH1SPTRCLR register"]
pub struct DCH1SPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SPTRCLR register"]
pub mod dch1sptrclr;
#[doc = "DCH1SPTRSET register"]
pub struct DCH1SPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SPTRSET register"]
pub mod dch1sptrset;
#[doc = "DCH1SPTRINV register"]
pub struct DCH1SPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1SPTRINV register"]
pub mod dch1sptrinv;
#[doc = "DCH1DPTR register"]
pub struct DCH1DPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DPTR register"]
pub mod dch1dptr;
#[doc = "DCH1DPTRCLR register"]
pub struct DCH1DPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DPTRCLR register"]
pub mod dch1dptrclr;
#[doc = "DCH1DPTRSET register"]
pub struct DCH1DPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DPTRSET register"]
pub mod dch1dptrset;
#[doc = "DCH1DPTRINV register"]
pub struct DCH1DPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DPTRINV register"]
pub mod dch1dptrinv;
#[doc = "DCH1CSIZ register"]
pub struct DCH1CSIZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CSIZ register"]
pub mod dch1csiz;
#[doc = "DCH1CSIZCLR register"]
pub struct DCH1CSIZCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CSIZCLR register"]
pub mod dch1csizclr;
#[doc = "DCH1CSIZSET register"]
pub struct DCH1CSIZSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CSIZSET register"]
pub mod dch1csizset;
#[doc = "DCH1CSIZINV register"]
pub struct DCH1CSIZINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CSIZINV register"]
pub mod dch1csizinv;
#[doc = "DCH1CPTR register"]
pub struct DCH1CPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CPTR register"]
pub mod dch1cptr;
#[doc = "DCH1CPTRCLR register"]
pub struct DCH1CPTRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CPTRCLR register"]
pub mod dch1cptrclr;
#[doc = "DCH1CPTRSET register"]
pub struct DCH1CPTRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CPTRSET register"]
pub mod dch1cptrset;
#[doc = "DCH1CPTRINV register"]
pub struct DCH1CPTRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1CPTRINV register"]
pub mod dch1cptrinv;
#[doc = "DCH1DAT register"]
pub struct DCH1DAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DAT register"]
pub mod dch1dat;
#[doc = "DCH1DATCLR register"]
pub struct DCH1DATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DATCLR register"]
pub mod dch1datclr;
#[doc = "DCH1DATSET register"]
pub struct DCH1DATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DATSET register"]
pub mod dch1datset;
#[doc = "DCH1DATINV register"]
pub struct DCH1DATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCH1DATINV register"]
pub mod dch1datinv;
