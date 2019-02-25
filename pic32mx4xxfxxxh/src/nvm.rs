#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - NVMCON register"]
    pub nvmcon: NVMCON,
    #[doc = "0x04 - NVMCONCLR register"]
    pub nvmconclr: NVMCONCLR,
    #[doc = "0x08 - NVMCONSET register"]
    pub nvmconset: NVMCONSET,
    #[doc = "0x0c - NVMCONINV register"]
    pub nvmconinv: NVMCONINV,
    #[doc = "0x10 - NVMKEY register"]
    pub nvmkey: NVMKEY,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - NVMADDR register"]
    pub nvmaddr: NVMADDR,
    #[doc = "0x24 - NVMADDRCLR register"]
    pub nvmaddrclr: NVMADDRCLR,
    #[doc = "0x28 - NVMADDRSET register"]
    pub nvmaddrset: NVMADDRSET,
    #[doc = "0x2c - NVMADDRINV register"]
    pub nvmaddrinv: NVMADDRINV,
    #[doc = "0x30 - NVMDATA register"]
    pub nvmdata: NVMDATA,
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - NVMSRCADDR register"]
    pub nvmsrcaddr: NVMSRCADDR,
}
#[doc = "NVMCON register"]
pub struct NVMCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMCON register"]
pub mod nvmcon;
#[doc = "NVMCONCLR register"]
pub struct NVMCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMCONCLR register"]
pub mod nvmconclr;
#[doc = "NVMCONSET register"]
pub struct NVMCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMCONSET register"]
pub mod nvmconset;
#[doc = "NVMCONINV register"]
pub struct NVMCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMCONINV register"]
pub mod nvmconinv;
#[doc = "NVMKEY register"]
pub struct NVMKEY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMKEY register"]
pub mod nvmkey;
#[doc = "NVMADDR register"]
pub struct NVMADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMADDR register"]
pub mod nvmaddr;
#[doc = "NVMADDRCLR register"]
pub struct NVMADDRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMADDRCLR register"]
pub mod nvmaddrclr;
#[doc = "NVMADDRSET register"]
pub struct NVMADDRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMADDRSET register"]
pub mod nvmaddrset;
#[doc = "NVMADDRINV register"]
pub struct NVMADDRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMADDRINV register"]
pub mod nvmaddrinv;
#[doc = "NVMDATA register"]
pub struct NVMDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMDATA register"]
pub mod nvmdata;
#[doc = "NVMSRCADDR register"]
pub struct NVMSRCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "NVMSRCADDR register"]
pub mod nvmsrcaddr;
