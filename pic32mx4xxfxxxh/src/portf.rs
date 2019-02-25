#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELF register"]
    pub anself: ANSELF,
    #[doc = "0x04 - ANSELFCLR register"]
    pub anselfclr: ANSELFCLR,
    #[doc = "0x08 - ANSELFSET register"]
    pub anselfset: ANSELFSET,
    #[doc = "0x0c - ANSELFINV register"]
    pub anselfinv: ANSELFINV,
    #[doc = "0x10 - TRISF register"]
    pub trisf: TRISF,
    #[doc = "0x14 - TRISFCLR register"]
    pub trisfclr: TRISFCLR,
    #[doc = "0x18 - TRISFSET register"]
    pub trisfset: TRISFSET,
    #[doc = "0x1c - TRISFINV register"]
    pub trisfinv: TRISFINV,
    #[doc = "0x20 - PORTF register"]
    pub portf: PORTF,
    #[doc = "0x24 - PORTFCLR register"]
    pub portfclr: PORTFCLR,
    #[doc = "0x28 - PORTFSET register"]
    pub portfset: PORTFSET,
    #[doc = "0x2c - PORTFINV register"]
    pub portfinv: PORTFINV,
    #[doc = "0x30 - LATF register"]
    pub latf: LATF,
    #[doc = "0x34 - LATFCLR register"]
    pub latfclr: LATFCLR,
    #[doc = "0x38 - LATFSET register"]
    pub latfset: LATFSET,
    #[doc = "0x3c - LATFINV register"]
    pub latfinv: LATFINV,
    #[doc = "0x40 - ODCF register"]
    pub odcf: ODCF,
    #[doc = "0x44 - ODCFCLR register"]
    pub odcfclr: ODCFCLR,
    #[doc = "0x48 - ODCFSET register"]
    pub odcfset: ODCFSET,
    #[doc = "0x4c - ODCFINV register"]
    pub odcfinv: ODCFINV,
    #[doc = "0x50 - CNPUF register"]
    pub cnpuf: CNPUF,
    #[doc = "0x54 - CNPUFCLR register"]
    pub cnpufclr: CNPUFCLR,
    #[doc = "0x58 - CNPUFSET register"]
    pub cnpufset: CNPUFSET,
    #[doc = "0x5c - CNPUFINV register"]
    pub cnpufinv: CNPUFINV,
    #[doc = "0x60 - CNPDF register"]
    pub cnpdf: CNPDF,
    #[doc = "0x64 - CNPDFCLR register"]
    pub cnpdfclr: CNPDFCLR,
    #[doc = "0x68 - CNPDFSET register"]
    pub cnpdfset: CNPDFSET,
    #[doc = "0x6c - CNPDFINV register"]
    pub cnpdfinv: CNPDFINV,
    #[doc = "0x70 - CNCONF register"]
    pub cnconf: CNCONF,
    #[doc = "0x74 - CNCONFCLR register"]
    pub cnconfclr: CNCONFCLR,
    #[doc = "0x78 - CNCONFSET register"]
    pub cnconfset: CNCONFSET,
    #[doc = "0x7c - CNCONFINV register"]
    pub cnconfinv: CNCONFINV,
    #[doc = "0x80 - CNENF register"]
    pub cnenf: CNENF,
    #[doc = "0x84 - CNENFCLR register"]
    pub cnenfclr: CNENFCLR,
    #[doc = "0x88 - CNENFSET register"]
    pub cnenfset: CNENFSET,
    #[doc = "0x8c - CNENFINV register"]
    pub cnenfinv: CNENFINV,
    #[doc = "0x90 - CNSTATF register"]
    pub cnstatf: CNSTATF,
    #[doc = "0x94 - CNSTATFCLR register"]
    pub cnstatfclr: CNSTATFCLR,
    #[doc = "0x98 - CNSTATFSET register"]
    pub cnstatfset: CNSTATFSET,
    #[doc = "0x9c - CNSTATFINV register"]
    pub cnstatfinv: CNSTATFINV,
}
#[doc = "ANSELF register"]
pub struct ANSELF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELF register"]
pub mod anself;
#[doc = "ANSELFCLR register"]
pub struct ANSELFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELFCLR register"]
pub mod anselfclr;
#[doc = "ANSELFSET register"]
pub struct ANSELFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELFSET register"]
pub mod anselfset;
#[doc = "ANSELFINV register"]
pub struct ANSELFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELFINV register"]
pub mod anselfinv;
#[doc = "TRISF register"]
pub struct TRISF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISF register"]
pub mod trisf;
#[doc = "TRISFCLR register"]
pub struct TRISFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISFCLR register"]
pub mod trisfclr;
#[doc = "TRISFSET register"]
pub struct TRISFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISFSET register"]
pub mod trisfset;
#[doc = "TRISFINV register"]
pub struct TRISFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISFINV register"]
pub mod trisfinv;
#[doc = "PORTF register"]
pub struct PORTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTF register"]
pub mod portf;
#[doc = "PORTFCLR register"]
pub struct PORTFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTFCLR register"]
pub mod portfclr;
#[doc = "PORTFSET register"]
pub struct PORTFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTFSET register"]
pub mod portfset;
#[doc = "PORTFINV register"]
pub struct PORTFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTFINV register"]
pub mod portfinv;
#[doc = "LATF register"]
pub struct LATF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATF register"]
pub mod latf;
#[doc = "LATFCLR register"]
pub struct LATFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATFCLR register"]
pub mod latfclr;
#[doc = "LATFSET register"]
pub struct LATFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATFSET register"]
pub mod latfset;
#[doc = "LATFINV register"]
pub struct LATFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATFINV register"]
pub mod latfinv;
#[doc = "ODCF register"]
pub struct ODCF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCF register"]
pub mod odcf;
#[doc = "ODCFCLR register"]
pub struct ODCFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCFCLR register"]
pub mod odcfclr;
#[doc = "ODCFSET register"]
pub struct ODCFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCFSET register"]
pub mod odcfset;
#[doc = "ODCFINV register"]
pub struct ODCFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCFINV register"]
pub mod odcfinv;
#[doc = "CNPUF register"]
pub struct CNPUF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUF register"]
pub mod cnpuf;
#[doc = "CNPUFCLR register"]
pub struct CNPUFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUFCLR register"]
pub mod cnpufclr;
#[doc = "CNPUFSET register"]
pub struct CNPUFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUFSET register"]
pub mod cnpufset;
#[doc = "CNPUFINV register"]
pub struct CNPUFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUFINV register"]
pub mod cnpufinv;
#[doc = "CNPDF register"]
pub struct CNPDF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDF register"]
pub mod cnpdf;
#[doc = "CNPDFCLR register"]
pub struct CNPDFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDFCLR register"]
pub mod cnpdfclr;
#[doc = "CNPDFSET register"]
pub struct CNPDFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDFSET register"]
pub mod cnpdfset;
#[doc = "CNPDFINV register"]
pub struct CNPDFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDFINV register"]
pub mod cnpdfinv;
#[doc = "CNCONF register"]
pub struct CNCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONF register"]
pub mod cnconf;
#[doc = "CNCONFCLR register"]
pub struct CNCONFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONFCLR register"]
pub mod cnconfclr;
#[doc = "CNCONFSET register"]
pub struct CNCONFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONFSET register"]
pub mod cnconfset;
#[doc = "CNCONFINV register"]
pub struct CNCONFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONFINV register"]
pub mod cnconfinv;
#[doc = "CNENF register"]
pub struct CNENF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENF register"]
pub mod cnenf;
#[doc = "CNENFCLR register"]
pub struct CNENFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENFCLR register"]
pub mod cnenfclr;
#[doc = "CNENFSET register"]
pub struct CNENFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENFSET register"]
pub mod cnenfset;
#[doc = "CNENFINV register"]
pub struct CNENFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENFINV register"]
pub mod cnenfinv;
#[doc = "CNSTATF register"]
pub struct CNSTATF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATF register"]
pub mod cnstatf;
#[doc = "CNSTATFCLR register"]
pub struct CNSTATFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATFCLR register"]
pub mod cnstatfclr;
#[doc = "CNSTATFSET register"]
pub struct CNSTATFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATFSET register"]
pub mod cnstatfset;
#[doc = "CNSTATFINV register"]
pub struct CNSTATFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATFINV register"]
pub mod cnstatfinv;
