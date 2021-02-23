#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMCON register"]
    pub pmcon: crate::Reg<pmcon::PMCON_SPEC>,
    #[doc = "0x04 - PMCONCLR register"]
    pub pmconclr: crate::Reg<pmconclr::PMCONCLR_SPEC>,
    #[doc = "0x08 - PMCONSET register"]
    pub pmconset: crate::Reg<pmconset::PMCONSET_SPEC>,
    #[doc = "0x0c - PMCONINV register"]
    pub pmconinv: crate::Reg<pmconinv::PMCONINV_SPEC>,
    #[doc = "0x10 - PMMODE register"]
    pub pmmode: crate::Reg<pmmode::PMMODE_SPEC>,
    #[doc = "0x14 - PMMODECLR register"]
    pub pmmodeclr: crate::Reg<pmmodeclr::PMMODECLR_SPEC>,
    #[doc = "0x18 - PMMODESET register"]
    pub pmmodeset: crate::Reg<pmmodeset::PMMODESET_SPEC>,
    #[doc = "0x1c - PMMODEINV register"]
    pub pmmodeinv: crate::Reg<pmmodeinv::PMMODEINV_SPEC>,
    #[doc = "0x20 - PMADDR register"]
    pub pmaddr: crate::Reg<pmaddr::PMADDR_SPEC>,
    #[doc = "0x24 - PMADDRCLR register"]
    pub pmaddrclr: crate::Reg<pmaddrclr::PMADDRCLR_SPEC>,
    #[doc = "0x28 - PMADDRSET register"]
    pub pmaddrset: crate::Reg<pmaddrset::PMADDRSET_SPEC>,
    #[doc = "0x2c - PMADDRINV register"]
    pub pmaddrinv: crate::Reg<pmaddrinv::PMADDRINV_SPEC>,
    #[doc = "0x30 - PMDOUT register"]
    pub pmdout: crate::Reg<pmdout::PMDOUT_SPEC>,
    #[doc = "0x34 - PMDOUTCLR register"]
    pub pmdoutclr: crate::Reg<pmdoutclr::PMDOUTCLR_SPEC>,
    #[doc = "0x38 - PMDOUTSET register"]
    pub pmdoutset: crate::Reg<pmdoutset::PMDOUTSET_SPEC>,
    #[doc = "0x3c - PMDOUTINV register"]
    pub pmdoutinv: crate::Reg<pmdoutinv::PMDOUTINV_SPEC>,
    #[doc = "0x40 - PMDIN register"]
    pub pmdin: crate::Reg<pmdin::PMDIN_SPEC>,
    #[doc = "0x44 - PMDINCLR register"]
    pub pmdinclr: crate::Reg<pmdinclr::PMDINCLR_SPEC>,
    #[doc = "0x48 - PMDINSET register"]
    pub pmdinset: crate::Reg<pmdinset::PMDINSET_SPEC>,
    #[doc = "0x4c - PMDININV register"]
    pub pmdininv: crate::Reg<pmdininv::PMDININV_SPEC>,
    #[doc = "0x50 - PMAEN register"]
    pub pmaen: crate::Reg<pmaen::PMAEN_SPEC>,
    #[doc = "0x54 - PMAENCLR register"]
    pub pmaenclr: crate::Reg<pmaenclr::PMAENCLR_SPEC>,
    #[doc = "0x58 - PMAENSET register"]
    pub pmaenset: crate::Reg<pmaenset::PMAENSET_SPEC>,
    #[doc = "0x5c - PMAENINV register"]
    pub pmaeninv: crate::Reg<pmaeninv::PMAENINV_SPEC>,
    #[doc = "0x60 - PMSTAT register"]
    pub pmstat: crate::Reg<pmstat::PMSTAT_SPEC>,
    #[doc = "0x64 - PMSTATCLR register"]
    pub pmstatclr: crate::Reg<pmstatclr::PMSTATCLR_SPEC>,
    #[doc = "0x68 - PMSTATSET register"]
    pub pmstatset: crate::Reg<pmstatset::PMSTATSET_SPEC>,
    #[doc = "0x6c - PMSTATINV register"]
    pub pmstatinv: crate::Reg<pmstatinv::PMSTATINV_SPEC>,
    #[doc = "0x70 - PMWADDR register"]
    pub pmwaddr: crate::Reg<pmwaddr::PMWADDR_SPEC>,
    #[doc = "0x74 - PMWADDRCLR register"]
    pub pmwaddrclr: crate::Reg<pmwaddrclr::PMWADDRCLR_SPEC>,
    #[doc = "0x78 - PMWADDRSET register"]
    pub pmwaddrset: crate::Reg<pmwaddrset::PMWADDRSET_SPEC>,
    #[doc = "0x7c - PMWADDRINV register"]
    pub pmwaddrinv: crate::Reg<pmwaddrinv::PMWADDRINV_SPEC>,
    #[doc = "0x80 - PMRADDR register"]
    pub pmraddr: crate::Reg<pmraddr::PMRADDR_SPEC>,
    #[doc = "0x84 - PMRADDRCLR register"]
    pub pmraddrclr: crate::Reg<pmraddrclr::PMRADDRCLR_SPEC>,
    #[doc = "0x88 - PMRADDRSET register"]
    pub pmraddrset: crate::Reg<pmraddrset::PMRADDRSET_SPEC>,
    #[doc = "0x8c - PMRADDRINV register"]
    pub pmraddrinv: crate::Reg<pmraddrinv::PMRADDRINV_SPEC>,
    #[doc = "0x90 - PMRDIN register"]
    pub pmrdin: crate::Reg<pmrdin::PMRDIN_SPEC>,
    #[doc = "0x94 - PMRDINCLR register"]
    pub pmrdinclr: crate::Reg<pmrdinclr::PMRDINCLR_SPEC>,
    #[doc = "0x98 - PMRDINSET register"]
    pub pmrdinset: crate::Reg<pmrdinset::PMRDINSET_SPEC>,
    #[doc = "0x9c - PMRDININV register"]
    pub pmrdininv: crate::Reg<pmrdininv::PMRDININV_SPEC>,
}
#[doc = "PMCON register accessor: an alias for `Reg<PMCON_SPEC>`"]
pub type PMCON = crate::Reg<pmcon::PMCON_SPEC>;
#[doc = "PMCON register"]
pub mod pmcon;
#[doc = "PMCONCLR register accessor: an alias for `Reg<PMCONCLR_SPEC>`"]
pub type PMCONCLR = crate::Reg<pmconclr::PMCONCLR_SPEC>;
#[doc = "PMCONCLR register"]
pub mod pmconclr;
#[doc = "PMCONSET register accessor: an alias for `Reg<PMCONSET_SPEC>`"]
pub type PMCONSET = crate::Reg<pmconset::PMCONSET_SPEC>;
#[doc = "PMCONSET register"]
pub mod pmconset;
#[doc = "PMCONINV register accessor: an alias for `Reg<PMCONINV_SPEC>`"]
pub type PMCONINV = crate::Reg<pmconinv::PMCONINV_SPEC>;
#[doc = "PMCONINV register"]
pub mod pmconinv;
#[doc = "PMMODE register accessor: an alias for `Reg<PMMODE_SPEC>`"]
pub type PMMODE = crate::Reg<pmmode::PMMODE_SPEC>;
#[doc = "PMMODE register"]
pub mod pmmode;
#[doc = "PMMODECLR register accessor: an alias for `Reg<PMMODECLR_SPEC>`"]
pub type PMMODECLR = crate::Reg<pmmodeclr::PMMODECLR_SPEC>;
#[doc = "PMMODECLR register"]
pub mod pmmodeclr;
#[doc = "PMMODESET register accessor: an alias for `Reg<PMMODESET_SPEC>`"]
pub type PMMODESET = crate::Reg<pmmodeset::PMMODESET_SPEC>;
#[doc = "PMMODESET register"]
pub mod pmmodeset;
#[doc = "PMMODEINV register accessor: an alias for `Reg<PMMODEINV_SPEC>`"]
pub type PMMODEINV = crate::Reg<pmmodeinv::PMMODEINV_SPEC>;
#[doc = "PMMODEINV register"]
pub mod pmmodeinv;
#[doc = "PMADDR register accessor: an alias for `Reg<PMADDR_SPEC>`"]
pub type PMADDR = crate::Reg<pmaddr::PMADDR_SPEC>;
#[doc = "PMADDR register"]
pub mod pmaddr;
#[doc = "PMADDRCLR register accessor: an alias for `Reg<PMADDRCLR_SPEC>`"]
pub type PMADDRCLR = crate::Reg<pmaddrclr::PMADDRCLR_SPEC>;
#[doc = "PMADDRCLR register"]
pub mod pmaddrclr;
#[doc = "PMADDRSET register accessor: an alias for `Reg<PMADDRSET_SPEC>`"]
pub type PMADDRSET = crate::Reg<pmaddrset::PMADDRSET_SPEC>;
#[doc = "PMADDRSET register"]
pub mod pmaddrset;
#[doc = "PMADDRINV register accessor: an alias for `Reg<PMADDRINV_SPEC>`"]
pub type PMADDRINV = crate::Reg<pmaddrinv::PMADDRINV_SPEC>;
#[doc = "PMADDRINV register"]
pub mod pmaddrinv;
#[doc = "PMDOUT register accessor: an alias for `Reg<PMDOUT_SPEC>`"]
pub type PMDOUT = crate::Reg<pmdout::PMDOUT_SPEC>;
#[doc = "PMDOUT register"]
pub mod pmdout;
#[doc = "PMDOUTCLR register accessor: an alias for `Reg<PMDOUTCLR_SPEC>`"]
pub type PMDOUTCLR = crate::Reg<pmdoutclr::PMDOUTCLR_SPEC>;
#[doc = "PMDOUTCLR register"]
pub mod pmdoutclr;
#[doc = "PMDOUTSET register accessor: an alias for `Reg<PMDOUTSET_SPEC>`"]
pub type PMDOUTSET = crate::Reg<pmdoutset::PMDOUTSET_SPEC>;
#[doc = "PMDOUTSET register"]
pub mod pmdoutset;
#[doc = "PMDOUTINV register accessor: an alias for `Reg<PMDOUTINV_SPEC>`"]
pub type PMDOUTINV = crate::Reg<pmdoutinv::PMDOUTINV_SPEC>;
#[doc = "PMDOUTINV register"]
pub mod pmdoutinv;
#[doc = "PMDIN register accessor: an alias for `Reg<PMDIN_SPEC>`"]
pub type PMDIN = crate::Reg<pmdin::PMDIN_SPEC>;
#[doc = "PMDIN register"]
pub mod pmdin;
#[doc = "PMDINCLR register accessor: an alias for `Reg<PMDINCLR_SPEC>`"]
pub type PMDINCLR = crate::Reg<pmdinclr::PMDINCLR_SPEC>;
#[doc = "PMDINCLR register"]
pub mod pmdinclr;
#[doc = "PMDINSET register accessor: an alias for `Reg<PMDINSET_SPEC>`"]
pub type PMDINSET = crate::Reg<pmdinset::PMDINSET_SPEC>;
#[doc = "PMDINSET register"]
pub mod pmdinset;
#[doc = "PMDININV register accessor: an alias for `Reg<PMDININV_SPEC>`"]
pub type PMDININV = crate::Reg<pmdininv::PMDININV_SPEC>;
#[doc = "PMDININV register"]
pub mod pmdininv;
#[doc = "PMAEN register accessor: an alias for `Reg<PMAEN_SPEC>`"]
pub type PMAEN = crate::Reg<pmaen::PMAEN_SPEC>;
#[doc = "PMAEN register"]
pub mod pmaen;
#[doc = "PMAENCLR register accessor: an alias for `Reg<PMAENCLR_SPEC>`"]
pub type PMAENCLR = crate::Reg<pmaenclr::PMAENCLR_SPEC>;
#[doc = "PMAENCLR register"]
pub mod pmaenclr;
#[doc = "PMAENSET register accessor: an alias for `Reg<PMAENSET_SPEC>`"]
pub type PMAENSET = crate::Reg<pmaenset::PMAENSET_SPEC>;
#[doc = "PMAENSET register"]
pub mod pmaenset;
#[doc = "PMAENINV register accessor: an alias for `Reg<PMAENINV_SPEC>`"]
pub type PMAENINV = crate::Reg<pmaeninv::PMAENINV_SPEC>;
#[doc = "PMAENINV register"]
pub mod pmaeninv;
#[doc = "PMSTAT register accessor: an alias for `Reg<PMSTAT_SPEC>`"]
pub type PMSTAT = crate::Reg<pmstat::PMSTAT_SPEC>;
#[doc = "PMSTAT register"]
pub mod pmstat;
#[doc = "PMSTATCLR register accessor: an alias for `Reg<PMSTATCLR_SPEC>`"]
pub type PMSTATCLR = crate::Reg<pmstatclr::PMSTATCLR_SPEC>;
#[doc = "PMSTATCLR register"]
pub mod pmstatclr;
#[doc = "PMSTATSET register accessor: an alias for `Reg<PMSTATSET_SPEC>`"]
pub type PMSTATSET = crate::Reg<pmstatset::PMSTATSET_SPEC>;
#[doc = "PMSTATSET register"]
pub mod pmstatset;
#[doc = "PMSTATINV register accessor: an alias for `Reg<PMSTATINV_SPEC>`"]
pub type PMSTATINV = crate::Reg<pmstatinv::PMSTATINV_SPEC>;
#[doc = "PMSTATINV register"]
pub mod pmstatinv;
#[doc = "PMWADDR register accessor: an alias for `Reg<PMWADDR_SPEC>`"]
pub type PMWADDR = crate::Reg<pmwaddr::PMWADDR_SPEC>;
#[doc = "PMWADDR register"]
pub mod pmwaddr;
#[doc = "PMWADDRCLR register accessor: an alias for `Reg<PMWADDRCLR_SPEC>`"]
pub type PMWADDRCLR = crate::Reg<pmwaddrclr::PMWADDRCLR_SPEC>;
#[doc = "PMWADDRCLR register"]
pub mod pmwaddrclr;
#[doc = "PMWADDRSET register accessor: an alias for `Reg<PMWADDRSET_SPEC>`"]
pub type PMWADDRSET = crate::Reg<pmwaddrset::PMWADDRSET_SPEC>;
#[doc = "PMWADDRSET register"]
pub mod pmwaddrset;
#[doc = "PMWADDRINV register accessor: an alias for `Reg<PMWADDRINV_SPEC>`"]
pub type PMWADDRINV = crate::Reg<pmwaddrinv::PMWADDRINV_SPEC>;
#[doc = "PMWADDRINV register"]
pub mod pmwaddrinv;
#[doc = "PMRADDR register accessor: an alias for `Reg<PMRADDR_SPEC>`"]
pub type PMRADDR = crate::Reg<pmraddr::PMRADDR_SPEC>;
#[doc = "PMRADDR register"]
pub mod pmraddr;
#[doc = "PMRADDRCLR register accessor: an alias for `Reg<PMRADDRCLR_SPEC>`"]
pub type PMRADDRCLR = crate::Reg<pmraddrclr::PMRADDRCLR_SPEC>;
#[doc = "PMRADDRCLR register"]
pub mod pmraddrclr;
#[doc = "PMRADDRSET register accessor: an alias for `Reg<PMRADDRSET_SPEC>`"]
pub type PMRADDRSET = crate::Reg<pmraddrset::PMRADDRSET_SPEC>;
#[doc = "PMRADDRSET register"]
pub mod pmraddrset;
#[doc = "PMRADDRINV register accessor: an alias for `Reg<PMRADDRINV_SPEC>`"]
pub type PMRADDRINV = crate::Reg<pmraddrinv::PMRADDRINV_SPEC>;
#[doc = "PMRADDRINV register"]
pub mod pmraddrinv;
#[doc = "PMRDIN register accessor: an alias for `Reg<PMRDIN_SPEC>`"]
pub type PMRDIN = crate::Reg<pmrdin::PMRDIN_SPEC>;
#[doc = "PMRDIN register"]
pub mod pmrdin;
#[doc = "PMRDINCLR register accessor: an alias for `Reg<PMRDINCLR_SPEC>`"]
pub type PMRDINCLR = crate::Reg<pmrdinclr::PMRDINCLR_SPEC>;
#[doc = "PMRDINCLR register"]
pub mod pmrdinclr;
#[doc = "PMRDINSET register accessor: an alias for `Reg<PMRDINSET_SPEC>`"]
pub type PMRDINSET = crate::Reg<pmrdinset::PMRDINSET_SPEC>;
#[doc = "PMRDINSET register"]
pub mod pmrdinset;
#[doc = "PMRDININV register accessor: an alias for `Reg<PMRDININV_SPEC>`"]
pub type PMRDININV = crate::Reg<pmrdininv::PMRDININV_SPEC>;
#[doc = "PMRDININV register"]
pub mod pmrdininv;
