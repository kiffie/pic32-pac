#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC1CON register"]
    pub oc1con: crate::Reg<oc1con::OC1CON_SPEC>,
    #[doc = "0x04 - OC1CONCLR register"]
    pub oc1conclr: crate::Reg<oc1conclr::OC1CONCLR_SPEC>,
    #[doc = "0x08 - OC1CONSET register"]
    pub oc1conset: crate::Reg<oc1conset::OC1CONSET_SPEC>,
    #[doc = "0x0c - OC1CONINV register"]
    pub oc1coninv: crate::Reg<oc1coninv::OC1CONINV_SPEC>,
    #[doc = "0x10 - OC1R register"]
    pub oc1r: crate::Reg<oc1r::OC1R_SPEC>,
    #[doc = "0x14 - OC1RCLR register"]
    pub oc1rclr: crate::Reg<oc1rclr::OC1RCLR_SPEC>,
    #[doc = "0x18 - OC1RSET register"]
    pub oc1rset: crate::Reg<oc1rset::OC1RSET_SPEC>,
    #[doc = "0x1c - OC1RINV register"]
    pub oc1rinv: crate::Reg<oc1rinv::OC1RINV_SPEC>,
    #[doc = "0x20 - OC1RS register"]
    pub oc1rs: crate::Reg<oc1rs::OC1RS_SPEC>,
    #[doc = "0x24 - OC1RSCLR register"]
    pub oc1rsclr: crate::Reg<oc1rsclr::OC1RSCLR_SPEC>,
    #[doc = "0x28 - OC1RSSET register"]
    pub oc1rsset: crate::Reg<oc1rsset::OC1RSSET_SPEC>,
    #[doc = "0x2c - OC1RSINV register"]
    pub oc1rsinv: crate::Reg<oc1rsinv::OC1RSINV_SPEC>,
}
#[doc = "OC1CON register accessor: an alias for `Reg<OC1CON_SPEC>`"]
pub type OC1CON = crate::Reg<oc1con::OC1CON_SPEC>;
#[doc = "OC1CON register"]
pub mod oc1con;
#[doc = "OC1CONCLR register accessor: an alias for `Reg<OC1CONCLR_SPEC>`"]
pub type OC1CONCLR = crate::Reg<oc1conclr::OC1CONCLR_SPEC>;
#[doc = "OC1CONCLR register"]
pub mod oc1conclr;
#[doc = "OC1CONSET register accessor: an alias for `Reg<OC1CONSET_SPEC>`"]
pub type OC1CONSET = crate::Reg<oc1conset::OC1CONSET_SPEC>;
#[doc = "OC1CONSET register"]
pub mod oc1conset;
#[doc = "OC1CONINV register accessor: an alias for `Reg<OC1CONINV_SPEC>`"]
pub type OC1CONINV = crate::Reg<oc1coninv::OC1CONINV_SPEC>;
#[doc = "OC1CONINV register"]
pub mod oc1coninv;
#[doc = "OC1R register accessor: an alias for `Reg<OC1R_SPEC>`"]
pub type OC1R = crate::Reg<oc1r::OC1R_SPEC>;
#[doc = "OC1R register"]
pub mod oc1r;
#[doc = "OC1RCLR register accessor: an alias for `Reg<OC1RCLR_SPEC>`"]
pub type OC1RCLR = crate::Reg<oc1rclr::OC1RCLR_SPEC>;
#[doc = "OC1RCLR register"]
pub mod oc1rclr;
#[doc = "OC1RSET register accessor: an alias for `Reg<OC1RSET_SPEC>`"]
pub type OC1RSET = crate::Reg<oc1rset::OC1RSET_SPEC>;
#[doc = "OC1RSET register"]
pub mod oc1rset;
#[doc = "OC1RINV register accessor: an alias for `Reg<OC1RINV_SPEC>`"]
pub type OC1RINV = crate::Reg<oc1rinv::OC1RINV_SPEC>;
#[doc = "OC1RINV register"]
pub mod oc1rinv;
#[doc = "OC1RS register accessor: an alias for `Reg<OC1RS_SPEC>`"]
pub type OC1RS = crate::Reg<oc1rs::OC1RS_SPEC>;
#[doc = "OC1RS register"]
pub mod oc1rs;
#[doc = "OC1RSCLR register accessor: an alias for `Reg<OC1RSCLR_SPEC>`"]
pub type OC1RSCLR = crate::Reg<oc1rsclr::OC1RSCLR_SPEC>;
#[doc = "OC1RSCLR register"]
pub mod oc1rsclr;
#[doc = "OC1RSSET register accessor: an alias for `Reg<OC1RSSET_SPEC>`"]
pub type OC1RSSET = crate::Reg<oc1rsset::OC1RSSET_SPEC>;
#[doc = "OC1RSSET register"]
pub mod oc1rsset;
#[doc = "OC1RSINV register accessor: an alias for `Reg<OC1RSINV_SPEC>`"]
pub type OC1RSINV = crate::Reg<oc1rsinv::OC1RSINV_SPEC>;
#[doc = "OC1RSINV register"]
pub mod oc1rsinv;
