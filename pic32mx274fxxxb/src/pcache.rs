#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CHECON register"]
    pub checon: CHECON,
    #[doc = "0x04 - CHECONCLR register"]
    pub checonclr: CHECONCLR,
    #[doc = "0x08 - CHECONSET register"]
    pub checonset: CHECONSET,
    #[doc = "0x0c - CHECONINV register"]
    pub checoninv: CHECONINV,
    #[doc = "0x10 - CHEACC register"]
    pub cheacc: CHEACC,
    #[doc = "0x14 - CHEACCCLR register"]
    pub cheaccclr: CHEACCCLR,
    #[doc = "0x18 - CHEACCSET register"]
    pub cheaccset: CHEACCSET,
    #[doc = "0x1c - CHEACCINV register"]
    pub cheaccinv: CHEACCINV,
    #[doc = "0x20 - CHETAG register"]
    pub chetag: CHETAG,
    #[doc = "0x24 - CHETAGCLR register"]
    pub chetagclr: CHETAGCLR,
    #[doc = "0x28 - CHETAGSET register"]
    pub chetagset: CHETAGSET,
    #[doc = "0x2c - CHETAGINV register"]
    pub chetaginv: CHETAGINV,
    #[doc = "0x30 - CHEMSK register"]
    pub chemsk: CHEMSK,
    #[doc = "0x34 - CHEMSKCLR register"]
    pub chemskclr: CHEMSKCLR,
    #[doc = "0x38 - CHEMSKSET register"]
    pub chemskset: CHEMSKSET,
    #[doc = "0x3c - CHEMSKINV register"]
    pub chemskinv: CHEMSKINV,
    #[doc = "0x40 - CHEW0 register"]
    pub chew0: CHEW0,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - CHEW1 register"]
    pub chew1: CHEW1,
    _reserved18: [u8; 12usize],
    #[doc = "0x60 - CHEW2 register"]
    pub chew2: CHEW2,
    _reserved19: [u8; 12usize],
    #[doc = "0x70 - CHEW3 register"]
    pub chew3: CHEW3,
    _reserved20: [u8; 12usize],
    #[doc = "0x80 - CHELRU register"]
    pub chelru: CHELRU,
    _reserved21: [u8; 12usize],
    #[doc = "0x90 - CHEHIT register"]
    pub chehit: CHEHIT,
    _reserved22: [u8; 12usize],
    #[doc = "0xa0 - CHEMIS register"]
    pub chemis: CHEMIS,
    _reserved23: [u8; 12usize],
    #[doc = "0xb0 - RESERVED1 register"]
    pub reserved1: RESERVED1,
    _reserved24: [u8; 12usize],
    #[doc = "0xc0 - CHEPFABT register"]
    pub chepfabt: CHEPFABT,
}
#[doc = "CHECON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [checon](checon) module"]
pub type CHECON = crate::Reg<u32, _CHECON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHECON;
#[doc = "`read()` method returns [checon::R](checon::R) reader structure"]
impl crate::Readable for CHECON {}
#[doc = "`write(|w| ..)` method takes [checon::W](checon::W) writer structure"]
impl crate::Writable for CHECON {}
#[doc = "CHECON register"]
pub mod checon;
#[doc = "CHECONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [checonclr](checonclr) module"]
pub type CHECONCLR = crate::Reg<u32, _CHECONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHECONCLR;
#[doc = "`read()` method returns [checonclr::R](checonclr::R) reader structure"]
impl crate::Readable for CHECONCLR {}
#[doc = "`write(|w| ..)` method takes [checonclr::W](checonclr::W) writer structure"]
impl crate::Writable for CHECONCLR {}
#[doc = "CHECONCLR register"]
pub mod checonclr;
#[doc = "CHECONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [checonset](checonset) module"]
pub type CHECONSET = crate::Reg<u32, _CHECONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHECONSET;
#[doc = "`read()` method returns [checonset::R](checonset::R) reader structure"]
impl crate::Readable for CHECONSET {}
#[doc = "`write(|w| ..)` method takes [checonset::W](checonset::W) writer structure"]
impl crate::Writable for CHECONSET {}
#[doc = "CHECONSET register"]
pub mod checonset;
#[doc = "CHECONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [checoninv](checoninv) module"]
pub type CHECONINV = crate::Reg<u32, _CHECONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHECONINV;
#[doc = "`read()` method returns [checoninv::R](checoninv::R) reader structure"]
impl crate::Readable for CHECONINV {}
#[doc = "`write(|w| ..)` method takes [checoninv::W](checoninv::W) writer structure"]
impl crate::Writable for CHECONINV {}
#[doc = "CHECONINV register"]
pub mod checoninv;
#[doc = "CHEACC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cheacc](cheacc) module"]
pub type CHEACC = crate::Reg<u32, _CHEACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEACC;
#[doc = "`read()` method returns [cheacc::R](cheacc::R) reader structure"]
impl crate::Readable for CHEACC {}
#[doc = "`write(|w| ..)` method takes [cheacc::W](cheacc::W) writer structure"]
impl crate::Writable for CHEACC {}
#[doc = "CHEACC register"]
pub mod cheacc;
#[doc = "CHEACCCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cheaccclr](cheaccclr) module"]
pub type CHEACCCLR = crate::Reg<u32, _CHEACCCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEACCCLR;
#[doc = "`read()` method returns [cheaccclr::R](cheaccclr::R) reader structure"]
impl crate::Readable for CHEACCCLR {}
#[doc = "`write(|w| ..)` method takes [cheaccclr::W](cheaccclr::W) writer structure"]
impl crate::Writable for CHEACCCLR {}
#[doc = "CHEACCCLR register"]
pub mod cheaccclr;
#[doc = "CHEACCSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cheaccset](cheaccset) module"]
pub type CHEACCSET = crate::Reg<u32, _CHEACCSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEACCSET;
#[doc = "`read()` method returns [cheaccset::R](cheaccset::R) reader structure"]
impl crate::Readable for CHEACCSET {}
#[doc = "`write(|w| ..)` method takes [cheaccset::W](cheaccset::W) writer structure"]
impl crate::Writable for CHEACCSET {}
#[doc = "CHEACCSET register"]
pub mod cheaccset;
#[doc = "CHEACCINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cheaccinv](cheaccinv) module"]
pub type CHEACCINV = crate::Reg<u32, _CHEACCINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEACCINV;
#[doc = "`read()` method returns [cheaccinv::R](cheaccinv::R) reader structure"]
impl crate::Readable for CHEACCINV {}
#[doc = "`write(|w| ..)` method takes [cheaccinv::W](cheaccinv::W) writer structure"]
impl crate::Writable for CHEACCINV {}
#[doc = "CHEACCINV register"]
pub mod cheaccinv;
#[doc = "CHETAG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chetag](chetag) module"]
pub type CHETAG = crate::Reg<u32, _CHETAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHETAG;
#[doc = "`read()` method returns [chetag::R](chetag::R) reader structure"]
impl crate::Readable for CHETAG {}
#[doc = "`write(|w| ..)` method takes [chetag::W](chetag::W) writer structure"]
impl crate::Writable for CHETAG {}
#[doc = "CHETAG register"]
pub mod chetag;
#[doc = "CHETAGCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chetagclr](chetagclr) module"]
pub type CHETAGCLR = crate::Reg<u32, _CHETAGCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHETAGCLR;
#[doc = "`read()` method returns [chetagclr::R](chetagclr::R) reader structure"]
impl crate::Readable for CHETAGCLR {}
#[doc = "`write(|w| ..)` method takes [chetagclr::W](chetagclr::W) writer structure"]
impl crate::Writable for CHETAGCLR {}
#[doc = "CHETAGCLR register"]
pub mod chetagclr;
#[doc = "CHETAGSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chetagset](chetagset) module"]
pub type CHETAGSET = crate::Reg<u32, _CHETAGSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHETAGSET;
#[doc = "`read()` method returns [chetagset::R](chetagset::R) reader structure"]
impl crate::Readable for CHETAGSET {}
#[doc = "`write(|w| ..)` method takes [chetagset::W](chetagset::W) writer structure"]
impl crate::Writable for CHETAGSET {}
#[doc = "CHETAGSET register"]
pub mod chetagset;
#[doc = "CHETAGINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chetaginv](chetaginv) module"]
pub type CHETAGINV = crate::Reg<u32, _CHETAGINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHETAGINV;
#[doc = "`read()` method returns [chetaginv::R](chetaginv::R) reader structure"]
impl crate::Readable for CHETAGINV {}
#[doc = "`write(|w| ..)` method takes [chetaginv::W](chetaginv::W) writer structure"]
impl crate::Writable for CHETAGINV {}
#[doc = "CHETAGINV register"]
pub mod chetaginv;
#[doc = "CHEMSK register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chemsk](chemsk) module"]
pub type CHEMSK = crate::Reg<u32, _CHEMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEMSK;
#[doc = "`read()` method returns [chemsk::R](chemsk::R) reader structure"]
impl crate::Readable for CHEMSK {}
#[doc = "`write(|w| ..)` method takes [chemsk::W](chemsk::W) writer structure"]
impl crate::Writable for CHEMSK {}
#[doc = "CHEMSK register"]
pub mod chemsk;
#[doc = "CHEMSKCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chemskclr](chemskclr) module"]
pub type CHEMSKCLR = crate::Reg<u32, _CHEMSKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEMSKCLR;
#[doc = "`read()` method returns [chemskclr::R](chemskclr::R) reader structure"]
impl crate::Readable for CHEMSKCLR {}
#[doc = "`write(|w| ..)` method takes [chemskclr::W](chemskclr::W) writer structure"]
impl crate::Writable for CHEMSKCLR {}
#[doc = "CHEMSKCLR register"]
pub mod chemskclr;
#[doc = "CHEMSKSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chemskset](chemskset) module"]
pub type CHEMSKSET = crate::Reg<u32, _CHEMSKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEMSKSET;
#[doc = "`read()` method returns [chemskset::R](chemskset::R) reader structure"]
impl crate::Readable for CHEMSKSET {}
#[doc = "`write(|w| ..)` method takes [chemskset::W](chemskset::W) writer structure"]
impl crate::Writable for CHEMSKSET {}
#[doc = "CHEMSKSET register"]
pub mod chemskset;
#[doc = "CHEMSKINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chemskinv](chemskinv) module"]
pub type CHEMSKINV = crate::Reg<u32, _CHEMSKINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEMSKINV;
#[doc = "`read()` method returns [chemskinv::R](chemskinv::R) reader structure"]
impl crate::Readable for CHEMSKINV {}
#[doc = "`write(|w| ..)` method takes [chemskinv::W](chemskinv::W) writer structure"]
impl crate::Writable for CHEMSKINV {}
#[doc = "CHEMSKINV register"]
pub mod chemskinv;
#[doc = "CHEW0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chew0](chew0) module"]
pub type CHEW0 = crate::Reg<u32, _CHEW0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEW0;
#[doc = "`read()` method returns [chew0::R](chew0::R) reader structure"]
impl crate::Readable for CHEW0 {}
#[doc = "`write(|w| ..)` method takes [chew0::W](chew0::W) writer structure"]
impl crate::Writable for CHEW0 {}
#[doc = "CHEW0 register"]
pub mod chew0;
#[doc = "CHEW1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chew1](chew1) module"]
pub type CHEW1 = crate::Reg<u32, _CHEW1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEW1;
#[doc = "`read()` method returns [chew1::R](chew1::R) reader structure"]
impl crate::Readable for CHEW1 {}
#[doc = "`write(|w| ..)` method takes [chew1::W](chew1::W) writer structure"]
impl crate::Writable for CHEW1 {}
#[doc = "CHEW1 register"]
pub mod chew1;
#[doc = "CHEW2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chew2](chew2) module"]
pub type CHEW2 = crate::Reg<u32, _CHEW2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEW2;
#[doc = "`read()` method returns [chew2::R](chew2::R) reader structure"]
impl crate::Readable for CHEW2 {}
#[doc = "`write(|w| ..)` method takes [chew2::W](chew2::W) writer structure"]
impl crate::Writable for CHEW2 {}
#[doc = "CHEW2 register"]
pub mod chew2;
#[doc = "CHEW3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chew3](chew3) module"]
pub type CHEW3 = crate::Reg<u32, _CHEW3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEW3;
#[doc = "`read()` method returns [chew3::R](chew3::R) reader structure"]
impl crate::Readable for CHEW3 {}
#[doc = "`write(|w| ..)` method takes [chew3::W](chew3::W) writer structure"]
impl crate::Writable for CHEW3 {}
#[doc = "CHEW3 register"]
pub mod chew3;
#[doc = "CHELRU register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chelru](chelru) module"]
pub type CHELRU = crate::Reg<u32, _CHELRU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHELRU;
#[doc = "`read()` method returns [chelru::R](chelru::R) reader structure"]
impl crate::Readable for CHELRU {}
#[doc = "`write(|w| ..)` method takes [chelru::W](chelru::W) writer structure"]
impl crate::Writable for CHELRU {}
#[doc = "CHELRU register"]
pub mod chelru;
#[doc = "CHEHIT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chehit](chehit) module"]
pub type CHEHIT = crate::Reg<u32, _CHEHIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEHIT;
#[doc = "`read()` method returns [chehit::R](chehit::R) reader structure"]
impl crate::Readable for CHEHIT {}
#[doc = "`write(|w| ..)` method takes [chehit::W](chehit::W) writer structure"]
impl crate::Writable for CHEHIT {}
#[doc = "CHEHIT register"]
pub mod chehit;
#[doc = "CHEMIS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chemis](chemis) module"]
pub type CHEMIS = crate::Reg<u32, _CHEMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEMIS;
#[doc = "`read()` method returns [chemis::R](chemis::R) reader structure"]
impl crate::Readable for CHEMIS {}
#[doc = "`write(|w| ..)` method takes [chemis::W](chemis::W) writer structure"]
impl crate::Writable for CHEMIS {}
#[doc = "CHEMIS register"]
pub mod chemis;
#[doc = "RESERVED1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved1](reserved1) module"]
pub type RESERVED1 = crate::Reg<u32, _RESERVED1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESERVED1;
#[doc = "`read()` method returns [reserved1::R](reserved1::R) reader structure"]
impl crate::Readable for RESERVED1 {}
#[doc = "`write(|w| ..)` method takes [reserved1::W](reserved1::W) writer structure"]
impl crate::Writable for RESERVED1 {}
#[doc = "RESERVED1 register"]
pub mod reserved1;
#[doc = "CHEPFABT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chepfabt](chepfabt) module"]
pub type CHEPFABT = crate::Reg<u32, _CHEPFABT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEPFABT;
#[doc = "`read()` method returns [chepfabt::R](chepfabt::R) reader structure"]
impl crate::Readable for CHEPFABT {}
#[doc = "`write(|w| ..)` method takes [chepfabt::W](chepfabt::W) writer structure"]
impl crate::Writable for CHEPFABT {}
#[doc = "CHEPFABT register"]
pub mod chepfabt;
