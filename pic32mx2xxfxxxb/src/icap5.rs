#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC5CON register"]
    pub ic5con: crate::Reg<ic5con::IC5CON_SPEC>,
    #[doc = "0x04 - IC5CONCLR register"]
    pub ic5conclr: crate::Reg<ic5conclr::IC5CONCLR_SPEC>,
    #[doc = "0x08 - IC5CONSET register"]
    pub ic5conset: crate::Reg<ic5conset::IC5CONSET_SPEC>,
    #[doc = "0x0c - IC5CONINV register"]
    pub ic5coninv: crate::Reg<ic5coninv::IC5CONINV_SPEC>,
    #[doc = "0x10 - IC5BUF register"]
    pub ic5buf: crate::Reg<ic5buf::IC5BUF_SPEC>,
}
#[doc = "IC5CON register accessor: an alias for `Reg<IC5CON_SPEC>`"]
pub type IC5CON = crate::Reg<ic5con::IC5CON_SPEC>;
#[doc = "IC5CON register"]
pub mod ic5con;
#[doc = "IC5CONCLR register accessor: an alias for `Reg<IC5CONCLR_SPEC>`"]
pub type IC5CONCLR = crate::Reg<ic5conclr::IC5CONCLR_SPEC>;
#[doc = "IC5CONCLR register"]
pub mod ic5conclr;
#[doc = "IC5CONSET register accessor: an alias for `Reg<IC5CONSET_SPEC>`"]
pub type IC5CONSET = crate::Reg<ic5conset::IC5CONSET_SPEC>;
#[doc = "IC5CONSET register"]
pub mod ic5conset;
#[doc = "IC5CONINV register accessor: an alias for `Reg<IC5CONINV_SPEC>`"]
pub type IC5CONINV = crate::Reg<ic5coninv::IC5CONINV_SPEC>;
#[doc = "IC5CONINV register"]
pub mod ic5coninv;
#[doc = "IC5BUF register accessor: an alias for `Reg<IC5BUF_SPEC>`"]
pub type IC5BUF = crate::Reg<ic5buf::IC5BUF_SPEC>;
#[doc = "IC5BUF register"]
pub mod ic5buf;
