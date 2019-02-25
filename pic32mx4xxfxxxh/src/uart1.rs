#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U1MODE register"]
    pub u1mode: U1MODE,
    #[doc = "0x04 - U1MODECLR register"]
    pub u1modeclr: U1MODECLR,
    #[doc = "0x08 - U1MODESET register"]
    pub u1modeset: U1MODESET,
    #[doc = "0x0c - U1MODEINV register"]
    pub u1modeinv: U1MODEINV,
    #[doc = "0x10 - U1STA register"]
    pub u1sta: U1STA,
    #[doc = "0x14 - U1STACLR register"]
    pub u1staclr: U1STACLR,
    #[doc = "0x18 - U1STASET register"]
    pub u1staset: U1STASET,
    #[doc = "0x1c - U1STAINV register"]
    pub u1stainv: U1STAINV,
    #[doc = "0x20 - U1TXREG register"]
    pub u1txreg: U1TXREG,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - U1RXREG register"]
    pub u1rxreg: U1RXREG,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - U1BRG register"]
    pub u1brg: U1BRG,
    #[doc = "0x44 - U1BRGCLR register"]
    pub u1brgclr: U1BRGCLR,
    #[doc = "0x48 - U1BRGSET register"]
    pub u1brgset: U1BRGSET,
    #[doc = "0x4c - U1BRGINV register"]
    pub u1brginv: U1BRGINV,
}
#[doc = "U1MODE register"]
pub struct U1MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1MODE register"]
pub mod u1mode;
#[doc = "U1MODECLR register"]
pub struct U1MODECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1MODECLR register"]
pub mod u1modeclr;
#[doc = "U1MODESET register"]
pub struct U1MODESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1MODESET register"]
pub mod u1modeset;
#[doc = "U1MODEINV register"]
pub struct U1MODEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1MODEINV register"]
pub mod u1modeinv;
#[doc = "U1STA register"]
pub struct U1STA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1STA register"]
pub mod u1sta;
#[doc = "U1STACLR register"]
pub struct U1STACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1STACLR register"]
pub mod u1staclr;
#[doc = "U1STASET register"]
pub struct U1STASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1STASET register"]
pub mod u1staset;
#[doc = "U1STAINV register"]
pub struct U1STAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1STAINV register"]
pub mod u1stainv;
#[doc = "U1TXREG register"]
pub struct U1TXREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1TXREG register"]
pub mod u1txreg;
#[doc = "U1RXREG register"]
pub struct U1RXREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1RXREG register"]
pub mod u1rxreg;
#[doc = "U1BRG register"]
pub struct U1BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BRG register"]
pub mod u1brg;
#[doc = "U1BRGCLR register"]
pub struct U1BRGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BRGCLR register"]
pub mod u1brgclr;
#[doc = "U1BRGSET register"]
pub struct U1BRGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BRGSET register"]
pub mod u1brgset;
#[doc = "U1BRGINV register"]
pub struct U1BRGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BRGINV register"]
pub mod u1brginv;
