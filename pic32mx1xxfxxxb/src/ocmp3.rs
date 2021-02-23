#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC3CON register"]
    pub oc3con: crate::Reg<oc3con::OC3CON_SPEC>,
    #[doc = "0x04 - OC3CONCLR register"]
    pub oc3conclr: crate::Reg<oc3conclr::OC3CONCLR_SPEC>,
    #[doc = "0x08 - OC3CONSET register"]
    pub oc3conset: crate::Reg<oc3conset::OC3CONSET_SPEC>,
    #[doc = "0x0c - OC3CONINV register"]
    pub oc3coninv: crate::Reg<oc3coninv::OC3CONINV_SPEC>,
    #[doc = "0x10 - OC3R register"]
    pub oc3r: crate::Reg<oc3r::OC3R_SPEC>,
    #[doc = "0x14 - OC3RCLR register"]
    pub oc3rclr: crate::Reg<oc3rclr::OC3RCLR_SPEC>,
    #[doc = "0x18 - OC3RSET register"]
    pub oc3rset: crate::Reg<oc3rset::OC3RSET_SPEC>,
    #[doc = "0x1c - OC3RINV register"]
    pub oc3rinv: crate::Reg<oc3rinv::OC3RINV_SPEC>,
    #[doc = "0x20 - OC3RS register"]
    pub oc3rs: crate::Reg<oc3rs::OC3RS_SPEC>,
    #[doc = "0x24 - OC3RSCLR register"]
    pub oc3rsclr: crate::Reg<oc3rsclr::OC3RSCLR_SPEC>,
    #[doc = "0x28 - OC3RSSET register"]
    pub oc3rsset: crate::Reg<oc3rsset::OC3RSSET_SPEC>,
    #[doc = "0x2c - OC3RSINV register"]
    pub oc3rsinv: crate::Reg<oc3rsinv::OC3RSINV_SPEC>,
}
#[doc = "OC3CON register accessor: an alias for `Reg<OC3CON_SPEC>`"]
pub type OC3CON = crate::Reg<oc3con::OC3CON_SPEC>;
#[doc = "OC3CON register"]
pub mod oc3con;
#[doc = "OC3CONCLR register accessor: an alias for `Reg<OC3CONCLR_SPEC>`"]
pub type OC3CONCLR = crate::Reg<oc3conclr::OC3CONCLR_SPEC>;
#[doc = "OC3CONCLR register"]
pub mod oc3conclr;
#[doc = "OC3CONSET register accessor: an alias for `Reg<OC3CONSET_SPEC>`"]
pub type OC3CONSET = crate::Reg<oc3conset::OC3CONSET_SPEC>;
#[doc = "OC3CONSET register"]
pub mod oc3conset;
#[doc = "OC3CONINV register accessor: an alias for `Reg<OC3CONINV_SPEC>`"]
pub type OC3CONINV = crate::Reg<oc3coninv::OC3CONINV_SPEC>;
#[doc = "OC3CONINV register"]
pub mod oc3coninv;
#[doc = "OC3R register accessor: an alias for `Reg<OC3R_SPEC>`"]
pub type OC3R = crate::Reg<oc3r::OC3R_SPEC>;
#[doc = "OC3R register"]
pub mod oc3r;
#[doc = "OC3RCLR register accessor: an alias for `Reg<OC3RCLR_SPEC>`"]
pub type OC3RCLR = crate::Reg<oc3rclr::OC3RCLR_SPEC>;
#[doc = "OC3RCLR register"]
pub mod oc3rclr;
#[doc = "OC3RSET register accessor: an alias for `Reg<OC3RSET_SPEC>`"]
pub type OC3RSET = crate::Reg<oc3rset::OC3RSET_SPEC>;
#[doc = "OC3RSET register"]
pub mod oc3rset;
#[doc = "OC3RINV register accessor: an alias for `Reg<OC3RINV_SPEC>`"]
pub type OC3RINV = crate::Reg<oc3rinv::OC3RINV_SPEC>;
#[doc = "OC3RINV register"]
pub mod oc3rinv;
#[doc = "OC3RS register accessor: an alias for `Reg<OC3RS_SPEC>`"]
pub type OC3RS = crate::Reg<oc3rs::OC3RS_SPEC>;
#[doc = "OC3RS register"]
pub mod oc3rs;
#[doc = "OC3RSCLR register accessor: an alias for `Reg<OC3RSCLR_SPEC>`"]
pub type OC3RSCLR = crate::Reg<oc3rsclr::OC3RSCLR_SPEC>;
#[doc = "OC3RSCLR register"]
pub mod oc3rsclr;
#[doc = "OC3RSSET register accessor: an alias for `Reg<OC3RSSET_SPEC>`"]
pub type OC3RSSET = crate::Reg<oc3rsset::OC3RSSET_SPEC>;
#[doc = "OC3RSSET register"]
pub mod oc3rsset;
#[doc = "OC3RSINV register accessor: an alias for `Reg<OC3RSINV_SPEC>`"]
pub type OC3RSINV = crate::Reg<oc3rsinv::OC3RSINV_SPEC>;
#[doc = "OC3RSINV register"]
pub mod oc3rsinv;
