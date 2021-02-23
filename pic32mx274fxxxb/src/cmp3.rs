#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM3CON register"]
    pub cm3con: crate::Reg<cm3con::CM3CON_SPEC>,
    #[doc = "0x04 - CM3CONCLR register"]
    pub cm3conclr: crate::Reg<cm3conclr::CM3CONCLR_SPEC>,
    #[doc = "0x08 - CM3CONSET register"]
    pub cm3conset: crate::Reg<cm3conset::CM3CONSET_SPEC>,
    #[doc = "0x0c - CM3CONINV register"]
    pub cm3coninv: crate::Reg<cm3coninv::CM3CONINV_SPEC>,
}
#[doc = "CM3CON register accessor: an alias for `Reg<CM3CON_SPEC>`"]
pub type CM3CON = crate::Reg<cm3con::CM3CON_SPEC>;
#[doc = "CM3CON register"]
pub mod cm3con;
#[doc = "CM3CONCLR register accessor: an alias for `Reg<CM3CONCLR_SPEC>`"]
pub type CM3CONCLR = crate::Reg<cm3conclr::CM3CONCLR_SPEC>;
#[doc = "CM3CONCLR register"]
pub mod cm3conclr;
#[doc = "CM3CONSET register accessor: an alias for `Reg<CM3CONSET_SPEC>`"]
pub type CM3CONSET = crate::Reg<cm3conset::CM3CONSET_SPEC>;
#[doc = "CM3CONSET register"]
pub mod cm3conset;
#[doc = "CM3CONINV register accessor: an alias for `Reg<CM3CONINV_SPEC>`"]
pub type CM3CONINV = crate::Reg<cm3coninv::CM3CONINV_SPEC>;
#[doc = "CM3CONINV register"]
pub mod cm3coninv;
