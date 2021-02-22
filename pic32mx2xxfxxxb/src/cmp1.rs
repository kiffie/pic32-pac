#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM1CON register"]
    pub cm1con: crate::Reg<cm1con::CM1CON_SPEC>,
    #[doc = "0x04 - CM1CONCLR register"]
    pub cm1conclr: crate::Reg<cm1conclr::CM1CONCLR_SPEC>,
    #[doc = "0x08 - CM1CONSET register"]
    pub cm1conset: crate::Reg<cm1conset::CM1CONSET_SPEC>,
    #[doc = "0x0c - CM1CONINV register"]
    pub cm1coninv: crate::Reg<cm1coninv::CM1CONINV_SPEC>,
}
#[doc = "CM1CON register accessor: an alias for `Reg<CM1CON_SPEC>`"]
pub type CM1CON = crate::Reg<cm1con::CM1CON_SPEC>;
#[doc = "CM1CON register"]
pub mod cm1con;
#[doc = "CM1CONCLR register accessor: an alias for `Reg<CM1CONCLR_SPEC>`"]
pub type CM1CONCLR = crate::Reg<cm1conclr::CM1CONCLR_SPEC>;
#[doc = "CM1CONCLR register"]
pub mod cm1conclr;
#[doc = "CM1CONSET register accessor: an alias for `Reg<CM1CONSET_SPEC>`"]
pub type CM1CONSET = crate::Reg<cm1conset::CM1CONSET_SPEC>;
#[doc = "CM1CONSET register"]
pub mod cm1conset;
#[doc = "CM1CONINV register accessor: an alias for `Reg<CM1CONINV_SPEC>`"]
pub type CM1CONINV = crate::Reg<cm1coninv::CM1CONINV_SPEC>;
#[doc = "CM1CONINV register"]
pub mod cm1coninv;
