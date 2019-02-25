#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELD register"]
    pub anseld: ANSELD,
    #[doc = "0x04 - ANSELDCLR register"]
    pub anseldclr: ANSELDCLR,
    #[doc = "0x08 - ANSELDSET register"]
    pub anseldset: ANSELDSET,
    #[doc = "0x0c - ANSELDINV register"]
    pub anseldinv: ANSELDINV,
    #[doc = "0x10 - TRISD register"]
    pub trisd: TRISD,
    #[doc = "0x14 - TRISDCLR register"]
    pub trisdclr: TRISDCLR,
    #[doc = "0x18 - TRISDSET register"]
    pub trisdset: TRISDSET,
    #[doc = "0x1c - TRISDINV register"]
    pub trisdinv: TRISDINV,
    #[doc = "0x20 - PORTD register"]
    pub portd: PORTD,
    #[doc = "0x24 - PORTDCLR register"]
    pub portdclr: PORTDCLR,
    #[doc = "0x28 - PORTDSET register"]
    pub portdset: PORTDSET,
    #[doc = "0x2c - PORTDINV register"]
    pub portdinv: PORTDINV,
    #[doc = "0x30 - LATD register"]
    pub latd: LATD,
    #[doc = "0x34 - LATDCLR register"]
    pub latdclr: LATDCLR,
    #[doc = "0x38 - LATDSET register"]
    pub latdset: LATDSET,
    #[doc = "0x3c - LATDINV register"]
    pub latdinv: LATDINV,
    #[doc = "0x40 - ODCD register"]
    pub odcd: ODCD,
    #[doc = "0x44 - ODCDCLR register"]
    pub odcdclr: ODCDCLR,
    #[doc = "0x48 - ODCDSET register"]
    pub odcdset: ODCDSET,
    #[doc = "0x4c - ODCDINV register"]
    pub odcdinv: ODCDINV,
    #[doc = "0x50 - CNPUD register"]
    pub cnpud: CNPUD,
    #[doc = "0x54 - CNPUDCLR register"]
    pub cnpudclr: CNPUDCLR,
    #[doc = "0x58 - CNPUDSET register"]
    pub cnpudset: CNPUDSET,
    #[doc = "0x5c - CNPUDINV register"]
    pub cnpudinv: CNPUDINV,
    #[doc = "0x60 - CNPDD register"]
    pub cnpdd: CNPDD,
    #[doc = "0x64 - CNPDDCLR register"]
    pub cnpddclr: CNPDDCLR,
    #[doc = "0x68 - CNPDDSET register"]
    pub cnpddset: CNPDDSET,
    #[doc = "0x6c - CNPDDINV register"]
    pub cnpddinv: CNPDDINV,
    #[doc = "0x70 - CNCOND register"]
    pub cncond: CNCOND,
    #[doc = "0x74 - CNCONDCLR register"]
    pub cncondclr: CNCONDCLR,
    #[doc = "0x78 - CNCONDSET register"]
    pub cncondset: CNCONDSET,
    #[doc = "0x7c - CNCONDINV register"]
    pub cncondinv: CNCONDINV,
    #[doc = "0x80 - CNEND register"]
    pub cnend: CNEND,
    #[doc = "0x84 - CNENDCLR register"]
    pub cnendclr: CNENDCLR,
    #[doc = "0x88 - CNENDSET register"]
    pub cnendset: CNENDSET,
    #[doc = "0x8c - CNENDINV register"]
    pub cnendinv: CNENDINV,
    #[doc = "0x90 - CNSTATD register"]
    pub cnstatd: CNSTATD,
    #[doc = "0x94 - CNSTATDCLR register"]
    pub cnstatdclr: CNSTATDCLR,
    #[doc = "0x98 - CNSTATDSET register"]
    pub cnstatdset: CNSTATDSET,
    #[doc = "0x9c - CNSTATDINV register"]
    pub cnstatdinv: CNSTATDINV,
}
#[doc = "ANSELD register"]
pub struct ANSELD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELD register"]
pub mod anseld;
#[doc = "ANSELDCLR register"]
pub struct ANSELDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELDCLR register"]
pub mod anseldclr;
#[doc = "ANSELDSET register"]
pub struct ANSELDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELDSET register"]
pub mod anseldset;
#[doc = "ANSELDINV register"]
pub struct ANSELDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELDINV register"]
pub mod anseldinv;
#[doc = "TRISD register"]
pub struct TRISD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISD register"]
pub mod trisd;
#[doc = "TRISDCLR register"]
pub struct TRISDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISDCLR register"]
pub mod trisdclr;
#[doc = "TRISDSET register"]
pub struct TRISDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISDSET register"]
pub mod trisdset;
#[doc = "TRISDINV register"]
pub struct TRISDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISDINV register"]
pub mod trisdinv;
#[doc = "PORTD register"]
pub struct PORTD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTD register"]
pub mod portd;
#[doc = "PORTDCLR register"]
pub struct PORTDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTDCLR register"]
pub mod portdclr;
#[doc = "PORTDSET register"]
pub struct PORTDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTDSET register"]
pub mod portdset;
#[doc = "PORTDINV register"]
pub struct PORTDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTDINV register"]
pub mod portdinv;
#[doc = "LATD register"]
pub struct LATD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATD register"]
pub mod latd;
#[doc = "LATDCLR register"]
pub struct LATDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATDCLR register"]
pub mod latdclr;
#[doc = "LATDSET register"]
pub struct LATDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATDSET register"]
pub mod latdset;
#[doc = "LATDINV register"]
pub struct LATDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATDINV register"]
pub mod latdinv;
#[doc = "ODCD register"]
pub struct ODCD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCD register"]
pub mod odcd;
#[doc = "ODCDCLR register"]
pub struct ODCDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCDCLR register"]
pub mod odcdclr;
#[doc = "ODCDSET register"]
pub struct ODCDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCDSET register"]
pub mod odcdset;
#[doc = "ODCDINV register"]
pub struct ODCDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCDINV register"]
pub mod odcdinv;
#[doc = "CNPUD register"]
pub struct CNPUD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUD register"]
pub mod cnpud;
#[doc = "CNPUDCLR register"]
pub struct CNPUDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUDCLR register"]
pub mod cnpudclr;
#[doc = "CNPUDSET register"]
pub struct CNPUDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUDSET register"]
pub mod cnpudset;
#[doc = "CNPUDINV register"]
pub struct CNPUDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUDINV register"]
pub mod cnpudinv;
#[doc = "CNPDD register"]
pub struct CNPDD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDD register"]
pub mod cnpdd;
#[doc = "CNPDDCLR register"]
pub struct CNPDDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDDCLR register"]
pub mod cnpddclr;
#[doc = "CNPDDSET register"]
pub struct CNPDDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDDSET register"]
pub mod cnpddset;
#[doc = "CNPDDINV register"]
pub struct CNPDDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDDINV register"]
pub mod cnpddinv;
#[doc = "CNCOND register"]
pub struct CNCOND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCOND register"]
pub mod cncond;
#[doc = "CNCONDCLR register"]
pub struct CNCONDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONDCLR register"]
pub mod cncondclr;
#[doc = "CNCONDSET register"]
pub struct CNCONDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONDSET register"]
pub mod cncondset;
#[doc = "CNCONDINV register"]
pub struct CNCONDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONDINV register"]
pub mod cncondinv;
#[doc = "CNEND register"]
pub struct CNEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNEND register"]
pub mod cnend;
#[doc = "CNENDCLR register"]
pub struct CNENDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENDCLR register"]
pub mod cnendclr;
#[doc = "CNENDSET register"]
pub struct CNENDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENDSET register"]
pub mod cnendset;
#[doc = "CNENDINV register"]
pub struct CNENDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENDINV register"]
pub mod cnendinv;
#[doc = "CNSTATD register"]
pub struct CNSTATD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATD register"]
pub mod cnstatd;
#[doc = "CNSTATDCLR register"]
pub struct CNSTATDCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATDCLR register"]
pub mod cnstatdclr;
#[doc = "CNSTATDSET register"]
pub struct CNSTATDSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATDSET register"]
pub mod cnstatdset;
#[doc = "CNSTATDINV register"]
pub struct CNSTATDINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATDINV register"]
pub mod cnstatdinv;
