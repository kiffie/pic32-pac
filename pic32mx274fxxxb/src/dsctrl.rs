#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSCON register"]
    pub dscon: DSCON,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - DSWAKE register"]
    pub dswake: DSWAKE,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - DSGPR0 register"]
    pub dsgpr0: DSGPR0,
    _reserved3: [u8; 28usize],
    #[doc = "0x40 - DSGPR1 register"]
    pub dsgpr1: DSGPR1,
    #[doc = "0x44 - DSGPR2 register"]
    pub dsgpr2: DSGPR2,
    #[doc = "0x48 - DSGPR3 register"]
    pub dsgpr3: DSGPR3,
    #[doc = "0x4c - DSGPR4 register"]
    pub dsgpr4: DSGPR4,
    #[doc = "0x50 - DSGPR5 register"]
    pub dsgpr5: DSGPR5,
    #[doc = "0x54 - DSGPR6 register"]
    pub dsgpr6: DSGPR6,
    #[doc = "0x58 - DSGPR7 register"]
    pub dsgpr7: DSGPR7,
    #[doc = "0x5c - DSGPR8 register"]
    pub dsgpr8: DSGPR8,
    #[doc = "0x60 - DSGPR9 register"]
    pub dsgpr9: DSGPR9,
    #[doc = "0x64 - DSGPR10 register"]
    pub dsgpr10: DSGPR10,
    #[doc = "0x68 - DSGPR11 register"]
    pub dsgpr11: DSGPR11,
    #[doc = "0x6c - DSGPR12 register"]
    pub dsgpr12: DSGPR12,
    #[doc = "0x70 - DSGPR13 register"]
    pub dsgpr13: DSGPR13,
    #[doc = "0x74 - DSGPR14 register"]
    pub dsgpr14: DSGPR14,
    #[doc = "0x78 - DSGPR15 register"]
    pub dsgpr15: DSGPR15,
    #[doc = "0x7c - DSGPR16 register"]
    pub dsgpr16: DSGPR16,
    #[doc = "0x80 - DSGPR17 register"]
    pub dsgpr17: DSGPR17,
    #[doc = "0x84 - DSGPR18 register"]
    pub dsgpr18: DSGPR18,
    #[doc = "0x88 - DSGPR19 register"]
    pub dsgpr19: DSGPR19,
    #[doc = "0x8c - DSGPR20 register"]
    pub dsgpr20: DSGPR20,
    #[doc = "0x90 - DSGPR21 register"]
    pub dsgpr21: DSGPR21,
    #[doc = "0x94 - DSGPR22 register"]
    pub dsgpr22: DSGPR22,
    #[doc = "0x98 - DSGPR23 register"]
    pub dsgpr23: DSGPR23,
    #[doc = "0x9c - DSGPR24 register"]
    pub dsgpr24: DSGPR24,
    #[doc = "0xa0 - DSGPR25 register"]
    pub dsgpr25: DSGPR25,
    #[doc = "0xa4 - DSGPR26 register"]
    pub dsgpr26: DSGPR26,
    #[doc = "0xa8 - DSGPR27 register"]
    pub dsgpr27: DSGPR27,
    #[doc = "0xac - DSGPR28 register"]
    pub dsgpr28: DSGPR28,
    #[doc = "0xb0 - DSGPR29 register"]
    pub dsgpr29: DSGPR29,
    #[doc = "0xb4 - DSGPR30 register"]
    pub dsgpr30: DSGPR30,
    #[doc = "0xb8 - DSGPR31 register"]
    pub dsgpr31: DSGPR31,
    #[doc = "0xbc - DSGPR32 register"]
    pub dsgpr32: DSGPR32,
}
#[doc = "DSCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscon](dscon) module"]
pub type DSCON = crate::Reg<u32, _DSCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCON;
#[doc = "`read()` method returns [dscon::R](dscon::R) reader structure"]
impl crate::Readable for DSCON {}
#[doc = "`write(|w| ..)` method takes [dscon::W](dscon::W) writer structure"]
impl crate::Writable for DSCON {}
#[doc = "DSCON register"]
pub mod dscon;
#[doc = "DSWAKE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dswake](dswake) module"]
pub type DSWAKE = crate::Reg<u32, _DSWAKE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSWAKE;
#[doc = "`read()` method returns [dswake::R](dswake::R) reader structure"]
impl crate::Readable for DSWAKE {}
#[doc = "`write(|w| ..)` method takes [dswake::W](dswake::W) writer structure"]
impl crate::Writable for DSWAKE {}
#[doc = "DSWAKE register"]
pub mod dswake;
#[doc = "DSGPR0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr0](dsgpr0) module"]
pub type DSGPR0 = crate::Reg<u32, _DSGPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR0;
#[doc = "`read()` method returns [dsgpr0::R](dsgpr0::R) reader structure"]
impl crate::Readable for DSGPR0 {}
#[doc = "`write(|w| ..)` method takes [dsgpr0::W](dsgpr0::W) writer structure"]
impl crate::Writable for DSGPR0 {}
#[doc = "DSGPR0 register"]
pub mod dsgpr0;
#[doc = "DSGPR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr1](dsgpr1) module"]
pub type DSGPR1 = crate::Reg<u32, _DSGPR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR1;
#[doc = "`read()` method returns [dsgpr1::R](dsgpr1::R) reader structure"]
impl crate::Readable for DSGPR1 {}
#[doc = "`write(|w| ..)` method takes [dsgpr1::W](dsgpr1::W) writer structure"]
impl crate::Writable for DSGPR1 {}
#[doc = "DSGPR1 register"]
pub mod dsgpr1;
#[doc = "DSGPR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr2](dsgpr2) module"]
pub type DSGPR2 = crate::Reg<u32, _DSGPR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR2;
#[doc = "`read()` method returns [dsgpr2::R](dsgpr2::R) reader structure"]
impl crate::Readable for DSGPR2 {}
#[doc = "`write(|w| ..)` method takes [dsgpr2::W](dsgpr2::W) writer structure"]
impl crate::Writable for DSGPR2 {}
#[doc = "DSGPR2 register"]
pub mod dsgpr2;
#[doc = "DSGPR3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr3](dsgpr3) module"]
pub type DSGPR3 = crate::Reg<u32, _DSGPR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR3;
#[doc = "`read()` method returns [dsgpr3::R](dsgpr3::R) reader structure"]
impl crate::Readable for DSGPR3 {}
#[doc = "`write(|w| ..)` method takes [dsgpr3::W](dsgpr3::W) writer structure"]
impl crate::Writable for DSGPR3 {}
#[doc = "DSGPR3 register"]
pub mod dsgpr3;
#[doc = "DSGPR4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr4](dsgpr4) module"]
pub type DSGPR4 = crate::Reg<u32, _DSGPR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR4;
#[doc = "`read()` method returns [dsgpr4::R](dsgpr4::R) reader structure"]
impl crate::Readable for DSGPR4 {}
#[doc = "`write(|w| ..)` method takes [dsgpr4::W](dsgpr4::W) writer structure"]
impl crate::Writable for DSGPR4 {}
#[doc = "DSGPR4 register"]
pub mod dsgpr4;
#[doc = "DSGPR5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr5](dsgpr5) module"]
pub type DSGPR5 = crate::Reg<u32, _DSGPR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR5;
#[doc = "`read()` method returns [dsgpr5::R](dsgpr5::R) reader structure"]
impl crate::Readable for DSGPR5 {}
#[doc = "`write(|w| ..)` method takes [dsgpr5::W](dsgpr5::W) writer structure"]
impl crate::Writable for DSGPR5 {}
#[doc = "DSGPR5 register"]
pub mod dsgpr5;
#[doc = "DSGPR6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr6](dsgpr6) module"]
pub type DSGPR6 = crate::Reg<u32, _DSGPR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR6;
#[doc = "`read()` method returns [dsgpr6::R](dsgpr6::R) reader structure"]
impl crate::Readable for DSGPR6 {}
#[doc = "`write(|w| ..)` method takes [dsgpr6::W](dsgpr6::W) writer structure"]
impl crate::Writable for DSGPR6 {}
#[doc = "DSGPR6 register"]
pub mod dsgpr6;
#[doc = "DSGPR7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr7](dsgpr7) module"]
pub type DSGPR7 = crate::Reg<u32, _DSGPR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR7;
#[doc = "`read()` method returns [dsgpr7::R](dsgpr7::R) reader structure"]
impl crate::Readable for DSGPR7 {}
#[doc = "`write(|w| ..)` method takes [dsgpr7::W](dsgpr7::W) writer structure"]
impl crate::Writable for DSGPR7 {}
#[doc = "DSGPR7 register"]
pub mod dsgpr7;
#[doc = "DSGPR8 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr8](dsgpr8) module"]
pub type DSGPR8 = crate::Reg<u32, _DSGPR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR8;
#[doc = "`read()` method returns [dsgpr8::R](dsgpr8::R) reader structure"]
impl crate::Readable for DSGPR8 {}
#[doc = "`write(|w| ..)` method takes [dsgpr8::W](dsgpr8::W) writer structure"]
impl crate::Writable for DSGPR8 {}
#[doc = "DSGPR8 register"]
pub mod dsgpr8;
#[doc = "DSGPR9 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr9](dsgpr9) module"]
pub type DSGPR9 = crate::Reg<u32, _DSGPR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR9;
#[doc = "`read()` method returns [dsgpr9::R](dsgpr9::R) reader structure"]
impl crate::Readable for DSGPR9 {}
#[doc = "`write(|w| ..)` method takes [dsgpr9::W](dsgpr9::W) writer structure"]
impl crate::Writable for DSGPR9 {}
#[doc = "DSGPR9 register"]
pub mod dsgpr9;
#[doc = "DSGPR10 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr10](dsgpr10) module"]
pub type DSGPR10 = crate::Reg<u32, _DSGPR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR10;
#[doc = "`read()` method returns [dsgpr10::R](dsgpr10::R) reader structure"]
impl crate::Readable for DSGPR10 {}
#[doc = "`write(|w| ..)` method takes [dsgpr10::W](dsgpr10::W) writer structure"]
impl crate::Writable for DSGPR10 {}
#[doc = "DSGPR10 register"]
pub mod dsgpr10;
#[doc = "DSGPR11 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr11](dsgpr11) module"]
pub type DSGPR11 = crate::Reg<u32, _DSGPR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR11;
#[doc = "`read()` method returns [dsgpr11::R](dsgpr11::R) reader structure"]
impl crate::Readable for DSGPR11 {}
#[doc = "`write(|w| ..)` method takes [dsgpr11::W](dsgpr11::W) writer structure"]
impl crate::Writable for DSGPR11 {}
#[doc = "DSGPR11 register"]
pub mod dsgpr11;
#[doc = "DSGPR12 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr12](dsgpr12) module"]
pub type DSGPR12 = crate::Reg<u32, _DSGPR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR12;
#[doc = "`read()` method returns [dsgpr12::R](dsgpr12::R) reader structure"]
impl crate::Readable for DSGPR12 {}
#[doc = "`write(|w| ..)` method takes [dsgpr12::W](dsgpr12::W) writer structure"]
impl crate::Writable for DSGPR12 {}
#[doc = "DSGPR12 register"]
pub mod dsgpr12;
#[doc = "DSGPR13 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr13](dsgpr13) module"]
pub type DSGPR13 = crate::Reg<u32, _DSGPR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR13;
#[doc = "`read()` method returns [dsgpr13::R](dsgpr13::R) reader structure"]
impl crate::Readable for DSGPR13 {}
#[doc = "`write(|w| ..)` method takes [dsgpr13::W](dsgpr13::W) writer structure"]
impl crate::Writable for DSGPR13 {}
#[doc = "DSGPR13 register"]
pub mod dsgpr13;
#[doc = "DSGPR14 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr14](dsgpr14) module"]
pub type DSGPR14 = crate::Reg<u32, _DSGPR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR14;
#[doc = "`read()` method returns [dsgpr14::R](dsgpr14::R) reader structure"]
impl crate::Readable for DSGPR14 {}
#[doc = "`write(|w| ..)` method takes [dsgpr14::W](dsgpr14::W) writer structure"]
impl crate::Writable for DSGPR14 {}
#[doc = "DSGPR14 register"]
pub mod dsgpr14;
#[doc = "DSGPR15 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr15](dsgpr15) module"]
pub type DSGPR15 = crate::Reg<u32, _DSGPR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR15;
#[doc = "`read()` method returns [dsgpr15::R](dsgpr15::R) reader structure"]
impl crate::Readable for DSGPR15 {}
#[doc = "`write(|w| ..)` method takes [dsgpr15::W](dsgpr15::W) writer structure"]
impl crate::Writable for DSGPR15 {}
#[doc = "DSGPR15 register"]
pub mod dsgpr15;
#[doc = "DSGPR16 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr16](dsgpr16) module"]
pub type DSGPR16 = crate::Reg<u32, _DSGPR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR16;
#[doc = "`read()` method returns [dsgpr16::R](dsgpr16::R) reader structure"]
impl crate::Readable for DSGPR16 {}
#[doc = "`write(|w| ..)` method takes [dsgpr16::W](dsgpr16::W) writer structure"]
impl crate::Writable for DSGPR16 {}
#[doc = "DSGPR16 register"]
pub mod dsgpr16;
#[doc = "DSGPR17 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr17](dsgpr17) module"]
pub type DSGPR17 = crate::Reg<u32, _DSGPR17>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR17;
#[doc = "`read()` method returns [dsgpr17::R](dsgpr17::R) reader structure"]
impl crate::Readable for DSGPR17 {}
#[doc = "`write(|w| ..)` method takes [dsgpr17::W](dsgpr17::W) writer structure"]
impl crate::Writable for DSGPR17 {}
#[doc = "DSGPR17 register"]
pub mod dsgpr17;
#[doc = "DSGPR18 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr18](dsgpr18) module"]
pub type DSGPR18 = crate::Reg<u32, _DSGPR18>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR18;
#[doc = "`read()` method returns [dsgpr18::R](dsgpr18::R) reader structure"]
impl crate::Readable for DSGPR18 {}
#[doc = "`write(|w| ..)` method takes [dsgpr18::W](dsgpr18::W) writer structure"]
impl crate::Writable for DSGPR18 {}
#[doc = "DSGPR18 register"]
pub mod dsgpr18;
#[doc = "DSGPR19 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr19](dsgpr19) module"]
pub type DSGPR19 = crate::Reg<u32, _DSGPR19>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR19;
#[doc = "`read()` method returns [dsgpr19::R](dsgpr19::R) reader structure"]
impl crate::Readable for DSGPR19 {}
#[doc = "`write(|w| ..)` method takes [dsgpr19::W](dsgpr19::W) writer structure"]
impl crate::Writable for DSGPR19 {}
#[doc = "DSGPR19 register"]
pub mod dsgpr19;
#[doc = "DSGPR20 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr20](dsgpr20) module"]
pub type DSGPR20 = crate::Reg<u32, _DSGPR20>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR20;
#[doc = "`read()` method returns [dsgpr20::R](dsgpr20::R) reader structure"]
impl crate::Readable for DSGPR20 {}
#[doc = "`write(|w| ..)` method takes [dsgpr20::W](dsgpr20::W) writer structure"]
impl crate::Writable for DSGPR20 {}
#[doc = "DSGPR20 register"]
pub mod dsgpr20;
#[doc = "DSGPR21 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr21](dsgpr21) module"]
pub type DSGPR21 = crate::Reg<u32, _DSGPR21>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR21;
#[doc = "`read()` method returns [dsgpr21::R](dsgpr21::R) reader structure"]
impl crate::Readable for DSGPR21 {}
#[doc = "`write(|w| ..)` method takes [dsgpr21::W](dsgpr21::W) writer structure"]
impl crate::Writable for DSGPR21 {}
#[doc = "DSGPR21 register"]
pub mod dsgpr21;
#[doc = "DSGPR22 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr22](dsgpr22) module"]
pub type DSGPR22 = crate::Reg<u32, _DSGPR22>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR22;
#[doc = "`read()` method returns [dsgpr22::R](dsgpr22::R) reader structure"]
impl crate::Readable for DSGPR22 {}
#[doc = "`write(|w| ..)` method takes [dsgpr22::W](dsgpr22::W) writer structure"]
impl crate::Writable for DSGPR22 {}
#[doc = "DSGPR22 register"]
pub mod dsgpr22;
#[doc = "DSGPR23 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr23](dsgpr23) module"]
pub type DSGPR23 = crate::Reg<u32, _DSGPR23>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR23;
#[doc = "`read()` method returns [dsgpr23::R](dsgpr23::R) reader structure"]
impl crate::Readable for DSGPR23 {}
#[doc = "`write(|w| ..)` method takes [dsgpr23::W](dsgpr23::W) writer structure"]
impl crate::Writable for DSGPR23 {}
#[doc = "DSGPR23 register"]
pub mod dsgpr23;
#[doc = "DSGPR24 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr24](dsgpr24) module"]
pub type DSGPR24 = crate::Reg<u32, _DSGPR24>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR24;
#[doc = "`read()` method returns [dsgpr24::R](dsgpr24::R) reader structure"]
impl crate::Readable for DSGPR24 {}
#[doc = "`write(|w| ..)` method takes [dsgpr24::W](dsgpr24::W) writer structure"]
impl crate::Writable for DSGPR24 {}
#[doc = "DSGPR24 register"]
pub mod dsgpr24;
#[doc = "DSGPR25 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr25](dsgpr25) module"]
pub type DSGPR25 = crate::Reg<u32, _DSGPR25>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR25;
#[doc = "`read()` method returns [dsgpr25::R](dsgpr25::R) reader structure"]
impl crate::Readable for DSGPR25 {}
#[doc = "`write(|w| ..)` method takes [dsgpr25::W](dsgpr25::W) writer structure"]
impl crate::Writable for DSGPR25 {}
#[doc = "DSGPR25 register"]
pub mod dsgpr25;
#[doc = "DSGPR26 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr26](dsgpr26) module"]
pub type DSGPR26 = crate::Reg<u32, _DSGPR26>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR26;
#[doc = "`read()` method returns [dsgpr26::R](dsgpr26::R) reader structure"]
impl crate::Readable for DSGPR26 {}
#[doc = "`write(|w| ..)` method takes [dsgpr26::W](dsgpr26::W) writer structure"]
impl crate::Writable for DSGPR26 {}
#[doc = "DSGPR26 register"]
pub mod dsgpr26;
#[doc = "DSGPR27 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr27](dsgpr27) module"]
pub type DSGPR27 = crate::Reg<u32, _DSGPR27>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR27;
#[doc = "`read()` method returns [dsgpr27::R](dsgpr27::R) reader structure"]
impl crate::Readable for DSGPR27 {}
#[doc = "`write(|w| ..)` method takes [dsgpr27::W](dsgpr27::W) writer structure"]
impl crate::Writable for DSGPR27 {}
#[doc = "DSGPR27 register"]
pub mod dsgpr27;
#[doc = "DSGPR28 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr28](dsgpr28) module"]
pub type DSGPR28 = crate::Reg<u32, _DSGPR28>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR28;
#[doc = "`read()` method returns [dsgpr28::R](dsgpr28::R) reader structure"]
impl crate::Readable for DSGPR28 {}
#[doc = "`write(|w| ..)` method takes [dsgpr28::W](dsgpr28::W) writer structure"]
impl crate::Writable for DSGPR28 {}
#[doc = "DSGPR28 register"]
pub mod dsgpr28;
#[doc = "DSGPR29 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr29](dsgpr29) module"]
pub type DSGPR29 = crate::Reg<u32, _DSGPR29>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR29;
#[doc = "`read()` method returns [dsgpr29::R](dsgpr29::R) reader structure"]
impl crate::Readable for DSGPR29 {}
#[doc = "`write(|w| ..)` method takes [dsgpr29::W](dsgpr29::W) writer structure"]
impl crate::Writable for DSGPR29 {}
#[doc = "DSGPR29 register"]
pub mod dsgpr29;
#[doc = "DSGPR30 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr30](dsgpr30) module"]
pub type DSGPR30 = crate::Reg<u32, _DSGPR30>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR30;
#[doc = "`read()` method returns [dsgpr30::R](dsgpr30::R) reader structure"]
impl crate::Readable for DSGPR30 {}
#[doc = "`write(|w| ..)` method takes [dsgpr30::W](dsgpr30::W) writer structure"]
impl crate::Writable for DSGPR30 {}
#[doc = "DSGPR30 register"]
pub mod dsgpr30;
#[doc = "DSGPR31 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr31](dsgpr31) module"]
pub type DSGPR31 = crate::Reg<u32, _DSGPR31>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR31;
#[doc = "`read()` method returns [dsgpr31::R](dsgpr31::R) reader structure"]
impl crate::Readable for DSGPR31 {}
#[doc = "`write(|w| ..)` method takes [dsgpr31::W](dsgpr31::W) writer structure"]
impl crate::Writable for DSGPR31 {}
#[doc = "DSGPR31 register"]
pub mod dsgpr31;
#[doc = "DSGPR32 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr32](dsgpr32) module"]
pub type DSGPR32 = crate::Reg<u32, _DSGPR32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSGPR32;
#[doc = "`read()` method returns [dsgpr32::R](dsgpr32::R) reader structure"]
impl crate::Readable for DSGPR32 {}
#[doc = "`write(|w| ..)` method takes [dsgpr32::W](dsgpr32::W) writer structure"]
impl crate::Writable for DSGPR32 {}
#[doc = "DSGPR32 register"]
pub mod dsgpr32;
