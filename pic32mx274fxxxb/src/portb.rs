#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELB register"]
    pub ansel: crate::Reg<ansel::ANSEL_SPEC>,
    #[doc = "0x04 - ANSELBCLR register"]
    pub anselclr: crate::Reg<anselclr::ANSELCLR_SPEC>,
    #[doc = "0x08 - ANSELBSET register"]
    pub anselset: crate::Reg<anselset::ANSELSET_SPEC>,
    #[doc = "0x0c - ANSELBINV register"]
    pub anselinv: crate::Reg<anselinv::ANSELINV_SPEC>,
    #[doc = "0x10 - TRISB register"]
    pub tris: crate::Reg<tris::TRIS_SPEC>,
    #[doc = "0x14 - TRISBCLR register"]
    pub trisclr: crate::Reg<trisclr::TRISCLR_SPEC>,
    #[doc = "0x18 - TRISBSET register"]
    pub trisset: crate::Reg<trisset::TRISSET_SPEC>,
    #[doc = "0x1c - TRISBINV register"]
    pub trisinv: crate::Reg<trisinv::TRISINV_SPEC>,
    #[doc = "0x20 - PORTB register"]
    pub port: crate::Reg<port::PORT_SPEC>,
    #[doc = "0x24 - PORTBCLR register"]
    pub portclr: crate::Reg<portclr::PORTCLR_SPEC>,
    #[doc = "0x28 - PORTBSET register"]
    pub portset: crate::Reg<portset::PORTSET_SPEC>,
    #[doc = "0x2c - PORTBINV register"]
    pub portinv: crate::Reg<portinv::PORTINV_SPEC>,
    #[doc = "0x30 - LATB register"]
    pub lat: crate::Reg<lat::LAT_SPEC>,
    #[doc = "0x34 - LATBCLR register"]
    pub latclr: crate::Reg<latclr::LATCLR_SPEC>,
    #[doc = "0x38 - LATBSET register"]
    pub latset: crate::Reg<latset::LATSET_SPEC>,
    #[doc = "0x3c - LATBINV register"]
    pub latinv: crate::Reg<latinv::LATINV_SPEC>,
    #[doc = "0x40 - ODCB register"]
    pub odc: crate::Reg<odc::ODC_SPEC>,
    #[doc = "0x44 - ODCBCLR register"]
    pub odcclr: crate::Reg<odcclr::ODCCLR_SPEC>,
    #[doc = "0x48 - ODCBSET register"]
    pub odcset: crate::Reg<odcset::ODCSET_SPEC>,
    #[doc = "0x4c - ODCBINV register"]
    pub odcinv: crate::Reg<odcinv::ODCINV_SPEC>,
    #[doc = "0x50 - CNPUB register"]
    pub cnpu: crate::Reg<cnpu::CNPU_SPEC>,
    #[doc = "0x54 - CNPUBCLR register"]
    pub cnpuclr: crate::Reg<cnpuclr::CNPUCLR_SPEC>,
    #[doc = "0x58 - CNPUBSET register"]
    pub cnpuset: crate::Reg<cnpuset::CNPUSET_SPEC>,
    #[doc = "0x5c - CNPUBINV register"]
    pub cnpuinv: crate::Reg<cnpuinv::CNPUINV_SPEC>,
    #[doc = "0x60 - CNPDB register"]
    pub cnpd: crate::Reg<cnpd::CNPD_SPEC>,
    #[doc = "0x64 - CNPDBCLR register"]
    pub cnpdclr: crate::Reg<cnpdclr::CNPDCLR_SPEC>,
    #[doc = "0x68 - CNPDBSET register"]
    pub cnpdset: crate::Reg<cnpdset::CNPDSET_SPEC>,
    #[doc = "0x6c - CNPDBINV register"]
    pub cnpdinv: crate::Reg<cnpdinv::CNPDINV_SPEC>,
    #[doc = "0x70 - CNCONB register"]
    pub cncon: crate::Reg<cncon::CNCON_SPEC>,
    #[doc = "0x74 - CNCONBCLR register"]
    pub cnconclr: crate::Reg<cnconclr::CNCONCLR_SPEC>,
    #[doc = "0x78 - CNCONBSET register"]
    pub cnconset: crate::Reg<cnconset::CNCONSET_SPEC>,
    #[doc = "0x7c - CNCONBINV register"]
    pub cnconinv: crate::Reg<cnconinv::CNCONINV_SPEC>,
    #[doc = "0x80 - CNENB register"]
    pub cnen: crate::Reg<cnen::CNEN_SPEC>,
    #[doc = "0x84 - CNENBCLR register"]
    pub cnenclr: crate::Reg<cnenclr::CNENCLR_SPEC>,
    #[doc = "0x88 - CNENBSET register"]
    pub cnenset: crate::Reg<cnenset::CNENSET_SPEC>,
    #[doc = "0x8c - CNENBINV register"]
    pub cneninv: crate::Reg<cneninv::CNENINV_SPEC>,
    #[doc = "0x90 - CNSTATB register"]
    pub cnstat: crate::Reg<cnstat::CNSTAT_SPEC>,
    #[doc = "0x94 - CNSTATBCLR register"]
    pub cnstatclr: crate::Reg<cnstatclr::CNSTATCLR_SPEC>,
    #[doc = "0x98 - CNSTATBSET register"]
    pub cnstatset: crate::Reg<cnstatset::CNSTATSET_SPEC>,
    #[doc = "0x9c - CNSTATBINV register"]
    pub cnstatinv: crate::Reg<cnstatinv::CNSTATINV_SPEC>,
}
#[doc = "ANSEL register accessor: an alias for `Reg<ANSEL_SPEC>`"]
pub type ANSEL = crate::Reg<ansel::ANSEL_SPEC>;
#[doc = "ANSELB register"]
pub mod ansel;
#[doc = "ANSELCLR register accessor: an alias for `Reg<ANSELCLR_SPEC>`"]
pub type ANSELCLR = crate::Reg<anselclr::ANSELCLR_SPEC>;
#[doc = "ANSELBCLR register"]
pub mod anselclr;
#[doc = "ANSELSET register accessor: an alias for `Reg<ANSELSET_SPEC>`"]
pub type ANSELSET = crate::Reg<anselset::ANSELSET_SPEC>;
#[doc = "ANSELBSET register"]
pub mod anselset;
#[doc = "ANSELINV register accessor: an alias for `Reg<ANSELINV_SPEC>`"]
pub type ANSELINV = crate::Reg<anselinv::ANSELINV_SPEC>;
#[doc = "ANSELBINV register"]
pub mod anselinv;
#[doc = "TRIS register accessor: an alias for `Reg<TRIS_SPEC>`"]
pub type TRIS = crate::Reg<tris::TRIS_SPEC>;
#[doc = "TRISB register"]
pub mod tris;
#[doc = "TRISCLR register accessor: an alias for `Reg<TRISCLR_SPEC>`"]
pub type TRISCLR = crate::Reg<trisclr::TRISCLR_SPEC>;
#[doc = "TRISBCLR register"]
pub mod trisclr;
#[doc = "TRISSET register accessor: an alias for `Reg<TRISSET_SPEC>`"]
pub type TRISSET = crate::Reg<trisset::TRISSET_SPEC>;
#[doc = "TRISBSET register"]
pub mod trisset;
#[doc = "TRISINV register accessor: an alias for `Reg<TRISINV_SPEC>`"]
pub type TRISINV = crate::Reg<trisinv::TRISINV_SPEC>;
#[doc = "TRISBINV register"]
pub mod trisinv;
#[doc = "PORT register accessor: an alias for `Reg<PORT_SPEC>`"]
pub type PORT = crate::Reg<port::PORT_SPEC>;
#[doc = "PORTB register"]
pub mod port;
#[doc = "PORTCLR register accessor: an alias for `Reg<PORTCLR_SPEC>`"]
pub type PORTCLR = crate::Reg<portclr::PORTCLR_SPEC>;
#[doc = "PORTBCLR register"]
pub mod portclr;
#[doc = "PORTSET register accessor: an alias for `Reg<PORTSET_SPEC>`"]
pub type PORTSET = crate::Reg<portset::PORTSET_SPEC>;
#[doc = "PORTBSET register"]
pub mod portset;
#[doc = "PORTINV register accessor: an alias for `Reg<PORTINV_SPEC>`"]
pub type PORTINV = crate::Reg<portinv::PORTINV_SPEC>;
#[doc = "PORTBINV register"]
pub mod portinv;
#[doc = "LAT register accessor: an alias for `Reg<LAT_SPEC>`"]
pub type LAT = crate::Reg<lat::LAT_SPEC>;
#[doc = "LATB register"]
pub mod lat;
#[doc = "LATCLR register accessor: an alias for `Reg<LATCLR_SPEC>`"]
pub type LATCLR = crate::Reg<latclr::LATCLR_SPEC>;
#[doc = "LATBCLR register"]
pub mod latclr;
#[doc = "LATSET register accessor: an alias for `Reg<LATSET_SPEC>`"]
pub type LATSET = crate::Reg<latset::LATSET_SPEC>;
#[doc = "LATBSET register"]
pub mod latset;
#[doc = "LATINV register accessor: an alias for `Reg<LATINV_SPEC>`"]
pub type LATINV = crate::Reg<latinv::LATINV_SPEC>;
#[doc = "LATBINV register"]
pub mod latinv;
#[doc = "ODC register accessor: an alias for `Reg<ODC_SPEC>`"]
pub type ODC = crate::Reg<odc::ODC_SPEC>;
#[doc = "ODCB register"]
pub mod odc;
#[doc = "ODCCLR register accessor: an alias for `Reg<ODCCLR_SPEC>`"]
pub type ODCCLR = crate::Reg<odcclr::ODCCLR_SPEC>;
#[doc = "ODCBCLR register"]
pub mod odcclr;
#[doc = "ODCSET register accessor: an alias for `Reg<ODCSET_SPEC>`"]
pub type ODCSET = crate::Reg<odcset::ODCSET_SPEC>;
#[doc = "ODCBSET register"]
pub mod odcset;
#[doc = "ODCINV register accessor: an alias for `Reg<ODCINV_SPEC>`"]
pub type ODCINV = crate::Reg<odcinv::ODCINV_SPEC>;
#[doc = "ODCBINV register"]
pub mod odcinv;
#[doc = "CNPU register accessor: an alias for `Reg<CNPU_SPEC>`"]
pub type CNPU = crate::Reg<cnpu::CNPU_SPEC>;
#[doc = "CNPUB register"]
pub mod cnpu;
#[doc = "CNPUCLR register accessor: an alias for `Reg<CNPUCLR_SPEC>`"]
pub type CNPUCLR = crate::Reg<cnpuclr::CNPUCLR_SPEC>;
#[doc = "CNPUBCLR register"]
pub mod cnpuclr;
#[doc = "CNPUSET register accessor: an alias for `Reg<CNPUSET_SPEC>`"]
pub type CNPUSET = crate::Reg<cnpuset::CNPUSET_SPEC>;
#[doc = "CNPUBSET register"]
pub mod cnpuset;
#[doc = "CNPUINV register accessor: an alias for `Reg<CNPUINV_SPEC>`"]
pub type CNPUINV = crate::Reg<cnpuinv::CNPUINV_SPEC>;
#[doc = "CNPUBINV register"]
pub mod cnpuinv;
#[doc = "CNPD register accessor: an alias for `Reg<CNPD_SPEC>`"]
pub type CNPD = crate::Reg<cnpd::CNPD_SPEC>;
#[doc = "CNPDB register"]
pub mod cnpd;
#[doc = "CNPDCLR register accessor: an alias for `Reg<CNPDCLR_SPEC>`"]
pub type CNPDCLR = crate::Reg<cnpdclr::CNPDCLR_SPEC>;
#[doc = "CNPDBCLR register"]
pub mod cnpdclr;
#[doc = "CNPDSET register accessor: an alias for `Reg<CNPDSET_SPEC>`"]
pub type CNPDSET = crate::Reg<cnpdset::CNPDSET_SPEC>;
#[doc = "CNPDBSET register"]
pub mod cnpdset;
#[doc = "CNPDINV register accessor: an alias for `Reg<CNPDINV_SPEC>`"]
pub type CNPDINV = crate::Reg<cnpdinv::CNPDINV_SPEC>;
#[doc = "CNPDBINV register"]
pub mod cnpdinv;
#[doc = "CNCON register accessor: an alias for `Reg<CNCON_SPEC>`"]
pub type CNCON = crate::Reg<cncon::CNCON_SPEC>;
#[doc = "CNCONB register"]
pub mod cncon;
#[doc = "CNCONCLR register accessor: an alias for `Reg<CNCONCLR_SPEC>`"]
pub type CNCONCLR = crate::Reg<cnconclr::CNCONCLR_SPEC>;
#[doc = "CNCONBCLR register"]
pub mod cnconclr;
#[doc = "CNCONSET register accessor: an alias for `Reg<CNCONSET_SPEC>`"]
pub type CNCONSET = crate::Reg<cnconset::CNCONSET_SPEC>;
#[doc = "CNCONBSET register"]
pub mod cnconset;
#[doc = "CNCONINV register accessor: an alias for `Reg<CNCONINV_SPEC>`"]
pub type CNCONINV = crate::Reg<cnconinv::CNCONINV_SPEC>;
#[doc = "CNCONBINV register"]
pub mod cnconinv;
#[doc = "CNEN register accessor: an alias for `Reg<CNEN_SPEC>`"]
pub type CNEN = crate::Reg<cnen::CNEN_SPEC>;
#[doc = "CNENB register"]
pub mod cnen;
#[doc = "CNENCLR register accessor: an alias for `Reg<CNENCLR_SPEC>`"]
pub type CNENCLR = crate::Reg<cnenclr::CNENCLR_SPEC>;
#[doc = "CNENBCLR register"]
pub mod cnenclr;
#[doc = "CNENSET register accessor: an alias for `Reg<CNENSET_SPEC>`"]
pub type CNENSET = crate::Reg<cnenset::CNENSET_SPEC>;
#[doc = "CNENBSET register"]
pub mod cnenset;
#[doc = "CNENINV register accessor: an alias for `Reg<CNENINV_SPEC>`"]
pub type CNENINV = crate::Reg<cneninv::CNENINV_SPEC>;
#[doc = "CNENBINV register"]
pub mod cneninv;
#[doc = "CNSTAT register accessor: an alias for `Reg<CNSTAT_SPEC>`"]
pub type CNSTAT = crate::Reg<cnstat::CNSTAT_SPEC>;
#[doc = "CNSTATB register"]
pub mod cnstat;
#[doc = "CNSTATCLR register accessor: an alias for `Reg<CNSTATCLR_SPEC>`"]
pub type CNSTATCLR = crate::Reg<cnstatclr::CNSTATCLR_SPEC>;
#[doc = "CNSTATBCLR register"]
pub mod cnstatclr;
#[doc = "CNSTATSET register accessor: an alias for `Reg<CNSTATSET_SPEC>`"]
pub type CNSTATSET = crate::Reg<cnstatset::CNSTATSET_SPEC>;
#[doc = "CNSTATBSET register"]
pub mod cnstatset;
#[doc = "CNSTATINV register accessor: an alias for `Reg<CNSTATINV_SPEC>`"]
pub type CNSTATINV = crate::Reg<cnstatinv::CNSTATINV_SPEC>;
#[doc = "CNSTATBINV register"]
pub mod cnstatinv;
