#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELG register"]
    pub anselg: ANSELG,
    #[doc = "0x04 - ANSELGCLR register"]
    pub anselgclr: ANSELGCLR,
    #[doc = "0x08 - ANSELGSET register"]
    pub anselgset: ANSELGSET,
    #[doc = "0x0c - ANSELGINV register"]
    pub anselginv: ANSELGINV,
    #[doc = "0x10 - TRISG register"]
    pub trisg: TRISG,
    #[doc = "0x14 - TRISGCLR register"]
    pub trisgclr: TRISGCLR,
    #[doc = "0x18 - TRISGSET register"]
    pub trisgset: TRISGSET,
    #[doc = "0x1c - TRISGINV register"]
    pub trisginv: TRISGINV,
    #[doc = "0x20 - PORTG register"]
    pub portg: PORTG,
    #[doc = "0x24 - PORTGCLR register"]
    pub portgclr: PORTGCLR,
    #[doc = "0x28 - PORTGSET register"]
    pub portgset: PORTGSET,
    #[doc = "0x2c - PORTGINV register"]
    pub portginv: PORTGINV,
    #[doc = "0x30 - LATG register"]
    pub latg: LATG,
    #[doc = "0x34 - LATGCLR register"]
    pub latgclr: LATGCLR,
    #[doc = "0x38 - LATGSET register"]
    pub latgset: LATGSET,
    #[doc = "0x3c - LATGINV register"]
    pub latginv: LATGINV,
    #[doc = "0x40 - ODCG register"]
    pub odcg: ODCG,
    #[doc = "0x44 - ODCGCLR register"]
    pub odcgclr: ODCGCLR,
    #[doc = "0x48 - ODCGSET register"]
    pub odcgset: ODCGSET,
    #[doc = "0x4c - ODCGINV register"]
    pub odcginv: ODCGINV,
    #[doc = "0x50 - CNPUG register"]
    pub cnpug: CNPUG,
    #[doc = "0x54 - CNPUGCLR register"]
    pub cnpugclr: CNPUGCLR,
    #[doc = "0x58 - CNPUGSET register"]
    pub cnpugset: CNPUGSET,
    #[doc = "0x5c - CNPUGINV register"]
    pub cnpuginv: CNPUGINV,
    #[doc = "0x60 - CNPDG register"]
    pub cnpdg: CNPDG,
    #[doc = "0x64 - CNPDGCLR register"]
    pub cnpdgclr: CNPDGCLR,
    #[doc = "0x68 - CNPDGSET register"]
    pub cnpdgset: CNPDGSET,
    #[doc = "0x6c - CNPDGINV register"]
    pub cnpdginv: CNPDGINV,
    #[doc = "0x70 - CNCONG register"]
    pub cncong: CNCONG,
    #[doc = "0x74 - CNCONGCLR register"]
    pub cncongclr: CNCONGCLR,
    #[doc = "0x78 - CNCONGSET register"]
    pub cncongset: CNCONGSET,
    #[doc = "0x7c - CNCONGINV register"]
    pub cnconginv: CNCONGINV,
    #[doc = "0x80 - CNENG register"]
    pub cneng: CNENG,
    #[doc = "0x84 - CNENGCLR register"]
    pub cnengclr: CNENGCLR,
    #[doc = "0x88 - CNENGSET register"]
    pub cnengset: CNENGSET,
    #[doc = "0x8c - CNENGINV register"]
    pub cnenginv: CNENGINV,
    #[doc = "0x90 - CNSTATG register"]
    pub cnstatg: CNSTATG,
    #[doc = "0x94 - CNSTATGCLR register"]
    pub cnstatgclr: CNSTATGCLR,
    #[doc = "0x98 - CNSTATGSET register"]
    pub cnstatgset: CNSTATGSET,
    #[doc = "0x9c - CNSTATGINV register"]
    pub cnstatginv: CNSTATGINV,
}
#[doc = "ANSELG register"]
pub struct ANSELG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELG register"]
pub mod anselg;
#[doc = "ANSELGCLR register"]
pub struct ANSELGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELGCLR register"]
pub mod anselgclr;
#[doc = "ANSELGSET register"]
pub struct ANSELGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELGSET register"]
pub mod anselgset;
#[doc = "ANSELGINV register"]
pub struct ANSELGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELGINV register"]
pub mod anselginv;
#[doc = "TRISG register"]
pub struct TRISG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISG register"]
pub mod trisg;
#[doc = "TRISGCLR register"]
pub struct TRISGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISGCLR register"]
pub mod trisgclr;
#[doc = "TRISGSET register"]
pub struct TRISGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISGSET register"]
pub mod trisgset;
#[doc = "TRISGINV register"]
pub struct TRISGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISGINV register"]
pub mod trisginv;
#[doc = "PORTG register"]
pub struct PORTG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTG register"]
pub mod portg;
#[doc = "PORTGCLR register"]
pub struct PORTGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTGCLR register"]
pub mod portgclr;
#[doc = "PORTGSET register"]
pub struct PORTGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTGSET register"]
pub mod portgset;
#[doc = "PORTGINV register"]
pub struct PORTGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTGINV register"]
pub mod portginv;
#[doc = "LATG register"]
pub struct LATG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATG register"]
pub mod latg;
#[doc = "LATGCLR register"]
pub struct LATGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATGCLR register"]
pub mod latgclr;
#[doc = "LATGSET register"]
pub struct LATGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATGSET register"]
pub mod latgset;
#[doc = "LATGINV register"]
pub struct LATGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATGINV register"]
pub mod latginv;
#[doc = "ODCG register"]
pub struct ODCG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCG register"]
pub mod odcg;
#[doc = "ODCGCLR register"]
pub struct ODCGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCGCLR register"]
pub mod odcgclr;
#[doc = "ODCGSET register"]
pub struct ODCGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCGSET register"]
pub mod odcgset;
#[doc = "ODCGINV register"]
pub struct ODCGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCGINV register"]
pub mod odcginv;
#[doc = "CNPUG register"]
pub struct CNPUG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUG register"]
pub mod cnpug;
#[doc = "CNPUGCLR register"]
pub struct CNPUGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUGCLR register"]
pub mod cnpugclr;
#[doc = "CNPUGSET register"]
pub struct CNPUGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUGSET register"]
pub mod cnpugset;
#[doc = "CNPUGINV register"]
pub struct CNPUGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUGINV register"]
pub mod cnpuginv;
#[doc = "CNPDG register"]
pub struct CNPDG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDG register"]
pub mod cnpdg;
#[doc = "CNPDGCLR register"]
pub struct CNPDGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDGCLR register"]
pub mod cnpdgclr;
#[doc = "CNPDGSET register"]
pub struct CNPDGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDGSET register"]
pub mod cnpdgset;
#[doc = "CNPDGINV register"]
pub struct CNPDGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDGINV register"]
pub mod cnpdginv;
#[doc = "CNCONG register"]
pub struct CNCONG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONG register"]
pub mod cncong;
#[doc = "CNCONGCLR register"]
pub struct CNCONGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONGCLR register"]
pub mod cncongclr;
#[doc = "CNCONGSET register"]
pub struct CNCONGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONGSET register"]
pub mod cncongset;
#[doc = "CNCONGINV register"]
pub struct CNCONGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONGINV register"]
pub mod cnconginv;
#[doc = "CNENG register"]
pub struct CNENG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENG register"]
pub mod cneng;
#[doc = "CNENGCLR register"]
pub struct CNENGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENGCLR register"]
pub mod cnengclr;
#[doc = "CNENGSET register"]
pub struct CNENGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENGSET register"]
pub mod cnengset;
#[doc = "CNENGINV register"]
pub struct CNENGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENGINV register"]
pub mod cnenginv;
#[doc = "CNSTATG register"]
pub struct CNSTATG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATG register"]
pub mod cnstatg;
#[doc = "CNSTATGCLR register"]
pub struct CNSTATGCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATGCLR register"]
pub mod cnstatgclr;
#[doc = "CNSTATGSET register"]
pub struct CNSTATGSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATGSET register"]
pub mod cnstatgset;
#[doc = "CNSTATGINV register"]
pub struct CNSTATGINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATGINV register"]
pub mod cnstatginv;
