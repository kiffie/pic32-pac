#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM2CON register"]
    pub cm2con: crate::Reg<cm2con::CM2CON_SPEC>,
    #[doc = "0x04 - CM2CONCLR register"]
    pub cm2conclr: crate::Reg<cm2conclr::CM2CONCLR_SPEC>,
    #[doc = "0x08 - CM2CONSET register"]
    pub cm2conset: crate::Reg<cm2conset::CM2CONSET_SPEC>,
    #[doc = "0x0c - CM2CONINV register"]
    pub cm2coninv: crate::Reg<cm2coninv::CM2CONINV_SPEC>,
}
#[doc = "CM2CON register accessor: an alias for `Reg<CM2CON_SPEC>`"]
pub type CM2CON = crate::Reg<cm2con::CM2CON_SPEC>;
#[doc = "CM2CON register"]
pub mod cm2con;
#[doc = "CM2CONCLR register accessor: an alias for `Reg<CM2CONCLR_SPEC>`"]
pub type CM2CONCLR = crate::Reg<cm2conclr::CM2CONCLR_SPEC>;
#[doc = "CM2CONCLR register"]
pub mod cm2conclr;
#[doc = "CM2CONSET register accessor: an alias for `Reg<CM2CONSET_SPEC>`"]
pub type CM2CONSET = crate::Reg<cm2conset::CM2CONSET_SPEC>;
#[doc = "CM2CONSET register"]
pub mod cm2conset;
#[doc = "CM2CONINV register accessor: an alias for `Reg<CM2CONINV_SPEC>`"]
pub type CM2CONINV = crate::Reg<cm2coninv::CM2CONINV_SPEC>;
#[doc = "CM2CONINV register"]
pub mod cm2coninv;
