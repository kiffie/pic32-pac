#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U4MODE register"]
    pub u4mode: U4MODE,
    #[doc = "0x04 - U4MODECLR register"]
    pub u4modeclr: U4MODECLR,
    #[doc = "0x08 - U4MODESET register"]
    pub u4modeset: U4MODESET,
    #[doc = "0x0c - U4MODEINV register"]
    pub u4modeinv: U4MODEINV,
    #[doc = "0x10 - U4STA register"]
    pub u4sta: U4STA,
    #[doc = "0x14 - U4STACLR register"]
    pub u4staclr: U4STACLR,
    #[doc = "0x18 - U4STASET register"]
    pub u4staset: U4STASET,
    #[doc = "0x1c - U4STAINV register"]
    pub u4stainv: U4STAINV,
    #[doc = "0x20 - U4TXREG register"]
    pub u4txreg: U4TXREG,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - U4RXREG register"]
    pub u4rxreg: U4RXREG,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - U4BRG register"]
    pub u4brg: U4BRG,
    #[doc = "0x44 - U4BRGCLR register"]
    pub u4brgclr: U4BRGCLR,
    #[doc = "0x48 - U4BRGSET register"]
    pub u4brgset: U4BRGSET,
    #[doc = "0x4c - U4BRGINV register"]
    pub u4brginv: U4BRGINV,
}
#[doc = "U4MODE register"]
pub struct U4MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4MODE register"]
pub mod u4mode;
#[doc = "U4MODECLR register"]
pub struct U4MODECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4MODECLR register"]
pub mod u4modeclr;
#[doc = "U4MODESET register"]
pub struct U4MODESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4MODESET register"]
pub mod u4modeset;
#[doc = "U4MODEINV register"]
pub struct U4MODEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4MODEINV register"]
pub mod u4modeinv;
#[doc = "U4STA register"]
pub struct U4STA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4STA register"]
pub mod u4sta;
#[doc = "U4STACLR register"]
pub struct U4STACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4STACLR register"]
pub mod u4staclr;
#[doc = "U4STASET register"]
pub struct U4STASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4STASET register"]
pub mod u4staset;
#[doc = "U4STAINV register"]
pub struct U4STAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4STAINV register"]
pub mod u4stainv;
#[doc = "U4TXREG register"]
pub struct U4TXREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4TXREG register"]
pub mod u4txreg;
#[doc = "U4RXREG register"]
pub struct U4RXREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4RXREG register"]
pub mod u4rxreg;
#[doc = "U4BRG register"]
pub struct U4BRG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4BRG register"]
pub mod u4brg;
#[doc = "U4BRGCLR register"]
pub struct U4BRGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4BRGCLR register"]
pub mod u4brgclr;
#[doc = "U4BRGSET register"]
pub struct U4BRGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4BRGSET register"]
pub mod u4brgset;
#[doc = "U4BRGINV register"]
pub struct U4BRGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4BRGINV register"]
pub mod u4brginv;
