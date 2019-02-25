#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC4CON register"]
    pub oc4con: OC4CON,
    #[doc = "0x04 - OC4CONCLR register"]
    pub oc4conclr: OC4CONCLR,
    #[doc = "0x08 - OC4CONSET register"]
    pub oc4conset: OC4CONSET,
    #[doc = "0x0c - OC4CONINV register"]
    pub oc4coninv: OC4CONINV,
    #[doc = "0x10 - OC4R register"]
    pub oc4r: OC4R,
    #[doc = "0x14 - OC4RCLR register"]
    pub oc4rclr: OC4RCLR,
    #[doc = "0x18 - OC4RSET register"]
    pub oc4rset: OC4RSET,
    #[doc = "0x1c - OC4RINV register"]
    pub oc4rinv: OC4RINV,
    #[doc = "0x20 - OC4RS register"]
    pub oc4rs: OC4RS,
    #[doc = "0x24 - OC4RSCLR register"]
    pub oc4rsclr: OC4RSCLR,
    #[doc = "0x28 - OC4RSSET register"]
    pub oc4rsset: OC4RSSET,
    #[doc = "0x2c - OC4RSINV register"]
    pub oc4rsinv: OC4RSINV,
}
#[doc = "OC4CON register"]
pub struct OC4CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4CON register"]
pub mod oc4con;
#[doc = "OC4CONCLR register"]
pub struct OC4CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4CONCLR register"]
pub mod oc4conclr;
#[doc = "OC4CONSET register"]
pub struct OC4CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4CONSET register"]
pub mod oc4conset;
#[doc = "OC4CONINV register"]
pub struct OC4CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4CONINV register"]
pub mod oc4coninv;
#[doc = "OC4R register"]
pub struct OC4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4R register"]
pub mod oc4r;
#[doc = "OC4RCLR register"]
pub struct OC4RCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4RCLR register"]
pub mod oc4rclr;
#[doc = "OC4RSET register"]
pub struct OC4RSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4RSET register"]
pub mod oc4rset;
#[doc = "OC4RINV register"]
pub struct OC4RINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4RINV register"]
pub mod oc4rinv;
#[doc = "OC4RS register"]
pub struct OC4RS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4RS register"]
pub mod oc4rs;
#[doc = "OC4RSCLR register"]
pub struct OC4RSCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4RSCLR register"]
pub mod oc4rsclr;
#[doc = "OC4RSSET register"]
pub struct OC4RSSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4RSSET register"]
pub mod oc4rsset;
#[doc = "OC4RSINV register"]
pub struct OC4RSINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OC4RSINV register"]
pub mod oc4rsinv;
