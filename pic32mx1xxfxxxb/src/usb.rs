#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U1OTGIR register"]
    pub u1otgir: U1OTGIR,
    #[doc = "0x04 - U1OTGIRCLR register"]
    pub u1otgirclr: U1OTGIRCLR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - U1OTGIE register"]
    pub u1otgie: U1OTGIE,
    #[doc = "0x14 - U1OTGIECLR register"]
    pub u1otgieclr: U1OTGIECLR,
    #[doc = "0x18 - U1OTGIESET register"]
    pub u1otgieset: U1OTGIESET,
    #[doc = "0x1c - U1OTGIEINV register"]
    pub u1otgieinv: U1OTGIEINV,
    #[doc = "0x20 - U1OTGSTAT register"]
    pub u1otgstat: U1OTGSTAT,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - U1OTGCON register"]
    pub u1otgcon: U1OTGCON,
    #[doc = "0x34 - U1OTGCONCLR register"]
    pub u1otgconclr: U1OTGCONCLR,
    #[doc = "0x38 - U1OTGCONSET register"]
    pub u1otgconset: U1OTGCONSET,
    #[doc = "0x3c - U1OTGCONINV register"]
    pub u1otgconinv: U1OTGCONINV,
    #[doc = "0x40 - U1PWRC register"]
    pub u1pwrc: U1PWRC,
    #[doc = "0x44 - U1PWRCCLR register"]
    pub u1pwrcclr: U1PWRCCLR,
    #[doc = "0x48 - U1PWRCSET register"]
    pub u1pwrcset: U1PWRCSET,
    #[doc = "0x4c - U1PWRCINV register"]
    pub u1pwrcinv: U1PWRCINV,
    _reserved2: [u8; 368usize],
    #[doc = "0x1c0 - U1IR register"]
    pub u1ir: U1IR,
    #[doc = "0x1c4 - U1IRCLR register"]
    pub u1irclr: U1IRCLR,
    _reserved3: [u8; 8usize],
    #[doc = "0x1d0 - U1IE register"]
    pub u1ie: U1IE,
    #[doc = "0x1d4 - U1IECLR register"]
    pub u1ieclr: U1IECLR,
    #[doc = "0x1d8 - U1IESET register"]
    pub u1ieset: U1IESET,
    #[doc = "0x1dc - U1IEINV register"]
    pub u1ieinv: U1IEINV,
    #[doc = "0x1e0 - U1EIR register"]
    pub u1eir: U1EIR,
    #[doc = "0x1e4 - U1EIRCLR register"]
    pub u1eirclr: U1EIRCLR,
    _reserved4: [u8; 8usize],
    #[doc = "0x1f0 - U1EIE register"]
    pub u1eie: U1EIE,
    #[doc = "0x1f4 - U1EIECLR register"]
    pub u1eieclr: U1EIECLR,
    #[doc = "0x1f8 - U1EIESET register"]
    pub u1eieset: U1EIESET,
    #[doc = "0x1fc - U1EIEINV register"]
    pub u1eieinv: U1EIEINV,
    #[doc = "0x200 - U1STAT register"]
    pub u1stat: U1STAT,
    _reserved5: [u8; 12usize],
    #[doc = "0x210 - U1CON register"]
    pub u1con: U1CON,
    #[doc = "0x214 - U1CONCLR register"]
    pub u1conclr: U1CONCLR,
    #[doc = "0x218 - U1CONSET register"]
    pub u1conset: U1CONSET,
    #[doc = "0x21c - U1CONINV register"]
    pub u1coninv: U1CONINV,
    #[doc = "0x220 - U1ADDR register"]
    pub u1addr: U1ADDR,
    #[doc = "0x224 - U1ADDRCLR register"]
    pub u1addrclr: U1ADDRCLR,
    #[doc = "0x228 - U1ADDRSET register"]
    pub u1addrset: U1ADDRSET,
    #[doc = "0x22c - U1ADDRINV register"]
    pub u1addrinv: U1ADDRINV,
    #[doc = "0x230 - U1BDTP1 register"]
    pub u1bdtp1: U1BDTP1,
    #[doc = "0x234 - U1BDTP1CLR register"]
    pub u1bdtp1clr: U1BDTP1CLR,
    #[doc = "0x238 - U1BDTP1SET register"]
    pub u1bdtp1set: U1BDTP1SET,
    #[doc = "0x23c - U1BDTP1INV register"]
    pub u1bdtp1inv: U1BDTP1INV,
    #[doc = "0x240 - U1FRML register"]
    pub u1frml: U1FRML,
    _reserved6: [u8; 12usize],
    #[doc = "0x250 - U1FRMH register"]
    pub u1frmh: U1FRMH,
    _reserved7: [u8; 12usize],
    #[doc = "0x260 - U1TOK register"]
    pub u1tok: U1TOK,
    #[doc = "0x264 - U1TOKCLR register"]
    pub u1tokclr: U1TOKCLR,
    #[doc = "0x268 - U1TOKSET register"]
    pub u1tokset: U1TOKSET,
    #[doc = "0x26c - U1TOKINV register"]
    pub u1tokinv: U1TOKINV,
    #[doc = "0x270 - U1SOF register"]
    pub u1sof: U1SOF,
    #[doc = "0x274 - U1SOFCLR register"]
    pub u1sofclr: U1SOFCLR,
    #[doc = "0x278 - U1SOFSET register"]
    pub u1sofset: U1SOFSET,
    #[doc = "0x27c - U1SOFINV register"]
    pub u1sofinv: U1SOFINV,
    #[doc = "0x280 - U1BDTP2 register"]
    pub u1bdtp2: U1BDTP2,
    #[doc = "0x284 - U1BDTP2CLR register"]
    pub u1bdtp2clr: U1BDTP2CLR,
    #[doc = "0x288 - U1BDTP2SET register"]
    pub u1bdtp2set: U1BDTP2SET,
    #[doc = "0x28c - U1BDTP2INV register"]
    pub u1bdtp2inv: U1BDTP2INV,
    #[doc = "0x290 - U1BDTP3 register"]
    pub u1bdtp3: U1BDTP3,
    #[doc = "0x294 - U1BDTP3CLR register"]
    pub u1bdtp3clr: U1BDTP3CLR,
    #[doc = "0x298 - U1BDTP3SET register"]
    pub u1bdtp3set: U1BDTP3SET,
    #[doc = "0x29c - U1BDTP3INV register"]
    pub u1bdtp3inv: U1BDTP3INV,
    #[doc = "0x2a0 - U1CNFG1 register"]
    pub u1cnfg1: U1CNFG1,
    #[doc = "0x2a4 - U1CNFG1CLR register"]
    pub u1cnfg1clr: U1CNFG1CLR,
    #[doc = "0x2a8 - U1CNFG1SET register"]
    pub u1cnfg1set: U1CNFG1SET,
    #[doc = "0x2ac - U1CNFG1INV register"]
    pub u1cnfg1inv: U1CNFG1INV,
    _reserved8: [u8; 16usize],
    #[doc = "0x2c0 - U1EP0 register"]
    pub u1ep0: U1EP0,
    #[doc = "0x2c4 - U1EP0CLR register"]
    pub u1ep0clr: U1EP0CLR,
    #[doc = "0x2c8 - U1EP0SET register"]
    pub u1ep0set: U1EP0SET,
    #[doc = "0x2cc - U1EP0INV register"]
    pub u1ep0inv: U1EP0INV,
    #[doc = "0x2d0 - U1EP1 register"]
    pub u1ep1: U1EP1,
    #[doc = "0x2d4 - U1EP1CLR register"]
    pub u1ep1clr: U1EP1CLR,
    #[doc = "0x2d8 - U1EP1SET register"]
    pub u1ep1set: U1EP1SET,
    #[doc = "0x2dc - U1EP1INV register"]
    pub u1ep1inv: U1EP1INV,
    #[doc = "0x2e0 - U1EP2 register"]
    pub u1ep2: U1EP2,
    #[doc = "0x2e4 - U1EP2CLR register"]
    pub u1ep2clr: U1EP2CLR,
    #[doc = "0x2e8 - U1EP2SET register"]
    pub u1ep2set: U1EP2SET,
    #[doc = "0x2ec - U1EP2INV register"]
    pub u1ep2inv: U1EP2INV,
    #[doc = "0x2f0 - U1EP3 register"]
    pub u1ep3: U1EP3,
    #[doc = "0x2f4 - U1EP3CLR register"]
    pub u1ep3clr: U1EP3CLR,
    #[doc = "0x2f8 - U1EP3SET register"]
    pub u1ep3set: U1EP3SET,
    #[doc = "0x2fc - U1EP3INV register"]
    pub u1ep3inv: U1EP3INV,
    #[doc = "0x300 - U1EP4 register"]
    pub u1ep4: U1EP4,
    #[doc = "0x304 - U1EP4CLR register"]
    pub u1ep4clr: U1EP4CLR,
    #[doc = "0x308 - U1EP4SET register"]
    pub u1ep4set: U1EP4SET,
    #[doc = "0x30c - U1EP4INV register"]
    pub u1ep4inv: U1EP4INV,
    #[doc = "0x310 - U1EP5 register"]
    pub u1ep5: U1EP5,
    #[doc = "0x314 - U1EP5CLR register"]
    pub u1ep5clr: U1EP5CLR,
    #[doc = "0x318 - U1EP5SET register"]
    pub u1ep5set: U1EP5SET,
    #[doc = "0x31c - U1EP5INV register"]
    pub u1ep5inv: U1EP5INV,
    #[doc = "0x320 - U1EP6 register"]
    pub u1ep6: U1EP6,
    #[doc = "0x324 - U1EP6CLR register"]
    pub u1ep6clr: U1EP6CLR,
    #[doc = "0x328 - U1EP6SET register"]
    pub u1ep6set: U1EP6SET,
    #[doc = "0x32c - U1EP6INV register"]
    pub u1ep6inv: U1EP6INV,
    #[doc = "0x330 - U1EP7 register"]
    pub u1ep7: U1EP7,
    #[doc = "0x334 - U1EP7CLR register"]
    pub u1ep7clr: U1EP7CLR,
    #[doc = "0x338 - U1EP7SET register"]
    pub u1ep7set: U1EP7SET,
    #[doc = "0x33c - U1EP7INV register"]
    pub u1ep7inv: U1EP7INV,
    #[doc = "0x340 - U1EP8 register"]
    pub u1ep8: U1EP8,
    #[doc = "0x344 - U1EP8CLR register"]
    pub u1ep8clr: U1EP8CLR,
    #[doc = "0x348 - U1EP8SET register"]
    pub u1ep8set: U1EP8SET,
    #[doc = "0x34c - U1EP8INV register"]
    pub u1ep8inv: U1EP8INV,
    #[doc = "0x350 - U1EP9 register"]
    pub u1ep9: U1EP9,
    #[doc = "0x354 - U1EP9CLR register"]
    pub u1ep9clr: U1EP9CLR,
    #[doc = "0x358 - U1EP9SET register"]
    pub u1ep9set: U1EP9SET,
    #[doc = "0x35c - U1EP9INV register"]
    pub u1ep9inv: U1EP9INV,
    #[doc = "0x360 - U1EP10 register"]
    pub u1ep10: U1EP10,
    #[doc = "0x364 - U1EP10CLR register"]
    pub u1ep10clr: U1EP10CLR,
    #[doc = "0x368 - U1EP10SET register"]
    pub u1ep10set: U1EP10SET,
    #[doc = "0x36c - U1EP10INV register"]
    pub u1ep10inv: U1EP10INV,
    #[doc = "0x370 - U1EP11 register"]
    pub u1ep11: U1EP11,
    #[doc = "0x374 - U1EP11CLR register"]
    pub u1ep11clr: U1EP11CLR,
    #[doc = "0x378 - U1EP11SET register"]
    pub u1ep11set: U1EP11SET,
    #[doc = "0x37c - U1EP11INV register"]
    pub u1ep11inv: U1EP11INV,
    #[doc = "0x380 - U1EP12 register"]
    pub u1ep12: U1EP12,
    #[doc = "0x384 - U1EP12CLR register"]
    pub u1ep12clr: U1EP12CLR,
    #[doc = "0x388 - U1EP12SET register"]
    pub u1ep12set: U1EP12SET,
    #[doc = "0x38c - U1EP12INV register"]
    pub u1ep12inv: U1EP12INV,
    #[doc = "0x390 - U1EP13 register"]
    pub u1ep13: U1EP13,
    #[doc = "0x394 - U1EP13CLR register"]
    pub u1ep13clr: U1EP13CLR,
    #[doc = "0x398 - U1EP13SET register"]
    pub u1ep13set: U1EP13SET,
    #[doc = "0x39c - U1EP13INV register"]
    pub u1ep13inv: U1EP13INV,
    #[doc = "0x3a0 - U1EP14 register"]
    pub u1ep14: U1EP14,
    #[doc = "0x3a4 - U1EP14CLR register"]
    pub u1ep14clr: U1EP14CLR,
    #[doc = "0x3a8 - U1EP14SET register"]
    pub u1ep14set: U1EP14SET,
    #[doc = "0x3ac - U1EP14INV register"]
    pub u1ep14inv: U1EP14INV,
    #[doc = "0x3b0 - U1EP15 register"]
    pub u1ep15: U1EP15,
    #[doc = "0x3b4 - U1EP15CLR register"]
    pub u1ep15clr: U1EP15CLR,
    #[doc = "0x3b8 - U1EP15SET register"]
    pub u1ep15set: U1EP15SET,
    #[doc = "0x3bc - U1EP15INV register"]
    pub u1ep15inv: U1EP15INV,
}
#[doc = "U1OTGIR register"]
pub struct U1OTGIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGIR register"]
pub mod u1otgir;
#[doc = "U1OTGIRCLR register"]
pub struct U1OTGIRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGIRCLR register"]
pub mod u1otgirclr;
#[doc = "U1OTGIE register"]
pub struct U1OTGIE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGIE register"]
pub mod u1otgie;
#[doc = "U1OTGIECLR register"]
pub struct U1OTGIECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGIECLR register"]
pub mod u1otgieclr;
#[doc = "U1OTGIESET register"]
pub struct U1OTGIESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGIESET register"]
pub mod u1otgieset;
#[doc = "U1OTGIEINV register"]
pub struct U1OTGIEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGIEINV register"]
pub mod u1otgieinv;
#[doc = "U1OTGSTAT register"]
pub struct U1OTGSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGSTAT register"]
pub mod u1otgstat;
#[doc = "U1OTGCON register"]
pub struct U1OTGCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGCON register"]
pub mod u1otgcon;
#[doc = "U1OTGCONCLR register"]
pub struct U1OTGCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGCONCLR register"]
pub mod u1otgconclr;
#[doc = "U1OTGCONSET register"]
pub struct U1OTGCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGCONSET register"]
pub mod u1otgconset;
#[doc = "U1OTGCONINV register"]
pub struct U1OTGCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1OTGCONINV register"]
pub mod u1otgconinv;
#[doc = "U1PWRC register"]
pub struct U1PWRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1PWRC register"]
pub mod u1pwrc;
#[doc = "U1PWRCCLR register"]
pub struct U1PWRCCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1PWRCCLR register"]
pub mod u1pwrcclr;
#[doc = "U1PWRCSET register"]
pub struct U1PWRCSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1PWRCSET register"]
pub mod u1pwrcset;
#[doc = "U1PWRCINV register"]
pub struct U1PWRCINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1PWRCINV register"]
pub mod u1pwrcinv;
#[doc = "U1IR register"]
pub struct U1IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1IR register"]
pub mod u1ir;
#[doc = "U1IRCLR register"]
pub struct U1IRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1IRCLR register"]
pub mod u1irclr;
#[doc = "U1IE register"]
pub struct U1IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1IE register"]
pub mod u1ie;
#[doc = "U1IECLR register"]
pub struct U1IECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1IECLR register"]
pub mod u1ieclr;
#[doc = "U1IESET register"]
pub struct U1IESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1IESET register"]
pub mod u1ieset;
#[doc = "U1IEINV register"]
pub struct U1IEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1IEINV register"]
pub mod u1ieinv;
#[doc = "U1EIR register"]
pub struct U1EIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EIR register"]
pub mod u1eir;
#[doc = "U1EIRCLR register"]
pub struct U1EIRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EIRCLR register"]
pub mod u1eirclr;
#[doc = "U1EIE register"]
pub struct U1EIE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EIE register"]
pub mod u1eie;
#[doc = "U1EIECLR register"]
pub struct U1EIECLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EIECLR register"]
pub mod u1eieclr;
#[doc = "U1EIESET register"]
pub struct U1EIESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EIESET register"]
pub mod u1eieset;
#[doc = "U1EIEINV register"]
pub struct U1EIEINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EIEINV register"]
pub mod u1eieinv;
#[doc = "U1STAT register"]
pub struct U1STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1STAT register"]
pub mod u1stat;
#[doc = "U1CON register"]
pub struct U1CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CON register"]
pub mod u1con;
#[doc = "U1CONCLR register"]
pub struct U1CONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CONCLR register"]
pub mod u1conclr;
#[doc = "U1CONSET register"]
pub struct U1CONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CONSET register"]
pub mod u1conset;
#[doc = "U1CONINV register"]
pub struct U1CONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CONINV register"]
pub mod u1coninv;
#[doc = "U1ADDR register"]
pub struct U1ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1ADDR register"]
pub mod u1addr;
#[doc = "U1ADDRCLR register"]
pub struct U1ADDRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1ADDRCLR register"]
pub mod u1addrclr;
#[doc = "U1ADDRSET register"]
pub struct U1ADDRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1ADDRSET register"]
pub mod u1addrset;
#[doc = "U1ADDRINV register"]
pub struct U1ADDRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1ADDRINV register"]
pub mod u1addrinv;
#[doc = "U1BDTP1 register"]
pub struct U1BDTP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP1 register"]
pub mod u1bdtp1;
#[doc = "U1BDTP1CLR register"]
pub struct U1BDTP1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP1CLR register"]
pub mod u1bdtp1clr;
#[doc = "U1BDTP1SET register"]
pub struct U1BDTP1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP1SET register"]
pub mod u1bdtp1set;
#[doc = "U1BDTP1INV register"]
pub struct U1BDTP1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP1INV register"]
pub mod u1bdtp1inv;
#[doc = "U1FRML register"]
pub struct U1FRML {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1FRML register"]
pub mod u1frml;
#[doc = "U1FRMH register"]
pub struct U1FRMH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1FRMH register"]
pub mod u1frmh;
#[doc = "U1TOK register"]
pub struct U1TOK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1TOK register"]
pub mod u1tok;
#[doc = "U1TOKCLR register"]
pub struct U1TOKCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1TOKCLR register"]
pub mod u1tokclr;
#[doc = "U1TOKSET register"]
pub struct U1TOKSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1TOKSET register"]
pub mod u1tokset;
#[doc = "U1TOKINV register"]
pub struct U1TOKINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1TOKINV register"]
pub mod u1tokinv;
#[doc = "U1SOF register"]
pub struct U1SOF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1SOF register"]
pub mod u1sof;
#[doc = "U1SOFCLR register"]
pub struct U1SOFCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1SOFCLR register"]
pub mod u1sofclr;
#[doc = "U1SOFSET register"]
pub struct U1SOFSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1SOFSET register"]
pub mod u1sofset;
#[doc = "U1SOFINV register"]
pub struct U1SOFINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1SOFINV register"]
pub mod u1sofinv;
#[doc = "U1BDTP2 register"]
pub struct U1BDTP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP2 register"]
pub mod u1bdtp2;
#[doc = "U1BDTP2CLR register"]
pub struct U1BDTP2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP2CLR register"]
pub mod u1bdtp2clr;
#[doc = "U1BDTP2SET register"]
pub struct U1BDTP2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP2SET register"]
pub mod u1bdtp2set;
#[doc = "U1BDTP2INV register"]
pub struct U1BDTP2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP2INV register"]
pub mod u1bdtp2inv;
#[doc = "U1BDTP3 register"]
pub struct U1BDTP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP3 register"]
pub mod u1bdtp3;
#[doc = "U1BDTP3CLR register"]
pub struct U1BDTP3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP3CLR register"]
pub mod u1bdtp3clr;
#[doc = "U1BDTP3SET register"]
pub struct U1BDTP3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP3SET register"]
pub mod u1bdtp3set;
#[doc = "U1BDTP3INV register"]
pub struct U1BDTP3INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1BDTP3INV register"]
pub mod u1bdtp3inv;
#[doc = "U1CNFG1 register"]
pub struct U1CNFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CNFG1 register"]
pub mod u1cnfg1;
#[doc = "U1CNFG1CLR register"]
pub struct U1CNFG1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CNFG1CLR register"]
pub mod u1cnfg1clr;
#[doc = "U1CNFG1SET register"]
pub struct U1CNFG1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CNFG1SET register"]
pub mod u1cnfg1set;
#[doc = "U1CNFG1INV register"]
pub struct U1CNFG1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CNFG1INV register"]
pub mod u1cnfg1inv;
#[doc = "U1EP0 register"]
pub struct U1EP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP0 register"]
pub mod u1ep0;
#[doc = "U1EP0CLR register"]
pub struct U1EP0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP0CLR register"]
pub mod u1ep0clr;
#[doc = "U1EP0SET register"]
pub struct U1EP0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP0SET register"]
pub mod u1ep0set;
#[doc = "U1EP0INV register"]
pub struct U1EP0INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP0INV register"]
pub mod u1ep0inv;
#[doc = "U1EP1 register"]
pub struct U1EP1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP1 register"]
pub mod u1ep1;
#[doc = "U1EP1CLR register"]
pub struct U1EP1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP1CLR register"]
pub mod u1ep1clr;
#[doc = "U1EP1SET register"]
pub struct U1EP1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP1SET register"]
pub mod u1ep1set;
#[doc = "U1EP1INV register"]
pub struct U1EP1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP1INV register"]
pub mod u1ep1inv;
#[doc = "U1EP2 register"]
pub struct U1EP2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP2 register"]
pub mod u1ep2;
#[doc = "U1EP2CLR register"]
pub struct U1EP2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP2CLR register"]
pub mod u1ep2clr;
#[doc = "U1EP2SET register"]
pub struct U1EP2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP2SET register"]
pub mod u1ep2set;
#[doc = "U1EP2INV register"]
pub struct U1EP2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP2INV register"]
pub mod u1ep2inv;
#[doc = "U1EP3 register"]
pub struct U1EP3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP3 register"]
pub mod u1ep3;
#[doc = "U1EP3CLR register"]
pub struct U1EP3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP3CLR register"]
pub mod u1ep3clr;
#[doc = "U1EP3SET register"]
pub struct U1EP3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP3SET register"]
pub mod u1ep3set;
#[doc = "U1EP3INV register"]
pub struct U1EP3INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP3INV register"]
pub mod u1ep3inv;
#[doc = "U1EP4 register"]
pub struct U1EP4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP4 register"]
pub mod u1ep4;
#[doc = "U1EP4CLR register"]
pub struct U1EP4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP4CLR register"]
pub mod u1ep4clr;
#[doc = "U1EP4SET register"]
pub struct U1EP4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP4SET register"]
pub mod u1ep4set;
#[doc = "U1EP4INV register"]
pub struct U1EP4INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP4INV register"]
pub mod u1ep4inv;
#[doc = "U1EP5 register"]
pub struct U1EP5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP5 register"]
pub mod u1ep5;
#[doc = "U1EP5CLR register"]
pub struct U1EP5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP5CLR register"]
pub mod u1ep5clr;
#[doc = "U1EP5SET register"]
pub struct U1EP5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP5SET register"]
pub mod u1ep5set;
#[doc = "U1EP5INV register"]
pub struct U1EP5INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP5INV register"]
pub mod u1ep5inv;
#[doc = "U1EP6 register"]
pub struct U1EP6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP6 register"]
pub mod u1ep6;
#[doc = "U1EP6CLR register"]
pub struct U1EP6CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP6CLR register"]
pub mod u1ep6clr;
#[doc = "U1EP6SET register"]
pub struct U1EP6SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP6SET register"]
pub mod u1ep6set;
#[doc = "U1EP6INV register"]
pub struct U1EP6INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP6INV register"]
pub mod u1ep6inv;
#[doc = "U1EP7 register"]
pub struct U1EP7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP7 register"]
pub mod u1ep7;
#[doc = "U1EP7CLR register"]
pub struct U1EP7CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP7CLR register"]
pub mod u1ep7clr;
#[doc = "U1EP7SET register"]
pub struct U1EP7SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP7SET register"]
pub mod u1ep7set;
#[doc = "U1EP7INV register"]
pub struct U1EP7INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP7INV register"]
pub mod u1ep7inv;
#[doc = "U1EP8 register"]
pub struct U1EP8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP8 register"]
pub mod u1ep8;
#[doc = "U1EP8CLR register"]
pub struct U1EP8CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP8CLR register"]
pub mod u1ep8clr;
#[doc = "U1EP8SET register"]
pub struct U1EP8SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP8SET register"]
pub mod u1ep8set;
#[doc = "U1EP8INV register"]
pub struct U1EP8INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP8INV register"]
pub mod u1ep8inv;
#[doc = "U1EP9 register"]
pub struct U1EP9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP9 register"]
pub mod u1ep9;
#[doc = "U1EP9CLR register"]
pub struct U1EP9CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP9CLR register"]
pub mod u1ep9clr;
#[doc = "U1EP9SET register"]
pub struct U1EP9SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP9SET register"]
pub mod u1ep9set;
#[doc = "U1EP9INV register"]
pub struct U1EP9INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP9INV register"]
pub mod u1ep9inv;
#[doc = "U1EP10 register"]
pub struct U1EP10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP10 register"]
pub mod u1ep10;
#[doc = "U1EP10CLR register"]
pub struct U1EP10CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP10CLR register"]
pub mod u1ep10clr;
#[doc = "U1EP10SET register"]
pub struct U1EP10SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP10SET register"]
pub mod u1ep10set;
#[doc = "U1EP10INV register"]
pub struct U1EP10INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP10INV register"]
pub mod u1ep10inv;
#[doc = "U1EP11 register"]
pub struct U1EP11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP11 register"]
pub mod u1ep11;
#[doc = "U1EP11CLR register"]
pub struct U1EP11CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP11CLR register"]
pub mod u1ep11clr;
#[doc = "U1EP11SET register"]
pub struct U1EP11SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP11SET register"]
pub mod u1ep11set;
#[doc = "U1EP11INV register"]
pub struct U1EP11INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP11INV register"]
pub mod u1ep11inv;
#[doc = "U1EP12 register"]
pub struct U1EP12 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP12 register"]
pub mod u1ep12;
#[doc = "U1EP12CLR register"]
pub struct U1EP12CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP12CLR register"]
pub mod u1ep12clr;
#[doc = "U1EP12SET register"]
pub struct U1EP12SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP12SET register"]
pub mod u1ep12set;
#[doc = "U1EP12INV register"]
pub struct U1EP12INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP12INV register"]
pub mod u1ep12inv;
#[doc = "U1EP13 register"]
pub struct U1EP13 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP13 register"]
pub mod u1ep13;
#[doc = "U1EP13CLR register"]
pub struct U1EP13CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP13CLR register"]
pub mod u1ep13clr;
#[doc = "U1EP13SET register"]
pub struct U1EP13SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP13SET register"]
pub mod u1ep13set;
#[doc = "U1EP13INV register"]
pub struct U1EP13INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP13INV register"]
pub mod u1ep13inv;
#[doc = "U1EP14 register"]
pub struct U1EP14 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP14 register"]
pub mod u1ep14;
#[doc = "U1EP14CLR register"]
pub struct U1EP14CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP14CLR register"]
pub mod u1ep14clr;
#[doc = "U1EP14SET register"]
pub struct U1EP14SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP14SET register"]
pub mod u1ep14set;
#[doc = "U1EP14INV register"]
pub struct U1EP14INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP14INV register"]
pub mod u1ep14inv;
#[doc = "U1EP15 register"]
pub struct U1EP15 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP15 register"]
pub mod u1ep15;
#[doc = "U1EP15CLR register"]
pub struct U1EP15CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP15CLR register"]
pub mod u1ep15clr;
#[doc = "U1EP15SET register"]
pub struct U1EP15SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP15SET register"]
pub mod u1ep15set;
#[doc = "U1EP15INV register"]
pub struct U1EP15INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1EP15INV register"]
pub mod u1ep15inv;
