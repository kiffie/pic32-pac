#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTCCON register"]
    pub rtccon: RTCCON,
    #[doc = "0x04 - RTCCONCLR register"]
    pub rtcconclr: RTCCONCLR,
    #[doc = "0x08 - RTCCONSET register"]
    pub rtcconset: RTCCONSET,
    #[doc = "0x0c - RTCCONINV register"]
    pub rtcconinv: RTCCONINV,
    #[doc = "0x10 - RTCALRM register"]
    pub rtcalrm: RTCALRM,
    #[doc = "0x14 - RTCALRMCLR register"]
    pub rtcalrmclr: RTCALRMCLR,
    #[doc = "0x18 - RTCALRMSET register"]
    pub rtcalrmset: RTCALRMSET,
    #[doc = "0x1c - RTCALRMINV register"]
    pub rtcalrminv: RTCALRMINV,
    #[doc = "0x20 - RTCTIME register"]
    pub rtctime: RTCTIME,
    #[doc = "0x24 - RTCTIMECLR register"]
    pub rtctimeclr: RTCTIMECLR,
    #[doc = "0x28 - RTCTIMESET register"]
    pub rtctimeset: RTCTIMESET,
    #[doc = "0x2c - RTCTIMEINV register"]
    pub rtctimeinv: RTCTIMEINV,
    #[doc = "0x30 - RTCDATE register"]
    pub rtcdate: RTCDATE,
    #[doc = "0x34 - RTCDATECLR register"]
    pub rtcdateclr: RTCDATECLR,
    #[doc = "0x38 - RTCDATESET register"]
    pub rtcdateset: RTCDATESET,
    #[doc = "0x3c - RTCDATEINV register"]
    pub rtcdateinv: RTCDATEINV,
    #[doc = "0x40 - ALRMTIME register"]
    pub alrmtime: ALRMTIME,
    #[doc = "0x44 - ALRMTIMECLR register"]
    pub alrmtimeclr: ALRMTIMECLR,
    #[doc = "0x48 - ALRMTIMESET register"]
    pub alrmtimeset: ALRMTIMESET,
    #[doc = "0x4c - ALRMTIMEINV register"]
    pub alrmtimeinv: ALRMTIMEINV,
    #[doc = "0x50 - ALRMDATE register"]
    pub alrmdate: ALRMDATE,
    #[doc = "0x54 - ALRMDATECLR register"]
    pub alrmdateclr: ALRMDATECLR,
    #[doc = "0x58 - ALRMDATESET register"]
    pub alrmdateset: ALRMDATESET,
    #[doc = "0x5c - ALRMDATEINV register"]
    pub alrmdateinv: ALRMDATEINV,
}
#[doc = "RTCCON register"]
pub struct RTCCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCCON register"]
pub mod rtccon;
#[doc = "RTCCONCLR register"]
pub struct RTCCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCCONCLR register"]
pub mod rtcconclr;
#[doc = "RTCCONSET register"]
pub struct RTCCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCCONSET register"]
pub mod rtcconset;
#[doc = "RTCCONINV register"]
pub struct RTCCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCCONINV register"]
pub mod rtcconinv;
#[doc = "RTCALRM register"]
pub struct RTCALRM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCALRM register"]
pub mod rtcalrm;
#[doc = "RTCALRMCLR register"]
pub struct RTCALRMCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCALRMCLR register"]
pub mod rtcalrmclr;
#[doc = "RTCALRMSET register"]
pub struct RTCALRMSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCALRMSET register"]
pub mod rtcalrmset;
#[doc = "RTCALRMINV register"]
pub struct RTCALRMINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCALRMINV register"]
pub mod rtcalrminv;
#[doc = "RTCTIME register"]
pub struct RTCTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCTIME register"]
pub mod rtctime;
#[doc = "RTCTIMECLR register"]
pub struct RTCTIMECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCTIMECLR register"]
pub mod rtctimeclr;
#[doc = "RTCTIMESET register"]
pub struct RTCTIMESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCTIMESET register"]
pub mod rtctimeset;
#[doc = "RTCTIMEINV register"]
pub struct RTCTIMEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCTIMEINV register"]
pub mod rtctimeinv;
#[doc = "RTCDATE register"]
pub struct RTCDATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCDATE register"]
pub mod rtcdate;
#[doc = "RTCDATECLR register"]
pub struct RTCDATECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCDATECLR register"]
pub mod rtcdateclr;
#[doc = "RTCDATESET register"]
pub struct RTCDATESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCDATESET register"]
pub mod rtcdateset;
#[doc = "RTCDATEINV register"]
pub struct RTCDATEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTCDATEINV register"]
pub mod rtcdateinv;
#[doc = "ALRMTIME register"]
pub struct ALRMTIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ALRMTIME register"]
pub mod alrmtime;
#[doc = "ALRMTIMECLR register"]
pub struct ALRMTIMECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ALRMTIMECLR register"]
pub mod alrmtimeclr;
#[doc = "ALRMTIMESET register"]
pub struct ALRMTIMESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ALRMTIMESET register"]
pub mod alrmtimeset;
#[doc = "ALRMTIMEINV register"]
pub struct ALRMTIMEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ALRMTIMEINV register"]
pub mod alrmtimeinv;
#[doc = "ALRMDATE register"]
pub struct ALRMDATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ALRMDATE register"]
pub mod alrmdate;
#[doc = "ALRMDATECLR register"]
pub struct ALRMDATECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ALRMDATECLR register"]
pub mod alrmdateclr;
#[doc = "ALRMDATESET register"]
pub struct ALRMDATESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ALRMDATESET register"]
pub mod alrmdateset;
#[doc = "ALRMDATEINV register"]
pub struct ALRMDATEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ALRMDATEINV register"]
pub mod alrmdateinv;
