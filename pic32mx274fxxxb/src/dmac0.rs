#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH0CON register"]
    pub cont: CONT,
    #[doc = "0x04 - DCH0CONCLR register"]
    pub conclr: CONCLR,
    #[doc = "0x08 - DCH0CONSET register"]
    pub conset: CONSET,
    #[doc = "0x0c - DCH0CONINV register"]
    pub coninv: CONINV,
    #[doc = "0x10 - DCH0ECON register"]
    pub econ: ECON,
    #[doc = "0x14 - DCH0ECONCLR register"]
    pub econclr: ECONCLR,
    #[doc = "0x18 - DCH0ECONSET register"]
    pub econset: ECONSET,
    #[doc = "0x1c - DCH0ECONINV register"]
    pub econinv: ECONINV,
    #[doc = "0x20 - DCH0INT register"]
    pub int: INT,
    #[doc = "0x24 - DCH0INTCLR register"]
    pub intclr: INTCLR,
    #[doc = "0x28 - DCH0INTSET register"]
    pub intset: INTSET,
    #[doc = "0x2c - DCH0INTINV register"]
    pub intinv: INTINV,
    #[doc = "0x30 - DCH0SSA register"]
    pub ssa: SSA,
    #[doc = "0x34 - DCH0SSACLR register"]
    pub ssaclr: SSACLR,
    #[doc = "0x38 - DCH0SSASET register"]
    pub ssaset: SSASET,
    #[doc = "0x3c - DCH0SSAINV register"]
    pub ssainv: SSAINV,
    #[doc = "0x40 - DCH0DSA register"]
    pub dsa: DSA,
    #[doc = "0x44 - DCH0DSACLR register"]
    pub dsaclr: DSACLR,
    #[doc = "0x48 - DCH0DSASET register"]
    pub dsaset: DSASET,
    #[doc = "0x4c - DCH0DSAINV register"]
    pub dsainv: DSAINV,
    #[doc = "0x50 - DCH0SSIZ register"]
    pub ssiz: SSIZ,
    #[doc = "0x54 - DCH0SSIZCLR register"]
    pub ssizclr: SSIZCLR,
    #[doc = "0x58 - DCH0SSIZSET register"]
    pub ssizset: SSIZSET,
    #[doc = "0x5c - DCH0SSIZINV register"]
    pub ssizinv: SSIZINV,
    #[doc = "0x60 - DCH0DSIZ register"]
    pub dsiz: DSIZ,
    #[doc = "0x64 - DCH0DSIZCLR register"]
    pub dsizclr: DSIZCLR,
    #[doc = "0x68 - DCH0DSIZSET register"]
    pub dsizset: DSIZSET,
    #[doc = "0x6c - DCH0DSIZINV register"]
    pub dsizinv: DSIZINV,
    #[doc = "0x70 - DCH0SPTR register"]
    pub sptr: SPTR,
    #[doc = "0x74 - DCH0SPTRCLR register"]
    pub sptrclr: SPTRCLR,
    #[doc = "0x78 - DCH0SPTRSET register"]
    pub sptrset: SPTRSET,
    #[doc = "0x7c - DCH0SPTRINV register"]
    pub sptrinv: SPTRINV,
    #[doc = "0x80 - DCH0DPTR register"]
    pub dptr: DPTR,
    #[doc = "0x84 - DCH0DPTRCLR register"]
    pub dptrclr: DPTRCLR,
    #[doc = "0x88 - DCH0DPTRSET register"]
    pub dptrset: DPTRSET,
    #[doc = "0x8c - DCH0DPTRINV register"]
    pub dptrinv: DPTRINV,
    #[doc = "0x90 - DCH0CSIZ register"]
    pub csiz: CSIZ,
    #[doc = "0x94 - DCH0CSIZCLR register"]
    pub csizclr: CSIZCLR,
    #[doc = "0x98 - DCH0CSIZSET register"]
    pub csizset: CSIZSET,
    #[doc = "0x9c - DCH0CSIZINV register"]
    pub csizinv: CSIZINV,
    #[doc = "0xa0 - DCH0CPTR register"]
    pub cptr: CPTR,
    #[doc = "0xa4 - DCH0CPTRCLR register"]
    pub cptrclr: CPTRCLR,
    #[doc = "0xa8 - DCH0CPTRSET register"]
    pub cptrset: CPTRSET,
    #[doc = "0xac - DCH0CPTRINV register"]
    pub cptrinv: CPTRINV,
    #[doc = "0xb0 - DCH0DAT register"]
    pub dat: DAT,
    #[doc = "0xb4 - DCH0DATCLR register"]
    pub datclr: DATCLR,
    #[doc = "0xb8 - DCH0DATSET register"]
    pub datset: DATSET,
    #[doc = "0xbc - DCH0DATINV register"]
    pub datinv: DATINV,
}
#[doc = "DCH0CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cont](cont) module"]
pub type CONT = crate::Reg<u32, _CONT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONT;
#[doc = "`read()` method returns [cont::R](cont::R) reader structure"]
impl crate::Readable for CONT {}
#[doc = "`write(|w| ..)` method takes [cont::W](cont::W) writer structure"]
impl crate::Writable for CONT {}
#[doc = "DCH0CON register"]
pub mod cont;
#[doc = "DCH0CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conclr](conclr) module"]
pub type CONCLR = crate::Reg<u32, _CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONCLR;
#[doc = "`read()` method returns [conclr::R](conclr::R) reader structure"]
impl crate::Readable for CONCLR {}
#[doc = "`write(|w| ..)` method takes [conclr::W](conclr::W) writer structure"]
impl crate::Writable for CONCLR {}
#[doc = "DCH0CONCLR register"]
pub mod conclr;
#[doc = "DCH0CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conset](conset) module"]
pub type CONSET = crate::Reg<u32, _CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONSET;
#[doc = "`read()` method returns [conset::R](conset::R) reader structure"]
impl crate::Readable for CONSET {}
#[doc = "`write(|w| ..)` method takes [conset::W](conset::W) writer structure"]
impl crate::Writable for CONSET {}
#[doc = "DCH0CONSET register"]
pub mod conset;
#[doc = "DCH0CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coninv](coninv) module"]
pub type CONINV = crate::Reg<u32, _CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONINV;
#[doc = "`read()` method returns [coninv::R](coninv::R) reader structure"]
impl crate::Readable for CONINV {}
#[doc = "`write(|w| ..)` method takes [coninv::W](coninv::W) writer structure"]
impl crate::Writable for CONINV {}
#[doc = "DCH0CONINV register"]
pub mod coninv;
#[doc = "DCH0ECON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [econ](econ) module"]
pub type ECON = crate::Reg<u32, _ECON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECON;
#[doc = "`read()` method returns [econ::R](econ::R) reader structure"]
impl crate::Readable for ECON {}
#[doc = "`write(|w| ..)` method takes [econ::W](econ::W) writer structure"]
impl crate::Writable for ECON {}
#[doc = "DCH0ECON register"]
pub mod econ;
#[doc = "DCH0ECONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [econclr](econclr) module"]
pub type ECONCLR = crate::Reg<u32, _ECONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECONCLR;
#[doc = "`read()` method returns [econclr::R](econclr::R) reader structure"]
impl crate::Readable for ECONCLR {}
#[doc = "`write(|w| ..)` method takes [econclr::W](econclr::W) writer structure"]
impl crate::Writable for ECONCLR {}
#[doc = "DCH0ECONCLR register"]
pub mod econclr;
#[doc = "DCH0ECONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [econset](econset) module"]
pub type ECONSET = crate::Reg<u32, _ECONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECONSET;
#[doc = "`read()` method returns [econset::R](econset::R) reader structure"]
impl crate::Readable for ECONSET {}
#[doc = "`write(|w| ..)` method takes [econset::W](econset::W) writer structure"]
impl crate::Writable for ECONSET {}
#[doc = "DCH0ECONSET register"]
pub mod econset;
#[doc = "DCH0ECONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [econinv](econinv) module"]
pub type ECONINV = crate::Reg<u32, _ECONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECONINV;
#[doc = "`read()` method returns [econinv::R](econinv::R) reader structure"]
impl crate::Readable for ECONINV {}
#[doc = "`write(|w| ..)` method takes [econinv::W](econinv::W) writer structure"]
impl crate::Writable for ECONINV {}
#[doc = "DCH0ECONINV register"]
pub mod econinv;
#[doc = "DCH0INT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](int) module"]
pub type INT = crate::Reg<u32, _INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT;
#[doc = "`read()` method returns [int::R](int::R) reader structure"]
impl crate::Readable for INT {}
#[doc = "`write(|w| ..)` method takes [int::W](int::W) writer structure"]
impl crate::Writable for INT {}
#[doc = "DCH0INT register"]
pub mod int;
#[doc = "DCH0INTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`read()` method returns [intclr::R](intclr::R) reader structure"]
impl crate::Readable for INTCLR {}
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "DCH0INTCLR register"]
pub mod intclr;
#[doc = "DCH0INTSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`read()` method returns [intset::R](intset::R) reader structure"]
impl crate::Readable for INTSET {}
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "DCH0INTSET register"]
pub mod intset;
#[doc = "DCH0INTINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intinv](intinv) module"]
pub type INTINV = crate::Reg<u32, _INTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTINV;
#[doc = "`read()` method returns [intinv::R](intinv::R) reader structure"]
impl crate::Readable for INTINV {}
#[doc = "`write(|w| ..)` method takes [intinv::W](intinv::W) writer structure"]
impl crate::Writable for INTINV {}
#[doc = "DCH0INTINV register"]
pub mod intinv;
#[doc = "DCH0SSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssa](ssa) module"]
pub type SSA = crate::Reg<u32, _SSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSA;
#[doc = "`read()` method returns [ssa::R](ssa::R) reader structure"]
impl crate::Readable for SSA {}
#[doc = "`write(|w| ..)` method takes [ssa::W](ssa::W) writer structure"]
impl crate::Writable for SSA {}
#[doc = "DCH0SSA register"]
pub mod ssa;
#[doc = "DCH0SSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssaclr](ssaclr) module"]
pub type SSACLR = crate::Reg<u32, _SSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSACLR;
#[doc = "`read()` method returns [ssaclr::R](ssaclr::R) reader structure"]
impl crate::Readable for SSACLR {}
#[doc = "`write(|w| ..)` method takes [ssaclr::W](ssaclr::W) writer structure"]
impl crate::Writable for SSACLR {}
#[doc = "DCH0SSACLR register"]
pub mod ssaclr;
#[doc = "DCH0SSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssaset](ssaset) module"]
pub type SSASET = crate::Reg<u32, _SSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSASET;
#[doc = "`read()` method returns [ssaset::R](ssaset::R) reader structure"]
impl crate::Readable for SSASET {}
#[doc = "`write(|w| ..)` method takes [ssaset::W](ssaset::W) writer structure"]
impl crate::Writable for SSASET {}
#[doc = "DCH0SSASET register"]
pub mod ssaset;
#[doc = "DCH0SSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssainv](ssainv) module"]
pub type SSAINV = crate::Reg<u32, _SSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSAINV;
#[doc = "`read()` method returns [ssainv::R](ssainv::R) reader structure"]
impl crate::Readable for SSAINV {}
#[doc = "`write(|w| ..)` method takes [ssainv::W](ssainv::W) writer structure"]
impl crate::Writable for SSAINV {}
#[doc = "DCH0SSAINV register"]
pub mod ssainv;
#[doc = "DCH0DSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsa](dsa) module"]
pub type DSA = crate::Reg<u32, _DSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSA;
#[doc = "`read()` method returns [dsa::R](dsa::R) reader structure"]
impl crate::Readable for DSA {}
#[doc = "`write(|w| ..)` method takes [dsa::W](dsa::W) writer structure"]
impl crate::Writable for DSA {}
#[doc = "DCH0DSA register"]
pub mod dsa;
#[doc = "DCH0DSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsaclr](dsaclr) module"]
pub type DSACLR = crate::Reg<u32, _DSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSACLR;
#[doc = "`read()` method returns [dsaclr::R](dsaclr::R) reader structure"]
impl crate::Readable for DSACLR {}
#[doc = "`write(|w| ..)` method takes [dsaclr::W](dsaclr::W) writer structure"]
impl crate::Writable for DSACLR {}
#[doc = "DCH0DSACLR register"]
pub mod dsaclr;
#[doc = "DCH0DSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsaset](dsaset) module"]
pub type DSASET = crate::Reg<u32, _DSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSASET;
#[doc = "`read()` method returns [dsaset::R](dsaset::R) reader structure"]
impl crate::Readable for DSASET {}
#[doc = "`write(|w| ..)` method takes [dsaset::W](dsaset::W) writer structure"]
impl crate::Writable for DSASET {}
#[doc = "DCH0DSASET register"]
pub mod dsaset;
#[doc = "DCH0DSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsainv](dsainv) module"]
pub type DSAINV = crate::Reg<u32, _DSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSAINV;
#[doc = "`read()` method returns [dsainv::R](dsainv::R) reader structure"]
impl crate::Readable for DSAINV {}
#[doc = "`write(|w| ..)` method takes [dsainv::W](dsainv::W) writer structure"]
impl crate::Writable for DSAINV {}
#[doc = "DCH0DSAINV register"]
pub mod dsainv;
#[doc = "DCH0SSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiz](ssiz) module"]
pub type SSIZ = crate::Reg<u32, _SSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIZ;
#[doc = "`read()` method returns [ssiz::R](ssiz::R) reader structure"]
impl crate::Readable for SSIZ {}
#[doc = "`write(|w| ..)` method takes [ssiz::W](ssiz::W) writer structure"]
impl crate::Writable for SSIZ {}
#[doc = "DCH0SSIZ register"]
pub mod ssiz;
#[doc = "DCH0SSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssizclr](ssizclr) module"]
pub type SSIZCLR = crate::Reg<u32, _SSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIZCLR;
#[doc = "`read()` method returns [ssizclr::R](ssizclr::R) reader structure"]
impl crate::Readable for SSIZCLR {}
#[doc = "`write(|w| ..)` method takes [ssizclr::W](ssizclr::W) writer structure"]
impl crate::Writable for SSIZCLR {}
#[doc = "DCH0SSIZCLR register"]
pub mod ssizclr;
#[doc = "DCH0SSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssizset](ssizset) module"]
pub type SSIZSET = crate::Reg<u32, _SSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIZSET;
#[doc = "`read()` method returns [ssizset::R](ssizset::R) reader structure"]
impl crate::Readable for SSIZSET {}
#[doc = "`write(|w| ..)` method takes [ssizset::W](ssizset::W) writer structure"]
impl crate::Writable for SSIZSET {}
#[doc = "DCH0SSIZSET register"]
pub mod ssizset;
#[doc = "DCH0SSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssizinv](ssizinv) module"]
pub type SSIZINV = crate::Reg<u32, _SSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSIZINV;
#[doc = "`read()` method returns [ssizinv::R](ssizinv::R) reader structure"]
impl crate::Readable for SSIZINV {}
#[doc = "`write(|w| ..)` method takes [ssizinv::W](ssizinv::W) writer structure"]
impl crate::Writable for SSIZINV {}
#[doc = "DCH0SSIZINV register"]
pub mod ssizinv;
#[doc = "DCH0DSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsiz](dsiz) module"]
pub type DSIZ = crate::Reg<u32, _DSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSIZ;
#[doc = "`read()` method returns [dsiz::R](dsiz::R) reader structure"]
impl crate::Readable for DSIZ {}
#[doc = "`write(|w| ..)` method takes [dsiz::W](dsiz::W) writer structure"]
impl crate::Writable for DSIZ {}
#[doc = "DCH0DSIZ register"]
pub mod dsiz;
#[doc = "DCH0DSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsizclr](dsizclr) module"]
pub type DSIZCLR = crate::Reg<u32, _DSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSIZCLR;
#[doc = "`read()` method returns [dsizclr::R](dsizclr::R) reader structure"]
impl crate::Readable for DSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dsizclr::W](dsizclr::W) writer structure"]
impl crate::Writable for DSIZCLR {}
#[doc = "DCH0DSIZCLR register"]
pub mod dsizclr;
#[doc = "DCH0DSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsizset](dsizset) module"]
pub type DSIZSET = crate::Reg<u32, _DSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSIZSET;
#[doc = "`read()` method returns [dsizset::R](dsizset::R) reader structure"]
impl crate::Readable for DSIZSET {}
#[doc = "`write(|w| ..)` method takes [dsizset::W](dsizset::W) writer structure"]
impl crate::Writable for DSIZSET {}
#[doc = "DCH0DSIZSET register"]
pub mod dsizset;
#[doc = "DCH0DSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsizinv](dsizinv) module"]
pub type DSIZINV = crate::Reg<u32, _DSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSIZINV;
#[doc = "`read()` method returns [dsizinv::R](dsizinv::R) reader structure"]
impl crate::Readable for DSIZINV {}
#[doc = "`write(|w| ..)` method takes [dsizinv::W](dsizinv::W) writer structure"]
impl crate::Writable for DSIZINV {}
#[doc = "DCH0DSIZINV register"]
pub mod dsizinv;
#[doc = "DCH0SPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptr](sptr) module"]
pub type SPTR = crate::Reg<u32, _SPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPTR;
#[doc = "`read()` method returns [sptr::R](sptr::R) reader structure"]
impl crate::Readable for SPTR {}
#[doc = "`write(|w| ..)` method takes [sptr::W](sptr::W) writer structure"]
impl crate::Writable for SPTR {}
#[doc = "DCH0SPTR register"]
pub mod sptr;
#[doc = "DCH0SPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptrclr](sptrclr) module"]
pub type SPTRCLR = crate::Reg<u32, _SPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPTRCLR;
#[doc = "`read()` method returns [sptrclr::R](sptrclr::R) reader structure"]
impl crate::Readable for SPTRCLR {}
#[doc = "`write(|w| ..)` method takes [sptrclr::W](sptrclr::W) writer structure"]
impl crate::Writable for SPTRCLR {}
#[doc = "DCH0SPTRCLR register"]
pub mod sptrclr;
#[doc = "DCH0SPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptrset](sptrset) module"]
pub type SPTRSET = crate::Reg<u32, _SPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPTRSET;
#[doc = "`read()` method returns [sptrset::R](sptrset::R) reader structure"]
impl crate::Readable for SPTRSET {}
#[doc = "`write(|w| ..)` method takes [sptrset::W](sptrset::W) writer structure"]
impl crate::Writable for SPTRSET {}
#[doc = "DCH0SPTRSET register"]
pub mod sptrset;
#[doc = "DCH0SPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptrinv](sptrinv) module"]
pub type SPTRINV = crate::Reg<u32, _SPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPTRINV;
#[doc = "`read()` method returns [sptrinv::R](sptrinv::R) reader structure"]
impl crate::Readable for SPTRINV {}
#[doc = "`write(|w| ..)` method takes [sptrinv::W](sptrinv::W) writer structure"]
impl crate::Writable for SPTRINV {}
#[doc = "DCH0SPTRINV register"]
pub mod sptrinv;
#[doc = "DCH0DPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dptr](dptr) module"]
pub type DPTR = crate::Reg<u32, _DPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPTR;
#[doc = "`read()` method returns [dptr::R](dptr::R) reader structure"]
impl crate::Readable for DPTR {}
#[doc = "`write(|w| ..)` method takes [dptr::W](dptr::W) writer structure"]
impl crate::Writable for DPTR {}
#[doc = "DCH0DPTR register"]
pub mod dptr;
#[doc = "DCH0DPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dptrclr](dptrclr) module"]
pub type DPTRCLR = crate::Reg<u32, _DPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPTRCLR;
#[doc = "`read()` method returns [dptrclr::R](dptrclr::R) reader structure"]
impl crate::Readable for DPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dptrclr::W](dptrclr::W) writer structure"]
impl crate::Writable for DPTRCLR {}
#[doc = "DCH0DPTRCLR register"]
pub mod dptrclr;
#[doc = "DCH0DPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dptrset](dptrset) module"]
pub type DPTRSET = crate::Reg<u32, _DPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPTRSET;
#[doc = "`read()` method returns [dptrset::R](dptrset::R) reader structure"]
impl crate::Readable for DPTRSET {}
#[doc = "`write(|w| ..)` method takes [dptrset::W](dptrset::W) writer structure"]
impl crate::Writable for DPTRSET {}
#[doc = "DCH0DPTRSET register"]
pub mod dptrset;
#[doc = "DCH0DPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dptrinv](dptrinv) module"]
pub type DPTRINV = crate::Reg<u32, _DPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPTRINV;
#[doc = "`read()` method returns [dptrinv::R](dptrinv::R) reader structure"]
impl crate::Readable for DPTRINV {}
#[doc = "`write(|w| ..)` method takes [dptrinv::W](dptrinv::W) writer structure"]
impl crate::Writable for DPTRINV {}
#[doc = "DCH0DPTRINV register"]
pub mod dptrinv;
#[doc = "DCH0CSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csiz](csiz) module"]
pub type CSIZ = crate::Reg<u32, _CSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIZ;
#[doc = "`read()` method returns [csiz::R](csiz::R) reader structure"]
impl crate::Readable for CSIZ {}
#[doc = "`write(|w| ..)` method takes [csiz::W](csiz::W) writer structure"]
impl crate::Writable for CSIZ {}
#[doc = "DCH0CSIZ register"]
pub mod csiz;
#[doc = "DCH0CSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csizclr](csizclr) module"]
pub type CSIZCLR = crate::Reg<u32, _CSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIZCLR;
#[doc = "`read()` method returns [csizclr::R](csizclr::R) reader structure"]
impl crate::Readable for CSIZCLR {}
#[doc = "`write(|w| ..)` method takes [csizclr::W](csizclr::W) writer structure"]
impl crate::Writable for CSIZCLR {}
#[doc = "DCH0CSIZCLR register"]
pub mod csizclr;
#[doc = "DCH0CSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csizset](csizset) module"]
pub type CSIZSET = crate::Reg<u32, _CSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIZSET;
#[doc = "`read()` method returns [csizset::R](csizset::R) reader structure"]
impl crate::Readable for CSIZSET {}
#[doc = "`write(|w| ..)` method takes [csizset::W](csizset::W) writer structure"]
impl crate::Writable for CSIZSET {}
#[doc = "DCH0CSIZSET register"]
pub mod csizset;
#[doc = "DCH0CSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csizinv](csizinv) module"]
pub type CSIZINV = crate::Reg<u32, _CSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIZINV;
#[doc = "`read()` method returns [csizinv::R](csizinv::R) reader structure"]
impl crate::Readable for CSIZINV {}
#[doc = "`write(|w| ..)` method takes [csizinv::W](csizinv::W) writer structure"]
impl crate::Writable for CSIZINV {}
#[doc = "DCH0CSIZINV register"]
pub mod csizinv;
#[doc = "DCH0CPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cptr](cptr) module"]
pub type CPTR = crate::Reg<u32, _CPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPTR;
#[doc = "`read()` method returns [cptr::R](cptr::R) reader structure"]
impl crate::Readable for CPTR {}
#[doc = "`write(|w| ..)` method takes [cptr::W](cptr::W) writer structure"]
impl crate::Writable for CPTR {}
#[doc = "DCH0CPTR register"]
pub mod cptr;
#[doc = "DCH0CPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cptrclr](cptrclr) module"]
pub type CPTRCLR = crate::Reg<u32, _CPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPTRCLR;
#[doc = "`read()` method returns [cptrclr::R](cptrclr::R) reader structure"]
impl crate::Readable for CPTRCLR {}
#[doc = "`write(|w| ..)` method takes [cptrclr::W](cptrclr::W) writer structure"]
impl crate::Writable for CPTRCLR {}
#[doc = "DCH0CPTRCLR register"]
pub mod cptrclr;
#[doc = "DCH0CPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cptrset](cptrset) module"]
pub type CPTRSET = crate::Reg<u32, _CPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPTRSET;
#[doc = "`read()` method returns [cptrset::R](cptrset::R) reader structure"]
impl crate::Readable for CPTRSET {}
#[doc = "`write(|w| ..)` method takes [cptrset::W](cptrset::W) writer structure"]
impl crate::Writable for CPTRSET {}
#[doc = "DCH0CPTRSET register"]
pub mod cptrset;
#[doc = "DCH0CPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cptrinv](cptrinv) module"]
pub type CPTRINV = crate::Reg<u32, _CPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPTRINV;
#[doc = "`read()` method returns [cptrinv::R](cptrinv::R) reader structure"]
impl crate::Readable for CPTRINV {}
#[doc = "`write(|w| ..)` method takes [cptrinv::W](cptrinv::W) writer structure"]
impl crate::Writable for CPTRINV {}
#[doc = "DCH0CPTRINV register"]
pub mod cptrinv;
#[doc = "DCH0DAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dat](dat) module"]
pub type DAT = crate::Reg<u32, _DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAT;
#[doc = "`read()` method returns [dat::R](dat::R) reader structure"]
impl crate::Readable for DAT {}
#[doc = "`write(|w| ..)` method takes [dat::W](dat::W) writer structure"]
impl crate::Writable for DAT {}
#[doc = "DCH0DAT register"]
pub mod dat;
#[doc = "DCH0DATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datclr](datclr) module"]
pub type DATCLR = crate::Reg<u32, _DATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATCLR;
#[doc = "`read()` method returns [datclr::R](datclr::R) reader structure"]
impl crate::Readable for DATCLR {}
#[doc = "`write(|w| ..)` method takes [datclr::W](datclr::W) writer structure"]
impl crate::Writable for DATCLR {}
#[doc = "DCH0DATCLR register"]
pub mod datclr;
#[doc = "DCH0DATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datset](datset) module"]
pub type DATSET = crate::Reg<u32, _DATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATSET;
#[doc = "`read()` method returns [datset::R](datset::R) reader structure"]
impl crate::Readable for DATSET {}
#[doc = "`write(|w| ..)` method takes [datset::W](datset::W) writer structure"]
impl crate::Writable for DATSET {}
#[doc = "DCH0DATSET register"]
pub mod datset;
#[doc = "DCH0DATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datinv](datinv) module"]
pub type DATINV = crate::Reg<u32, _DATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATINV;
#[doc = "`read()` method returns [datinv::R](datinv::R) reader structure"]
impl crate::Readable for DATINV {}
#[doc = "`write(|w| ..)` method takes [datinv::W](datinv::W) writer structure"]
impl crate::Writable for DATINV {}
#[doc = "DCH0DATINV register"]
pub mod datinv;
