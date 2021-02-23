#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC4CON register"]
    pub ic4con: crate::Reg<ic4con::IC4CON_SPEC>,
    #[doc = "0x04 - IC4CONCLR register"]
    pub ic4conclr: crate::Reg<ic4conclr::IC4CONCLR_SPEC>,
    #[doc = "0x08 - IC4CONSET register"]
    pub ic4conset: crate::Reg<ic4conset::IC4CONSET_SPEC>,
    #[doc = "0x0c - IC4CONINV register"]
    pub ic4coninv: crate::Reg<ic4coninv::IC4CONINV_SPEC>,
    #[doc = "0x10 - IC4BUF register"]
    pub ic4buf: crate::Reg<ic4buf::IC4BUF_SPEC>,
}
#[doc = "IC4CON register accessor: an alias for `Reg<IC4CON_SPEC>`"]
pub type IC4CON = crate::Reg<ic4con::IC4CON_SPEC>;
#[doc = "IC4CON register"]
pub mod ic4con;
#[doc = "IC4CONCLR register accessor: an alias for `Reg<IC4CONCLR_SPEC>`"]
pub type IC4CONCLR = crate::Reg<ic4conclr::IC4CONCLR_SPEC>;
#[doc = "IC4CONCLR register"]
pub mod ic4conclr;
#[doc = "IC4CONSET register accessor: an alias for `Reg<IC4CONSET_SPEC>`"]
pub type IC4CONSET = crate::Reg<ic4conset::IC4CONSET_SPEC>;
#[doc = "IC4CONSET register"]
pub mod ic4conset;
#[doc = "IC4CONINV register accessor: an alias for `Reg<IC4CONINV_SPEC>`"]
pub type IC4CONINV = crate::Reg<ic4coninv::IC4CONINV_SPEC>;
#[doc = "IC4CONINV register"]
pub mod ic4coninv;
#[doc = "IC4BUF register accessor: an alias for `Reg<IC4BUF_SPEC>`"]
pub type IC4BUF = crate::Reg<ic4buf::IC4BUF_SPEC>;
#[doc = "IC4BUF register"]
pub mod ic4buf;
