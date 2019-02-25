#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U2MODE register"]
    pub u2mode: U2MODE,
    #[doc = "0x04 - U2MODECLR register"]
    pub u2modeclr: U2MODECLR,
    #[doc = "0x08 - U2MODESET register"]
    pub u2modeset: U2MODESET,
    #[doc = "0x0c - U2MODEINV register"]
    pub u2modeinv: U2MODEINV,
    #[doc = "0x10 - U2STA register"]
    pub u2sta: U2STA,
    #[doc = "0x14 - U2STACLR register"]
    pub u2staclr: U2STACLR,
    #[doc = "0x18 - U2STASET register"]
    pub u2staset: U2STASET,
    #[doc = "0x1c - U2STAINV register"]
    pub u2stainv: U2STAINV,
    #[doc = "0x20 - U2TXREG register"]
    pub u2txreg: U2TXREG,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - U2RXREG register"]
    pub u2rxreg: U2RXREG,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - U2BRG register"]
    pub u2brg: U2BRG,
    #[doc = "0x44 - U2BRGCLR register"]
    pub u2brgclr: U2BRGCLR,
    #[doc = "0x48 - U2BRGSET register"]
    pub u2brgset: U2BRGSET,
    #[doc = "0x4c - U2BRGINV register"]
    pub u2brginv: U2BRGINV,
}
#[doc = "U2MODE register"]
pub struct U2MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2MODE register"]
pub mod u2mode;
#[doc = "U2MODECLR register"]
pub struct U2MODECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2MODECLR register"]
pub mod u2modeclr;
#[doc = "U2MODESET register"]
pub struct U2MODESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2MODESET register"]
pub mod u2modeset;
#[doc = "U2MODEINV register"]
pub struct U2MODEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2MODEINV register"]
pub mod u2modeinv;
#[doc = "U2STA register"]
pub struct U2STA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2STA register"]
pub mod u2sta;
#[doc = "U2STACLR register"]
pub struct U2STACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2STACLR register"]
pub mod u2staclr;
#[doc = "U2STASET register"]
pub struct U2STASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2STASET register"]
pub mod u2staset;
#[doc = "U2STAINV register"]
pub struct U2STAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2STAINV register"]
pub mod u2stainv;
#[doc = "U2TXREG register"]
pub struct U2TXREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2TXREG register"]
pub mod u2txreg;
#[doc = "U2RXREG register"]
pub struct U2RXREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2RXREG register"]
pub mod u2rxreg;
#[doc = "U2BRG register"]
pub struct U2BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2BRG register"]
pub mod u2brg;
#[doc = "U2BRGCLR register"]
pub struct U2BRGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2BRGCLR register"]
pub mod u2brgclr;
#[doc = "U2BRGSET register"]
pub struct U2BRGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2BRGSET register"]
pub mod u2brgset;
#[doc = "U2BRGINV register"]
pub struct U2BRGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2BRGINV register"]
pub mod u2brginv;
