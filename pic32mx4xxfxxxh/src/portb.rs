#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELB register"]
    pub anselb: ANSELB,
    #[doc = "0x04 - ANSELBCLR register"]
    pub anselbclr: ANSELBCLR,
    #[doc = "0x08 - ANSELBSET register"]
    pub anselbset: ANSELBSET,
    #[doc = "0x0c - ANSELBINV register"]
    pub anselbinv: ANSELBINV,
    #[doc = "0x10 - TRISB register"]
    pub trisb: TRISB,
    #[doc = "0x14 - TRISBCLR register"]
    pub trisbclr: TRISBCLR,
    #[doc = "0x18 - TRISBSET register"]
    pub trisbset: TRISBSET,
    #[doc = "0x1c - TRISBINV register"]
    pub trisbinv: TRISBINV,
    #[doc = "0x20 - PORTB register"]
    pub portb: PORTB,
    #[doc = "0x24 - PORTBCLR register"]
    pub portbclr: PORTBCLR,
    #[doc = "0x28 - PORTBSET register"]
    pub portbset: PORTBSET,
    #[doc = "0x2c - PORTBINV register"]
    pub portbinv: PORTBINV,
    #[doc = "0x30 - LATB register"]
    pub latb: LATB,
    #[doc = "0x34 - LATBCLR register"]
    pub latbclr: LATBCLR,
    #[doc = "0x38 - LATBSET register"]
    pub latbset: LATBSET,
    #[doc = "0x3c - LATBINV register"]
    pub latbinv: LATBINV,
    #[doc = "0x40 - ODCB register"]
    pub odcb: ODCB,
    #[doc = "0x44 - ODCBCLR register"]
    pub odcbclr: ODCBCLR,
    #[doc = "0x48 - ODCBSET register"]
    pub odcbset: ODCBSET,
    #[doc = "0x4c - ODCBINV register"]
    pub odcbinv: ODCBINV,
    #[doc = "0x50 - CNPUB register"]
    pub cnpub: CNPUB,
    #[doc = "0x54 - CNPUBCLR register"]
    pub cnpubclr: CNPUBCLR,
    #[doc = "0x58 - CNPUBSET register"]
    pub cnpubset: CNPUBSET,
    #[doc = "0x5c - CNPUBINV register"]
    pub cnpubinv: CNPUBINV,
    #[doc = "0x60 - CNPDB register"]
    pub cnpdb: CNPDB,
    #[doc = "0x64 - CNPDBCLR register"]
    pub cnpdbclr: CNPDBCLR,
    #[doc = "0x68 - CNPDBSET register"]
    pub cnpdbset: CNPDBSET,
    #[doc = "0x6c - CNPDBINV register"]
    pub cnpdbinv: CNPDBINV,
    #[doc = "0x70 - CNCONB register"]
    pub cnconb: CNCONB,
    #[doc = "0x74 - CNCONBCLR register"]
    pub cnconbclr: CNCONBCLR,
    #[doc = "0x78 - CNCONBSET register"]
    pub cnconbset: CNCONBSET,
    #[doc = "0x7c - CNCONBINV register"]
    pub cnconbinv: CNCONBINV,
    #[doc = "0x80 - CNENB register"]
    pub cnenb: CNENB,
    #[doc = "0x84 - CNENBCLR register"]
    pub cnenbclr: CNENBCLR,
    #[doc = "0x88 - CNENBSET register"]
    pub cnenbset: CNENBSET,
    #[doc = "0x8c - CNENBINV register"]
    pub cnenbinv: CNENBINV,
    #[doc = "0x90 - CNSTATB register"]
    pub cnstatb: CNSTATB,
    #[doc = "0x94 - CNSTATBCLR register"]
    pub cnstatbclr: CNSTATBCLR,
    #[doc = "0x98 - CNSTATBSET register"]
    pub cnstatbset: CNSTATBSET,
    #[doc = "0x9c - CNSTATBINV register"]
    pub cnstatbinv: CNSTATBINV,
}
#[doc = "ANSELB register"]
pub struct ANSELB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELB register"]
pub mod anselb;
#[doc = "ANSELBCLR register"]
pub struct ANSELBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELBCLR register"]
pub mod anselbclr;
#[doc = "ANSELBSET register"]
pub struct ANSELBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELBSET register"]
pub mod anselbset;
#[doc = "ANSELBINV register"]
pub struct ANSELBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELBINV register"]
pub mod anselbinv;
#[doc = "TRISB register"]
pub struct TRISB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISB register"]
pub mod trisb;
#[doc = "TRISBCLR register"]
pub struct TRISBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISBCLR register"]
pub mod trisbclr;
#[doc = "TRISBSET register"]
pub struct TRISBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISBSET register"]
pub mod trisbset;
#[doc = "TRISBINV register"]
pub struct TRISBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISBINV register"]
pub mod trisbinv;
#[doc = "PORTB register"]
pub struct PORTB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTB register"]
pub mod portb;
#[doc = "PORTBCLR register"]
pub struct PORTBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTBCLR register"]
pub mod portbclr;
#[doc = "PORTBSET register"]
pub struct PORTBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTBSET register"]
pub mod portbset;
#[doc = "PORTBINV register"]
pub struct PORTBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTBINV register"]
pub mod portbinv;
#[doc = "LATB register"]
pub struct LATB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATB register"]
pub mod latb;
#[doc = "LATBCLR register"]
pub struct LATBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATBCLR register"]
pub mod latbclr;
#[doc = "LATBSET register"]
pub struct LATBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATBSET register"]
pub mod latbset;
#[doc = "LATBINV register"]
pub struct LATBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATBINV register"]
pub mod latbinv;
#[doc = "ODCB register"]
pub struct ODCB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCB register"]
pub mod odcb;
#[doc = "ODCBCLR register"]
pub struct ODCBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCBCLR register"]
pub mod odcbclr;
#[doc = "ODCBSET register"]
pub struct ODCBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCBSET register"]
pub mod odcbset;
#[doc = "ODCBINV register"]
pub struct ODCBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCBINV register"]
pub mod odcbinv;
#[doc = "CNPUB register"]
pub struct CNPUB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUB register"]
pub mod cnpub;
#[doc = "CNPUBCLR register"]
pub struct CNPUBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUBCLR register"]
pub mod cnpubclr;
#[doc = "CNPUBSET register"]
pub struct CNPUBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUBSET register"]
pub mod cnpubset;
#[doc = "CNPUBINV register"]
pub struct CNPUBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUBINV register"]
pub mod cnpubinv;
#[doc = "CNPDB register"]
pub struct CNPDB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDB register"]
pub mod cnpdb;
#[doc = "CNPDBCLR register"]
pub struct CNPDBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDBCLR register"]
pub mod cnpdbclr;
#[doc = "CNPDBSET register"]
pub struct CNPDBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDBSET register"]
pub mod cnpdbset;
#[doc = "CNPDBINV register"]
pub struct CNPDBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDBINV register"]
pub mod cnpdbinv;
#[doc = "CNCONB register"]
pub struct CNCONB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONB register"]
pub mod cnconb;
#[doc = "CNCONBCLR register"]
pub struct CNCONBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONBCLR register"]
pub mod cnconbclr;
#[doc = "CNCONBSET register"]
pub struct CNCONBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONBSET register"]
pub mod cnconbset;
#[doc = "CNCONBINV register"]
pub struct CNCONBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONBINV register"]
pub mod cnconbinv;
#[doc = "CNENB register"]
pub struct CNENB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENB register"]
pub mod cnenb;
#[doc = "CNENBCLR register"]
pub struct CNENBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENBCLR register"]
pub mod cnenbclr;
#[doc = "CNENBSET register"]
pub struct CNENBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENBSET register"]
pub mod cnenbset;
#[doc = "CNENBINV register"]
pub struct CNENBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENBINV register"]
pub mod cnenbinv;
#[doc = "CNSTATB register"]
pub struct CNSTATB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATB register"]
pub mod cnstatb;
#[doc = "CNSTATBCLR register"]
pub struct CNSTATBCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATBCLR register"]
pub mod cnstatbclr;
#[doc = "CNSTATBSET register"]
pub struct CNSTATBSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATBSET register"]
pub mod cnstatbset;
#[doc = "CNSTATBINV register"]
pub struct CNSTATBINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATBINV register"]
pub mod cnstatbinv;
