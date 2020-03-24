#[doc = r"Register block"]
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
    _reserved4: [u8; 4usize],
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
    _reserved13: [u8; 12usize],
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
    _reserved19: [u8; 36usize],
    #[doc = "0x80 - SDI1R register"]
    pub sdi1r: SDI1R,
    #[doc = "0x84 - SS1R register"]
    pub ss1r: SS1R,
    _reserved21: [u8; 4usize],
    #[doc = "0x8c - SDI2R register"]
    pub sdi2r: SDI2R,
    #[doc = "0x90 - SS2R register"]
    pub ss2r: SS2R,
    _reserved23: [u8; 32usize],
    #[doc = "0xb4 - REFCLKIR register"]
    pub refclkir: REFCLKIR,
    _reserved24: [u8; 68usize],
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
    _reserved29: [u8; 12usize],
    #[doc = "0x11c - RPA8R register"]
    pub rpa8r: RPA8R,
    #[doc = "0x120 - RPA9R register"]
    pub rpa9r: RPA9R,
    _reserved31: [u8; 4usize],
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
    _reserved43: [u8; 4usize],
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
#[doc = "INT1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1r](int1r) module"]
pub type INT1R = crate::Reg<u32, _INT1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT1R;
#[doc = "`read()` method returns [int1r::R](int1r::R) reader structure"]
impl crate::Readable for INT1R {}
#[doc = "`write(|w| ..)` method takes [int1r::W](int1r::W) writer structure"]
impl crate::Writable for INT1R {}
#[doc = "INT1R register"]
pub mod int1r;
#[doc = "INT2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int2r](int2r) module"]
pub type INT2R = crate::Reg<u32, _INT2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT2R;
#[doc = "`read()` method returns [int2r::R](int2r::R) reader structure"]
impl crate::Readable for INT2R {}
#[doc = "`write(|w| ..)` method takes [int2r::W](int2r::W) writer structure"]
impl crate::Writable for INT2R {}
#[doc = "INT2R register"]
pub mod int2r;
#[doc = "INT3R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int3r](int3r) module"]
pub type INT3R = crate::Reg<u32, _INT3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT3R;
#[doc = "`read()` method returns [int3r::R](int3r::R) reader structure"]
impl crate::Readable for INT3R {}
#[doc = "`write(|w| ..)` method takes [int3r::W](int3r::W) writer structure"]
impl crate::Writable for INT3R {}
#[doc = "INT3R register"]
pub mod int3r;
#[doc = "INT4R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int4r](int4r) module"]
pub type INT4R = crate::Reg<u32, _INT4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT4R;
#[doc = "`read()` method returns [int4r::R](int4r::R) reader structure"]
impl crate::Readable for INT4R {}
#[doc = "`write(|w| ..)` method takes [int4r::W](int4r::W) writer structure"]
impl crate::Writable for INT4R {}
#[doc = "INT4R register"]
pub mod int4r;
#[doc = "T2CKR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2ckr](t2ckr) module"]
pub type T2CKR = crate::Reg<u32, _T2CKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T2CKR;
#[doc = "`read()` method returns [t2ckr::R](t2ckr::R) reader structure"]
impl crate::Readable for T2CKR {}
#[doc = "`write(|w| ..)` method takes [t2ckr::W](t2ckr::W) writer structure"]
impl crate::Writable for T2CKR {}
#[doc = "T2CKR register"]
pub mod t2ckr;
#[doc = "T3CKR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3ckr](t3ckr) module"]
pub type T3CKR = crate::Reg<u32, _T3CKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T3CKR;
#[doc = "`read()` method returns [t3ckr::R](t3ckr::R) reader structure"]
impl crate::Readable for T3CKR {}
#[doc = "`write(|w| ..)` method takes [t3ckr::W](t3ckr::W) writer structure"]
impl crate::Writable for T3CKR {}
#[doc = "T3CKR register"]
pub mod t3ckr;
#[doc = "T4CKR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t4ckr](t4ckr) module"]
pub type T4CKR = crate::Reg<u32, _T4CKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T4CKR;
#[doc = "`read()` method returns [t4ckr::R](t4ckr::R) reader structure"]
impl crate::Readable for T4CKR {}
#[doc = "`write(|w| ..)` method takes [t4ckr::W](t4ckr::W) writer structure"]
impl crate::Writable for T4CKR {}
#[doc = "T4CKR register"]
pub mod t4ckr;
#[doc = "T5CKR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t5ckr](t5ckr) module"]
pub type T5CKR = crate::Reg<u32, _T5CKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T5CKR;
#[doc = "`read()` method returns [t5ckr::R](t5ckr::R) reader structure"]
impl crate::Readable for T5CKR {}
#[doc = "`write(|w| ..)` method takes [t5ckr::W](t5ckr::W) writer structure"]
impl crate::Writable for T5CKR {}
#[doc = "T5CKR register"]
pub mod t5ckr;
#[doc = "IC1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1r](ic1r) module"]
pub type IC1R = crate::Reg<u32, _IC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC1R;
#[doc = "`read()` method returns [ic1r::R](ic1r::R) reader structure"]
impl crate::Readable for IC1R {}
#[doc = "`write(|w| ..)` method takes [ic1r::W](ic1r::W) writer structure"]
impl crate::Writable for IC1R {}
#[doc = "IC1R register"]
pub mod ic1r;
#[doc = "IC2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic2r](ic2r) module"]
pub type IC2R = crate::Reg<u32, _IC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC2R;
#[doc = "`read()` method returns [ic2r::R](ic2r::R) reader structure"]
impl crate::Readable for IC2R {}
#[doc = "`write(|w| ..)` method takes [ic2r::W](ic2r::W) writer structure"]
impl crate::Writable for IC2R {}
#[doc = "IC2R register"]
pub mod ic2r;
#[doc = "IC3R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic3r](ic3r) module"]
pub type IC3R = crate::Reg<u32, _IC3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC3R;
#[doc = "`read()` method returns [ic3r::R](ic3r::R) reader structure"]
impl crate::Readable for IC3R {}
#[doc = "`write(|w| ..)` method takes [ic3r::W](ic3r::W) writer structure"]
impl crate::Writable for IC3R {}
#[doc = "IC3R register"]
pub mod ic3r;
#[doc = "IC4R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic4r](ic4r) module"]
pub type IC4R = crate::Reg<u32, _IC4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC4R;
#[doc = "`read()` method returns [ic4r::R](ic4r::R) reader structure"]
impl crate::Readable for IC4R {}
#[doc = "`write(|w| ..)` method takes [ic4r::W](ic4r::W) writer structure"]
impl crate::Writable for IC4R {}
#[doc = "IC4R register"]
pub mod ic4r;
#[doc = "IC5R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic5r](ic5r) module"]
pub type IC5R = crate::Reg<u32, _IC5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC5R;
#[doc = "`read()` method returns [ic5r::R](ic5r::R) reader structure"]
impl crate::Readable for IC5R {}
#[doc = "`write(|w| ..)` method takes [ic5r::W](ic5r::W) writer structure"]
impl crate::Writable for IC5R {}
#[doc = "IC5R register"]
pub mod ic5r;
#[doc = "OCFAR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocfar](ocfar) module"]
pub type OCFAR = crate::Reg<u32, _OCFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCFAR;
#[doc = "`read()` method returns [ocfar::R](ocfar::R) reader structure"]
impl crate::Readable for OCFAR {}
#[doc = "`write(|w| ..)` method takes [ocfar::W](ocfar::W) writer structure"]
impl crate::Writable for OCFAR {}
#[doc = "OCFAR register"]
pub mod ocfar;
#[doc = "OCFBR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocfbr](ocfbr) module"]
pub type OCFBR = crate::Reg<u32, _OCFBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCFBR;
#[doc = "`read()` method returns [ocfbr::R](ocfbr::R) reader structure"]
impl crate::Readable for OCFBR {}
#[doc = "`write(|w| ..)` method takes [ocfbr::W](ocfbr::W) writer structure"]
impl crate::Writable for OCFBR {}
#[doc = "OCFBR register"]
pub mod ocfbr;
#[doc = "U1RXR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1rxr](u1rxr) module"]
pub type U1RXR = crate::Reg<u32, _U1RXR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1RXR;
#[doc = "`read()` method returns [u1rxr::R](u1rxr::R) reader structure"]
impl crate::Readable for U1RXR {}
#[doc = "`write(|w| ..)` method takes [u1rxr::W](u1rxr::W) writer structure"]
impl crate::Writable for U1RXR {}
#[doc = "U1RXR register"]
pub mod u1rxr;
#[doc = "U1CTSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ctsr](u1ctsr) module"]
pub type U1CTSR = crate::Reg<u32, _U1CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CTSR;
#[doc = "`read()` method returns [u1ctsr::R](u1ctsr::R) reader structure"]
impl crate::Readable for U1CTSR {}
#[doc = "`write(|w| ..)` method takes [u1ctsr::W](u1ctsr::W) writer structure"]
impl crate::Writable for U1CTSR {}
#[doc = "U1CTSR register"]
pub mod u1ctsr;
#[doc = "U2RXR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u2rxr](u2rxr) module"]
pub type U2RXR = crate::Reg<u32, _U2RXR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U2RXR;
#[doc = "`read()` method returns [u2rxr::R](u2rxr::R) reader structure"]
impl crate::Readable for U2RXR {}
#[doc = "`write(|w| ..)` method takes [u2rxr::W](u2rxr::W) writer structure"]
impl crate::Writable for U2RXR {}
#[doc = "U2RXR register"]
pub mod u2rxr;
#[doc = "U2CTSR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u2ctsr](u2ctsr) module"]
pub type U2CTSR = crate::Reg<u32, _U2CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U2CTSR;
#[doc = "`read()` method returns [u2ctsr::R](u2ctsr::R) reader structure"]
impl crate::Readable for U2CTSR {}
#[doc = "`write(|w| ..)` method takes [u2ctsr::W](u2ctsr::W) writer structure"]
impl crate::Writable for U2CTSR {}
#[doc = "U2CTSR register"]
pub mod u2ctsr;
#[doc = "SDI1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdi1r](sdi1r) module"]
pub type SDI1R = crate::Reg<u32, _SDI1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDI1R;
#[doc = "`read()` method returns [sdi1r::R](sdi1r::R) reader structure"]
impl crate::Readable for SDI1R {}
#[doc = "`write(|w| ..)` method takes [sdi1r::W](sdi1r::W) writer structure"]
impl crate::Writable for SDI1R {}
#[doc = "SDI1R register"]
pub mod sdi1r;
#[doc = "SS1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss1r](ss1r) module"]
pub type SS1R = crate::Reg<u32, _SS1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SS1R;
#[doc = "`read()` method returns [ss1r::R](ss1r::R) reader structure"]
impl crate::Readable for SS1R {}
#[doc = "`write(|w| ..)` method takes [ss1r::W](ss1r::W) writer structure"]
impl crate::Writable for SS1R {}
#[doc = "SS1R register"]
pub mod ss1r;
#[doc = "SDI2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdi2r](sdi2r) module"]
pub type SDI2R = crate::Reg<u32, _SDI2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDI2R;
#[doc = "`read()` method returns [sdi2r::R](sdi2r::R) reader structure"]
impl crate::Readable for SDI2R {}
#[doc = "`write(|w| ..)` method takes [sdi2r::W](sdi2r::W) writer structure"]
impl crate::Writable for SDI2R {}
#[doc = "SDI2R register"]
pub mod sdi2r;
#[doc = "SS2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss2r](ss2r) module"]
pub type SS2R = crate::Reg<u32, _SS2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SS2R;
#[doc = "`read()` method returns [ss2r::R](ss2r::R) reader structure"]
impl crate::Readable for SS2R {}
#[doc = "`write(|w| ..)` method takes [ss2r::W](ss2r::W) writer structure"]
impl crate::Writable for SS2R {}
#[doc = "SS2R register"]
pub mod ss2r;
#[doc = "REFCLKIR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refclkir](refclkir) module"]
pub type REFCLKIR = crate::Reg<u32, _REFCLKIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFCLKIR;
#[doc = "`read()` method returns [refclkir::R](refclkir::R) reader structure"]
impl crate::Readable for REFCLKIR {}
#[doc = "`write(|w| ..)` method takes [refclkir::W](refclkir::W) writer structure"]
impl crate::Writable for REFCLKIR {}
#[doc = "REFCLKIR register"]
pub mod refclkir;
#[doc = "RPA0R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa0r](rpa0r) module"]
pub type RPA0R = crate::Reg<u32, _RPA0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPA0R;
#[doc = "`read()` method returns [rpa0r::R](rpa0r::R) reader structure"]
impl crate::Readable for RPA0R {}
#[doc = "`write(|w| ..)` method takes [rpa0r::W](rpa0r::W) writer structure"]
impl crate::Writable for RPA0R {}
#[doc = "RPA0R register"]
pub mod rpa0r;
#[doc = "RPA1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa1r](rpa1r) module"]
pub type RPA1R = crate::Reg<u32, _RPA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPA1R;
#[doc = "`read()` method returns [rpa1r::R](rpa1r::R) reader structure"]
impl crate::Readable for RPA1R {}
#[doc = "`write(|w| ..)` method takes [rpa1r::W](rpa1r::W) writer structure"]
impl crate::Writable for RPA1R {}
#[doc = "RPA1R register"]
pub mod rpa1r;
#[doc = "RPA2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa2r](rpa2r) module"]
pub type RPA2R = crate::Reg<u32, _RPA2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPA2R;
#[doc = "`read()` method returns [rpa2r::R](rpa2r::R) reader structure"]
impl crate::Readable for RPA2R {}
#[doc = "`write(|w| ..)` method takes [rpa2r::W](rpa2r::W) writer structure"]
impl crate::Writable for RPA2R {}
#[doc = "RPA2R register"]
pub mod rpa2r;
#[doc = "RPA3R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa3r](rpa3r) module"]
pub type RPA3R = crate::Reg<u32, _RPA3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPA3R;
#[doc = "`read()` method returns [rpa3r::R](rpa3r::R) reader structure"]
impl crate::Readable for RPA3R {}
#[doc = "`write(|w| ..)` method takes [rpa3r::W](rpa3r::W) writer structure"]
impl crate::Writable for RPA3R {}
#[doc = "RPA3R register"]
pub mod rpa3r;
#[doc = "RPA4R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa4r](rpa4r) module"]
pub type RPA4R = crate::Reg<u32, _RPA4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPA4R;
#[doc = "`read()` method returns [rpa4r::R](rpa4r::R) reader structure"]
impl crate::Readable for RPA4R {}
#[doc = "`write(|w| ..)` method takes [rpa4r::W](rpa4r::W) writer structure"]
impl crate::Writable for RPA4R {}
#[doc = "RPA4R register"]
pub mod rpa4r;
#[doc = "RPA8R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa8r](rpa8r) module"]
pub type RPA8R = crate::Reg<u32, _RPA8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPA8R;
#[doc = "`read()` method returns [rpa8r::R](rpa8r::R) reader structure"]
impl crate::Readable for RPA8R {}
#[doc = "`write(|w| ..)` method takes [rpa8r::W](rpa8r::W) writer structure"]
impl crate::Writable for RPA8R {}
#[doc = "RPA8R register"]
pub mod rpa8r;
#[doc = "RPA9R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa9r](rpa9r) module"]
pub type RPA9R = crate::Reg<u32, _RPA9R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPA9R;
#[doc = "`read()` method returns [rpa9r::R](rpa9r::R) reader structure"]
impl crate::Readable for RPA9R {}
#[doc = "`write(|w| ..)` method takes [rpa9r::W](rpa9r::W) writer structure"]
impl crate::Writable for RPA9R {}
#[doc = "RPA9R register"]
pub mod rpa9r;
#[doc = "RPB0R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb0r](rpb0r) module"]
pub type RPB0R = crate::Reg<u32, _RPB0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB0R;
#[doc = "`read()` method returns [rpb0r::R](rpb0r::R) reader structure"]
impl crate::Readable for RPB0R {}
#[doc = "`write(|w| ..)` method takes [rpb0r::W](rpb0r::W) writer structure"]
impl crate::Writable for RPB0R {}
#[doc = "RPB0R register"]
pub mod rpb0r;
#[doc = "RPB1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb1r](rpb1r) module"]
pub type RPB1R = crate::Reg<u32, _RPB1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB1R;
#[doc = "`read()` method returns [rpb1r::R](rpb1r::R) reader structure"]
impl crate::Readable for RPB1R {}
#[doc = "`write(|w| ..)` method takes [rpb1r::W](rpb1r::W) writer structure"]
impl crate::Writable for RPB1R {}
#[doc = "RPB1R register"]
pub mod rpb1r;
#[doc = "RPB2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb2r](rpb2r) module"]
pub type RPB2R = crate::Reg<u32, _RPB2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB2R;
#[doc = "`read()` method returns [rpb2r::R](rpb2r::R) reader structure"]
impl crate::Readable for RPB2R {}
#[doc = "`write(|w| ..)` method takes [rpb2r::W](rpb2r::W) writer structure"]
impl crate::Writable for RPB2R {}
#[doc = "RPB2R register"]
pub mod rpb2r;
#[doc = "RPB3R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb3r](rpb3r) module"]
pub type RPB3R = crate::Reg<u32, _RPB3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB3R;
#[doc = "`read()` method returns [rpb3r::R](rpb3r::R) reader structure"]
impl crate::Readable for RPB3R {}
#[doc = "`write(|w| ..)` method takes [rpb3r::W](rpb3r::W) writer structure"]
impl crate::Writable for RPB3R {}
#[doc = "RPB3R register"]
pub mod rpb3r;
#[doc = "RPB4R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb4r](rpb4r) module"]
pub type RPB4R = crate::Reg<u32, _RPB4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB4R;
#[doc = "`read()` method returns [rpb4r::R](rpb4r::R) reader structure"]
impl crate::Readable for RPB4R {}
#[doc = "`write(|w| ..)` method takes [rpb4r::W](rpb4r::W) writer structure"]
impl crate::Writable for RPB4R {}
#[doc = "RPB4R register"]
pub mod rpb4r;
#[doc = "RPB5R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb5r](rpb5r) module"]
pub type RPB5R = crate::Reg<u32, _RPB5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB5R;
#[doc = "`read()` method returns [rpb5r::R](rpb5r::R) reader structure"]
impl crate::Readable for RPB5R {}
#[doc = "`write(|w| ..)` method takes [rpb5r::W](rpb5r::W) writer structure"]
impl crate::Writable for RPB5R {}
#[doc = "RPB5R register"]
pub mod rpb5r;
#[doc = "RPB6R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb6r](rpb6r) module"]
pub type RPB6R = crate::Reg<u32, _RPB6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB6R;
#[doc = "`read()` method returns [rpb6r::R](rpb6r::R) reader structure"]
impl crate::Readable for RPB6R {}
#[doc = "`write(|w| ..)` method takes [rpb6r::W](rpb6r::W) writer structure"]
impl crate::Writable for RPB6R {}
#[doc = "RPB6R register"]
pub mod rpb6r;
#[doc = "RPB7R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb7r](rpb7r) module"]
pub type RPB7R = crate::Reg<u32, _RPB7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB7R;
#[doc = "`read()` method returns [rpb7r::R](rpb7r::R) reader structure"]
impl crate::Readable for RPB7R {}
#[doc = "`write(|w| ..)` method takes [rpb7r::W](rpb7r::W) writer structure"]
impl crate::Writable for RPB7R {}
#[doc = "RPB7R register"]
pub mod rpb7r;
#[doc = "RPB8R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb8r](rpb8r) module"]
pub type RPB8R = crate::Reg<u32, _RPB8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB8R;
#[doc = "`read()` method returns [rpb8r::R](rpb8r::R) reader structure"]
impl crate::Readable for RPB8R {}
#[doc = "`write(|w| ..)` method takes [rpb8r::W](rpb8r::W) writer structure"]
impl crate::Writable for RPB8R {}
#[doc = "RPB8R register"]
pub mod rpb8r;
#[doc = "RPB9R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb9r](rpb9r) module"]
pub type RPB9R = crate::Reg<u32, _RPB9R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB9R;
#[doc = "`read()` method returns [rpb9r::R](rpb9r::R) reader structure"]
impl crate::Readable for RPB9R {}
#[doc = "`write(|w| ..)` method takes [rpb9r::W](rpb9r::W) writer structure"]
impl crate::Writable for RPB9R {}
#[doc = "RPB9R register"]
pub mod rpb9r;
#[doc = "RPB10R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb10r](rpb10r) module"]
pub type RPB10R = crate::Reg<u32, _RPB10R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB10R;
#[doc = "`read()` method returns [rpb10r::R](rpb10r::R) reader structure"]
impl crate::Readable for RPB10R {}
#[doc = "`write(|w| ..)` method takes [rpb10r::W](rpb10r::W) writer structure"]
impl crate::Writable for RPB10R {}
#[doc = "RPB10R register"]
pub mod rpb10r;
#[doc = "RPB11R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb11r](rpb11r) module"]
pub type RPB11R = crate::Reg<u32, _RPB11R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB11R;
#[doc = "`read()` method returns [rpb11r::R](rpb11r::R) reader structure"]
impl crate::Readable for RPB11R {}
#[doc = "`write(|w| ..)` method takes [rpb11r::W](rpb11r::W) writer structure"]
impl crate::Writable for RPB11R {}
#[doc = "RPB11R register"]
pub mod rpb11r;
#[doc = "RPB13R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb13r](rpb13r) module"]
pub type RPB13R = crate::Reg<u32, _RPB13R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB13R;
#[doc = "`read()` method returns [rpb13r::R](rpb13r::R) reader structure"]
impl crate::Readable for RPB13R {}
#[doc = "`write(|w| ..)` method takes [rpb13r::W](rpb13r::W) writer structure"]
impl crate::Writable for RPB13R {}
#[doc = "RPB13R register"]
pub mod rpb13r;
#[doc = "RPB14R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb14r](rpb14r) module"]
pub type RPB14R = crate::Reg<u32, _RPB14R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB14R;
#[doc = "`read()` method returns [rpb14r::R](rpb14r::R) reader structure"]
impl crate::Readable for RPB14R {}
#[doc = "`write(|w| ..)` method takes [rpb14r::W](rpb14r::W) writer structure"]
impl crate::Writable for RPB14R {}
#[doc = "RPB14R register"]
pub mod rpb14r;
#[doc = "RPB15R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpb15r](rpb15r) module"]
pub type RPB15R = crate::Reg<u32, _RPB15R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPB15R;
#[doc = "`read()` method returns [rpb15r::R](rpb15r::R) reader structure"]
impl crate::Readable for RPB15R {}
#[doc = "`write(|w| ..)` method takes [rpb15r::W](rpb15r::W) writer structure"]
impl crate::Writable for RPB15R {}
#[doc = "RPB15R register"]
pub mod rpb15r;
#[doc = "RPC0R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc0r](rpc0r) module"]
pub type RPC0R = crate::Reg<u32, _RPC0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC0R;
#[doc = "`read()` method returns [rpc0r::R](rpc0r::R) reader structure"]
impl crate::Readable for RPC0R {}
#[doc = "`write(|w| ..)` method takes [rpc0r::W](rpc0r::W) writer structure"]
impl crate::Writable for RPC0R {}
#[doc = "RPC0R register"]
pub mod rpc0r;
#[doc = "RPC1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc1r](rpc1r) module"]
pub type RPC1R = crate::Reg<u32, _RPC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC1R;
#[doc = "`read()` method returns [rpc1r::R](rpc1r::R) reader structure"]
impl crate::Readable for RPC1R {}
#[doc = "`write(|w| ..)` method takes [rpc1r::W](rpc1r::W) writer structure"]
impl crate::Writable for RPC1R {}
#[doc = "RPC1R register"]
pub mod rpc1r;
#[doc = "RPC2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc2r](rpc2r) module"]
pub type RPC2R = crate::Reg<u32, _RPC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC2R;
#[doc = "`read()` method returns [rpc2r::R](rpc2r::R) reader structure"]
impl crate::Readable for RPC2R {}
#[doc = "`write(|w| ..)` method takes [rpc2r::W](rpc2r::W) writer structure"]
impl crate::Writable for RPC2R {}
#[doc = "RPC2R register"]
pub mod rpc2r;
#[doc = "RPC3R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc3r](rpc3r) module"]
pub type RPC3R = crate::Reg<u32, _RPC3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC3R;
#[doc = "`read()` method returns [rpc3r::R](rpc3r::R) reader structure"]
impl crate::Readable for RPC3R {}
#[doc = "`write(|w| ..)` method takes [rpc3r::W](rpc3r::W) writer structure"]
impl crate::Writable for RPC3R {}
#[doc = "RPC3R register"]
pub mod rpc3r;
#[doc = "RPC4R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc4r](rpc4r) module"]
pub type RPC4R = crate::Reg<u32, _RPC4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC4R;
#[doc = "`read()` method returns [rpc4r::R](rpc4r::R) reader structure"]
impl crate::Readable for RPC4R {}
#[doc = "`write(|w| ..)` method takes [rpc4r::W](rpc4r::W) writer structure"]
impl crate::Writable for RPC4R {}
#[doc = "RPC4R register"]
pub mod rpc4r;
#[doc = "RPC5R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc5r](rpc5r) module"]
pub type RPC5R = crate::Reg<u32, _RPC5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC5R;
#[doc = "`read()` method returns [rpc5r::R](rpc5r::R) reader structure"]
impl crate::Readable for RPC5R {}
#[doc = "`write(|w| ..)` method takes [rpc5r::W](rpc5r::W) writer structure"]
impl crate::Writable for RPC5R {}
#[doc = "RPC5R register"]
pub mod rpc5r;
#[doc = "RPC6R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc6r](rpc6r) module"]
pub type RPC6R = crate::Reg<u32, _RPC6R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC6R;
#[doc = "`read()` method returns [rpc6r::R](rpc6r::R) reader structure"]
impl crate::Readable for RPC6R {}
#[doc = "`write(|w| ..)` method takes [rpc6r::W](rpc6r::W) writer structure"]
impl crate::Writable for RPC6R {}
#[doc = "RPC6R register"]
pub mod rpc6r;
#[doc = "RPC7R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc7r](rpc7r) module"]
pub type RPC7R = crate::Reg<u32, _RPC7R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC7R;
#[doc = "`read()` method returns [rpc7r::R](rpc7r::R) reader structure"]
impl crate::Readable for RPC7R {}
#[doc = "`write(|w| ..)` method takes [rpc7r::W](rpc7r::W) writer structure"]
impl crate::Writable for RPC7R {}
#[doc = "RPC7R register"]
pub mod rpc7r;
#[doc = "RPC8R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc8r](rpc8r) module"]
pub type RPC8R = crate::Reg<u32, _RPC8R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC8R;
#[doc = "`read()` method returns [rpc8r::R](rpc8r::R) reader structure"]
impl crate::Readable for RPC8R {}
#[doc = "`write(|w| ..)` method takes [rpc8r::W](rpc8r::W) writer structure"]
impl crate::Writable for RPC8R {}
#[doc = "RPC8R register"]
pub mod rpc8r;
#[doc = "RPC9R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpc9r](rpc9r) module"]
pub type RPC9R = crate::Reg<u32, _RPC9R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RPC9R;
#[doc = "`read()` method returns [rpc9r::R](rpc9r::R) reader structure"]
impl crate::Readable for RPC9R {}
#[doc = "`write(|w| ..)` method takes [rpc9r::W](rpc9r::W) writer structure"]
impl crate::Writable for RPC9R {}
#[doc = "RPC9R register"]
pub mod rpc9r;
