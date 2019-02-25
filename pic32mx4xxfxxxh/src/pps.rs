#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - INT1R register"]
    pub int1r: INT1R,
    #[doc = "0x04 - INT2R register"]
    pub int2r: INT2R,
    #[doc = "0x08 - INT3R register"]
    pub int3r: INT3R,
    #[doc = "0x0c - INT4R register"]
    pub int4r: INT4R,
    _reserved0: [u8; 4usize],
    #[doc = "0x14 - T2CKR register"]
    pub t2ckr: T2CKR,
    #[doc = "0x18 - T3CKR register"]
    pub t3ckr: T3CKR,
    #[doc = "0x1c - T4CKR register"]
    pub t4ckr: T4CKR,
    #[doc = "0x20 - T5CKR register"]
    pub t5ckr: T5CKR,
    #[doc = "0x24 - IC1R register"]
    pub ic1r: IC1R,
    #[doc = "0x28 - IC2R register"]
    pub ic2r: IC2R,
    #[doc = "0x2c - IC3R register"]
    pub ic3r: IC3R,
    #[doc = "0x30 - IC4R register"]
    pub ic4r: IC4R,
    #[doc = "0x34 - IC5R register"]
    pub ic5r: IC5R,
    _reserved1: [u8; 12usize],
    #[doc = "0x44 - OCFAR register"]
    pub ocfar: OCFAR,
    _reserved2: [u8; 4usize],
    #[doc = "0x4c - U1RXR register"]
    pub u1rxr: U1RXR,
    #[doc = "0x50 - U1CTSR register"]
    pub u1ctsr: U1CTSR,
    #[doc = "0x54 - U2RXR register"]
    pub u2rxr: U2RXR,
    #[doc = "0x58 - U2CTSR register"]
    pub u2ctsr: U2CTSR,
    #[doc = "0x5c - U3RXR register"]
    pub u3rxr: U3RXR,
    #[doc = "0x60 - U3CTSR register"]
    pub u3ctsr: U3CTSR,
    #[doc = "0x64 - U4RXR register"]
    pub u4rxr: U4RXR,
    #[doc = "0x68 - U4CTSR register"]
    pub u4ctsr: U4CTSR,
    #[doc = "0x6c - U5RXR register"]
    pub u5rxr: U5RXR,
    #[doc = "0x70 - U5CTSR register"]
    pub u5ctsr: U5CTSR,
    _reserved3: [u8; 12usize],
    #[doc = "0x80 - SDI1R register"]
    pub sdi1r: SDI1R,
    #[doc = "0x84 - SS1R register"]
    pub ss1r: SS1R,
    _reserved4: [u8; 4usize],
    #[doc = "0x8c - SDI2R register"]
    pub sdi2r: SDI2R,
    #[doc = "0x90 - SS2R register"]
    pub ss2r: SS2R,
    _reserved5: [u8; 56usize],
    #[doc = "0xcc - REFCLKIR register"]
    pub refclkir: REFCLKIR,
    _reserved6: [u8; 108usize],
    #[doc = "0x13c - RPB0R register"]
    pub rpb0r: RPB0R,
    #[doc = "0x140 - RPB1R register"]
    pub rpb1r: RPB1R,
    #[doc = "0x144 - RPB2R register"]
    pub rpb2r: RPB2R,
    #[doc = "0x148 - RPB3R register"]
    pub rpb3r: RPB3R,
    _reserved7: [u8; 4usize],
    #[doc = "0x150 - RPB5R register"]
    pub rpb5r: RPB5R,
    #[doc = "0x154 - RPB6R register"]
    pub rpb6r: RPB6R,
    #[doc = "0x158 - RPB7R register"]
    pub rpb7r: RPB7R,
    #[doc = "0x15c - RPB8R register"]
    pub rpb8r: RPB8R,
    #[doc = "0x160 - RPB9R register"]
    pub rpb9r: RPB9R,
    #[doc = "0x164 - RPB10R register"]
    pub rpb10r: RPB10R,
    _reserved8: [u8; 12usize],
    #[doc = "0x174 - RPB14R register"]
    pub rpb14r: RPB14R,
    #[doc = "0x178 - RPB15R register"]
    pub rpb15r: RPB15R,
    _reserved9: [u8; 52usize],
    #[doc = "0x1b0 - RPC13R register"]
    pub rpc13r: RPC13R,
    #[doc = "0x1b4 - RPC14R register"]
    pub rpc14r: RPC14R,
    _reserved10: [u8; 4usize],
    #[doc = "0x1bc - RPD0R register"]
    pub rpd0r: RPD0R,
    #[doc = "0x1c0 - RPD1R register"]
    pub rpd1r: RPD1R,
    #[doc = "0x1c4 - RPD2R register"]
    pub rpd2r: RPD2R,
    #[doc = "0x1c8 - RPD3R register"]
    pub rpd3r: RPD3R,
    #[doc = "0x1cc - RPD4R register"]
    pub rpd4r: RPD4R,
    #[doc = "0x1d0 - RPD5R register"]
    pub rpd5r: RPD5R,
    _reserved11: [u8; 8usize],
    #[doc = "0x1dc - RPD8R register"]
    pub rpd8r: RPD8R,
    #[doc = "0x1e0 - RPD9R register"]
    pub rpd9r: RPD9R,
    #[doc = "0x1e4 - RPD10R register"]
    pub rpd10r: RPD10R,
    #[doc = "0x1e8 - RPD11R register"]
    pub rpd11r: RPD11R,
    _reserved12: [u8; 28usize],
    #[doc = "0x208 - RPE3R register"]
    pub rpe3r: RPE3R,
    _reserved13: [u8; 4usize],
    #[doc = "0x210 - RPE5R register"]
    pub rpe5r: RPE5R,
    _reserved14: [u8; 40usize],
    #[doc = "0x23c - RPF0R register"]
    pub rpf0r: RPF0R,
    #[doc = "0x240 - RPF1R register"]
    pub rpf1r: RPF1R,
    _reserved15: [u8; 8usize],
    #[doc = "0x24c - RPF4R register"]
    pub rpf4r: RPF4R,
    #[doc = "0x250 - RPF5R register"]
    pub rpf5r: RPF5R,
    _reserved16: [u8; 64usize],
    #[doc = "0x294 - RPG6R register"]
    pub rpg6r: RPG6R,
    #[doc = "0x298 - RPG7R register"]
    pub rpg7r: RPG7R,
    #[doc = "0x29c - RPG8R register"]
    pub rpg8r: RPG8R,
    #[doc = "0x2a0 - RPG9R register"]
    pub rpg9r: RPG9R,
}
#[doc = "INT1R register"]
pub struct INT1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INT1R register"]
pub mod int1r;
#[doc = "INT2R register"]
pub struct INT2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INT2R register"]
pub mod int2r;
#[doc = "INT3R register"]
pub struct INT3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INT3R register"]
pub mod int3r;
#[doc = "INT4R register"]
pub struct INT4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INT4R register"]
pub mod int4r;
#[doc = "T2CKR register"]
pub struct T2CKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T2CKR register"]
pub mod t2ckr;
#[doc = "T3CKR register"]
pub struct T3CKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T3CKR register"]
pub mod t3ckr;
#[doc = "T4CKR register"]
pub struct T4CKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T4CKR register"]
pub mod t4ckr;
#[doc = "T5CKR register"]
pub struct T5CKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "T5CKR register"]
pub mod t5ckr;
#[doc = "IC1R register"]
pub struct IC1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC1R register"]
pub mod ic1r;
#[doc = "IC2R register"]
pub struct IC2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC2R register"]
pub mod ic2r;
#[doc = "IC3R register"]
pub struct IC3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC3R register"]
pub mod ic3r;
#[doc = "IC4R register"]
pub struct IC4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC4R register"]
pub mod ic4r;
#[doc = "IC5R register"]
pub struct IC5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IC5R register"]
pub mod ic5r;
#[doc = "OCFAR register"]
pub struct OCFAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OCFAR register"]
pub mod ocfar;
#[doc = "U1RXR register"]
pub struct U1RXR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1RXR register"]
pub mod u1rxr;
#[doc = "U1CTSR register"]
pub struct U1CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U1CTSR register"]
pub mod u1ctsr;
#[doc = "U2RXR register"]
pub struct U2RXR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2RXR register"]
pub mod u2rxr;
#[doc = "U2CTSR register"]
pub struct U2CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U2CTSR register"]
pub mod u2ctsr;
#[doc = "U3RXR register"]
pub struct U3RXR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3RXR register"]
pub mod u3rxr;
#[doc = "U3CTSR register"]
pub struct U3CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U3CTSR register"]
pub mod u3ctsr;
#[doc = "U4RXR register"]
pub struct U4RXR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4RXR register"]
pub mod u4rxr;
#[doc = "U4CTSR register"]
pub struct U4CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U4CTSR register"]
pub mod u4ctsr;
#[doc = "U5RXR register"]
pub struct U5RXR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U5RXR register"]
pub mod u5rxr;
#[doc = "U5CTSR register"]
pub struct U5CTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "U5CTSR register"]
pub mod u5ctsr;
#[doc = "SDI1R register"]
pub struct SDI1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDI1R register"]
pub mod sdi1r;
#[doc = "SS1R register"]
pub struct SS1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SS1R register"]
pub mod ss1r;
#[doc = "SDI2R register"]
pub struct SDI2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDI2R register"]
pub mod sdi2r;
#[doc = "SS2R register"]
pub struct SS2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SS2R register"]
pub mod ss2r;
#[doc = "REFCLKIR register"]
pub struct REFCLKIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "REFCLKIR register"]
pub mod refclkir;
#[doc = "RPB0R register"]
pub struct RPB0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB0R register"]
pub mod rpb0r;
#[doc = "RPB1R register"]
pub struct RPB1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB1R register"]
pub mod rpb1r;
#[doc = "RPB2R register"]
pub struct RPB2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB2R register"]
pub mod rpb2r;
#[doc = "RPB3R register"]
pub struct RPB3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB3R register"]
pub mod rpb3r;
#[doc = "RPB5R register"]
pub struct RPB5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB5R register"]
pub mod rpb5r;
#[doc = "RPB6R register"]
pub struct RPB6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB6R register"]
pub mod rpb6r;
#[doc = "RPB7R register"]
pub struct RPB7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB7R register"]
pub mod rpb7r;
#[doc = "RPB8R register"]
pub struct RPB8R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB8R register"]
pub mod rpb8r;
#[doc = "RPB9R register"]
pub struct RPB9R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB9R register"]
pub mod rpb9r;
#[doc = "RPB10R register"]
pub struct RPB10R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB10R register"]
pub mod rpb10r;
#[doc = "RPB14R register"]
pub struct RPB14R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB14R register"]
pub mod rpb14r;
#[doc = "RPB15R register"]
pub struct RPB15R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB15R register"]
pub mod rpb15r;
#[doc = "RPC13R register"]
pub struct RPC13R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC13R register"]
pub mod rpc13r;
#[doc = "RPC14R register"]
pub struct RPC14R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC14R register"]
pub mod rpc14r;
#[doc = "RPD0R register"]
pub struct RPD0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD0R register"]
pub mod rpd0r;
#[doc = "RPD1R register"]
pub struct RPD1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD1R register"]
pub mod rpd1r;
#[doc = "RPD2R register"]
pub struct RPD2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD2R register"]
pub mod rpd2r;
#[doc = "RPD3R register"]
pub struct RPD3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD3R register"]
pub mod rpd3r;
#[doc = "RPD4R register"]
pub struct RPD4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD4R register"]
pub mod rpd4r;
#[doc = "RPD5R register"]
pub struct RPD5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD5R register"]
pub mod rpd5r;
#[doc = "RPD8R register"]
pub struct RPD8R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD8R register"]
pub mod rpd8r;
#[doc = "RPD9R register"]
pub struct RPD9R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD9R register"]
pub mod rpd9r;
#[doc = "RPD10R register"]
pub struct RPD10R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD10R register"]
pub mod rpd10r;
#[doc = "RPD11R register"]
pub struct RPD11R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPD11R register"]
pub mod rpd11r;
#[doc = "RPE3R register"]
pub struct RPE3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPE3R register"]
pub mod rpe3r;
#[doc = "RPE5R register"]
pub struct RPE5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPE5R register"]
pub mod rpe5r;
#[doc = "RPF0R register"]
pub struct RPF0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPF0R register"]
pub mod rpf0r;
#[doc = "RPF1R register"]
pub struct RPF1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPF1R register"]
pub mod rpf1r;
#[doc = "RPF4R register"]
pub struct RPF4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPF4R register"]
pub mod rpf4r;
#[doc = "RPF5R register"]
pub struct RPF5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPF5R register"]
pub mod rpf5r;
#[doc = "RPG6R register"]
pub struct RPG6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPG6R register"]
pub mod rpg6r;
#[doc = "RPG7R register"]
pub struct RPG7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPG7R register"]
pub mod rpg7r;
#[doc = "RPG8R register"]
pub struct RPG8R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPG8R register"]
pub mod rpg8r;
#[doc = "RPG9R register"]
pub struct RPG9R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPG9R register"]
pub mod rpg9r;
