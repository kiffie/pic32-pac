#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC3CON register"]
    pub ic3con: crate::Reg<ic3con::IC3CON_SPEC>,
    #[doc = "0x04 - IC3CONCLR register"]
    pub ic3conclr: crate::Reg<ic3conclr::IC3CONCLR_SPEC>,
    #[doc = "0x08 - IC3CONSET register"]
    pub ic3conset: crate::Reg<ic3conset::IC3CONSET_SPEC>,
    #[doc = "0x0c - IC3CONINV register"]
    pub ic3coninv: crate::Reg<ic3coninv::IC3CONINV_SPEC>,
    #[doc = "0x10 - IC3BUF register"]
    pub ic3buf: crate::Reg<ic3buf::IC3BUF_SPEC>,
}
#[doc = "IC3CON register accessor: an alias for `Reg<IC3CON_SPEC>`"]
pub type IC3CON = crate::Reg<ic3con::IC3CON_SPEC>;
#[doc = "IC3CON register"]
pub mod ic3con;
#[doc = "IC3CONCLR register accessor: an alias for `Reg<IC3CONCLR_SPEC>`"]
pub type IC3CONCLR = crate::Reg<ic3conclr::IC3CONCLR_SPEC>;
#[doc = "IC3CONCLR register"]
pub mod ic3conclr;
#[doc = "IC3CONSET register accessor: an alias for `Reg<IC3CONSET_SPEC>`"]
pub type IC3CONSET = crate::Reg<ic3conset::IC3CONSET_SPEC>;
#[doc = "IC3CONSET register"]
pub mod ic3conset;
#[doc = "IC3CONINV register accessor: an alias for `Reg<IC3CONINV_SPEC>`"]
pub type IC3CONINV = crate::Reg<ic3coninv::IC3CONINV_SPEC>;
#[doc = "IC3CONINV register"]
pub mod ic3coninv;
#[doc = "IC3BUF register accessor: an alias for `Reg<IC3BUF_SPEC>`"]
pub type IC3BUF = crate::Reg<ic3buf::IC3BUF_SPEC>;
#[doc = "IC3BUF register"]
pub mod ic3buf;
