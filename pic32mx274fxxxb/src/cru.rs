#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSCCON register"]
    pub osccon: crate::Reg<osccon::OSCCON_SPEC>,
    #[doc = "0x04 - OSCCONCLR register"]
    pub oscconclr: crate::Reg<oscconclr::OSCCONCLR_SPEC>,
    #[doc = "0x08 - OSCCONSET register"]
    pub oscconset: crate::Reg<oscconset::OSCCONSET_SPEC>,
    #[doc = "0x0c - OSCCONINV register"]
    pub oscconinv: crate::Reg<oscconinv::OSCCONINV_SPEC>,
    #[doc = "0x10 - OSCTUN register"]
    pub osctun: crate::Reg<osctun::OSCTUN_SPEC>,
    #[doc = "0x14 - OSCTUNCLR register"]
    pub osctunclr: crate::Reg<osctunclr::OSCTUNCLR_SPEC>,
    #[doc = "0x18 - OSCTUNSET register"]
    pub osctunset: crate::Reg<osctunset::OSCTUNSET_SPEC>,
    #[doc = "0x1c - OSCTUNINV register"]
    pub osctuninv: crate::Reg<osctuninv::OSCTUNINV_SPEC>,
    #[doc = "0x20 - SPLLCON register"]
    pub spllcon: crate::Reg<spllcon::SPLLCON_SPEC>,
    #[doc = "0x24 - SPLLCONCLR register"]
    pub spllconclr: crate::Reg<spllconclr::SPLLCONCLR_SPEC>,
    #[doc = "0x28 - SPLLCONSET register"]
    pub spllconset: crate::Reg<spllconset::SPLLCONSET_SPEC>,
    #[doc = "0x2c - SPLLCONINV register"]
    pub spllconinv: crate::Reg<spllconinv::SPLLCONINV_SPEC>,
    #[doc = "0x30 - UPLLCON register"]
    pub upllcon: crate::Reg<upllcon::UPLLCON_SPEC>,
    #[doc = "0x34 - UPLLCONCLR register"]
    pub upllconclr: crate::Reg<upllconclr::UPLLCONCLR_SPEC>,
    #[doc = "0x38 - UPLLCONSET register"]
    pub upllconset: crate::Reg<upllconset::UPLLCONSET_SPEC>,
    #[doc = "0x3c - UPLLCONINV register"]
    pub upllconinv: crate::Reg<upllconinv::UPLLCONINV_SPEC>,
    #[doc = "0x40 - RCON register"]
    pub rcon: crate::Reg<rcon::RCON_SPEC>,
    #[doc = "0x44 - RCONCLR register"]
    pub rconclr: crate::Reg<rconclr::RCONCLR_SPEC>,
    #[doc = "0x48 - RCONSET register"]
    pub rconset: crate::Reg<rconset::RCONSET_SPEC>,
    #[doc = "0x4c - RCONINV register"]
    pub rconinv: crate::Reg<rconinv::RCONINV_SPEC>,
    #[doc = "0x50 - RSWRST register"]
    pub rswrst: crate::Reg<rswrst::RSWRST_SPEC>,
    #[doc = "0x54 - RSWRSTCLR register"]
    pub rswrstclr: crate::Reg<rswrstclr::RSWRSTCLR_SPEC>,
    #[doc = "0x58 - RSWRSTSET register"]
    pub rswrstset: crate::Reg<rswrstset::RSWRSTSET_SPEC>,
    #[doc = "0x5c - RSWRSTINV register"]
    pub rswrstinv: crate::Reg<rswrstinv::RSWRSTINV_SPEC>,
    #[doc = "0x60 - RNMICON register"]
    pub rnmicon: crate::Reg<rnmicon::RNMICON_SPEC>,
    #[doc = "0x64 - RNMICONCLR register"]
    pub rnmiconclr: crate::Reg<rnmiconclr::RNMICONCLR_SPEC>,
    #[doc = "0x68 - RNMICONSET register"]
    pub rnmiconset: crate::Reg<rnmiconset::RNMICONSET_SPEC>,
    #[doc = "0x6c - RNMICONINV register"]
    pub rnmiconinv: crate::Reg<rnmiconinv::RNMICONINV_SPEC>,
    #[doc = "0x70 - PWRCON register"]
    pub pwrcon: crate::Reg<pwrcon::PWRCON_SPEC>,
    #[doc = "0x74 - PWRCONCLR register"]
    pub pwrconclr: crate::Reg<pwrconclr::PWRCONCLR_SPEC>,
    #[doc = "0x78 - PWRCONSET register"]
    pub pwrconset: crate::Reg<pwrconset::PWRCONSET_SPEC>,
    #[doc = "0x7c - PWRCONINV register"]
    pub pwrconinv: crate::Reg<pwrconinv::PWRCONINV_SPEC>,
    #[doc = "0x80 - REFO1CON register"]
    pub refo1con: crate::Reg<refo1con::REFO1CON_SPEC>,
    #[doc = "0x84 - REFO1CONCLR register"]
    pub refo1conclr: crate::Reg<refo1conclr::REFO1CONCLR_SPEC>,
    #[doc = "0x88 - REFO1CONSET register"]
    pub refo1conset: crate::Reg<refo1conset::REFO1CONSET_SPEC>,
    #[doc = "0x8c - REFO1CONINV register"]
    pub refo1coninv: crate::Reg<refo1coninv::REFO1CONINV_SPEC>,
    #[doc = "0x90 - REFO1TRIM register"]
    pub refo1trim: crate::Reg<refo1trim::REFO1TRIM_SPEC>,
    #[doc = "0x94 - REFO1TRIMCLR register"]
    pub refo1trimclr: crate::Reg<refo1trimclr::REFO1TRIMCLR_SPEC>,
    #[doc = "0x98 - REFO1TRIMSET register"]
    pub refo1trimset: crate::Reg<refo1trimset::REFO1TRIMSET_SPEC>,
    #[doc = "0x9c - REFO1TRIMINV register"]
    pub refo1triminv: crate::Reg<refo1triminv::REFO1TRIMINV_SPEC>,
    _reserved40: [u8; 96usize],
    #[doc = "0x100 - PB1DIV register"]
    pub pb1div: crate::Reg<pb1div::PB1DIV_SPEC>,
    #[doc = "0x104 - PB1DIVCLR register"]
    pub pb1divclr: crate::Reg<pb1divclr::PB1DIVCLR_SPEC>,
    #[doc = "0x108 - PB1DIVSET register"]
    pub pb1divset: crate::Reg<pb1divset::PB1DIVSET_SPEC>,
    #[doc = "0x10c - PB1DIVINV register"]
    pub pb1divinv: crate::Reg<pb1divinv::PB1DIVINV_SPEC>,
    _reserved44: [u8; 192usize],
    #[doc = "0x1d0 - CLKSTAT register"]
    pub clkstat: crate::Reg<clkstat::CLKSTAT_SPEC>,
    #[doc = "0x1d4 - CLKSTATCLR register"]
    pub clkstatclr: crate::Reg<clkstatclr::CLKSTATCLR_SPEC>,
    #[doc = "0x1d8 - CLKSTATSET register"]
    pub clkstatset: crate::Reg<clkstatset::CLKSTATSET_SPEC>,
    #[doc = "0x1dc - CLKSTATINV register"]
    pub clkstatinv: crate::Reg<clkstatinv::CLKSTATINV_SPEC>,
}
#[doc = "OSCCON register accessor: an alias for `Reg<OSCCON_SPEC>`"]
pub type OSCCON = crate::Reg<osccon::OSCCON_SPEC>;
#[doc = "OSCCON register"]
pub mod osccon;
#[doc = "OSCCONCLR register accessor: an alias for `Reg<OSCCONCLR_SPEC>`"]
pub type OSCCONCLR = crate::Reg<oscconclr::OSCCONCLR_SPEC>;
#[doc = "OSCCONCLR register"]
pub mod oscconclr;
#[doc = "OSCCONSET register accessor: an alias for `Reg<OSCCONSET_SPEC>`"]
pub type OSCCONSET = crate::Reg<oscconset::OSCCONSET_SPEC>;
#[doc = "OSCCONSET register"]
pub mod oscconset;
#[doc = "OSCCONINV register accessor: an alias for `Reg<OSCCONINV_SPEC>`"]
pub type OSCCONINV = crate::Reg<oscconinv::OSCCONINV_SPEC>;
#[doc = "OSCCONINV register"]
pub mod oscconinv;
#[doc = "OSCTUN register accessor: an alias for `Reg<OSCTUN_SPEC>`"]
pub type OSCTUN = crate::Reg<osctun::OSCTUN_SPEC>;
#[doc = "OSCTUN register"]
pub mod osctun;
#[doc = "OSCTUNCLR register accessor: an alias for `Reg<OSCTUNCLR_SPEC>`"]
pub type OSCTUNCLR = crate::Reg<osctunclr::OSCTUNCLR_SPEC>;
#[doc = "OSCTUNCLR register"]
pub mod osctunclr;
#[doc = "OSCTUNSET register accessor: an alias for `Reg<OSCTUNSET_SPEC>`"]
pub type OSCTUNSET = crate::Reg<osctunset::OSCTUNSET_SPEC>;
#[doc = "OSCTUNSET register"]
pub mod osctunset;
#[doc = "OSCTUNINV register accessor: an alias for `Reg<OSCTUNINV_SPEC>`"]
pub type OSCTUNINV = crate::Reg<osctuninv::OSCTUNINV_SPEC>;
#[doc = "OSCTUNINV register"]
pub mod osctuninv;
#[doc = "SPLLCON register accessor: an alias for `Reg<SPLLCON_SPEC>`"]
pub type SPLLCON = crate::Reg<spllcon::SPLLCON_SPEC>;
#[doc = "SPLLCON register"]
pub mod spllcon;
#[doc = "SPLLCONCLR register accessor: an alias for `Reg<SPLLCONCLR_SPEC>`"]
pub type SPLLCONCLR = crate::Reg<spllconclr::SPLLCONCLR_SPEC>;
#[doc = "SPLLCONCLR register"]
pub mod spllconclr;
#[doc = "SPLLCONSET register accessor: an alias for `Reg<SPLLCONSET_SPEC>`"]
pub type SPLLCONSET = crate::Reg<spllconset::SPLLCONSET_SPEC>;
#[doc = "SPLLCONSET register"]
pub mod spllconset;
#[doc = "SPLLCONINV register accessor: an alias for `Reg<SPLLCONINV_SPEC>`"]
pub type SPLLCONINV = crate::Reg<spllconinv::SPLLCONINV_SPEC>;
#[doc = "SPLLCONINV register"]
pub mod spllconinv;
#[doc = "UPLLCON register accessor: an alias for `Reg<UPLLCON_SPEC>`"]
pub type UPLLCON = crate::Reg<upllcon::UPLLCON_SPEC>;
#[doc = "UPLLCON register"]
pub mod upllcon;
#[doc = "UPLLCONCLR register accessor: an alias for `Reg<UPLLCONCLR_SPEC>`"]
pub type UPLLCONCLR = crate::Reg<upllconclr::UPLLCONCLR_SPEC>;
#[doc = "UPLLCONCLR register"]
pub mod upllconclr;
#[doc = "UPLLCONSET register accessor: an alias for `Reg<UPLLCONSET_SPEC>`"]
pub type UPLLCONSET = crate::Reg<upllconset::UPLLCONSET_SPEC>;
#[doc = "UPLLCONSET register"]
pub mod upllconset;
#[doc = "UPLLCONINV register accessor: an alias for `Reg<UPLLCONINV_SPEC>`"]
pub type UPLLCONINV = crate::Reg<upllconinv::UPLLCONINV_SPEC>;
#[doc = "UPLLCONINV register"]
pub mod upllconinv;
#[doc = "RCON register accessor: an alias for `Reg<RCON_SPEC>`"]
pub type RCON = crate::Reg<rcon::RCON_SPEC>;
#[doc = "RCON register"]
pub mod rcon;
#[doc = "RCONCLR register accessor: an alias for `Reg<RCONCLR_SPEC>`"]
pub type RCONCLR = crate::Reg<rconclr::RCONCLR_SPEC>;
#[doc = "RCONCLR register"]
pub mod rconclr;
#[doc = "RCONSET register accessor: an alias for `Reg<RCONSET_SPEC>`"]
pub type RCONSET = crate::Reg<rconset::RCONSET_SPEC>;
#[doc = "RCONSET register"]
pub mod rconset;
#[doc = "RCONINV register accessor: an alias for `Reg<RCONINV_SPEC>`"]
pub type RCONINV = crate::Reg<rconinv::RCONINV_SPEC>;
#[doc = "RCONINV register"]
pub mod rconinv;
#[doc = "RSWRST register accessor: an alias for `Reg<RSWRST_SPEC>`"]
pub type RSWRST = crate::Reg<rswrst::RSWRST_SPEC>;
#[doc = "RSWRST register"]
pub mod rswrst;
#[doc = "RSWRSTCLR register accessor: an alias for `Reg<RSWRSTCLR_SPEC>`"]
pub type RSWRSTCLR = crate::Reg<rswrstclr::RSWRSTCLR_SPEC>;
#[doc = "RSWRSTCLR register"]
pub mod rswrstclr;
#[doc = "RSWRSTSET register accessor: an alias for `Reg<RSWRSTSET_SPEC>`"]
pub type RSWRSTSET = crate::Reg<rswrstset::RSWRSTSET_SPEC>;
#[doc = "RSWRSTSET register"]
pub mod rswrstset;
#[doc = "RSWRSTINV register accessor: an alias for `Reg<RSWRSTINV_SPEC>`"]
pub type RSWRSTINV = crate::Reg<rswrstinv::RSWRSTINV_SPEC>;
#[doc = "RSWRSTINV register"]
pub mod rswrstinv;
#[doc = "RNMICON register accessor: an alias for `Reg<RNMICON_SPEC>`"]
pub type RNMICON = crate::Reg<rnmicon::RNMICON_SPEC>;
#[doc = "RNMICON register"]
pub mod rnmicon;
#[doc = "RNMICONCLR register accessor: an alias for `Reg<RNMICONCLR_SPEC>`"]
pub type RNMICONCLR = crate::Reg<rnmiconclr::RNMICONCLR_SPEC>;
#[doc = "RNMICONCLR register"]
pub mod rnmiconclr;
#[doc = "RNMICONSET register accessor: an alias for `Reg<RNMICONSET_SPEC>`"]
pub type RNMICONSET = crate::Reg<rnmiconset::RNMICONSET_SPEC>;
#[doc = "RNMICONSET register"]
pub mod rnmiconset;
#[doc = "RNMICONINV register accessor: an alias for `Reg<RNMICONINV_SPEC>`"]
pub type RNMICONINV = crate::Reg<rnmiconinv::RNMICONINV_SPEC>;
#[doc = "RNMICONINV register"]
pub mod rnmiconinv;
#[doc = "PWRCON register accessor: an alias for `Reg<PWRCON_SPEC>`"]
pub type PWRCON = crate::Reg<pwrcon::PWRCON_SPEC>;
#[doc = "PWRCON register"]
pub mod pwrcon;
#[doc = "PWRCONCLR register accessor: an alias for `Reg<PWRCONCLR_SPEC>`"]
pub type PWRCONCLR = crate::Reg<pwrconclr::PWRCONCLR_SPEC>;
#[doc = "PWRCONCLR register"]
pub mod pwrconclr;
#[doc = "PWRCONSET register accessor: an alias for `Reg<PWRCONSET_SPEC>`"]
pub type PWRCONSET = crate::Reg<pwrconset::PWRCONSET_SPEC>;
#[doc = "PWRCONSET register"]
pub mod pwrconset;
#[doc = "PWRCONINV register accessor: an alias for `Reg<PWRCONINV_SPEC>`"]
pub type PWRCONINV = crate::Reg<pwrconinv::PWRCONINV_SPEC>;
#[doc = "PWRCONINV register"]
pub mod pwrconinv;
#[doc = "REFO1CON register accessor: an alias for `Reg<REFO1CON_SPEC>`"]
pub type REFO1CON = crate::Reg<refo1con::REFO1CON_SPEC>;
#[doc = "REFO1CON register"]
pub mod refo1con;
#[doc = "REFO1CONCLR register accessor: an alias for `Reg<REFO1CONCLR_SPEC>`"]
pub type REFO1CONCLR = crate::Reg<refo1conclr::REFO1CONCLR_SPEC>;
#[doc = "REFO1CONCLR register"]
pub mod refo1conclr;
#[doc = "REFO1CONSET register accessor: an alias for `Reg<REFO1CONSET_SPEC>`"]
pub type REFO1CONSET = crate::Reg<refo1conset::REFO1CONSET_SPEC>;
#[doc = "REFO1CONSET register"]
pub mod refo1conset;
#[doc = "REFO1CONINV register accessor: an alias for `Reg<REFO1CONINV_SPEC>`"]
pub type REFO1CONINV = crate::Reg<refo1coninv::REFO1CONINV_SPEC>;
#[doc = "REFO1CONINV register"]
pub mod refo1coninv;
#[doc = "REFO1TRIM register accessor: an alias for `Reg<REFO1TRIM_SPEC>`"]
pub type REFO1TRIM = crate::Reg<refo1trim::REFO1TRIM_SPEC>;
#[doc = "REFO1TRIM register"]
pub mod refo1trim;
#[doc = "REFO1TRIMCLR register accessor: an alias for `Reg<REFO1TRIMCLR_SPEC>`"]
pub type REFO1TRIMCLR = crate::Reg<refo1trimclr::REFO1TRIMCLR_SPEC>;
#[doc = "REFO1TRIMCLR register"]
pub mod refo1trimclr;
#[doc = "REFO1TRIMSET register accessor: an alias for `Reg<REFO1TRIMSET_SPEC>`"]
pub type REFO1TRIMSET = crate::Reg<refo1trimset::REFO1TRIMSET_SPEC>;
#[doc = "REFO1TRIMSET register"]
pub mod refo1trimset;
#[doc = "REFO1TRIMINV register accessor: an alias for `Reg<REFO1TRIMINV_SPEC>`"]
pub type REFO1TRIMINV = crate::Reg<refo1triminv::REFO1TRIMINV_SPEC>;
#[doc = "REFO1TRIMINV register"]
pub mod refo1triminv;
#[doc = "PB1DIV register accessor: an alias for `Reg<PB1DIV_SPEC>`"]
pub type PB1DIV = crate::Reg<pb1div::PB1DIV_SPEC>;
#[doc = "PB1DIV register"]
pub mod pb1div;
#[doc = "PB1DIVCLR register accessor: an alias for `Reg<PB1DIVCLR_SPEC>`"]
pub type PB1DIVCLR = crate::Reg<pb1divclr::PB1DIVCLR_SPEC>;
#[doc = "PB1DIVCLR register"]
pub mod pb1divclr;
#[doc = "PB1DIVSET register accessor: an alias for `Reg<PB1DIVSET_SPEC>`"]
pub type PB1DIVSET = crate::Reg<pb1divset::PB1DIVSET_SPEC>;
#[doc = "PB1DIVSET register"]
pub mod pb1divset;
#[doc = "PB1DIVINV register accessor: an alias for `Reg<PB1DIVINV_SPEC>`"]
pub type PB1DIVINV = crate::Reg<pb1divinv::PB1DIVINV_SPEC>;
#[doc = "PB1DIVINV register"]
pub mod pb1divinv;
#[doc = "CLKSTAT register accessor: an alias for `Reg<CLKSTAT_SPEC>`"]
pub type CLKSTAT = crate::Reg<clkstat::CLKSTAT_SPEC>;
#[doc = "CLKSTAT register"]
pub mod clkstat;
#[doc = "CLKSTATCLR register accessor: an alias for `Reg<CLKSTATCLR_SPEC>`"]
pub type CLKSTATCLR = crate::Reg<clkstatclr::CLKSTATCLR_SPEC>;
#[doc = "CLKSTATCLR register"]
pub mod clkstatclr;
#[doc = "CLKSTATSET register accessor: an alias for `Reg<CLKSTATSET_SPEC>`"]
pub type CLKSTATSET = crate::Reg<clkstatset::CLKSTATSET_SPEC>;
#[doc = "CLKSTATSET register"]
pub mod clkstatset;
#[doc = "CLKSTATINV register accessor: an alias for `Reg<CLKSTATINV_SPEC>`"]
pub type CLKSTATINV = crate::Reg<clkstatinv::CLKSTATINV_SPEC>;
#[doc = "CLKSTATINV register"]
pub mod clkstatinv;
