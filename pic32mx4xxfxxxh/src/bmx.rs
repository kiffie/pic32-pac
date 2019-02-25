#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BMXCON register"]
    pub bmxcon: BMXCON,
    #[doc = "0x04 - BMXCONCLR register"]
    pub bmxconclr: BMXCONCLR,
    #[doc = "0x08 - BMXCONSET register"]
    pub bmxconset: BMXCONSET,
    #[doc = "0x0c - BMXCONINV register"]
    pub bmxconinv: BMXCONINV,
    #[doc = "0x10 - BMXDKPBA register"]
    pub bmxdkpba: BMXDKPBA,
    #[doc = "0x14 - BMXDKPBACLR register"]
    pub bmxdkpbaclr: BMXDKPBACLR,
    #[doc = "0x18 - BMXDKPBASET register"]
    pub bmxdkpbaset: BMXDKPBASET,
    #[doc = "0x1c - BMXDKPBAINV register"]
    pub bmxdkpbainv: BMXDKPBAINV,
    #[doc = "0x20 - BMXDUDBA register"]
    pub bmxdudba: BMXDUDBA,
    #[doc = "0x24 - BMXDUDBACLR register"]
    pub bmxdudbaclr: BMXDUDBACLR,
    #[doc = "0x28 - BMXDUDBASET register"]
    pub bmxdudbaset: BMXDUDBASET,
    #[doc = "0x2c - BMXDUDBAINV register"]
    pub bmxdudbainv: BMXDUDBAINV,
    #[doc = "0x30 - BMXDUPBA register"]
    pub bmxdupba: BMXDUPBA,
    #[doc = "0x34 - BMXDUPBACLR register"]
    pub bmxdupbaclr: BMXDUPBACLR,
    #[doc = "0x38 - BMXDUPBASET register"]
    pub bmxdupbaset: BMXDUPBASET,
    #[doc = "0x3c - BMXDUPBAINV register"]
    pub bmxdupbainv: BMXDUPBAINV,
    #[doc = "0x40 - BMXDRMSZ register"]
    pub bmxdrmsz: BMXDRMSZ,
    _reserved0: [u8; 12usize],
    #[doc = "0x50 - BMXPUPBA register"]
    pub bmxpupba: BMXPUPBA,
    #[doc = "0x54 - BMXPUPBACLR register"]
    pub bmxpupbaclr: BMXPUPBACLR,
    #[doc = "0x58 - BMXPUPBASET register"]
    pub bmxpupbaset: BMXPUPBASET,
    #[doc = "0x5c - BMXPUPBAINV register"]
    pub bmxpupbainv: BMXPUPBAINV,
    #[doc = "0x60 - BMXPFMSZ register"]
    pub bmxpfmsz: BMXPFMSZ,
    _reserved1: [u8; 12usize],
    #[doc = "0x70 - BMXBOOTSZ register"]
    pub bmxbootsz: BMXBOOTSZ,
}
#[doc = "BMXCON register"]
pub struct BMXCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXCON register"]
pub mod bmxcon;
#[doc = "BMXCONCLR register"]
pub struct BMXCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXCONCLR register"]
pub mod bmxconclr;
#[doc = "BMXCONSET register"]
pub struct BMXCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXCONSET register"]
pub mod bmxconset;
#[doc = "BMXCONINV register"]
pub struct BMXCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXCONINV register"]
pub mod bmxconinv;
#[doc = "BMXDKPBA register"]
pub struct BMXDKPBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDKPBA register"]
pub mod bmxdkpba;
#[doc = "BMXDKPBACLR register"]
pub struct BMXDKPBACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDKPBACLR register"]
pub mod bmxdkpbaclr;
#[doc = "BMXDKPBASET register"]
pub struct BMXDKPBASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDKPBASET register"]
pub mod bmxdkpbaset;
#[doc = "BMXDKPBAINV register"]
pub struct BMXDKPBAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDKPBAINV register"]
pub mod bmxdkpbainv;
#[doc = "BMXDUDBA register"]
pub struct BMXDUDBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDUDBA register"]
pub mod bmxdudba;
#[doc = "BMXDUDBACLR register"]
pub struct BMXDUDBACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDUDBACLR register"]
pub mod bmxdudbaclr;
#[doc = "BMXDUDBASET register"]
pub struct BMXDUDBASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDUDBASET register"]
pub mod bmxdudbaset;
#[doc = "BMXDUDBAINV register"]
pub struct BMXDUDBAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDUDBAINV register"]
pub mod bmxdudbainv;
#[doc = "BMXDUPBA register"]
pub struct BMXDUPBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDUPBA register"]
pub mod bmxdupba;
#[doc = "BMXDUPBACLR register"]
pub struct BMXDUPBACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDUPBACLR register"]
pub mod bmxdupbaclr;
#[doc = "BMXDUPBASET register"]
pub struct BMXDUPBASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDUPBASET register"]
pub mod bmxdupbaset;
#[doc = "BMXDUPBAINV register"]
pub struct BMXDUPBAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDUPBAINV register"]
pub mod bmxdupbainv;
#[doc = "BMXDRMSZ register"]
pub struct BMXDRMSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXDRMSZ register"]
pub mod bmxdrmsz;
#[doc = "BMXPUPBA register"]
pub struct BMXPUPBA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXPUPBA register"]
pub mod bmxpupba;
#[doc = "BMXPUPBACLR register"]
pub struct BMXPUPBACLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXPUPBACLR register"]
pub mod bmxpupbaclr;
#[doc = "BMXPUPBASET register"]
pub struct BMXPUPBASET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXPUPBASET register"]
pub mod bmxpupbaset;
#[doc = "BMXPUPBAINV register"]
pub struct BMXPUPBAINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXPUPBAINV register"]
pub mod bmxpupbainv;
#[doc = "BMXPFMSZ register"]
pub struct BMXPFMSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXPFMSZ register"]
pub mod bmxpfmsz;
#[doc = "BMXBOOTSZ register"]
pub struct BMXBOOTSZ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "BMXBOOTSZ register"]
pub mod bmxbootsz;
