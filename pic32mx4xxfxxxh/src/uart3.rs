#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U3MODE register"]
    pub u3mode: U3MODE,
    #[doc = "0x04 - U3MODECLR register"]
    pub u3modeclr: U3MODECLR,
    #[doc = "0x08 - U3MODESET register"]
    pub u3modeset: U3MODESET,
    #[doc = "0x0c - U3MODEINV register"]
    pub u3modeinv: U3MODEINV,
    #[doc = "0x10 - U3STA register"]
    pub u3sta: U3STA,
    #[doc = "0x14 - U3STACLR register"]
    pub u3staclr: U3STACLR,
    #[doc = "0x18 - U3STASET register"]
    pub u3staset: U3STASET,
    #[doc = "0x1c - U3STAINV register"]
    pub u3stainv: U3STAINV,
    #[doc = "0x20 - U3TXREG register"]
    pub u3txreg: U3TXREG,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - U3RXREG register"]
    pub u3rxreg: U3RXREG,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - U3BRG register"]
    pub u3brg: U3BRG,
    #[doc = "0x44 - U3BRGCLR register"]
    pub u3brgclr: U3BRGCLR,
    #[doc = "0x48 - U3BRGSET register"]
    pub u3brgset: U3BRGSET,
    #[doc = "0x4c - U3BRGINV register"]
    pub u3brginv: U3BRGINV,
}
#[doc = "U3MODE register"]
pub struct U3MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3MODE register"]
pub mod u3mode;
#[doc = "U3MODECLR register"]
pub struct U3MODECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3MODECLR register"]
pub mod u3modeclr;
#[doc = "U3MODESET register"]
pub struct U3MODESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3MODESET register"]
pub mod u3modeset;
#[doc = "U3MODEINV register"]
pub struct U3MODEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3MODEINV register"]
pub mod u3modeinv;
#[doc = "U3STA register"]
pub struct U3STA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3STA register"]
pub mod u3sta;
#[doc = "U3STACLR register"]
pub struct U3STACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3STACLR register"]
pub mod u3staclr;
#[doc = "U3STASET register"]
pub struct U3STASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3STASET register"]
pub mod u3staset;
#[doc = "U3STAINV register"]
pub struct U3STAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3STAINV register"]
pub mod u3stainv;
#[doc = "U3TXREG register"]
pub struct U3TXREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3TXREG register"]
pub mod u3txreg;
#[doc = "U3RXREG register"]
pub struct U3RXREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3RXREG register"]
pub mod u3rxreg;
#[doc = "U3BRG register"]
pub struct U3BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3BRG register"]
pub mod u3brg;
#[doc = "U3BRGCLR register"]
pub struct U3BRGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3BRGCLR register"]
pub mod u3brgclr;
#[doc = "U3BRGSET register"]
pub struct U3BRGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3BRGSET register"]
pub mod u3brgset;
#[doc = "U3BRGINV register"]
pub struct U3BRGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3BRGINV register"]
pub mod u3brginv;
