#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC1CON register"]
    pub oc1con: OC1CON,
    #[doc = "0x04 - OC1CONCLR register"]
    pub oc1conclr: OC1CONCLR,
    #[doc = "0x08 - OC1CONSET register"]
    pub oc1conset: OC1CONSET,
    #[doc = "0x0c - OC1CONINV register"]
    pub oc1coninv: OC1CONINV,
    #[doc = "0x10 - OC1R register"]
    pub oc1r: OC1R,
    #[doc = "0x14 - OC1RCLR register"]
    pub oc1rclr: OC1RCLR,
    #[doc = "0x18 - OC1RSET register"]
    pub oc1rset: OC1RSET,
    #[doc = "0x1c - OC1RINV register"]
    pub oc1rinv: OC1RINV,
    #[doc = "0x20 - OC1RS register"]
    pub oc1rs: OC1RS,
    #[doc = "0x24 - OC1RSCLR register"]
    pub oc1rsclr: OC1RSCLR,
    #[doc = "0x28 - OC1RSSET register"]
    pub oc1rsset: OC1RSSET,
    #[doc = "0x2c - OC1RSINV register"]
    pub oc1rsinv: OC1RSINV,
}
#[doc = "OC1CON register"]
pub struct OC1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1CON register"]
pub mod oc1con;
#[doc = "OC1CONCLR register"]
pub struct OC1CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1CONCLR register"]
pub mod oc1conclr;
#[doc = "OC1CONSET register"]
pub struct OC1CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1CONSET register"]
pub mod oc1conset;
#[doc = "OC1CONINV register"]
pub struct OC1CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1CONINV register"]
pub mod oc1coninv;
#[doc = "OC1R register"]
pub struct OC1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1R register"]
pub mod oc1r;
#[doc = "OC1RCLR register"]
pub struct OC1RCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1RCLR register"]
pub mod oc1rclr;
#[doc = "OC1RSET register"]
pub struct OC1RSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1RSET register"]
pub mod oc1rset;
#[doc = "OC1RINV register"]
pub struct OC1RINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1RINV register"]
pub mod oc1rinv;
#[doc = "OC1RS register"]
pub struct OC1RS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1RS register"]
pub mod oc1rs;
#[doc = "OC1RSCLR register"]
pub struct OC1RSCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1RSCLR register"]
pub mod oc1rsclr;
#[doc = "OC1RSSET register"]
pub struct OC1RSSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1RSSET register"]
pub mod oc1rsset;
#[doc = "OC1RSINV register"]
pub struct OC1RSINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC1RSINV register"]
pub mod oc1rsinv;
