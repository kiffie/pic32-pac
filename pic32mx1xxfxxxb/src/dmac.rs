#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMACON register"]
    pub dmacon: DMACON,
    #[doc = "0x04 - DMACONCLR register"]
    pub dmaconclr: DMACONCLR,
    #[doc = "0x08 - DMACONSET register"]
    pub dmaconset: DMACONSET,
    #[doc = "0x0c - DMACONINV register"]
    pub dmaconinv: DMACONINV,
    #[doc = "0x10 - DMASTAT register"]
    pub dmastat: DMASTAT,
    #[doc = "0x14 - DMASTATCLR register"]
    pub dmastatclr: DMASTATCLR,
    #[doc = "0x18 - DMASTATSET register"]
    pub dmastatset: DMASTATSET,
    #[doc = "0x1c - DMASTATINV register"]
    pub dmastatinv: DMASTATINV,
    #[doc = "0x20 - DMAADDR register"]
    pub dmaaddr: DMAADDR,
    #[doc = "0x24 - DMAADDRCLR register"]
    pub dmaaddrclr: DMAADDRCLR,
    #[doc = "0x28 - DMAADDRSET register"]
    pub dmaaddrset: DMAADDRSET,
    #[doc = "0x2c - DMAADDRINV register"]
    pub dmaaddrinv: DMAADDRINV,
    #[doc = "0x30 - DCRCCON register"]
    pub dcrccon: DCRCCON,
    #[doc = "0x34 - DCRCCONCLR register"]
    pub dcrcconclr: DCRCCONCLR,
    #[doc = "0x38 - DCRCCONSET register"]
    pub dcrcconset: DCRCCONSET,
    #[doc = "0x3c - DCRCCONINV register"]
    pub dcrcconinv: DCRCCONINV,
    #[doc = "0x40 - DCRCDATA register"]
    pub dcrcdata: DCRCDATA,
    #[doc = "0x44 - DCRCDATACLR register"]
    pub dcrcdataclr: DCRCDATACLR,
    #[doc = "0x48 - DCRCDATASET register"]
    pub dcrcdataset: DCRCDATASET,
    #[doc = "0x4c - DCRCDATAINV register"]
    pub dcrcdatainv: DCRCDATAINV,
    #[doc = "0x50 - DCRCXOR register"]
    pub dcrcxor: DCRCXOR,
    #[doc = "0x54 - DCRCXORCLR register"]
    pub dcrcxorclr: DCRCXORCLR,
    #[doc = "0x58 - DCRCXORSET register"]
    pub dcrcxorset: DCRCXORSET,
    #[doc = "0x5c - DCRCXORINV register"]
    pub dcrcxorinv: DCRCXORINV,
}
#[doc = "DMACON register"]
pub struct DMACON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMACON register"]
pub mod dmacon;
#[doc = "DMACONCLR register"]
pub struct DMACONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMACONCLR register"]
pub mod dmaconclr;
#[doc = "DMACONSET register"]
pub struct DMACONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMACONSET register"]
pub mod dmaconset;
#[doc = "DMACONINV register"]
pub struct DMACONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMACONINV register"]
pub mod dmaconinv;
#[doc = "DMASTAT register"]
pub struct DMASTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMASTAT register"]
pub mod dmastat;
#[doc = "DMASTATCLR register"]
pub struct DMASTATCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMASTATCLR register"]
pub mod dmastatclr;
#[doc = "DMASTATSET register"]
pub struct DMASTATSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMASTATSET register"]
pub mod dmastatset;
#[doc = "DMASTATINV register"]
pub struct DMASTATINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMASTATINV register"]
pub mod dmastatinv;
#[doc = "DMAADDR register"]
pub struct DMAADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAADDR register"]
pub mod dmaaddr;
#[doc = "DMAADDRCLR register"]
pub struct DMAADDRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAADDRCLR register"]
pub mod dmaaddrclr;
#[doc = "DMAADDRSET register"]
pub struct DMAADDRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAADDRSET register"]
pub mod dmaaddrset;
#[doc = "DMAADDRINV register"]
pub struct DMAADDRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMAADDRINV register"]
pub mod dmaaddrinv;
#[doc = "DCRCCON register"]
pub struct DCRCCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCCON register"]
pub mod dcrccon;
#[doc = "DCRCCONCLR register"]
pub struct DCRCCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCCONCLR register"]
pub mod dcrcconclr;
#[doc = "DCRCCONSET register"]
pub struct DCRCCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCCONSET register"]
pub mod dcrcconset;
#[doc = "DCRCCONINV register"]
pub struct DCRCCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCCONINV register"]
pub mod dcrcconinv;
#[doc = "DCRCDATA register"]
pub struct DCRCDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCDATA register"]
pub mod dcrcdata;
#[doc = "DCRCDATACLR register"]
pub struct DCRCDATACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCDATACLR register"]
pub mod dcrcdataclr;
#[doc = "DCRCDATASET register"]
pub struct DCRCDATASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCDATASET register"]
pub mod dcrcdataset;
#[doc = "DCRCDATAINV register"]
pub struct DCRCDATAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCDATAINV register"]
pub mod dcrcdatainv;
#[doc = "DCRCXOR register"]
pub struct DCRCXOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCXOR register"]
pub mod dcrcxor;
#[doc = "DCRCXORCLR register"]
pub struct DCRCXORCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCXORCLR register"]
pub mod dcrcxorclr;
#[doc = "DCRCXORSET register"]
pub struct DCRCXORSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCXORSET register"]
pub mod dcrcxorset;
#[doc = "DCRCXORINV register"]
pub struct DCRCXORINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DCRCXORINV register"]
pub mod dcrcxorinv;
