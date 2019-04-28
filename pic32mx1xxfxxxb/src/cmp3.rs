#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM3CON register"]
    pub cm3con: CM3CON,
    #[doc = "0x04 - CM3CONCLR register"]
    pub cm3conclr: CM3CONCLR,
    #[doc = "0x08 - CM3CONSET register"]
    pub cm3conset: CM3CONSET,
    #[doc = "0x0c - CM3CONINV register"]
    pub cm3coninv: CM3CONINV,
}
#[doc = "CM3CON register"]
pub struct CM3CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM3CON register"]
pub mod cm3con;
#[doc = "CM3CONCLR register"]
pub struct CM3CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM3CONCLR register"]
pub mod cm3conclr;
#[doc = "CM3CONSET register"]
pub struct CM3CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM3CONSET register"]
pub mod cm3conset;
#[doc = "CM3CONINV register"]
pub struct CM3CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CM3CONINV register"]
pub mod cm3coninv;
