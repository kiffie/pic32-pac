#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI1CON register"]
    pub spi1con: SPI1CON,
    #[doc = "0x04 - SPI1CONCLR register"]
    pub spi1conclr: SPI1CONCLR,
    #[doc = "0x08 - SPI1CONSET register"]
    pub spi1conset: SPI1CONSET,
    #[doc = "0x0c - SPI1CONINV register"]
    pub spi1coninv: SPI1CONINV,
    #[doc = "0x10 - SPI1STAT register"]
    pub spi1stat: SPI1STAT,
    #[doc = "0x14 - SPI1STATCLR register"]
    pub spi1statclr: SPI1STATCLR,
    #[doc = "0x18 - SPI1STATSET register"]
    pub spi1statset: SPI1STATSET,
    #[doc = "0x1c - SPI1STATINV register"]
    pub spi1statinv: SPI1STATINV,
    #[doc = "0x20 - SPI1BUF register"]
    pub spi1buf: SPI1BUF,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - SPI1BRG register"]
    pub spi1brg: SPI1BRG,
    #[doc = "0x34 - SPI1BRGCLR register"]
    pub spi1brgclr: SPI1BRGCLR,
    #[doc = "0x38 - SPI1BRGSET register"]
    pub spi1brgset: SPI1BRGSET,
    #[doc = "0x3c - SPI1BRGINV register"]
    pub spi1brginv: SPI1BRGINV,
    #[doc = "0x40 - SPI1CON2 register"]
    pub spi1con2: SPI1CON2,
    #[doc = "0x44 - SPI1CON2CLR register"]
    pub spi1con2clr: SPI1CON2CLR,
    #[doc = "0x48 - SPI1CON2SET register"]
    pub spi1con2set: SPI1CON2SET,
    #[doc = "0x4c - SPI1CON2INV register"]
    pub spi1con2inv: SPI1CON2INV,
}
#[doc = "SPI1CON register"]
pub struct SPI1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1CON register"]
pub mod spi1con;
#[doc = "SPI1CONCLR register"]
pub struct SPI1CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1CONCLR register"]
pub mod spi1conclr;
#[doc = "SPI1CONSET register"]
pub struct SPI1CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1CONSET register"]
pub mod spi1conset;
#[doc = "SPI1CONINV register"]
pub struct SPI1CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1CONINV register"]
pub mod spi1coninv;
#[doc = "SPI1STAT register"]
pub struct SPI1STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1STAT register"]
pub mod spi1stat;
#[doc = "SPI1STATCLR register"]
pub struct SPI1STATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1STATCLR register"]
pub mod spi1statclr;
#[doc = "SPI1STATSET register"]
pub struct SPI1STATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1STATSET register"]
pub mod spi1statset;
#[doc = "SPI1STATINV register"]
pub struct SPI1STATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1STATINV register"]
pub mod spi1statinv;
#[doc = "SPI1BUF register"]
pub struct SPI1BUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1BUF register"]
pub mod spi1buf;
#[doc = "SPI1BRG register"]
pub struct SPI1BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1BRG register"]
pub mod spi1brg;
#[doc = "SPI1BRGCLR register"]
pub struct SPI1BRGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1BRGCLR register"]
pub mod spi1brgclr;
#[doc = "SPI1BRGSET register"]
pub struct SPI1BRGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1BRGSET register"]
pub mod spi1brgset;
#[doc = "SPI1BRGINV register"]
pub struct SPI1BRGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1BRGINV register"]
pub mod spi1brginv;
#[doc = "SPI1CON2 register"]
pub struct SPI1CON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1CON2 register"]
pub mod spi1con2;
#[doc = "SPI1CON2CLR register"]
pub struct SPI1CON2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1CON2CLR register"]
pub mod spi1con2clr;
#[doc = "SPI1CON2SET register"]
pub struct SPI1CON2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1CON2SET register"]
pub mod spi1con2set;
#[doc = "SPI1CON2INV register"]
pub struct SPI1CON2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI1CON2INV register"]
pub mod spi1con2inv;
