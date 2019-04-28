#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI2CON register"]
    pub spi2con: SPI2CON,
    #[doc = "0x04 - SPI2CONCLR register"]
    pub spi2conclr: SPI2CONCLR,
    #[doc = "0x08 - SPI2CONSET register"]
    pub spi2conset: SPI2CONSET,
    #[doc = "0x0c - SPI2CONINV register"]
    pub spi2coninv: SPI2CONINV,
    #[doc = "0x10 - SPI2STAT register"]
    pub spi2stat: SPI2STAT,
    #[doc = "0x14 - SPI2STATCLR register"]
    pub spi2statclr: SPI2STATCLR,
    #[doc = "0x18 - SPI2STATSET register"]
    pub spi2statset: SPI2STATSET,
    #[doc = "0x1c - SPI2STATINV register"]
    pub spi2statinv: SPI2STATINV,
    #[doc = "0x20 - SPI2BUF register"]
    pub spi2buf: SPI2BUF,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - SPI2BRG register"]
    pub spi2brg: SPI2BRG,
    #[doc = "0x34 - SPI2BRGCLR register"]
    pub spi2brgclr: SPI2BRGCLR,
    #[doc = "0x38 - SPI2BRGSET register"]
    pub spi2brgset: SPI2BRGSET,
    #[doc = "0x3c - SPI2BRGINV register"]
    pub spi2brginv: SPI2BRGINV,
    #[doc = "0x40 - SPI2CON2 register"]
    pub spi2con2: SPI2CON2,
    #[doc = "0x44 - SPI2CON2CLR register"]
    pub spi2con2clr: SPI2CON2CLR,
    #[doc = "0x48 - SPI2CON2SET register"]
    pub spi2con2set: SPI2CON2SET,
    #[doc = "0x4c - SPI2CON2INV register"]
    pub spi2con2inv: SPI2CON2INV,
}
#[doc = "SPI2CON register"]
pub struct SPI2CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2CON register"]
pub mod spi2con;
#[doc = "SPI2CONCLR register"]
pub struct SPI2CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2CONCLR register"]
pub mod spi2conclr;
#[doc = "SPI2CONSET register"]
pub struct SPI2CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2CONSET register"]
pub mod spi2conset;
#[doc = "SPI2CONINV register"]
pub struct SPI2CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2CONINV register"]
pub mod spi2coninv;
#[doc = "SPI2STAT register"]
pub struct SPI2STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2STAT register"]
pub mod spi2stat;
#[doc = "SPI2STATCLR register"]
pub struct SPI2STATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2STATCLR register"]
pub mod spi2statclr;
#[doc = "SPI2STATSET register"]
pub struct SPI2STATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2STATSET register"]
pub mod spi2statset;
#[doc = "SPI2STATINV register"]
pub struct SPI2STATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2STATINV register"]
pub mod spi2statinv;
#[doc = "SPI2BUF register"]
pub struct SPI2BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2BUF register"]
pub mod spi2buf;
#[doc = "SPI2BRG register"]
pub struct SPI2BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2BRG register"]
pub mod spi2brg;
#[doc = "SPI2BRGCLR register"]
pub struct SPI2BRGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2BRGCLR register"]
pub mod spi2brgclr;
#[doc = "SPI2BRGSET register"]
pub struct SPI2BRGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2BRGSET register"]
pub mod spi2brgset;
#[doc = "SPI2BRGINV register"]
pub struct SPI2BRGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2BRGINV register"]
pub mod spi2brginv;
#[doc = "SPI2CON2 register"]
pub struct SPI2CON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2CON2 register"]
pub mod spi2con2;
#[doc = "SPI2CON2CLR register"]
pub struct SPI2CON2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2CON2CLR register"]
pub mod spi2con2clr;
#[doc = "SPI2CON2SET register"]
pub struct SPI2CON2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2CON2SET register"]
pub mod spi2con2set;
#[doc = "SPI2CON2INV register"]
pub struct SPI2CON2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI2CON2INV register"]
pub mod spi2con2inv;
