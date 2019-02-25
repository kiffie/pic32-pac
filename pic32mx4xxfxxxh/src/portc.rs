#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELC register"]
    pub anselc: ANSELC,
    #[doc = "0x04 - ANSELCCLR register"]
    pub anselcclr: ANSELCCLR,
    #[doc = "0x08 - ANSELCSET register"]
    pub anselcset: ANSELCSET,
    #[doc = "0x0c - ANSELCINV register"]
    pub anselcinv: ANSELCINV,
    #[doc = "0x10 - TRISC register"]
    pub trisc: TRISC,
    #[doc = "0x14 - TRISCCLR register"]
    pub triscclr: TRISCCLR,
    #[doc = "0x18 - TRISCSET register"]
    pub triscset: TRISCSET,
    #[doc = "0x1c - TRISCINV register"]
    pub triscinv: TRISCINV,
    #[doc = "0x20 - PORTC register"]
    pub portc: PORTC,
    #[doc = "0x24 - PORTCCLR register"]
    pub portcclr: PORTCCLR,
    #[doc = "0x28 - PORTCSET register"]
    pub portcset: PORTCSET,
    #[doc = "0x2c - PORTCINV register"]
    pub portcinv: PORTCINV,
    #[doc = "0x30 - LATC register"]
    pub latc: LATC,
    #[doc = "0x34 - LATCCLR register"]
    pub latcclr: LATCCLR,
    #[doc = "0x38 - LATCSET register"]
    pub latcset: LATCSET,
    #[doc = "0x3c - LATCINV register"]
    pub latcinv: LATCINV,
    #[doc = "0x40 - ODCC register"]
    pub odcc: ODCC,
    #[doc = "0x44 - ODCCCLR register"]
    pub odccclr: ODCCCLR,
    #[doc = "0x48 - ODCCSET register"]
    pub odccset: ODCCSET,
    #[doc = "0x4c - ODCCINV register"]
    pub odccinv: ODCCINV,
    #[doc = "0x50 - CNPUC register"]
    pub cnpuc: CNPUC,
    #[doc = "0x54 - CNPUCCLR register"]
    pub cnpucclr: CNPUCCLR,
    #[doc = "0x58 - CNPUCSET register"]
    pub cnpucset: CNPUCSET,
    #[doc = "0x5c - CNPUCINV register"]
    pub cnpucinv: CNPUCINV,
    #[doc = "0x60 - CNPDC register"]
    pub cnpdc: CNPDC,
    #[doc = "0x64 - CNPDCCLR register"]
    pub cnpdcclr: CNPDCCLR,
    #[doc = "0x68 - CNPDCSET register"]
    pub cnpdcset: CNPDCSET,
    #[doc = "0x6c - CNPDCINV register"]
    pub cnpdcinv: CNPDCINV,
    #[doc = "0x70 - CNCONC register"]
    pub cnconc: CNCONC,
    #[doc = "0x74 - CNCONCCLR register"]
    pub cnconcclr: CNCONCCLR,
    #[doc = "0x78 - CNCONCSET register"]
    pub cnconcset: CNCONCSET,
    #[doc = "0x7c - CNCONCINV register"]
    pub cnconcinv: CNCONCINV,
    #[doc = "0x80 - CNENC register"]
    pub cnenc: CNENC,
    #[doc = "0x84 - CNENCCLR register"]
    pub cnencclr: CNENCCLR,
    #[doc = "0x88 - CNENCSET register"]
    pub cnencset: CNENCSET,
    #[doc = "0x8c - CNENCINV register"]
    pub cnencinv: CNENCINV,
    #[doc = "0x90 - CNSTATC register"]
    pub cnstatc: CNSTATC,
    #[doc = "0x94 - CNSTATCCLR register"]
    pub cnstatcclr: CNSTATCCLR,
    #[doc = "0x98 - CNSTATCSET register"]
    pub cnstatcset: CNSTATCSET,
    #[doc = "0x9c - CNSTATCINV register"]
    pub cnstatcinv: CNSTATCINV,
}
#[doc = "ANSELC register"]
pub struct ANSELC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELC register"]
pub mod anselc;
#[doc = "ANSELCCLR register"]
pub struct ANSELCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELCCLR register"]
pub mod anselcclr;
#[doc = "ANSELCSET register"]
pub struct ANSELCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELCSET register"]
pub mod anselcset;
#[doc = "ANSELCINV register"]
pub struct ANSELCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ANSELCINV register"]
pub mod anselcinv;
#[doc = "TRISC register"]
pub struct TRISC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISC register"]
pub mod trisc;
#[doc = "TRISCCLR register"]
pub struct TRISCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISCCLR register"]
pub mod triscclr;
#[doc = "TRISCSET register"]
pub struct TRISCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISCSET register"]
pub mod triscset;
#[doc = "TRISCINV register"]
pub struct TRISCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TRISCINV register"]
pub mod triscinv;
#[doc = "PORTC register"]
pub struct PORTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTC register"]
pub mod portc;
#[doc = "PORTCCLR register"]
pub struct PORTCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTCCLR register"]
pub mod portcclr;
#[doc = "PORTCSET register"]
pub struct PORTCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTCSET register"]
pub mod portcset;
#[doc = "PORTCINV register"]
pub struct PORTCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PORTCINV register"]
pub mod portcinv;
#[doc = "LATC register"]
pub struct LATC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATC register"]
pub mod latc;
#[doc = "LATCCLR register"]
pub struct LATCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATCCLR register"]
pub mod latcclr;
#[doc = "LATCSET register"]
pub struct LATCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATCSET register"]
pub mod latcset;
#[doc = "LATCINV register"]
pub struct LATCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LATCINV register"]
pub mod latcinv;
#[doc = "ODCC register"]
pub struct ODCC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCC register"]
pub mod odcc;
#[doc = "ODCCCLR register"]
pub struct ODCCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCCCLR register"]
pub mod odccclr;
#[doc = "ODCCSET register"]
pub struct ODCCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCCSET register"]
pub mod odccset;
#[doc = "ODCCINV register"]
pub struct ODCCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ODCCINV register"]
pub mod odccinv;
#[doc = "CNPUC register"]
pub struct CNPUC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUC register"]
pub mod cnpuc;
#[doc = "CNPUCCLR register"]
pub struct CNPUCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUCCLR register"]
pub mod cnpucclr;
#[doc = "CNPUCSET register"]
pub struct CNPUCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUCSET register"]
pub mod cnpucset;
#[doc = "CNPUCINV register"]
pub struct CNPUCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPUCINV register"]
pub mod cnpucinv;
#[doc = "CNPDC register"]
pub struct CNPDC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDC register"]
pub mod cnpdc;
#[doc = "CNPDCCLR register"]
pub struct CNPDCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDCCLR register"]
pub mod cnpdcclr;
#[doc = "CNPDCSET register"]
pub struct CNPDCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDCSET register"]
pub mod cnpdcset;
#[doc = "CNPDCINV register"]
pub struct CNPDCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNPDCINV register"]
pub mod cnpdcinv;
#[doc = "CNCONC register"]
pub struct CNCONC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONC register"]
pub mod cnconc;
#[doc = "CNCONCCLR register"]
pub struct CNCONCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONCCLR register"]
pub mod cnconcclr;
#[doc = "CNCONCSET register"]
pub struct CNCONCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONCSET register"]
pub mod cnconcset;
#[doc = "CNCONCINV register"]
pub struct CNCONCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNCONCINV register"]
pub mod cnconcinv;
#[doc = "CNENC register"]
pub struct CNENC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENC register"]
pub mod cnenc;
#[doc = "CNENCCLR register"]
pub struct CNENCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENCCLR register"]
pub mod cnencclr;
#[doc = "CNENCSET register"]
pub struct CNENCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENCSET register"]
pub mod cnencset;
#[doc = "CNENCINV register"]
pub struct CNENCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNENCINV register"]
pub mod cnencinv;
#[doc = "CNSTATC register"]
pub struct CNSTATC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATC register"]
pub mod cnstatc;
#[doc = "CNSTATCCLR register"]
pub struct CNSTATCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATCCLR register"]
pub mod cnstatcclr;
#[doc = "CNSTATCSET register"]
pub struct CNSTATCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATCSET register"]
pub mod cnstatcset;
#[doc = "CNSTATCINV register"]
pub struct CNSTATCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CNSTATCINV register"]
pub mod cnstatcinv;
