#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC1CON register"]
    pub ic1con: crate::Reg<ic1con::IC1CON_SPEC>,
    #[doc = "0x04 - IC1CONCLR register"]
    pub ic1conclr: crate::Reg<ic1conclr::IC1CONCLR_SPEC>,
    #[doc = "0x08 - IC1CONSET register"]
    pub ic1conset: crate::Reg<ic1conset::IC1CONSET_SPEC>,
    #[doc = "0x0c - IC1CONINV register"]
    pub ic1coninv: crate::Reg<ic1coninv::IC1CONINV_SPEC>,
    #[doc = "0x10 - IC1BUF register"]
    pub ic1buf: crate::Reg<ic1buf::IC1BUF_SPEC>,
}
#[doc = "IC1CON register accessor: an alias for `Reg<IC1CON_SPEC>`"]
pub type IC1CON = crate::Reg<ic1con::IC1CON_SPEC>;
#[doc = "IC1CON register"]
pub mod ic1con;
#[doc = "IC1CONCLR register accessor: an alias for `Reg<IC1CONCLR_SPEC>`"]
pub type IC1CONCLR = crate::Reg<ic1conclr::IC1CONCLR_SPEC>;
#[doc = "IC1CONCLR register"]
pub mod ic1conclr;
#[doc = "IC1CONSET register accessor: an alias for `Reg<IC1CONSET_SPEC>`"]
pub type IC1CONSET = crate::Reg<ic1conset::IC1CONSET_SPEC>;
#[doc = "IC1CONSET register"]
pub mod ic1conset;
#[doc = "IC1CONINV register accessor: an alias for `Reg<IC1CONINV_SPEC>`"]
pub type IC1CONINV = crate::Reg<ic1coninv::IC1CONINV_SPEC>;
#[doc = "IC1CONINV register"]
pub mod ic1coninv;
#[doc = "IC1BUF register accessor: an alias for `Reg<IC1BUF_SPEC>`"]
pub type IC1BUF = crate::Reg<ic1buf::IC1BUF_SPEC>;
#[doc = "IC1BUF register"]
pub mod ic1buf;
