#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C2CON register"]
    pub i2c2con: I2C2CON,
    #[doc = "0x04 - I2C2CONCLR register"]
    pub i2c2conclr: I2C2CONCLR,
    #[doc = "0x08 - I2C2CONSET register"]
    pub i2c2conset: I2C2CONSET,
    #[doc = "0x0c - I2C2CONINV register"]
    pub i2c2coninv: I2C2CONINV,
    #[doc = "0x10 - I2C2STAT register"]
    pub i2c2stat: I2C2STAT,
    #[doc = "0x14 - I2C2STATCLR register"]
    pub i2c2statclr: I2C2STATCLR,
    #[doc = "0x18 - I2C2STATSET register"]
    pub i2c2statset: I2C2STATSET,
    #[doc = "0x1c - I2C2STATINV register"]
    pub i2c2statinv: I2C2STATINV,
    #[doc = "0x20 - I2C2ADD register"]
    pub i2c2add: I2C2ADD,
    #[doc = "0x24 - I2C2ADDCLR register"]
    pub i2c2addclr: I2C2ADDCLR,
    #[doc = "0x28 - I2C2ADDSET register"]
    pub i2c2addset: I2C2ADDSET,
    #[doc = "0x2c - I2C2ADDINV register"]
    pub i2c2addinv: I2C2ADDINV,
    #[doc = "0x30 - I2C2MSK register"]
    pub i2c2msk: I2C2MSK,
    #[doc = "0x34 - I2C2MSKCLR register"]
    pub i2c2mskclr: I2C2MSKCLR,
    #[doc = "0x38 - I2C2MSKSET register"]
    pub i2c2mskset: I2C2MSKSET,
    #[doc = "0x3c - I2C2MSKINV register"]
    pub i2c2mskinv: I2C2MSKINV,
    #[doc = "0x40 - I2C2BRG register"]
    pub i2c2brg: I2C2BRG,
    #[doc = "0x44 - I2C2BRGCLR register"]
    pub i2c2brgclr: I2C2BRGCLR,
    #[doc = "0x48 - I2C2BRGSET register"]
    pub i2c2brgset: I2C2BRGSET,
    #[doc = "0x4c - I2C2BRGINV register"]
    pub i2c2brginv: I2C2BRGINV,
    #[doc = "0x50 - I2C2TRN register"]
    pub i2c2trn: I2C2TRN,
    #[doc = "0x54 - I2C2TRNCLR register"]
    pub i2c2trnclr: I2C2TRNCLR,
    #[doc = "0x58 - I2C2TRNSET register"]
    pub i2c2trnset: I2C2TRNSET,
    #[doc = "0x5c - I2C2TRNINV register"]
    pub i2c2trninv: I2C2TRNINV,
    #[doc = "0x60 - I2C2RCV register"]
    pub i2c2rcv: I2C2RCV,
}
#[doc = "I2C2CON register"]
pub struct I2C2CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2CON register"]
pub mod i2c2con;
#[doc = "I2C2CONCLR register"]
pub struct I2C2CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2CONCLR register"]
pub mod i2c2conclr;
#[doc = "I2C2CONSET register"]
pub struct I2C2CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2CONSET register"]
pub mod i2c2conset;
#[doc = "I2C2CONINV register"]
pub struct I2C2CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2CONINV register"]
pub mod i2c2coninv;
#[doc = "I2C2STAT register"]
pub struct I2C2STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2STAT register"]
pub mod i2c2stat;
#[doc = "I2C2STATCLR register"]
pub struct I2C2STATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2STATCLR register"]
pub mod i2c2statclr;
#[doc = "I2C2STATSET register"]
pub struct I2C2STATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2STATSET register"]
pub mod i2c2statset;
#[doc = "I2C2STATINV register"]
pub struct I2C2STATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2STATINV register"]
pub mod i2c2statinv;
#[doc = "I2C2ADD register"]
pub struct I2C2ADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2ADD register"]
pub mod i2c2add;
#[doc = "I2C2ADDCLR register"]
pub struct I2C2ADDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2ADDCLR register"]
pub mod i2c2addclr;
#[doc = "I2C2ADDSET register"]
pub struct I2C2ADDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2ADDSET register"]
pub mod i2c2addset;
#[doc = "I2C2ADDINV register"]
pub struct I2C2ADDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2ADDINV register"]
pub mod i2c2addinv;
#[doc = "I2C2MSK register"]
pub struct I2C2MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2MSK register"]
pub mod i2c2msk;
#[doc = "I2C2MSKCLR register"]
pub struct I2C2MSKCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2MSKCLR register"]
pub mod i2c2mskclr;
#[doc = "I2C2MSKSET register"]
pub struct I2C2MSKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2MSKSET register"]
pub mod i2c2mskset;
#[doc = "I2C2MSKINV register"]
pub struct I2C2MSKINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2MSKINV register"]
pub mod i2c2mskinv;
#[doc = "I2C2BRG register"]
pub struct I2C2BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2BRG register"]
pub mod i2c2brg;
#[doc = "I2C2BRGCLR register"]
pub struct I2C2BRGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2BRGCLR register"]
pub mod i2c2brgclr;
#[doc = "I2C2BRGSET register"]
pub struct I2C2BRGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2BRGSET register"]
pub mod i2c2brgset;
#[doc = "I2C2BRGINV register"]
pub struct I2C2BRGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2BRGINV register"]
pub mod i2c2brginv;
#[doc = "I2C2TRN register"]
pub struct I2C2TRN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2TRN register"]
pub mod i2c2trn;
#[doc = "I2C2TRNCLR register"]
pub struct I2C2TRNCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2TRNCLR register"]
pub mod i2c2trnclr;
#[doc = "I2C2TRNSET register"]
pub struct I2C2TRNSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2TRNSET register"]
pub mod i2c2trnset;
#[doc = "I2C2TRNINV register"]
pub struct I2C2TRNINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2TRNINV register"]
pub mod i2c2trninv;
#[doc = "I2C2RCV register"]
pub struct I2C2RCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C2RCV register"]
pub mod i2c2rcv;
