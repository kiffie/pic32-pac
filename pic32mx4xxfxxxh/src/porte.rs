#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELE register"]
    pub ansele: ANSELE,
    #[doc = "0x04 - ANSELECLR register"]
    pub anseleclr: ANSELECLR,
    #[doc = "0x08 - ANSELESET register"]
    pub anseleset: ANSELESET,
    #[doc = "0x0c - ANSELEINV register"]
    pub anseleinv: ANSELEINV,
    #[doc = "0x10 - TRISE register"]
    pub trise: TRISE,
    #[doc = "0x14 - TRISECLR register"]
    pub triseclr: TRISECLR,
    #[doc = "0x18 - TRISESET register"]
    pub triseset: TRISESET,
    #[doc = "0x1c - TRISEINV register"]
    pub triseinv: TRISEINV,
    #[doc = "0x20 - PORTE register"]
    pub porte: PORTE,
    #[doc = "0x24 - PORTECLR register"]
    pub porteclr: PORTECLR,
    #[doc = "0x28 - PORTESET register"]
    pub porteset: PORTESET,
    #[doc = "0x2c - PORTEINV register"]
    pub porteinv: PORTEINV,
    #[doc = "0x30 - LATE register"]
    pub late: LATE,
    #[doc = "0x34 - LATECLR register"]
    pub lateclr: LATECLR,
    #[doc = "0x38 - LATESET register"]
    pub lateset: LATESET,
    #[doc = "0x3c - LATEINV register"]
    pub lateinv: LATEINV,
    #[doc = "0x40 - ODCE register"]
    pub odce: ODCE,
    #[doc = "0x44 - ODCECLR register"]
    pub odceclr: ODCECLR,
    #[doc = "0x48 - ODCESET register"]
    pub odceset: ODCESET,
    #[doc = "0x4c - ODCEINV register"]
    pub odceinv: ODCEINV,
    #[doc = "0x50 - CNPUE register"]
    pub cnpue: CNPUE,
    #[doc = "0x54 - CNPUECLR register"]
    pub cnpueclr: CNPUECLR,
    #[doc = "0x58 - CNPUESET register"]
    pub cnpueset: CNPUESET,
    #[doc = "0x5c - CNPUEINV register"]
    pub cnpueinv: CNPUEINV,
    #[doc = "0x60 - CNPDE register"]
    pub cnpde: CNPDE,
    #[doc = "0x64 - CNPDECLR register"]
    pub cnpdeclr: CNPDECLR,
    #[doc = "0x68 - CNPDESET register"]
    pub cnpdeset: CNPDESET,
    #[doc = "0x6c - CNPDEINV register"]
    pub cnpdeinv: CNPDEINV,
    #[doc = "0x70 - CNCONE register"]
    pub cncone: CNCONE,
    #[doc = "0x74 - CNCONECLR register"]
    pub cnconeclr: CNCONECLR,
    #[doc = "0x78 - CNCONESET register"]
    pub cnconeset: CNCONESET,
    #[doc = "0x7c - CNCONEINV register"]
    pub cnconeinv: CNCONEINV,
    #[doc = "0x80 - CNENE register"]
    pub cnene: CNENE,
    #[doc = "0x84 - CNENECLR register"]
    pub cneneclr: CNENECLR,
    #[doc = "0x88 - CNENESET register"]
    pub cneneset: CNENESET,
    #[doc = "0x8c - CNENEINV register"]
    pub cneneinv: CNENEINV,
    #[doc = "0x90 - CNSTATE register"]
    pub cnstate: CNSTATE,
    #[doc = "0x94 - CNSTATECLR register"]
    pub cnstateclr: CNSTATECLR,
    #[doc = "0x98 - CNSTATESET register"]
    pub cnstateset: CNSTATESET,
    #[doc = "0x9c - CNSTATEINV register"]
    pub cnstateinv: CNSTATEINV,
}
#[doc = "ANSELE register"]
pub struct ANSELE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELE register"]
pub mod ansele;
#[doc = "ANSELECLR register"]
pub struct ANSELECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELECLR register"]
pub mod anseleclr;
#[doc = "ANSELESET register"]
pub struct ANSELESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELESET register"]
pub mod anseleset;
#[doc = "ANSELEINV register"]
pub struct ANSELEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELEINV register"]
pub mod anseleinv;
#[doc = "TRISE register"]
pub struct TRISE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISE register"]
pub mod trise;
#[doc = "TRISECLR register"]
pub struct TRISECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISECLR register"]
pub mod triseclr;
#[doc = "TRISESET register"]
pub struct TRISESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISESET register"]
pub mod triseset;
#[doc = "TRISEINV register"]
pub struct TRISEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISEINV register"]
pub mod triseinv;
#[doc = "PORTE register"]
pub struct PORTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTE register"]
pub mod porte;
#[doc = "PORTECLR register"]
pub struct PORTECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTECLR register"]
pub mod porteclr;
#[doc = "PORTESET register"]
pub struct PORTESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTESET register"]
pub mod porteset;
#[doc = "PORTEINV register"]
pub struct PORTEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTEINV register"]
pub mod porteinv;
#[doc = "LATE register"]
pub struct LATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATE register"]
pub mod late;
#[doc = "LATECLR register"]
pub struct LATECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATECLR register"]
pub mod lateclr;
#[doc = "LATESET register"]
pub struct LATESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATESET register"]
pub mod lateset;
#[doc = "LATEINV register"]
pub struct LATEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATEINV register"]
pub mod lateinv;
#[doc = "ODCE register"]
pub struct ODCE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCE register"]
pub mod odce;
#[doc = "ODCECLR register"]
pub struct ODCECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCECLR register"]
pub mod odceclr;
#[doc = "ODCESET register"]
pub struct ODCESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCESET register"]
pub mod odceset;
#[doc = "ODCEINV register"]
pub struct ODCEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCEINV register"]
pub mod odceinv;
#[doc = "CNPUE register"]
pub struct CNPUE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUE register"]
pub mod cnpue;
#[doc = "CNPUECLR register"]
pub struct CNPUECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUECLR register"]
pub mod cnpueclr;
#[doc = "CNPUESET register"]
pub struct CNPUESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUESET register"]
pub mod cnpueset;
#[doc = "CNPUEINV register"]
pub struct CNPUEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUEINV register"]
pub mod cnpueinv;
#[doc = "CNPDE register"]
pub struct CNPDE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDE register"]
pub mod cnpde;
#[doc = "CNPDECLR register"]
pub struct CNPDECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDECLR register"]
pub mod cnpdeclr;
#[doc = "CNPDESET register"]
pub struct CNPDESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDESET register"]
pub mod cnpdeset;
#[doc = "CNPDEINV register"]
pub struct CNPDEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDEINV register"]
pub mod cnpdeinv;
#[doc = "CNCONE register"]
pub struct CNCONE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONE register"]
pub mod cncone;
#[doc = "CNCONECLR register"]
pub struct CNCONECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONECLR register"]
pub mod cnconeclr;
#[doc = "CNCONESET register"]
pub struct CNCONESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONESET register"]
pub mod cnconeset;
#[doc = "CNCONEINV register"]
pub struct CNCONEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONEINV register"]
pub mod cnconeinv;
#[doc = "CNENE register"]
pub struct CNENE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENE register"]
pub mod cnene;
#[doc = "CNENECLR register"]
pub struct CNENECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENECLR register"]
pub mod cneneclr;
#[doc = "CNENESET register"]
pub struct CNENESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENESET register"]
pub mod cneneset;
#[doc = "CNENEINV register"]
pub struct CNENEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENEINV register"]
pub mod cneneinv;
#[doc = "CNSTATE register"]
pub struct CNSTATE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATE register"]
pub mod cnstate;
#[doc = "CNSTATECLR register"]
pub struct CNSTATECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATECLR register"]
pub mod cnstateclr;
#[doc = "CNSTATESET register"]
pub struct CNSTATESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATESET register"]
pub mod cnstateset;
#[doc = "CNSTATEINV register"]
pub struct CNSTATEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATEINV register"]
pub mod cnstateinv;
