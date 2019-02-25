#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC5CON register"]
    pub oc5con: OC5CON,
    #[doc = "0x04 - OC5CONCLR register"]
    pub oc5conclr: OC5CONCLR,
    #[doc = "0x08 - OC5CONSET register"]
    pub oc5conset: OC5CONSET,
    #[doc = "0x0c - OC5CONINV register"]
    pub oc5coninv: OC5CONINV,
    #[doc = "0x10 - OC5R register"]
    pub oc5r: OC5R,
    #[doc = "0x14 - OC5RCLR register"]
    pub oc5rclr: OC5RCLR,
    #[doc = "0x18 - OC5RSET register"]
    pub oc5rset: OC5RSET,
    #[doc = "0x1c - OC5RINV register"]
    pub oc5rinv: OC5RINV,
    #[doc = "0x20 - OC5RS register"]
    pub oc5rs: OC5RS,
    #[doc = "0x24 - OC5RSCLR register"]
    pub oc5rsclr: OC5RSCLR,
    #[doc = "0x28 - OC5RSSET register"]
    pub oc5rsset: OC5RSSET,
    #[doc = "0x2c - OC5RSINV register"]
    pub oc5rsinv: OC5RSINV,
}
#[doc = "OC5CON register"]
pub struct OC5CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5CON register"]
pub mod oc5con;
#[doc = "OC5CONCLR register"]
pub struct OC5CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5CONCLR register"]
pub mod oc5conclr;
#[doc = "OC5CONSET register"]
pub struct OC5CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5CONSET register"]
pub mod oc5conset;
#[doc = "OC5CONINV register"]
pub struct OC5CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5CONINV register"]
pub mod oc5coninv;
#[doc = "OC5R register"]
pub struct OC5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5R register"]
pub mod oc5r;
#[doc = "OC5RCLR register"]
pub struct OC5RCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5RCLR register"]
pub mod oc5rclr;
#[doc = "OC5RSET register"]
pub struct OC5RSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5RSET register"]
pub mod oc5rset;
#[doc = "OC5RINV register"]
pub struct OC5RINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5RINV register"]
pub mod oc5rinv;
#[doc = "OC5RS register"]
pub struct OC5RS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5RS register"]
pub mod oc5rs;
#[doc = "OC5RSCLR register"]
pub struct OC5RSCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5RSCLR register"]
pub mod oc5rsclr;
#[doc = "OC5RSSET register"]
pub struct OC5RSSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5RSSET register"]
pub mod oc5rsset;
#[doc = "OC5RSINV register"]
pub struct OC5RSINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC5RSINV register"]
pub mod oc5rsinv;
