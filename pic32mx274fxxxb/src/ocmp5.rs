#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC5CON register"]
    pub oc5con: crate::Reg<oc5con::OC5CON_SPEC>,
    #[doc = "0x04 - OC5CONCLR register"]
    pub oc5conclr: crate::Reg<oc5conclr::OC5CONCLR_SPEC>,
    #[doc = "0x08 - OC5CONSET register"]
    pub oc5conset: crate::Reg<oc5conset::OC5CONSET_SPEC>,
    #[doc = "0x0c - OC5CONINV register"]
    pub oc5coninv: crate::Reg<oc5coninv::OC5CONINV_SPEC>,
    #[doc = "0x10 - OC5R register"]
    pub oc5r: crate::Reg<oc5r::OC5R_SPEC>,
    #[doc = "0x14 - OC5RCLR register"]
    pub oc5rclr: crate::Reg<oc5rclr::OC5RCLR_SPEC>,
    #[doc = "0x18 - OC5RSET register"]
    pub oc5rset: crate::Reg<oc5rset::OC5RSET_SPEC>,
    #[doc = "0x1c - OC5RINV register"]
    pub oc5rinv: crate::Reg<oc5rinv::OC5RINV_SPEC>,
    #[doc = "0x20 - OC5RS register"]
    pub oc5rs: crate::Reg<oc5rs::OC5RS_SPEC>,
    #[doc = "0x24 - OC5RSCLR register"]
    pub oc5rsclr: crate::Reg<oc5rsclr::OC5RSCLR_SPEC>,
    #[doc = "0x28 - OC5RSSET register"]
    pub oc5rsset: crate::Reg<oc5rsset::OC5RSSET_SPEC>,
    #[doc = "0x2c - OC5RSINV register"]
    pub oc5rsinv: crate::Reg<oc5rsinv::OC5RSINV_SPEC>,
}
#[doc = "OC5CON register accessor: an alias for `Reg<OC5CON_SPEC>`"]
pub type OC5CON = crate::Reg<oc5con::OC5CON_SPEC>;
#[doc = "OC5CON register"]
pub mod oc5con;
#[doc = "OC5CONCLR register accessor: an alias for `Reg<OC5CONCLR_SPEC>`"]
pub type OC5CONCLR = crate::Reg<oc5conclr::OC5CONCLR_SPEC>;
#[doc = "OC5CONCLR register"]
pub mod oc5conclr;
#[doc = "OC5CONSET register accessor: an alias for `Reg<OC5CONSET_SPEC>`"]
pub type OC5CONSET = crate::Reg<oc5conset::OC5CONSET_SPEC>;
#[doc = "OC5CONSET register"]
pub mod oc5conset;
#[doc = "OC5CONINV register accessor: an alias for `Reg<OC5CONINV_SPEC>`"]
pub type OC5CONINV = crate::Reg<oc5coninv::OC5CONINV_SPEC>;
#[doc = "OC5CONINV register"]
pub mod oc5coninv;
#[doc = "OC5R register accessor: an alias for `Reg<OC5R_SPEC>`"]
pub type OC5R = crate::Reg<oc5r::OC5R_SPEC>;
#[doc = "OC5R register"]
pub mod oc5r;
#[doc = "OC5RCLR register accessor: an alias for `Reg<OC5RCLR_SPEC>`"]
pub type OC5RCLR = crate::Reg<oc5rclr::OC5RCLR_SPEC>;
#[doc = "OC5RCLR register"]
pub mod oc5rclr;
#[doc = "OC5RSET register accessor: an alias for `Reg<OC5RSET_SPEC>`"]
pub type OC5RSET = crate::Reg<oc5rset::OC5RSET_SPEC>;
#[doc = "OC5RSET register"]
pub mod oc5rset;
#[doc = "OC5RINV register accessor: an alias for `Reg<OC5RINV_SPEC>`"]
pub type OC5RINV = crate::Reg<oc5rinv::OC5RINV_SPEC>;
#[doc = "OC5RINV register"]
pub mod oc5rinv;
#[doc = "OC5RS register accessor: an alias for `Reg<OC5RS_SPEC>`"]
pub type OC5RS = crate::Reg<oc5rs::OC5RS_SPEC>;
#[doc = "OC5RS register"]
pub mod oc5rs;
#[doc = "OC5RSCLR register accessor: an alias for `Reg<OC5RSCLR_SPEC>`"]
pub type OC5RSCLR = crate::Reg<oc5rsclr::OC5RSCLR_SPEC>;
#[doc = "OC5RSCLR register"]
pub mod oc5rsclr;
#[doc = "OC5RSSET register accessor: an alias for `Reg<OC5RSSET_SPEC>`"]
pub type OC5RSSET = crate::Reg<oc5rsset::OC5RSSET_SPEC>;
#[doc = "OC5RSSET register"]
pub mod oc5rsset;
#[doc = "OC5RSINV register accessor: an alias for `Reg<OC5RSINV_SPEC>`"]
pub type OC5RSINV = crate::Reg<oc5rsinv::OC5RSINV_SPEC>;
#[doc = "OC5RSINV register"]
pub mod oc5rsinv;
