#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC2CON register"]
    pub ic2con: crate::Reg<ic2con::IC2CON_SPEC>,
    #[doc = "0x04 - IC2CONCLR register"]
    pub ic2conclr: crate::Reg<ic2conclr::IC2CONCLR_SPEC>,
    #[doc = "0x08 - IC2CONSET register"]
    pub ic2conset: crate::Reg<ic2conset::IC2CONSET_SPEC>,
    #[doc = "0x0c - IC2CONINV register"]
    pub ic2coninv: crate::Reg<ic2coninv::IC2CONINV_SPEC>,
    #[doc = "0x10 - IC2BUF register"]
    pub ic2buf: crate::Reg<ic2buf::IC2BUF_SPEC>,
}
#[doc = "IC2CON register accessor: an alias for `Reg<IC2CON_SPEC>`"]
pub type IC2CON = crate::Reg<ic2con::IC2CON_SPEC>;
#[doc = "IC2CON register"]
pub mod ic2con;
#[doc = "IC2CONCLR register accessor: an alias for `Reg<IC2CONCLR_SPEC>`"]
pub type IC2CONCLR = crate::Reg<ic2conclr::IC2CONCLR_SPEC>;
#[doc = "IC2CONCLR register"]
pub mod ic2conclr;
#[doc = "IC2CONSET register accessor: an alias for `Reg<IC2CONSET_SPEC>`"]
pub type IC2CONSET = crate::Reg<ic2conset::IC2CONSET_SPEC>;
#[doc = "IC2CONSET register"]
pub mod ic2conset;
#[doc = "IC2CONINV register accessor: an alias for `Reg<IC2CONINV_SPEC>`"]
pub type IC2CONINV = crate::Reg<ic2coninv::IC2CONINV_SPEC>;
#[doc = "IC2CONINV register"]
pub mod ic2coninv;
#[doc = "IC2BUF register accessor: an alias for `Reg<IC2BUF_SPEC>`"]
pub type IC2BUF = crate::Reg<ic2buf::IC2BUF_SPEC>;
#[doc = "IC2BUF register"]
pub mod ic2buf;
