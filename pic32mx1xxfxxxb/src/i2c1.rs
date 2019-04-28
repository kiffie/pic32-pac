#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C1CON register"]
    pub i2c1con: I2C1CON,
    #[doc = "0x04 - I2C1CONCLR register"]
    pub i2c1conclr: I2C1CONCLR,
    #[doc = "0x08 - I2C1CONSET register"]
    pub i2c1conset: I2C1CONSET,
    #[doc = "0x0c - I2C1CONINV register"]
    pub i2c1coninv: I2C1CONINV,
    #[doc = "0x10 - I2C1STAT register"]
    pub i2c1stat: I2C1STAT,
    #[doc = "0x14 - I2C1STATCLR register"]
    pub i2c1statclr: I2C1STATCLR,
    #[doc = "0x18 - I2C1STATSET register"]
    pub i2c1statset: I2C1STATSET,
    #[doc = "0x1c - I2C1STATINV register"]
    pub i2c1statinv: I2C1STATINV,
    #[doc = "0x20 - I2C1ADD register"]
    pub i2c1add: I2C1ADD,
    #[doc = "0x24 - I2C1ADDCLR register"]
    pub i2c1addclr: I2C1ADDCLR,
    #[doc = "0x28 - I2C1ADDSET register"]
    pub i2c1addset: I2C1ADDSET,
    #[doc = "0x2c - I2C1ADDINV register"]
    pub i2c1addinv: I2C1ADDINV,
    #[doc = "0x30 - I2C1MSK register"]
    pub i2c1msk: I2C1MSK,
    #[doc = "0x34 - I2C1MSKCLR register"]
    pub i2c1mskclr: I2C1MSKCLR,
    #[doc = "0x38 - I2C1MSKSET register"]
    pub i2c1mskset: I2C1MSKSET,
    #[doc = "0x3c - I2C1MSKINV register"]
    pub i2c1mskinv: I2C1MSKINV,
    #[doc = "0x40 - I2C1BRG register"]
    pub i2c1brg: I2C1BRG,
    #[doc = "0x44 - I2C1BRGCLR register"]
    pub i2c1brgclr: I2C1BRGCLR,
    #[doc = "0x48 - I2C1BRGSET register"]
    pub i2c1brgset: I2C1BRGSET,
    #[doc = "0x4c - I2C1BRGINV register"]
    pub i2c1brginv: I2C1BRGINV,
    #[doc = "0x50 - I2C1TRN register"]
    pub i2c1trn: I2C1TRN,
    #[doc = "0x54 - I2C1TRNCLR register"]
    pub i2c1trnclr: I2C1TRNCLR,
    #[doc = "0x58 - I2C1TRNSET register"]
    pub i2c1trnset: I2C1TRNSET,
    #[doc = "0x5c - I2C1TRNINV register"]
    pub i2c1trninv: I2C1TRNINV,
    #[doc = "0x60 - I2C1RCV register"]
    pub i2c1rcv: I2C1RCV,
}
#[doc = "I2C1CON register"]
pub struct I2C1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1CON register"]
pub mod i2c1con;
#[doc = "I2C1CONCLR register"]
pub struct I2C1CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1CONCLR register"]
pub mod i2c1conclr;
#[doc = "I2C1CONSET register"]
pub struct I2C1CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1CONSET register"]
pub mod i2c1conset;
#[doc = "I2C1CONINV register"]
pub struct I2C1CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1CONINV register"]
pub mod i2c1coninv;
#[doc = "I2C1STAT register"]
pub struct I2C1STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1STAT register"]
pub mod i2c1stat;
#[doc = "I2C1STATCLR register"]
pub struct I2C1STATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1STATCLR register"]
pub mod i2c1statclr;
#[doc = "I2C1STATSET register"]
pub struct I2C1STATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1STATSET register"]
pub mod i2c1statset;
#[doc = "I2C1STATINV register"]
pub struct I2C1STATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1STATINV register"]
pub mod i2c1statinv;
#[doc = "I2C1ADD register"]
pub struct I2C1ADD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1ADD register"]
pub mod i2c1add;
#[doc = "I2C1ADDCLR register"]
pub struct I2C1ADDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1ADDCLR register"]
pub mod i2c1addclr;
#[doc = "I2C1ADDSET register"]
pub struct I2C1ADDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1ADDSET register"]
pub mod i2c1addset;
#[doc = "I2C1ADDINV register"]
pub struct I2C1ADDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1ADDINV register"]
pub mod i2c1addinv;
#[doc = "I2C1MSK register"]
pub struct I2C1MSK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1MSK register"]
pub mod i2c1msk;
#[doc = "I2C1MSKCLR register"]
pub struct I2C1MSKCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1MSKCLR register"]
pub mod i2c1mskclr;
#[doc = "I2C1MSKSET register"]
pub struct I2C1MSKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1MSKSET register"]
pub mod i2c1mskset;
#[doc = "I2C1MSKINV register"]
pub struct I2C1MSKINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1MSKINV register"]
pub mod i2c1mskinv;
#[doc = "I2C1BRG register"]
pub struct I2C1BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1BRG register"]
pub mod i2c1brg;
#[doc = "I2C1BRGCLR register"]
pub struct I2C1BRGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1BRGCLR register"]
pub mod i2c1brgclr;
#[doc = "I2C1BRGSET register"]
pub struct I2C1BRGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1BRGSET register"]
pub mod i2c1brgset;
#[doc = "I2C1BRGINV register"]
pub struct I2C1BRGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1BRGINV register"]
pub mod i2c1brginv;
#[doc = "I2C1TRN register"]
pub struct I2C1TRN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1TRN register"]
pub mod i2c1trn;
#[doc = "I2C1TRNCLR register"]
pub struct I2C1TRNCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1TRNCLR register"]
pub mod i2c1trnclr;
#[doc = "I2C1TRNSET register"]
pub struct I2C1TRNSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1TRNSET register"]
pub mod i2c1trnset;
#[doc = "I2C1TRNINV register"]
pub struct I2C1TRNINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1TRNINV register"]
pub mod i2c1trninv;
#[doc = "I2C1RCV register"]
pub struct I2C1RCV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C1RCV register"]
pub mod i2c1rcv;
