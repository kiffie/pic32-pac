#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELA register"]
    pub ansel: crate::Reg<ansel::ANSEL_SPEC>,
    #[doc = "0x04 - ANSELACLR register"]
    pub anselclr: crate::Reg<anselclr::ANSELCLR_SPEC>,
    #[doc = "0x08 - ANSELASET register"]
    pub anselset: crate::Reg<anselset::ANSELSET_SPEC>,
    #[doc = "0x0c - ANSELAINV register"]
    pub anselinv: crate::Reg<anselinv::ANSELINV_SPEC>,
    #[doc = "0x10 - TRISA register"]
    pub tris: crate::Reg<tris::TRIS_SPEC>,
    #[doc = "0x14 - TRISACLR register"]
    pub trisclr: crate::Reg<trisclr::TRISCLR_SPEC>,
    #[doc = "0x18 - TRISASET register"]
    pub trisset: crate::Reg<trisset::TRISSET_SPEC>,
    #[doc = "0x1c - TRISAINV register"]
    pub trisinv: crate::Reg<trisinv::TRISINV_SPEC>,
    #[doc = "0x20 - PORTA register"]
    pub port: crate::Reg<port::PORT_SPEC>,
    #[doc = "0x24 - PORTACLR register"]
    pub portclr: crate::Reg<portclr::PORTCLR_SPEC>,
    #[doc = "0x28 - PORTASET register"]
    pub portset: crate::Reg<portset::PORTSET_SPEC>,
    #[doc = "0x2c - PORTAINV register"]
    pub portinv: crate::Reg<portinv::PORTINV_SPEC>,
    #[doc = "0x30 - LATA register"]
    pub lat: crate::Reg<lat::LAT_SPEC>,
    #[doc = "0x34 - LATACLR register"]
    pub latclr: crate::Reg<latclr::LATCLR_SPEC>,
    #[doc = "0x38 - LATASET register"]
    pub latset: crate::Reg<latset::LATSET_SPEC>,
    #[doc = "0x3c - LATAINV register"]
    pub latinv: crate::Reg<latinv::LATINV_SPEC>,
    #[doc = "0x40 - ODCA register"]
    pub odc: crate::Reg<odc::ODC_SPEC>,
    #[doc = "0x44 - ODCACLR register"]
    pub odcclr: crate::Reg<odcclr::ODCCLR_SPEC>,
    #[doc = "0x48 - ODCASET register"]
    pub odcset: crate::Reg<odcset::ODCSET_SPEC>,
    #[doc = "0x4c - ODCAINV register"]
    pub odcinv: crate::Reg<odcinv::ODCINV_SPEC>,
    #[doc = "0x50 - CNPUA register"]
    pub cnpu: crate::Reg<cnpu::CNPU_SPEC>,
    #[doc = "0x54 - CNPUACLR register"]
    pub cnpuclr: crate::Reg<cnpuclr::CNPUCLR_SPEC>,
    #[doc = "0x58 - CNPUASET register"]
    pub cnpuset: crate::Reg<cnpuset::CNPUSET_SPEC>,
    #[doc = "0x5c - CNPUAINV register"]
    pub cnpuinv: crate::Reg<cnpuinv::CNPUINV_SPEC>,
    #[doc = "0x60 - CNPDA register"]
    pub cnpd: crate::Reg<cnpd::CNPD_SPEC>,
    #[doc = "0x64 - CNPDACLR register"]
    pub cnpdclr: crate::Reg<cnpdclr::CNPDCLR_SPEC>,
    #[doc = "0x68 - CNPDASET register"]
    pub cnpdset: crate::Reg<cnpdset::CNPDSET_SPEC>,
    #[doc = "0x6c - CNPDAINV register"]
    pub cnpdinv: crate::Reg<cnpdinv::CNPDINV_SPEC>,
    #[doc = "0x70 - CNCONA register"]
    pub cncon: crate::Reg<cncon::CNCON_SPEC>,
    #[doc = "0x74 - CNCONACLR register"]
    pub cnconclr: crate::Reg<cnconclr::CNCONCLR_SPEC>,
    #[doc = "0x78 - CNCONASET register"]
    pub cnconset: crate::Reg<cnconset::CNCONSET_SPEC>,
    #[doc = "0x7c - CNCONAINV register"]
    pub cnconinv: crate::Reg<cnconinv::CNCONINV_SPEC>,
    #[doc = "0x80 - CNENA register"]
    pub cnen: crate::Reg<cnen::CNEN_SPEC>,
    #[doc = "0x84 - CNENACLR register"]
    pub cnenclr: crate::Reg<cnenclr::CNENCLR_SPEC>,
    #[doc = "0x88 - CNENASET register"]
    pub cnenset: crate::Reg<cnenset::CNENSET_SPEC>,
    #[doc = "0x8c - CNENAINV register"]
    pub cneninv: crate::Reg<cneninv::CNENINV_SPEC>,
    #[doc = "0x90 - CNSTATA register"]
    pub cnstat: crate::Reg<cnstat::CNSTAT_SPEC>,
    #[doc = "0x94 - CNSTATACLR register"]
    pub cnstatclr: crate::Reg<cnstatclr::CNSTATCLR_SPEC>,
    #[doc = "0x98 - CNSTATASET register"]
    pub cnstatset: crate::Reg<cnstatset::CNSTATSET_SPEC>,
    #[doc = "0x9c - CNSTATAINV register"]
    pub cnstatinv: crate::Reg<cnstatinv::CNSTATINV_SPEC>,
}
#[doc = "ANSEL register accessor: an alias for `Reg<ANSEL_SPEC>`"]
pub type ANSEL = crate::Reg<ansel::ANSEL_SPEC>;
#[doc = "ANSELA register"]
pub mod ansel;
#[doc = "ANSELCLR register accessor: an alias for `Reg<ANSELCLR_SPEC>`"]
pub type ANSELCLR = crate::Reg<anselclr::ANSELCLR_SPEC>;
#[doc = "ANSELACLR register"]
pub mod anselclr;
#[doc = "ANSELSET register accessor: an alias for `Reg<ANSELSET_SPEC>`"]
pub type ANSELSET = crate::Reg<anselset::ANSELSET_SPEC>;
#[doc = "ANSELASET register"]
pub mod anselset;
#[doc = "ANSELINV register accessor: an alias for `Reg<ANSELINV_SPEC>`"]
pub type ANSELINV = crate::Reg<anselinv::ANSELINV_SPEC>;
#[doc = "ANSELAINV register"]
pub mod anselinv;
#[doc = "TRIS register accessor: an alias for `Reg<TRIS_SPEC>`"]
pub type TRIS = crate::Reg<tris::TRIS_SPEC>;
#[doc = "TRISA register"]
pub mod tris;
#[doc = "TRISCLR register accessor: an alias for `Reg<TRISCLR_SPEC>`"]
pub type TRISCLR = crate::Reg<trisclr::TRISCLR_SPEC>;
#[doc = "TRISACLR register"]
pub mod trisclr;
#[doc = "TRISSET register accessor: an alias for `Reg<TRISSET_SPEC>`"]
pub type TRISSET = crate::Reg<trisset::TRISSET_SPEC>;
#[doc = "TRISASET register"]
pub mod trisset;
#[doc = "TRISINV register accessor: an alias for `Reg<TRISINV_SPEC>`"]
pub type TRISINV = crate::Reg<trisinv::TRISINV_SPEC>;
#[doc = "TRISAINV register"]
pub mod trisinv;
#[doc = "PORT register accessor: an alias for `Reg<PORT_SPEC>`"]
pub type PORT = crate::Reg<port::PORT_SPEC>;
#[doc = "PORTA register"]
pub mod port;
#[doc = "PORTCLR register accessor: an alias for `Reg<PORTCLR_SPEC>`"]
pub type PORTCLR = crate::Reg<portclr::PORTCLR_SPEC>;
#[doc = "PORTACLR register"]
pub mod portclr;
#[doc = "PORTSET register accessor: an alias for `Reg<PORTSET_SPEC>`"]
pub type PORTSET = crate::Reg<portset::PORTSET_SPEC>;
#[doc = "PORTASET register"]
pub mod portset;
#[doc = "PORTINV register accessor: an alias for `Reg<PORTINV_SPEC>`"]
pub type PORTINV = crate::Reg<portinv::PORTINV_SPEC>;
#[doc = "PORTAINV register"]
pub mod portinv;
#[doc = "LAT register accessor: an alias for `Reg<LAT_SPEC>`"]
pub type LAT = crate::Reg<lat::LAT_SPEC>;
#[doc = "LATA register"]
pub mod lat;
#[doc = "LATCLR register accessor: an alias for `Reg<LATCLR_SPEC>`"]
pub type LATCLR = crate::Reg<latclr::LATCLR_SPEC>;
#[doc = "LATACLR register"]
pub mod latclr;
#[doc = "LATSET register accessor: an alias for `Reg<LATSET_SPEC>`"]
pub type LATSET = crate::Reg<latset::LATSET_SPEC>;
#[doc = "LATASET register"]
pub mod latset;
#[doc = "LATINV register accessor: an alias for `Reg<LATINV_SPEC>`"]
pub type LATINV = crate::Reg<latinv::LATINV_SPEC>;
#[doc = "LATAINV register"]
pub mod latinv;
#[doc = "ODC register accessor: an alias for `Reg<ODC_SPEC>`"]
pub type ODC = crate::Reg<odc::ODC_SPEC>;
#[doc = "ODCA register"]
pub mod odc;
#[doc = "ODCCLR register accessor: an alias for `Reg<ODCCLR_SPEC>`"]
pub type ODCCLR = crate::Reg<odcclr::ODCCLR_SPEC>;
#[doc = "ODCACLR register"]
pub mod odcclr;
#[doc = "ODCSET register accessor: an alias for `Reg<ODCSET_SPEC>`"]
pub type ODCSET = crate::Reg<odcset::ODCSET_SPEC>;
#[doc = "ODCASET register"]
pub mod odcset;
#[doc = "ODCINV register accessor: an alias for `Reg<ODCINV_SPEC>`"]
pub type ODCINV = crate::Reg<odcinv::ODCINV_SPEC>;
#[doc = "ODCAINV register"]
pub mod odcinv;
#[doc = "CNPU register accessor: an alias for `Reg<CNPU_SPEC>`"]
pub type CNPU = crate::Reg<cnpu::CNPU_SPEC>;
#[doc = "CNPUA register"]
pub mod cnpu;
#[doc = "CNPUCLR register accessor: an alias for `Reg<CNPUCLR_SPEC>`"]
pub type CNPUCLR = crate::Reg<cnpuclr::CNPUCLR_SPEC>;
#[doc = "CNPUACLR register"]
pub mod cnpuclr;
#[doc = "CNPUSET register accessor: an alias for `Reg<CNPUSET_SPEC>`"]
pub type CNPUSET = crate::Reg<cnpuset::CNPUSET_SPEC>;
#[doc = "CNPUASET register"]
pub mod cnpuset;
#[doc = "CNPUINV register accessor: an alias for `Reg<CNPUINV_SPEC>`"]
pub type CNPUINV = crate::Reg<cnpuinv::CNPUINV_SPEC>;
#[doc = "CNPUAINV register"]
pub mod cnpuinv;
#[doc = "CNPD register accessor: an alias for `Reg<CNPD_SPEC>`"]
pub type CNPD = crate::Reg<cnpd::CNPD_SPEC>;
#[doc = "CNPDA register"]
pub mod cnpd;
#[doc = "CNPDCLR register accessor: an alias for `Reg<CNPDCLR_SPEC>`"]
pub type CNPDCLR = crate::Reg<cnpdclr::CNPDCLR_SPEC>;
#[doc = "CNPDACLR register"]
pub mod cnpdclr;
#[doc = "CNPDSET register accessor: an alias for `Reg<CNPDSET_SPEC>`"]
pub type CNPDSET = crate::Reg<cnpdset::CNPDSET_SPEC>;
#[doc = "CNPDASET register"]
pub mod cnpdset;
#[doc = "CNPDINV register accessor: an alias for `Reg<CNPDINV_SPEC>`"]
pub type CNPDINV = crate::Reg<cnpdinv::CNPDINV_SPEC>;
#[doc = "CNPDAINV register"]
pub mod cnpdinv;
#[doc = "CNCON register accessor: an alias for `Reg<CNCON_SPEC>`"]
pub type CNCON = crate::Reg<cncon::CNCON_SPEC>;
#[doc = "CNCONA register"]
pub mod cncon;
#[doc = "CNCONCLR register accessor: an alias for `Reg<CNCONCLR_SPEC>`"]
pub type CNCONCLR = crate::Reg<cnconclr::CNCONCLR_SPEC>;
#[doc = "CNCONACLR register"]
pub mod cnconclr;
#[doc = "CNCONSET register accessor: an alias for `Reg<CNCONSET_SPEC>`"]
pub type CNCONSET = crate::Reg<cnconset::CNCONSET_SPEC>;
#[doc = "CNCONASET register"]
pub mod cnconset;
#[doc = "CNCONINV register accessor: an alias for `Reg<CNCONINV_SPEC>`"]
pub type CNCONINV = crate::Reg<cnconinv::CNCONINV_SPEC>;
#[doc = "CNCONAINV register"]
pub mod cnconinv;
#[doc = "CNEN register accessor: an alias for `Reg<CNEN_SPEC>`"]
pub type CNEN = crate::Reg<cnen::CNEN_SPEC>;
#[doc = "CNENA register"]
pub mod cnen;
#[doc = "CNENCLR register accessor: an alias for `Reg<CNENCLR_SPEC>`"]
pub type CNENCLR = crate::Reg<cnenclr::CNENCLR_SPEC>;
#[doc = "CNENACLR register"]
pub mod cnenclr;
#[doc = "CNENSET register accessor: an alias for `Reg<CNENSET_SPEC>`"]
pub type CNENSET = crate::Reg<cnenset::CNENSET_SPEC>;
#[doc = "CNENASET register"]
pub mod cnenset;
#[doc = "CNENINV register accessor: an alias for `Reg<CNENINV_SPEC>`"]
pub type CNENINV = crate::Reg<cneninv::CNENINV_SPEC>;
#[doc = "CNENAINV register"]
pub mod cneninv;
#[doc = "CNSTAT register accessor: an alias for `Reg<CNSTAT_SPEC>`"]
pub type CNSTAT = crate::Reg<cnstat::CNSTAT_SPEC>;
#[doc = "CNSTATA register"]
pub mod cnstat;
#[doc = "CNSTATCLR register accessor: an alias for `Reg<CNSTATCLR_SPEC>`"]
pub type CNSTATCLR = crate::Reg<cnstatclr::CNSTATCLR_SPEC>;
#[doc = "CNSTATACLR register"]
pub mod cnstatclr;
#[doc = "CNSTATSET register accessor: an alias for `Reg<CNSTATSET_SPEC>`"]
pub type CNSTATSET = crate::Reg<cnstatset::CNSTATSET_SPEC>;
#[doc = "CNSTATASET register"]
pub mod cnstatset;
#[doc = "CNSTATINV register accessor: an alias for `Reg<CNSTATINV_SPEC>`"]
pub type CNSTATINV = crate::Reg<cnstatinv::CNSTATINV_SPEC>;
#[doc = "CNSTATAINV register"]
pub mod cnstatinv;
