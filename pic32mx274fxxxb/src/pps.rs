#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - INT1R register"]
    pub int1r: crate::Reg<int1r::INT1R_SPEC>,
    #[doc = "0x04 - INT2R register"]
    pub int2r: crate::Reg<int2r::INT2R_SPEC>,
    #[doc = "0x08 - INT3R register"]
    pub int3r: crate::Reg<int3r::INT3R_SPEC>,
    #[doc = "0x0c - INT4R register"]
    pub int4r: crate::Reg<int4r::INT4R_SPEC>,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - T2CKR register"]
    pub t2ckr: crate::Reg<t2ckr::T2CKR_SPEC>,
    #[doc = "0x18 - T3CKR register"]
    pub t3ckr: crate::Reg<t3ckr::T3CKR_SPEC>,
    #[doc = "0x1c - T4CKR register"]
    pub t4ckr: crate::Reg<t4ckr::T4CKR_SPEC>,
    #[doc = "0x20 - T5CKR register"]
    pub t5ckr: crate::Reg<t5ckr::T5CKR_SPEC>,
    #[doc = "0x24 - IC1R register"]
    pub ic1r: crate::Reg<ic1r::IC1R_SPEC>,
    #[doc = "0x28 - IC2R register"]
    pub ic2r: crate::Reg<ic2r::IC2R_SPEC>,
    #[doc = "0x2c - IC3R register"]
    pub ic3r: crate::Reg<ic3r::IC3R_SPEC>,
    #[doc = "0x30 - IC4R register"]
    pub ic4r: crate::Reg<ic4r::IC4R_SPEC>,
    #[doc = "0x34 - IC5R register"]
    pub ic5r: crate::Reg<ic5r::IC5R_SPEC>,
    _reserved13: [u8; 12usize],
    #[doc = "0x44 - OCFAR register"]
    pub ocfar: crate::Reg<ocfar::OCFAR_SPEC>,
    #[doc = "0x48 - OCFBR register"]
    pub ocfbr: crate::Reg<ocfbr::OCFBR_SPEC>,
    #[doc = "0x4c - U1RXR register"]
    pub u1rxr: crate::Reg<u1rxr::U1RXR_SPEC>,
    #[doc = "0x50 - U1CTSR register"]
    pub u1ctsr: crate::Reg<u1ctsr::U1CTSR_SPEC>,
    #[doc = "0x54 - U2RXR register"]
    pub u2rxr: crate::Reg<u2rxr::U2RXR_SPEC>,
    #[doc = "0x58 - U2CTSR register"]
    pub u2ctsr: crate::Reg<u2ctsr::U2CTSR_SPEC>,
    _reserved19: [u8; 36usize],
    #[doc = "0x80 - SDI1R register"]
    pub sdi1r: crate::Reg<sdi1r::SDI1R_SPEC>,
    #[doc = "0x84 - SS1R register"]
    pub ss1r: crate::Reg<ss1r::SS1R_SPEC>,
    _reserved21: [u8; 4usize],
    #[doc = "0x8c - SDI2R register"]
    pub sdi2r: crate::Reg<sdi2r::SDI2R_SPEC>,
    #[doc = "0x90 - SS2R register"]
    pub ss2r: crate::Reg<ss2r::SS2R_SPEC>,
    _reserved23: [u8; 32usize],
    #[doc = "0xb4 - REFCLKIR register"]
    pub refclkir: crate::Reg<refclkir::REFCLKIR_SPEC>,
    _reserved24: [u8; 68usize],
    #[doc = "0xfc - RPA0R register"]
    pub rpa0r: crate::Reg<rpa0r::RPA0R_SPEC>,
    #[doc = "0x100 - RPA1R register"]
    pub rpa1r: crate::Reg<rpa1r::RPA1R_SPEC>,
    #[doc = "0x104 - RPA2R register"]
    pub rpa2r: crate::Reg<rpa2r::RPA2R_SPEC>,
    #[doc = "0x108 - RPA3R register"]
    pub rpa3r: crate::Reg<rpa3r::RPA3R_SPEC>,
    #[doc = "0x10c - RPA4R register"]
    pub rpa4r: crate::Reg<rpa4r::RPA4R_SPEC>,
    _reserved29: [u8; 12usize],
    #[doc = "0x11c - RPA8R register"]
    pub rpa8r: crate::Reg<rpa8r::RPA8R_SPEC>,
    #[doc = "0x120 - RPA9R register"]
    pub rpa9r: crate::Reg<rpa9r::RPA9R_SPEC>,
    _reserved31: [u8; 4usize],
    #[doc = "0x128 - RPB0R register"]
    pub rpb0r: crate::Reg<rpb0r::RPB0R_SPEC>,
    #[doc = "0x12c - RPB1R register"]
    pub rpb1r: crate::Reg<rpb1r::RPB1R_SPEC>,
    #[doc = "0x130 - RPB2R register"]
    pub rpb2r: crate::Reg<rpb2r::RPB2R_SPEC>,
    #[doc = "0x134 - RPB3R register"]
    pub rpb3r: crate::Reg<rpb3r::RPB3R_SPEC>,
    #[doc = "0x138 - RPB4R register"]
    pub rpb4r: crate::Reg<rpb4r::RPB4R_SPEC>,
    #[doc = "0x13c - RPB5R register"]
    pub rpb5r: crate::Reg<rpb5r::RPB5R_SPEC>,
    #[doc = "0x140 - RPB6R register"]
    pub rpb6r: crate::Reg<rpb6r::RPB6R_SPEC>,
    #[doc = "0x144 - RPB7R register"]
    pub rpb7r: crate::Reg<rpb7r::RPB7R_SPEC>,
    #[doc = "0x148 - RPB8R register"]
    pub rpb8r: crate::Reg<rpb8r::RPB8R_SPEC>,
    #[doc = "0x14c - RPB9R register"]
    pub rpb9r: crate::Reg<rpb9r::RPB9R_SPEC>,
    #[doc = "0x150 - RPB10R register"]
    pub rpb10r: crate::Reg<rpb10r::RPB10R_SPEC>,
    #[doc = "0x154 - RPB11R register"]
    pub rpb11r: crate::Reg<rpb11r::RPB11R_SPEC>,
    _reserved43: [u8; 4usize],
    #[doc = "0x15c - RPB13R register"]
    pub rpb13r: crate::Reg<rpb13r::RPB13R_SPEC>,
    #[doc = "0x160 - RPB14R register"]
    pub rpb14r: crate::Reg<rpb14r::RPB14R_SPEC>,
    #[doc = "0x164 - RPB15R register"]
    pub rpb15r: crate::Reg<rpb15r::RPB15R_SPEC>,
    #[doc = "0x168 - RPC0R register"]
    pub rpc0r: crate::Reg<rpc0r::RPC0R_SPEC>,
    #[doc = "0x16c - RPC1R register"]
    pub rpc1r: crate::Reg<rpc1r::RPC1R_SPEC>,
    #[doc = "0x170 - RPC2R register"]
    pub rpc2r: crate::Reg<rpc2r::RPC2R_SPEC>,
    #[doc = "0x174 - RPC3R register"]
    pub rpc3r: crate::Reg<rpc3r::RPC3R_SPEC>,
    #[doc = "0x178 - RPC4R register"]
    pub rpc4r: crate::Reg<rpc4r::RPC4R_SPEC>,
    #[doc = "0x17c - RPC5R register"]
    pub rpc5r: crate::Reg<rpc5r::RPC5R_SPEC>,
    #[doc = "0x180 - RPC6R register"]
    pub rpc6r: crate::Reg<rpc6r::RPC6R_SPEC>,
    #[doc = "0x184 - RPC7R register"]
    pub rpc7r: crate::Reg<rpc7r::RPC7R_SPEC>,
    #[doc = "0x188 - RPC8R register"]
    pub rpc8r: crate::Reg<rpc8r::RPC8R_SPEC>,
    #[doc = "0x18c - RPC9R register"]
    pub rpc9r: crate::Reg<rpc9r::RPC9R_SPEC>,
}
#[doc = "INT1R register accessor: an alias for `Reg<INT1R_SPEC>`"]
pub type INT1R = crate::Reg<int1r::INT1R_SPEC>;
#[doc = "INT1R register"]
pub mod int1r;
#[doc = "INT2R register accessor: an alias for `Reg<INT2R_SPEC>`"]
pub type INT2R = crate::Reg<int2r::INT2R_SPEC>;
#[doc = "INT2R register"]
pub mod int2r;
#[doc = "INT3R register accessor: an alias for `Reg<INT3R_SPEC>`"]
pub type INT3R = crate::Reg<int3r::INT3R_SPEC>;
#[doc = "INT3R register"]
pub mod int3r;
#[doc = "INT4R register accessor: an alias for `Reg<INT4R_SPEC>`"]
pub type INT4R = crate::Reg<int4r::INT4R_SPEC>;
#[doc = "INT4R register"]
pub mod int4r;
#[doc = "T2CKR register accessor: an alias for `Reg<T2CKR_SPEC>`"]
pub type T2CKR = crate::Reg<t2ckr::T2CKR_SPEC>;
#[doc = "T2CKR register"]
pub mod t2ckr;
#[doc = "T3CKR register accessor: an alias for `Reg<T3CKR_SPEC>`"]
pub type T3CKR = crate::Reg<t3ckr::T3CKR_SPEC>;
#[doc = "T3CKR register"]
pub mod t3ckr;
#[doc = "T4CKR register accessor: an alias for `Reg<T4CKR_SPEC>`"]
pub type T4CKR = crate::Reg<t4ckr::T4CKR_SPEC>;
#[doc = "T4CKR register"]
pub mod t4ckr;
#[doc = "T5CKR register accessor: an alias for `Reg<T5CKR_SPEC>`"]
pub type T5CKR = crate::Reg<t5ckr::T5CKR_SPEC>;
#[doc = "T5CKR register"]
pub mod t5ckr;
#[doc = "IC1R register accessor: an alias for `Reg<IC1R_SPEC>`"]
pub type IC1R = crate::Reg<ic1r::IC1R_SPEC>;
#[doc = "IC1R register"]
pub mod ic1r;
#[doc = "IC2R register accessor: an alias for `Reg<IC2R_SPEC>`"]
pub type IC2R = crate::Reg<ic2r::IC2R_SPEC>;
#[doc = "IC2R register"]
pub mod ic2r;
#[doc = "IC3R register accessor: an alias for `Reg<IC3R_SPEC>`"]
pub type IC3R = crate::Reg<ic3r::IC3R_SPEC>;
#[doc = "IC3R register"]
pub mod ic3r;
#[doc = "IC4R register accessor: an alias for `Reg<IC4R_SPEC>`"]
pub type IC4R = crate::Reg<ic4r::IC4R_SPEC>;
#[doc = "IC4R register"]
pub mod ic4r;
#[doc = "IC5R register accessor: an alias for `Reg<IC5R_SPEC>`"]
pub type IC5R = crate::Reg<ic5r::IC5R_SPEC>;
#[doc = "IC5R register"]
pub mod ic5r;
#[doc = "OCFAR register accessor: an alias for `Reg<OCFAR_SPEC>`"]
pub type OCFAR = crate::Reg<ocfar::OCFAR_SPEC>;
#[doc = "OCFAR register"]
pub mod ocfar;
#[doc = "OCFBR register accessor: an alias for `Reg<OCFBR_SPEC>`"]
pub type OCFBR = crate::Reg<ocfbr::OCFBR_SPEC>;
#[doc = "OCFBR register"]
pub mod ocfbr;
#[doc = "U1RXR register accessor: an alias for `Reg<U1RXR_SPEC>`"]
pub type U1RXR = crate::Reg<u1rxr::U1RXR_SPEC>;
#[doc = "U1RXR register"]
pub mod u1rxr;
#[doc = "U1CTSR register accessor: an alias for `Reg<U1CTSR_SPEC>`"]
pub type U1CTSR = crate::Reg<u1ctsr::U1CTSR_SPEC>;
#[doc = "U1CTSR register"]
pub mod u1ctsr;
#[doc = "U2RXR register accessor: an alias for `Reg<U2RXR_SPEC>`"]
pub type U2RXR = crate::Reg<u2rxr::U2RXR_SPEC>;
#[doc = "U2RXR register"]
pub mod u2rxr;
#[doc = "U2CTSR register accessor: an alias for `Reg<U2CTSR_SPEC>`"]
pub type U2CTSR = crate::Reg<u2ctsr::U2CTSR_SPEC>;
#[doc = "U2CTSR register"]
pub mod u2ctsr;
#[doc = "SDI1R register accessor: an alias for `Reg<SDI1R_SPEC>`"]
pub type SDI1R = crate::Reg<sdi1r::SDI1R_SPEC>;
#[doc = "SDI1R register"]
pub mod sdi1r;
#[doc = "SS1R register accessor: an alias for `Reg<SS1R_SPEC>`"]
pub type SS1R = crate::Reg<ss1r::SS1R_SPEC>;
#[doc = "SS1R register"]
pub mod ss1r;
#[doc = "SDI2R register accessor: an alias for `Reg<SDI2R_SPEC>`"]
pub type SDI2R = crate::Reg<sdi2r::SDI2R_SPEC>;
#[doc = "SDI2R register"]
pub mod sdi2r;
#[doc = "SS2R register accessor: an alias for `Reg<SS2R_SPEC>`"]
pub type SS2R = crate::Reg<ss2r::SS2R_SPEC>;
#[doc = "SS2R register"]
pub mod ss2r;
#[doc = "REFCLKIR register accessor: an alias for `Reg<REFCLKIR_SPEC>`"]
pub type REFCLKIR = crate::Reg<refclkir::REFCLKIR_SPEC>;
#[doc = "REFCLKIR register"]
pub mod refclkir;
#[doc = "RPA0R register accessor: an alias for `Reg<RPA0R_SPEC>`"]
pub type RPA0R = crate::Reg<rpa0r::RPA0R_SPEC>;
#[doc = "RPA0R register"]
pub mod rpa0r;
#[doc = "RPA1R register accessor: an alias for `Reg<RPA1R_SPEC>`"]
pub type RPA1R = crate::Reg<rpa1r::RPA1R_SPEC>;
#[doc = "RPA1R register"]
pub mod rpa1r;
#[doc = "RPA2R register accessor: an alias for `Reg<RPA2R_SPEC>`"]
pub type RPA2R = crate::Reg<rpa2r::RPA2R_SPEC>;
#[doc = "RPA2R register"]
pub mod rpa2r;
#[doc = "RPA3R register accessor: an alias for `Reg<RPA3R_SPEC>`"]
pub type RPA3R = crate::Reg<rpa3r::RPA3R_SPEC>;
#[doc = "RPA3R register"]
pub mod rpa3r;
#[doc = "RPA4R register accessor: an alias for `Reg<RPA4R_SPEC>`"]
pub type RPA4R = crate::Reg<rpa4r::RPA4R_SPEC>;
#[doc = "RPA4R register"]
pub mod rpa4r;
#[doc = "RPA8R register accessor: an alias for `Reg<RPA8R_SPEC>`"]
pub type RPA8R = crate::Reg<rpa8r::RPA8R_SPEC>;
#[doc = "RPA8R register"]
pub mod rpa8r;
#[doc = "RPA9R register accessor: an alias for `Reg<RPA9R_SPEC>`"]
pub type RPA9R = crate::Reg<rpa9r::RPA9R_SPEC>;
#[doc = "RPA9R register"]
pub mod rpa9r;
#[doc = "RPB0R register accessor: an alias for `Reg<RPB0R_SPEC>`"]
pub type RPB0R = crate::Reg<rpb0r::RPB0R_SPEC>;
#[doc = "RPB0R register"]
pub mod rpb0r;
#[doc = "RPB1R register accessor: an alias for `Reg<RPB1R_SPEC>`"]
pub type RPB1R = crate::Reg<rpb1r::RPB1R_SPEC>;
#[doc = "RPB1R register"]
pub mod rpb1r;
#[doc = "RPB2R register accessor: an alias for `Reg<RPB2R_SPEC>`"]
pub type RPB2R = crate::Reg<rpb2r::RPB2R_SPEC>;
#[doc = "RPB2R register"]
pub mod rpb2r;
#[doc = "RPB3R register accessor: an alias for `Reg<RPB3R_SPEC>`"]
pub type RPB3R = crate::Reg<rpb3r::RPB3R_SPEC>;
#[doc = "RPB3R register"]
pub mod rpb3r;
#[doc = "RPB4R register accessor: an alias for `Reg<RPB4R_SPEC>`"]
pub type RPB4R = crate::Reg<rpb4r::RPB4R_SPEC>;
#[doc = "RPB4R register"]
pub mod rpb4r;
#[doc = "RPB5R register accessor: an alias for `Reg<RPB5R_SPEC>`"]
pub type RPB5R = crate::Reg<rpb5r::RPB5R_SPEC>;
#[doc = "RPB5R register"]
pub mod rpb5r;
#[doc = "RPB6R register accessor: an alias for `Reg<RPB6R_SPEC>`"]
pub type RPB6R = crate::Reg<rpb6r::RPB6R_SPEC>;
#[doc = "RPB6R register"]
pub mod rpb6r;
#[doc = "RPB7R register accessor: an alias for `Reg<RPB7R_SPEC>`"]
pub type RPB7R = crate::Reg<rpb7r::RPB7R_SPEC>;
#[doc = "RPB7R register"]
pub mod rpb7r;
#[doc = "RPB8R register accessor: an alias for `Reg<RPB8R_SPEC>`"]
pub type RPB8R = crate::Reg<rpb8r::RPB8R_SPEC>;
#[doc = "RPB8R register"]
pub mod rpb8r;
#[doc = "RPB9R register accessor: an alias for `Reg<RPB9R_SPEC>`"]
pub type RPB9R = crate::Reg<rpb9r::RPB9R_SPEC>;
#[doc = "RPB9R register"]
pub mod rpb9r;
#[doc = "RPB10R register accessor: an alias for `Reg<RPB10R_SPEC>`"]
pub type RPB10R = crate::Reg<rpb10r::RPB10R_SPEC>;
#[doc = "RPB10R register"]
pub mod rpb10r;
#[doc = "RPB11R register accessor: an alias for `Reg<RPB11R_SPEC>`"]
pub type RPB11R = crate::Reg<rpb11r::RPB11R_SPEC>;
#[doc = "RPB11R register"]
pub mod rpb11r;
#[doc = "RPB13R register accessor: an alias for `Reg<RPB13R_SPEC>`"]
pub type RPB13R = crate::Reg<rpb13r::RPB13R_SPEC>;
#[doc = "RPB13R register"]
pub mod rpb13r;
#[doc = "RPB14R register accessor: an alias for `Reg<RPB14R_SPEC>`"]
pub type RPB14R = crate::Reg<rpb14r::RPB14R_SPEC>;
#[doc = "RPB14R register"]
pub mod rpb14r;
#[doc = "RPB15R register accessor: an alias for `Reg<RPB15R_SPEC>`"]
pub type RPB15R = crate::Reg<rpb15r::RPB15R_SPEC>;
#[doc = "RPB15R register"]
pub mod rpb15r;
#[doc = "RPC0R register accessor: an alias for `Reg<RPC0R_SPEC>`"]
pub type RPC0R = crate::Reg<rpc0r::RPC0R_SPEC>;
#[doc = "RPC0R register"]
pub mod rpc0r;
#[doc = "RPC1R register accessor: an alias for `Reg<RPC1R_SPEC>`"]
pub type RPC1R = crate::Reg<rpc1r::RPC1R_SPEC>;
#[doc = "RPC1R register"]
pub mod rpc1r;
#[doc = "RPC2R register accessor: an alias for `Reg<RPC2R_SPEC>`"]
pub type RPC2R = crate::Reg<rpc2r::RPC2R_SPEC>;
#[doc = "RPC2R register"]
pub mod rpc2r;
#[doc = "RPC3R register accessor: an alias for `Reg<RPC3R_SPEC>`"]
pub type RPC3R = crate::Reg<rpc3r::RPC3R_SPEC>;
#[doc = "RPC3R register"]
pub mod rpc3r;
#[doc = "RPC4R register accessor: an alias for `Reg<RPC4R_SPEC>`"]
pub type RPC4R = crate::Reg<rpc4r::RPC4R_SPEC>;
#[doc = "RPC4R register"]
pub mod rpc4r;
#[doc = "RPC5R register accessor: an alias for `Reg<RPC5R_SPEC>`"]
pub type RPC5R = crate::Reg<rpc5r::RPC5R_SPEC>;
#[doc = "RPC5R register"]
pub mod rpc5r;
#[doc = "RPC6R register accessor: an alias for `Reg<RPC6R_SPEC>`"]
pub type RPC6R = crate::Reg<rpc6r::RPC6R_SPEC>;
#[doc = "RPC6R register"]
pub mod rpc6r;
#[doc = "RPC7R register accessor: an alias for `Reg<RPC7R_SPEC>`"]
pub type RPC7R = crate::Reg<rpc7r::RPC7R_SPEC>;
#[doc = "RPC7R register"]
pub mod rpc7r;
#[doc = "RPC8R register accessor: an alias for `Reg<RPC8R_SPEC>`"]
pub type RPC8R = crate::Reg<rpc8r::RPC8R_SPEC>;
#[doc = "RPC8R register"]
pub mod rpc8r;
#[doc = "RPC9R register accessor: an alias for `Reg<RPC9R_SPEC>`"]
pub type RPC9R = crate::Reg<rpc9r::RPC9R_SPEC>;
#[doc = "RPC9R register"]
pub mod rpc9r;
