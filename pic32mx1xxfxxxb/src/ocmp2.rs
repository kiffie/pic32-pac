#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC2CON register"]
    pub oc2con: crate::Reg<oc2con::OC2CON_SPEC>,
    #[doc = "0x04 - OC2CONCLR register"]
    pub oc2conclr: crate::Reg<oc2conclr::OC2CONCLR_SPEC>,
    #[doc = "0x08 - OC2CONSET register"]
    pub oc2conset: crate::Reg<oc2conset::OC2CONSET_SPEC>,
    #[doc = "0x0c - OC2CONINV register"]
    pub oc2coninv: crate::Reg<oc2coninv::OC2CONINV_SPEC>,
    #[doc = "0x10 - OC2R register"]
    pub oc2r: crate::Reg<oc2r::OC2R_SPEC>,
    #[doc = "0x14 - OC2RCLR register"]
    pub oc2rclr: crate::Reg<oc2rclr::OC2RCLR_SPEC>,
    #[doc = "0x18 - OC2RSET register"]
    pub oc2rset: crate::Reg<oc2rset::OC2RSET_SPEC>,
    #[doc = "0x1c - OC2RINV register"]
    pub oc2rinv: crate::Reg<oc2rinv::OC2RINV_SPEC>,
    #[doc = "0x20 - OC2RS register"]
    pub oc2rs: crate::Reg<oc2rs::OC2RS_SPEC>,
    #[doc = "0x24 - OC2RSCLR register"]
    pub oc2rsclr: crate::Reg<oc2rsclr::OC2RSCLR_SPEC>,
    #[doc = "0x28 - OC2RSSET register"]
    pub oc2rsset: crate::Reg<oc2rsset::OC2RSSET_SPEC>,
    #[doc = "0x2c - OC2RSINV register"]
    pub oc2rsinv: crate::Reg<oc2rsinv::OC2RSINV_SPEC>,
}
#[doc = "OC2CON register accessor: an alias for `Reg<OC2CON_SPEC>`"]
pub type OC2CON = crate::Reg<oc2con::OC2CON_SPEC>;
#[doc = "OC2CON register"]
pub mod oc2con;
#[doc = "OC2CONCLR register accessor: an alias for `Reg<OC2CONCLR_SPEC>`"]
pub type OC2CONCLR = crate::Reg<oc2conclr::OC2CONCLR_SPEC>;
#[doc = "OC2CONCLR register"]
pub mod oc2conclr;
#[doc = "OC2CONSET register accessor: an alias for `Reg<OC2CONSET_SPEC>`"]
pub type OC2CONSET = crate::Reg<oc2conset::OC2CONSET_SPEC>;
#[doc = "OC2CONSET register"]
pub mod oc2conset;
#[doc = "OC2CONINV register accessor: an alias for `Reg<OC2CONINV_SPEC>`"]
pub type OC2CONINV = crate::Reg<oc2coninv::OC2CONINV_SPEC>;
#[doc = "OC2CONINV register"]
pub mod oc2coninv;
#[doc = "OC2R register accessor: an alias for `Reg<OC2R_SPEC>`"]
pub type OC2R = crate::Reg<oc2r::OC2R_SPEC>;
#[doc = "OC2R register"]
pub mod oc2r;
#[doc = "OC2RCLR register accessor: an alias for `Reg<OC2RCLR_SPEC>`"]
pub type OC2RCLR = crate::Reg<oc2rclr::OC2RCLR_SPEC>;
#[doc = "OC2RCLR register"]
pub mod oc2rclr;
#[doc = "OC2RSET register accessor: an alias for `Reg<OC2RSET_SPEC>`"]
pub type OC2RSET = crate::Reg<oc2rset::OC2RSET_SPEC>;
#[doc = "OC2RSET register"]
pub mod oc2rset;
#[doc = "OC2RINV register accessor: an alias for `Reg<OC2RINV_SPEC>`"]
pub type OC2RINV = crate::Reg<oc2rinv::OC2RINV_SPEC>;
#[doc = "OC2RINV register"]
pub mod oc2rinv;
#[doc = "OC2RS register accessor: an alias for `Reg<OC2RS_SPEC>`"]
pub type OC2RS = crate::Reg<oc2rs::OC2RS_SPEC>;
#[doc = "OC2RS register"]
pub mod oc2rs;
#[doc = "OC2RSCLR register accessor: an alias for `Reg<OC2RSCLR_SPEC>`"]
pub type OC2RSCLR = crate::Reg<oc2rsclr::OC2RSCLR_SPEC>;
#[doc = "OC2RSCLR register"]
pub mod oc2rsclr;
#[doc = "OC2RSSET register accessor: an alias for `Reg<OC2RSSET_SPEC>`"]
pub type OC2RSSET = crate::Reg<oc2rsset::OC2RSSET_SPEC>;
#[doc = "OC2RSSET register"]
pub mod oc2rsset;
#[doc = "OC2RSINV register accessor: an alias for `Reg<OC2RSINV_SPEC>`"]
pub type OC2RSINV = crate::Reg<oc2rsinv::OC2RSINV_SPEC>;
#[doc = "OC2RSINV register"]
pub mod oc2rsinv;
