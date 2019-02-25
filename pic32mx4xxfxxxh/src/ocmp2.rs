#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC2CON register"]
    pub oc2con: OC2CON,
    #[doc = "0x04 - OC2CONCLR register"]
    pub oc2conclr: OC2CONCLR,
    #[doc = "0x08 - OC2CONSET register"]
    pub oc2conset: OC2CONSET,
    #[doc = "0x0c - OC2CONINV register"]
    pub oc2coninv: OC2CONINV,
    #[doc = "0x10 - OC2R register"]
    pub oc2r: OC2R,
    #[doc = "0x14 - OC2RCLR register"]
    pub oc2rclr: OC2RCLR,
    #[doc = "0x18 - OC2RSET register"]
    pub oc2rset: OC2RSET,
    #[doc = "0x1c - OC2RINV register"]
    pub oc2rinv: OC2RINV,
    #[doc = "0x20 - OC2RS register"]
    pub oc2rs: OC2RS,
    #[doc = "0x24 - OC2RSCLR register"]
    pub oc2rsclr: OC2RSCLR,
    #[doc = "0x28 - OC2RSSET register"]
    pub oc2rsset: OC2RSSET,
    #[doc = "0x2c - OC2RSINV register"]
    pub oc2rsinv: OC2RSINV,
}
#[doc = "OC2CON register"]
pub struct OC2CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2CON register"]
pub mod oc2con;
#[doc = "OC2CONCLR register"]
pub struct OC2CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2CONCLR register"]
pub mod oc2conclr;
#[doc = "OC2CONSET register"]
pub struct OC2CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2CONSET register"]
pub mod oc2conset;
#[doc = "OC2CONINV register"]
pub struct OC2CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2CONINV register"]
pub mod oc2coninv;
#[doc = "OC2R register"]
pub struct OC2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2R register"]
pub mod oc2r;
#[doc = "OC2RCLR register"]
pub struct OC2RCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2RCLR register"]
pub mod oc2rclr;
#[doc = "OC2RSET register"]
pub struct OC2RSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2RSET register"]
pub mod oc2rset;
#[doc = "OC2RINV register"]
pub struct OC2RINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2RINV register"]
pub mod oc2rinv;
#[doc = "OC2RS register"]
pub struct OC2RS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2RS register"]
pub mod oc2rs;
#[doc = "OC2RSCLR register"]
pub struct OC2RSCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2RSCLR register"]
pub mod oc2rsclr;
#[doc = "OC2RSSET register"]
pub struct OC2RSSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2RSSET register"]
pub mod oc2rsset;
#[doc = "OC2RSINV register"]
pub struct OC2RSINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC2RSINV register"]
pub mod oc2rsinv;
