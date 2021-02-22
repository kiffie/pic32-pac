#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC4CON register"]
    pub oc4con: crate::Reg<oc4con::OC4CON_SPEC>,
    #[doc = "0x04 - OC4CONCLR register"]
    pub oc4conclr: crate::Reg<oc4conclr::OC4CONCLR_SPEC>,
    #[doc = "0x08 - OC4CONSET register"]
    pub oc4conset: crate::Reg<oc4conset::OC4CONSET_SPEC>,
    #[doc = "0x0c - OC4CONINV register"]
    pub oc4coninv: crate::Reg<oc4coninv::OC4CONINV_SPEC>,
    #[doc = "0x10 - OC4R register"]
    pub oc4r: crate::Reg<oc4r::OC4R_SPEC>,
    #[doc = "0x14 - OC4RCLR register"]
    pub oc4rclr: crate::Reg<oc4rclr::OC4RCLR_SPEC>,
    #[doc = "0x18 - OC4RSET register"]
    pub oc4rset: crate::Reg<oc4rset::OC4RSET_SPEC>,
    #[doc = "0x1c - OC4RINV register"]
    pub oc4rinv: crate::Reg<oc4rinv::OC4RINV_SPEC>,
    #[doc = "0x20 - OC4RS register"]
    pub oc4rs: crate::Reg<oc4rs::OC4RS_SPEC>,
    #[doc = "0x24 - OC4RSCLR register"]
    pub oc4rsclr: crate::Reg<oc4rsclr::OC4RSCLR_SPEC>,
    #[doc = "0x28 - OC4RSSET register"]
    pub oc4rsset: crate::Reg<oc4rsset::OC4RSSET_SPEC>,
    #[doc = "0x2c - OC4RSINV register"]
    pub oc4rsinv: crate::Reg<oc4rsinv::OC4RSINV_SPEC>,
}
#[doc = "OC4CON register accessor: an alias for `Reg<OC4CON_SPEC>`"]
pub type OC4CON = crate::Reg<oc4con::OC4CON_SPEC>;
#[doc = "OC4CON register"]
pub mod oc4con;
#[doc = "OC4CONCLR register accessor: an alias for `Reg<OC4CONCLR_SPEC>`"]
pub type OC4CONCLR = crate::Reg<oc4conclr::OC4CONCLR_SPEC>;
#[doc = "OC4CONCLR register"]
pub mod oc4conclr;
#[doc = "OC4CONSET register accessor: an alias for `Reg<OC4CONSET_SPEC>`"]
pub type OC4CONSET = crate::Reg<oc4conset::OC4CONSET_SPEC>;
#[doc = "OC4CONSET register"]
pub mod oc4conset;
#[doc = "OC4CONINV register accessor: an alias for `Reg<OC4CONINV_SPEC>`"]
pub type OC4CONINV = crate::Reg<oc4coninv::OC4CONINV_SPEC>;
#[doc = "OC4CONINV register"]
pub mod oc4coninv;
#[doc = "OC4R register accessor: an alias for `Reg<OC4R_SPEC>`"]
pub type OC4R = crate::Reg<oc4r::OC4R_SPEC>;
#[doc = "OC4R register"]
pub mod oc4r;
#[doc = "OC4RCLR register accessor: an alias for `Reg<OC4RCLR_SPEC>`"]
pub type OC4RCLR = crate::Reg<oc4rclr::OC4RCLR_SPEC>;
#[doc = "OC4RCLR register"]
pub mod oc4rclr;
#[doc = "OC4RSET register accessor: an alias for `Reg<OC4RSET_SPEC>`"]
pub type OC4RSET = crate::Reg<oc4rset::OC4RSET_SPEC>;
#[doc = "OC4RSET register"]
pub mod oc4rset;
#[doc = "OC4RINV register accessor: an alias for `Reg<OC4RINV_SPEC>`"]
pub type OC4RINV = crate::Reg<oc4rinv::OC4RINV_SPEC>;
#[doc = "OC4RINV register"]
pub mod oc4rinv;
#[doc = "OC4RS register accessor: an alias for `Reg<OC4RS_SPEC>`"]
pub type OC4RS = crate::Reg<oc4rs::OC4RS_SPEC>;
#[doc = "OC4RS register"]
pub mod oc4rs;
#[doc = "OC4RSCLR register accessor: an alias for `Reg<OC4RSCLR_SPEC>`"]
pub type OC4RSCLR = crate::Reg<oc4rsclr::OC4RSCLR_SPEC>;
#[doc = "OC4RSCLR register"]
pub mod oc4rsclr;
#[doc = "OC4RSSET register accessor: an alias for `Reg<OC4RSSET_SPEC>`"]
pub type OC4RSSET = crate::Reg<oc4rsset::OC4RSSET_SPEC>;
#[doc = "OC4RSSET register"]
pub mod oc4rsset;
#[doc = "OC4RSINV register accessor: an alias for `Reg<OC4RSINV_SPEC>`"]
pub type OC4RSINV = crate::Reg<oc4rsinv::OC4RSINV_SPEC>;
#[doc = "OC4RSINV register"]
pub mod oc4rsinv;
