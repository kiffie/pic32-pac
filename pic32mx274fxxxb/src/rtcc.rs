#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTCCON register"]
    pub rtccon: crate::Reg<rtccon::RTCCON_SPEC>,
    #[doc = "0x04 - RTCCONCLR register"]
    pub rtcconclr: crate::Reg<rtcconclr::RTCCONCLR_SPEC>,
    #[doc = "0x08 - RTCCONSET register"]
    pub rtcconset: crate::Reg<rtcconset::RTCCONSET_SPEC>,
    #[doc = "0x0c - RTCCONINV register"]
    pub rtcconinv: crate::Reg<rtcconinv::RTCCONINV_SPEC>,
    #[doc = "0x10 - RTCALRM register"]
    pub rtcalrm: crate::Reg<rtcalrm::RTCALRM_SPEC>,
    #[doc = "0x14 - RTCALRMCLR register"]
    pub rtcalrmclr: crate::Reg<rtcalrmclr::RTCALRMCLR_SPEC>,
    #[doc = "0x18 - RTCALRMSET register"]
    pub rtcalrmset: crate::Reg<rtcalrmset::RTCALRMSET_SPEC>,
    #[doc = "0x1c - RTCALRMINV register"]
    pub rtcalrminv: crate::Reg<rtcalrminv::RTCALRMINV_SPEC>,
    #[doc = "0x20 - RTCTIME register"]
    pub rtctime: crate::Reg<rtctime::RTCTIME_SPEC>,
    #[doc = "0x24 - RTCTIMECLR register"]
    pub rtctimeclr: crate::Reg<rtctimeclr::RTCTIMECLR_SPEC>,
    #[doc = "0x28 - RTCTIMESET register"]
    pub rtctimeset: crate::Reg<rtctimeset::RTCTIMESET_SPEC>,
    #[doc = "0x2c - RTCTIMEINV register"]
    pub rtctimeinv: crate::Reg<rtctimeinv::RTCTIMEINV_SPEC>,
    #[doc = "0x30 - RTCDATE register"]
    pub rtcdate: crate::Reg<rtcdate::RTCDATE_SPEC>,
    #[doc = "0x34 - RTCDATECLR register"]
    pub rtcdateclr: crate::Reg<rtcdateclr::RTCDATECLR_SPEC>,
    #[doc = "0x38 - RTCDATESET register"]
    pub rtcdateset: crate::Reg<rtcdateset::RTCDATESET_SPEC>,
    #[doc = "0x3c - RTCDATEINV register"]
    pub rtcdateinv: crate::Reg<rtcdateinv::RTCDATEINV_SPEC>,
    #[doc = "0x40 - ALRMTIME register"]
    pub alrmtime: crate::Reg<alrmtime::ALRMTIME_SPEC>,
    #[doc = "0x44 - ALRMTIMECLR register"]
    pub alrmtimeclr: crate::Reg<alrmtimeclr::ALRMTIMECLR_SPEC>,
    #[doc = "0x48 - ALRMTIMESET register"]
    pub alrmtimeset: crate::Reg<alrmtimeset::ALRMTIMESET_SPEC>,
    #[doc = "0x4c - ALRMTIMEINV register"]
    pub alrmtimeinv: crate::Reg<alrmtimeinv::ALRMTIMEINV_SPEC>,
    #[doc = "0x50 - ALRMDATE register"]
    pub alrmdate: crate::Reg<alrmdate::ALRMDATE_SPEC>,
    #[doc = "0x54 - ALRMDATECLR register"]
    pub alrmdateclr: crate::Reg<alrmdateclr::ALRMDATECLR_SPEC>,
    #[doc = "0x58 - ALRMDATESET register"]
    pub alrmdateset: crate::Reg<alrmdateset::ALRMDATESET_SPEC>,
    #[doc = "0x5c - ALRMDATEINV register"]
    pub alrmdateinv: crate::Reg<alrmdateinv::ALRMDATEINV_SPEC>,
}
#[doc = "RTCCON register accessor: an alias for `Reg<RTCCON_SPEC>`"]
pub type RTCCON = crate::Reg<rtccon::RTCCON_SPEC>;
#[doc = "RTCCON register"]
pub mod rtccon;
#[doc = "RTCCONCLR register accessor: an alias for `Reg<RTCCONCLR_SPEC>`"]
pub type RTCCONCLR = crate::Reg<rtcconclr::RTCCONCLR_SPEC>;
#[doc = "RTCCONCLR register"]
pub mod rtcconclr;
#[doc = "RTCCONSET register accessor: an alias for `Reg<RTCCONSET_SPEC>`"]
pub type RTCCONSET = crate::Reg<rtcconset::RTCCONSET_SPEC>;
#[doc = "RTCCONSET register"]
pub mod rtcconset;
#[doc = "RTCCONINV register accessor: an alias for `Reg<RTCCONINV_SPEC>`"]
pub type RTCCONINV = crate::Reg<rtcconinv::RTCCONINV_SPEC>;
#[doc = "RTCCONINV register"]
pub mod rtcconinv;
#[doc = "RTCALRM register accessor: an alias for `Reg<RTCALRM_SPEC>`"]
pub type RTCALRM = crate::Reg<rtcalrm::RTCALRM_SPEC>;
#[doc = "RTCALRM register"]
pub mod rtcalrm;
#[doc = "RTCALRMCLR register accessor: an alias for `Reg<RTCALRMCLR_SPEC>`"]
pub type RTCALRMCLR = crate::Reg<rtcalrmclr::RTCALRMCLR_SPEC>;
#[doc = "RTCALRMCLR register"]
pub mod rtcalrmclr;
#[doc = "RTCALRMSET register accessor: an alias for `Reg<RTCALRMSET_SPEC>`"]
pub type RTCALRMSET = crate::Reg<rtcalrmset::RTCALRMSET_SPEC>;
#[doc = "RTCALRMSET register"]
pub mod rtcalrmset;
#[doc = "RTCALRMINV register accessor: an alias for `Reg<RTCALRMINV_SPEC>`"]
pub type RTCALRMINV = crate::Reg<rtcalrminv::RTCALRMINV_SPEC>;
#[doc = "RTCALRMINV register"]
pub mod rtcalrminv;
#[doc = "RTCTIME register accessor: an alias for `Reg<RTCTIME_SPEC>`"]
pub type RTCTIME = crate::Reg<rtctime::RTCTIME_SPEC>;
#[doc = "RTCTIME register"]
pub mod rtctime;
#[doc = "RTCTIMECLR register accessor: an alias for `Reg<RTCTIMECLR_SPEC>`"]
pub type RTCTIMECLR = crate::Reg<rtctimeclr::RTCTIMECLR_SPEC>;
#[doc = "RTCTIMECLR register"]
pub mod rtctimeclr;
#[doc = "RTCTIMESET register accessor: an alias for `Reg<RTCTIMESET_SPEC>`"]
pub type RTCTIMESET = crate::Reg<rtctimeset::RTCTIMESET_SPEC>;
#[doc = "RTCTIMESET register"]
pub mod rtctimeset;
#[doc = "RTCTIMEINV register accessor: an alias for `Reg<RTCTIMEINV_SPEC>`"]
pub type RTCTIMEINV = crate::Reg<rtctimeinv::RTCTIMEINV_SPEC>;
#[doc = "RTCTIMEINV register"]
pub mod rtctimeinv;
#[doc = "RTCDATE register accessor: an alias for `Reg<RTCDATE_SPEC>`"]
pub type RTCDATE = crate::Reg<rtcdate::RTCDATE_SPEC>;
#[doc = "RTCDATE register"]
pub mod rtcdate;
#[doc = "RTCDATECLR register accessor: an alias for `Reg<RTCDATECLR_SPEC>`"]
pub type RTCDATECLR = crate::Reg<rtcdateclr::RTCDATECLR_SPEC>;
#[doc = "RTCDATECLR register"]
pub mod rtcdateclr;
#[doc = "RTCDATESET register accessor: an alias for `Reg<RTCDATESET_SPEC>`"]
pub type RTCDATESET = crate::Reg<rtcdateset::RTCDATESET_SPEC>;
#[doc = "RTCDATESET register"]
pub mod rtcdateset;
#[doc = "RTCDATEINV register accessor: an alias for `Reg<RTCDATEINV_SPEC>`"]
pub type RTCDATEINV = crate::Reg<rtcdateinv::RTCDATEINV_SPEC>;
#[doc = "RTCDATEINV register"]
pub mod rtcdateinv;
#[doc = "ALRMTIME register accessor: an alias for `Reg<ALRMTIME_SPEC>`"]
pub type ALRMTIME = crate::Reg<alrmtime::ALRMTIME_SPEC>;
#[doc = "ALRMTIME register"]
pub mod alrmtime;
#[doc = "ALRMTIMECLR register accessor: an alias for `Reg<ALRMTIMECLR_SPEC>`"]
pub type ALRMTIMECLR = crate::Reg<alrmtimeclr::ALRMTIMECLR_SPEC>;
#[doc = "ALRMTIMECLR register"]
pub mod alrmtimeclr;
#[doc = "ALRMTIMESET register accessor: an alias for `Reg<ALRMTIMESET_SPEC>`"]
pub type ALRMTIMESET = crate::Reg<alrmtimeset::ALRMTIMESET_SPEC>;
#[doc = "ALRMTIMESET register"]
pub mod alrmtimeset;
#[doc = "ALRMTIMEINV register accessor: an alias for `Reg<ALRMTIMEINV_SPEC>`"]
pub type ALRMTIMEINV = crate::Reg<alrmtimeinv::ALRMTIMEINV_SPEC>;
#[doc = "ALRMTIMEINV register"]
pub mod alrmtimeinv;
#[doc = "ALRMDATE register accessor: an alias for `Reg<ALRMDATE_SPEC>`"]
pub type ALRMDATE = crate::Reg<alrmdate::ALRMDATE_SPEC>;
#[doc = "ALRMDATE register"]
pub mod alrmdate;
#[doc = "ALRMDATECLR register accessor: an alias for `Reg<ALRMDATECLR_SPEC>`"]
pub type ALRMDATECLR = crate::Reg<alrmdateclr::ALRMDATECLR_SPEC>;
#[doc = "ALRMDATECLR register"]
pub mod alrmdateclr;
#[doc = "ALRMDATESET register accessor: an alias for `Reg<ALRMDATESET_SPEC>`"]
pub type ALRMDATESET = crate::Reg<alrmdateset::ALRMDATESET_SPEC>;
#[doc = "ALRMDATESET register"]
pub mod alrmdateset;
#[doc = "ALRMDATEINV register accessor: an alias for `Reg<ALRMDATEINV_SPEC>`"]
pub type ALRMDATEINV = crate::Reg<alrmdateinv::ALRMDATEINV_SPEC>;
#[doc = "ALRMDATEINV register"]
pub mod alrmdateinv;
