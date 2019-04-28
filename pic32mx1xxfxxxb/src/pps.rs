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
    #[doc = "0x48 - OCFBR register"]
    pub ocfbr: OCFBR,
    #[doc = "0x4c - U1RXR register"]
    pub u1rxr: U1RXR,
    #[doc = "0x50 - U1CTSR register"]
    pub u1ctsr: U1CTSR,
    #[doc = "0x54 - U2RXR register"]
    pub u2rxr: U2RXR,
    #[doc = "0x58 - U2CTSR register"]
    pub u2ctsr: U2CTSR,
    _reserved2: [u8; 36usize],
    #[doc = "0x80 - SDI1R register"]
    pub sdi1r: SDI1R,
    #[doc = "0x84 - SS1R register"]
    pub ss1r: SS1R,
    _reserved3: [u8; 4usize],
    #[doc = "0x8c - SDI2R register"]
    pub sdi2r: SDI2R,
    #[doc = "0x90 - SS2R register"]
    pub ss2r: SS2R,
    _reserved4: [u8; 32usize],
    #[doc = "0xb4 - REFCLKIR register"]
    pub refclkir: REFCLKIR,
    _reserved5: [u8; 68usize],
    #[doc = "0xfc - RPA0R register"]
    pub rpa0r: RPA0R,
    #[doc = "0x100 - RPA1R register"]
    pub rpa1r: RPA1R,
    #[doc = "0x104 - RPA2R register"]
    pub rpa2r: RPA2R,
    #[doc = "0x108 - RPA3R register"]
    pub rpa3r: RPA3R,
    #[doc = "0x10c - RPA4R register"]
    pub rpa4r: RPA4R,
    _reserved6: [u8; 12usize],
    #[doc = "0x11c - RPA8R register"]
    pub rpa8r: RPA8R,
    #[doc = "0x120 - RPA9R register"]
    pub rpa9r: RPA9R,
    _reserved7: [u8; 4usize],
    #[doc = "0x128 - RPB0R register"]
    pub rpb0r: RPB0R,
    #[doc = "0x12c - RPB1R register"]
    pub rpb1r: RPB1R,
    #[doc = "0x130 - RPB2R register"]
    pub rpb2r: RPB2R,
    #[doc = "0x134 - RPB3R register"]
    pub rpb3r: RPB3R,
    #[doc = "0x138 - RPB4R register"]
    pub rpb4r: RPB4R,
    #[doc = "0x13c - RPB5R register"]
    pub rpb5r: RPB5R,
    #[doc = "0x140 - RPB6R register"]
    pub rpb6r: RPB6R,
    #[doc = "0x144 - RPB7R register"]
    pub rpb7r: RPB7R,
    #[doc = "0x148 - RPB8R register"]
    pub rpb8r: RPB8R,
    #[doc = "0x14c - RPB9R register"]
    pub rpb9r: RPB9R,
    #[doc = "0x150 - RPB10R register"]
    pub rpb10r: RPB10R,
    #[doc = "0x154 - RPB11R register"]
    pub rpb11r: RPB11R,
    #[doc = "0x158 - RPB12R register"]
    pub rpb12r: RPB12R,
    #[doc = "0x15c - RPB13R register"]
    pub rpb13r: RPB13R,
    #[doc = "0x160 - RPB14R register"]
    pub rpb14r: RPB14R,
    #[doc = "0x164 - RPB15R register"]
    pub rpb15r: RPB15R,
    #[doc = "0x168 - RPC0R register"]
    pub rpc0r: RPC0R,
    #[doc = "0x16c - RPC1R register"]
    pub rpc1r: RPC1R,
    #[doc = "0x170 - RPC2R register"]
    pub rpc2r: RPC2R,
    #[doc = "0x174 - RPC3R register"]
    pub rpc3r: RPC3R,
    #[doc = "0x178 - RPC4R register"]
    pub rpc4r: RPC4R,
    #[doc = "0x17c - RPC5R register"]
    pub rpc5r: RPC5R,
    #[doc = "0x180 - RPC6R register"]
    pub rpc6r: RPC6R,
    #[doc = "0x184 - RPC7R register"]
    pub rpc7r: RPC7R,
    #[doc = "0x188 - RPC8R register"]
    pub rpc8r: RPC8R,
    #[doc = "0x18c - RPC9R register"]
    pub rpc9r: RPC9R,
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
#[doc = "OCFBR register"]
pub struct OCFBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OCFBR register"]
pub mod ocfbr;
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
#[doc = "RPA0R register"]
pub struct RPA0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPA0R register"]
pub mod rpa0r;
#[doc = "RPA1R register"]
pub struct RPA1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPA1R register"]
pub mod rpa1r;
#[doc = "RPA2R register"]
pub struct RPA2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPA2R register"]
pub mod rpa2r;
#[doc = "RPA3R register"]
pub struct RPA3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPA3R register"]
pub mod rpa3r;
#[doc = "RPA4R register"]
pub struct RPA4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPA4R register"]
pub mod rpa4r;
#[doc = "RPA8R register"]
pub struct RPA8R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPA8R register"]
pub mod rpa8r;
#[doc = "RPA9R register"]
pub struct RPA9R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPA9R register"]
pub mod rpa9r;
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
#[doc = "RPB4R register"]
pub struct RPB4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB4R register"]
pub mod rpb4r;
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
#[doc = "RPB11R register"]
pub struct RPB11R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB11R register"]
pub mod rpb11r;
#[doc = "RPB12R register"]
pub struct RPB12R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB12R register"]
pub mod rpb12r;
#[doc = "RPB13R register"]
pub struct RPB13R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPB13R register"]
pub mod rpb13r;
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
#[doc = "RPC0R register"]
pub struct RPC0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC0R register"]
pub mod rpc0r;
#[doc = "RPC1R register"]
pub struct RPC1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC1R register"]
pub mod rpc1r;
#[doc = "RPC2R register"]
pub struct RPC2R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC2R register"]
pub mod rpc2r;
#[doc = "RPC3R register"]
pub struct RPC3R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC3R register"]
pub mod rpc3r;
#[doc = "RPC4R register"]
pub struct RPC4R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC4R register"]
pub mod rpc4r;
#[doc = "RPC5R register"]
pub struct RPC5R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC5R register"]
pub mod rpc5r;
#[doc = "RPC6R register"]
pub struct RPC6R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC6R register"]
pub mod rpc6r;
#[doc = "RPC7R register"]
pub struct RPC7R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC7R register"]
pub mod rpc7r;
#[doc = "RPC8R register"]
pub struct RPC8R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC8R register"]
pub mod rpc8r;
#[doc = "RPC9R register"]
pub struct RPC9R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RPC9R register"]
pub mod rpc9r;
