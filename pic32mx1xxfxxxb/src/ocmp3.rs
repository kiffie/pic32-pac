#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC3CON register"]
    pub oc3con: OC3CON,
    #[doc = "0x04 - OC3CONCLR register"]
    pub oc3conclr: OC3CONCLR,
    #[doc = "0x08 - OC3CONSET register"]
    pub oc3conset: OC3CONSET,
    #[doc = "0x0c - OC3CONINV register"]
    pub oc3coninv: OC3CONINV,
    #[doc = "0x10 - OC3R register"]
    pub oc3r: OC3R,
    #[doc = "0x14 - OC3RCLR register"]
    pub oc3rclr: OC3RCLR,
    #[doc = "0x18 - OC3RSET register"]
    pub oc3rset: OC3RSET,
    #[doc = "0x1c - OC3RINV register"]
    pub oc3rinv: OC3RINV,
    #[doc = "0x20 - OC3RS register"]
    pub oc3rs: OC3RS,
    #[doc = "0x24 - OC3RSCLR register"]
    pub oc3rsclr: OC3RSCLR,
    #[doc = "0x28 - OC3RSSET register"]
    pub oc3rsset: OC3RSSET,
    #[doc = "0x2c - OC3RSINV register"]
    pub oc3rsinv: OC3RSINV,
}
#[doc = "OC3CON register"]
pub struct OC3CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3CON register"]
pub mod oc3con;
#[doc = "OC3CONCLR register"]
pub struct OC3CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3CONCLR register"]
pub mod oc3conclr;
#[doc = "OC3CONSET register"]
pub struct OC3CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3CONSET register"]
pub mod oc3conset;
#[doc = "OC3CONINV register"]
pub struct OC3CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3CONINV register"]
pub mod oc3coninv;
#[doc = "OC3R register"]
pub struct OC3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3R register"]
pub mod oc3r;
#[doc = "OC3RCLR register"]
pub struct OC3RCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3RCLR register"]
pub mod oc3rclr;
#[doc = "OC3RSET register"]
pub struct OC3RSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3RSET register"]
pub mod oc3rset;
#[doc = "OC3RINV register"]
pub struct OC3RINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3RINV register"]
pub mod oc3rinv;
#[doc = "OC3RS register"]
pub struct OC3RS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3RS register"]
pub mod oc3rs;
#[doc = "OC3RSCLR register"]
pub struct OC3RSCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3RSCLR register"]
pub mod oc3rsclr;
#[doc = "OC3RSSET register"]
pub struct OC3RSSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3RSSET register"]
pub mod oc3rsset;
#[doc = "OC3RSINV register"]
pub struct OC3RSINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC3RSINV register"]
pub mod oc3rsinv;
