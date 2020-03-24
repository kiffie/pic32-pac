#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH2CON register"]
    pub dch2con: DCH2CON,
    #[doc = "0x04 - DCH2CONCLR register"]
    pub dch2conclr: DCH2CONCLR,
    #[doc = "0x08 - DCH2CONSET register"]
    pub dch2conset: DCH2CONSET,
    #[doc = "0x0c - DCH2CONINV register"]
    pub dch2coninv: DCH2CONINV,
    #[doc = "0x10 - DCH2ECON register"]
    pub dch2econ: DCH2ECON,
    #[doc = "0x14 - DCH2ECONCLR register"]
    pub dch2econclr: DCH2ECONCLR,
    #[doc = "0x18 - DCH2ECONSET register"]
    pub dch2econset: DCH2ECONSET,
    #[doc = "0x1c - DCH2ECONINV register"]
    pub dch2econinv: DCH2ECONINV,
    #[doc = "0x20 - DCH2INT register"]
    pub dch2int: DCH2INT,
    #[doc = "0x24 - DCH2INTCLR register"]
    pub dch2intclr: DCH2INTCLR,
    #[doc = "0x28 - DCH2INTSET register"]
    pub dch2intset: DCH2INTSET,
    #[doc = "0x2c - DCH2INTINV register"]
    pub dch2intinv: DCH2INTINV,
    #[doc = "0x30 - DCH2SSA register"]
    pub dch2ssa: DCH2SSA,
    #[doc = "0x34 - DCH2SSACLR register"]
    pub dch2ssaclr: DCH2SSACLR,
    #[doc = "0x38 - DCH2SSASET register"]
    pub dch2ssaset: DCH2SSASET,
    #[doc = "0x3c - DCH2SSAINV register"]
    pub dch2ssainv: DCH2SSAINV,
    #[doc = "0x40 - DCH2DSA register"]
    pub dch2dsa: DCH2DSA,
    #[doc = "0x44 - DCH2DSACLR register"]
    pub dch2dsaclr: DCH2DSACLR,
    #[doc = "0x48 - DCH2DSASET register"]
    pub dch2dsaset: DCH2DSASET,
    #[doc = "0x4c - DCH2DSAINV register"]
    pub dch2dsainv: DCH2DSAINV,
    #[doc = "0x50 - DCH2SSIZ register"]
    pub dch2ssiz: DCH2SSIZ,
    #[doc = "0x54 - DCH2SSIZCLR register"]
    pub dch2ssizclr: DCH2SSIZCLR,
    #[doc = "0x58 - DCH2SSIZSET register"]
    pub dch2ssizset: DCH2SSIZSET,
    #[doc = "0x5c - DCH2SSIZINV register"]
    pub dch2ssizinv: DCH2SSIZINV,
    #[doc = "0x60 - DCH2DSIZ register"]
    pub dch2dsiz: DCH2DSIZ,
    #[doc = "0x64 - DCH2DSIZCLR register"]
    pub dch2dsizclr: DCH2DSIZCLR,
    #[doc = "0x68 - DCH2DSIZSET register"]
    pub dch2dsizset: DCH2DSIZSET,
    #[doc = "0x6c - DCH2DSIZINV register"]
    pub dch2dsizinv: DCH2DSIZINV,
    #[doc = "0x70 - DCH2SPTR register"]
    pub dch2sptr: DCH2SPTR,
    #[doc = "0x74 - DCH2SPTRCLR register"]
    pub dch2sptrclr: DCH2SPTRCLR,
    #[doc = "0x78 - DCH2SPTRSET register"]
    pub dch2sptrset: DCH2SPTRSET,
    #[doc = "0x7c - DCH2SPTRINV register"]
    pub dch2sptrinv: DCH2SPTRINV,
    #[doc = "0x80 - DCH2DPTR register"]
    pub dch2dptr: DCH2DPTR,
    #[doc = "0x84 - DCH2DPTRCLR register"]
    pub dch2dptrclr: DCH2DPTRCLR,
    #[doc = "0x88 - DCH2DPTRSET register"]
    pub dch2dptrset: DCH2DPTRSET,
    #[doc = "0x8c - DCH2DPTRINV register"]
    pub dch2dptrinv: DCH2DPTRINV,
    #[doc = "0x90 - DCH2CSIZ register"]
    pub dch2csiz: DCH2CSIZ,
    #[doc = "0x94 - DCH2CSIZCLR register"]
    pub dch2csizclr: DCH2CSIZCLR,
    #[doc = "0x98 - DCH2CSIZSET register"]
    pub dch2csizset: DCH2CSIZSET,
    #[doc = "0x9c - DCH2CSIZINV register"]
    pub dch2csizinv: DCH2CSIZINV,
    #[doc = "0xa0 - DCH2CPTR register"]
    pub dch2cptr: DCH2CPTR,
    #[doc = "0xa4 - DCH2CPTRCLR register"]
    pub dch2cptrclr: DCH2CPTRCLR,
    #[doc = "0xa8 - DCH2CPTRSET register"]
    pub dch2cptrset: DCH2CPTRSET,
    #[doc = "0xac - DCH2CPTRINV register"]
    pub dch2cptrinv: DCH2CPTRINV,
    #[doc = "0xb0 - DCH2DAT register"]
    pub dch2dat: DCH2DAT,
    #[doc = "0xb4 - DCH2DATCLR register"]
    pub dch2datclr: DCH2DATCLR,
    #[doc = "0xb8 - DCH2DATSET register"]
    pub dch2datset: DCH2DATSET,
    #[doc = "0xbc - DCH2DATINV register"]
    pub dch2datinv: DCH2DATINV,
}
#[doc = "DCH2CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2con](dch2con) module"]
pub type DCH2CON = crate::Reg<u32, _DCH2CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CON;
#[doc = "`read()` method returns [dch2con::R](dch2con::R) reader structure"]
impl crate::Readable for DCH2CON {}
#[doc = "`write(|w| ..)` method takes [dch2con::W](dch2con::W) writer structure"]
impl crate::Writable for DCH2CON {}
#[doc = "DCH2CON register"]
pub mod dch2con;
#[doc = "DCH2CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2conclr](dch2conclr) module"]
pub type DCH2CONCLR = crate::Reg<u32, _DCH2CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CONCLR;
#[doc = "`read()` method returns [dch2conclr::R](dch2conclr::R) reader structure"]
impl crate::Readable for DCH2CONCLR {}
#[doc = "`write(|w| ..)` method takes [dch2conclr::W](dch2conclr::W) writer structure"]
impl crate::Writable for DCH2CONCLR {}
#[doc = "DCH2CONCLR register"]
pub mod dch2conclr;
#[doc = "DCH2CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2conset](dch2conset) module"]
pub type DCH2CONSET = crate::Reg<u32, _DCH2CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CONSET;
#[doc = "`read()` method returns [dch2conset::R](dch2conset::R) reader structure"]
impl crate::Readable for DCH2CONSET {}
#[doc = "`write(|w| ..)` method takes [dch2conset::W](dch2conset::W) writer structure"]
impl crate::Writable for DCH2CONSET {}
#[doc = "DCH2CONSET register"]
pub mod dch2conset;
#[doc = "DCH2CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2coninv](dch2coninv) module"]
pub type DCH2CONINV = crate::Reg<u32, _DCH2CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CONINV;
#[doc = "`read()` method returns [dch2coninv::R](dch2coninv::R) reader structure"]
impl crate::Readable for DCH2CONINV {}
#[doc = "`write(|w| ..)` method takes [dch2coninv::W](dch2coninv::W) writer structure"]
impl crate::Writable for DCH2CONINV {}
#[doc = "DCH2CONINV register"]
pub mod dch2coninv;
#[doc = "DCH2ECON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2econ](dch2econ) module"]
pub type DCH2ECON = crate::Reg<u32, _DCH2ECON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2ECON;
#[doc = "`read()` method returns [dch2econ::R](dch2econ::R) reader structure"]
impl crate::Readable for DCH2ECON {}
#[doc = "`write(|w| ..)` method takes [dch2econ::W](dch2econ::W) writer structure"]
impl crate::Writable for DCH2ECON {}
#[doc = "DCH2ECON register"]
pub mod dch2econ;
#[doc = "DCH2ECONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2econclr](dch2econclr) module"]
pub type DCH2ECONCLR = crate::Reg<u32, _DCH2ECONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2ECONCLR;
#[doc = "`read()` method returns [dch2econclr::R](dch2econclr::R) reader structure"]
impl crate::Readable for DCH2ECONCLR {}
#[doc = "`write(|w| ..)` method takes [dch2econclr::W](dch2econclr::W) writer structure"]
impl crate::Writable for DCH2ECONCLR {}
#[doc = "DCH2ECONCLR register"]
pub mod dch2econclr;
#[doc = "DCH2ECONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2econset](dch2econset) module"]
pub type DCH2ECONSET = crate::Reg<u32, _DCH2ECONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2ECONSET;
#[doc = "`read()` method returns [dch2econset::R](dch2econset::R) reader structure"]
impl crate::Readable for DCH2ECONSET {}
#[doc = "`write(|w| ..)` method takes [dch2econset::W](dch2econset::W) writer structure"]
impl crate::Writable for DCH2ECONSET {}
#[doc = "DCH2ECONSET register"]
pub mod dch2econset;
#[doc = "DCH2ECONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2econinv](dch2econinv) module"]
pub type DCH2ECONINV = crate::Reg<u32, _DCH2ECONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2ECONINV;
#[doc = "`read()` method returns [dch2econinv::R](dch2econinv::R) reader structure"]
impl crate::Readable for DCH2ECONINV {}
#[doc = "`write(|w| ..)` method takes [dch2econinv::W](dch2econinv::W) writer structure"]
impl crate::Writable for DCH2ECONINV {}
#[doc = "DCH2ECONINV register"]
pub mod dch2econinv;
#[doc = "DCH2INT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2int](dch2int) module"]
pub type DCH2INT = crate::Reg<u32, _DCH2INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2INT;
#[doc = "`read()` method returns [dch2int::R](dch2int::R) reader structure"]
impl crate::Readable for DCH2INT {}
#[doc = "`write(|w| ..)` method takes [dch2int::W](dch2int::W) writer structure"]
impl crate::Writable for DCH2INT {}
#[doc = "DCH2INT register"]
pub mod dch2int;
#[doc = "DCH2INTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2intclr](dch2intclr) module"]
pub type DCH2INTCLR = crate::Reg<u32, _DCH2INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2INTCLR;
#[doc = "`read()` method returns [dch2intclr::R](dch2intclr::R) reader structure"]
impl crate::Readable for DCH2INTCLR {}
#[doc = "`write(|w| ..)` method takes [dch2intclr::W](dch2intclr::W) writer structure"]
impl crate::Writable for DCH2INTCLR {}
#[doc = "DCH2INTCLR register"]
pub mod dch2intclr;
#[doc = "DCH2INTSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2intset](dch2intset) module"]
pub type DCH2INTSET = crate::Reg<u32, _DCH2INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2INTSET;
#[doc = "`read()` method returns [dch2intset::R](dch2intset::R) reader structure"]
impl crate::Readable for DCH2INTSET {}
#[doc = "`write(|w| ..)` method takes [dch2intset::W](dch2intset::W) writer structure"]
impl crate::Writable for DCH2INTSET {}
#[doc = "DCH2INTSET register"]
pub mod dch2intset;
#[doc = "DCH2INTINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2intinv](dch2intinv) module"]
pub type DCH2INTINV = crate::Reg<u32, _DCH2INTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2INTINV;
#[doc = "`read()` method returns [dch2intinv::R](dch2intinv::R) reader structure"]
impl crate::Readable for DCH2INTINV {}
#[doc = "`write(|w| ..)` method takes [dch2intinv::W](dch2intinv::W) writer structure"]
impl crate::Writable for DCH2INTINV {}
#[doc = "DCH2INTINV register"]
pub mod dch2intinv;
#[doc = "DCH2SSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2ssa](dch2ssa) module"]
pub type DCH2SSA = crate::Reg<u32, _DCH2SSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SSA;
#[doc = "`read()` method returns [dch2ssa::R](dch2ssa::R) reader structure"]
impl crate::Readable for DCH2SSA {}
#[doc = "`write(|w| ..)` method takes [dch2ssa::W](dch2ssa::W) writer structure"]
impl crate::Writable for DCH2SSA {}
#[doc = "DCH2SSA register"]
pub mod dch2ssa;
#[doc = "DCH2SSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2ssaclr](dch2ssaclr) module"]
pub type DCH2SSACLR = crate::Reg<u32, _DCH2SSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SSACLR;
#[doc = "`read()` method returns [dch2ssaclr::R](dch2ssaclr::R) reader structure"]
impl crate::Readable for DCH2SSACLR {}
#[doc = "`write(|w| ..)` method takes [dch2ssaclr::W](dch2ssaclr::W) writer structure"]
impl crate::Writable for DCH2SSACLR {}
#[doc = "DCH2SSACLR register"]
pub mod dch2ssaclr;
#[doc = "DCH2SSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2ssaset](dch2ssaset) module"]
pub type DCH2SSASET = crate::Reg<u32, _DCH2SSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SSASET;
#[doc = "`read()` method returns [dch2ssaset::R](dch2ssaset::R) reader structure"]
impl crate::Readable for DCH2SSASET {}
#[doc = "`write(|w| ..)` method takes [dch2ssaset::W](dch2ssaset::W) writer structure"]
impl crate::Writable for DCH2SSASET {}
#[doc = "DCH2SSASET register"]
pub mod dch2ssaset;
#[doc = "DCH2SSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2ssainv](dch2ssainv) module"]
pub type DCH2SSAINV = crate::Reg<u32, _DCH2SSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SSAINV;
#[doc = "`read()` method returns [dch2ssainv::R](dch2ssainv::R) reader structure"]
impl crate::Readable for DCH2SSAINV {}
#[doc = "`write(|w| ..)` method takes [dch2ssainv::W](dch2ssainv::W) writer structure"]
impl crate::Writable for DCH2SSAINV {}
#[doc = "DCH2SSAINV register"]
pub mod dch2ssainv;
#[doc = "DCH2DSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dsa](dch2dsa) module"]
pub type DCH2DSA = crate::Reg<u32, _DCH2DSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DSA;
#[doc = "`read()` method returns [dch2dsa::R](dch2dsa::R) reader structure"]
impl crate::Readable for DCH2DSA {}
#[doc = "`write(|w| ..)` method takes [dch2dsa::W](dch2dsa::W) writer structure"]
impl crate::Writable for DCH2DSA {}
#[doc = "DCH2DSA register"]
pub mod dch2dsa;
#[doc = "DCH2DSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dsaclr](dch2dsaclr) module"]
pub type DCH2DSACLR = crate::Reg<u32, _DCH2DSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DSACLR;
#[doc = "`read()` method returns [dch2dsaclr::R](dch2dsaclr::R) reader structure"]
impl crate::Readable for DCH2DSACLR {}
#[doc = "`write(|w| ..)` method takes [dch2dsaclr::W](dch2dsaclr::W) writer structure"]
impl crate::Writable for DCH2DSACLR {}
#[doc = "DCH2DSACLR register"]
pub mod dch2dsaclr;
#[doc = "DCH2DSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dsaset](dch2dsaset) module"]
pub type DCH2DSASET = crate::Reg<u32, _DCH2DSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DSASET;
#[doc = "`read()` method returns [dch2dsaset::R](dch2dsaset::R) reader structure"]
impl crate::Readable for DCH2DSASET {}
#[doc = "`write(|w| ..)` method takes [dch2dsaset::W](dch2dsaset::W) writer structure"]
impl crate::Writable for DCH2DSASET {}
#[doc = "DCH2DSASET register"]
pub mod dch2dsaset;
#[doc = "DCH2DSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dsainv](dch2dsainv) module"]
pub type DCH2DSAINV = crate::Reg<u32, _DCH2DSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DSAINV;
#[doc = "`read()` method returns [dch2dsainv::R](dch2dsainv::R) reader structure"]
impl crate::Readable for DCH2DSAINV {}
#[doc = "`write(|w| ..)` method takes [dch2dsainv::W](dch2dsainv::W) writer structure"]
impl crate::Writable for DCH2DSAINV {}
#[doc = "DCH2DSAINV register"]
pub mod dch2dsainv;
#[doc = "DCH2SSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2ssiz](dch2ssiz) module"]
pub type DCH2SSIZ = crate::Reg<u32, _DCH2SSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SSIZ;
#[doc = "`read()` method returns [dch2ssiz::R](dch2ssiz::R) reader structure"]
impl crate::Readable for DCH2SSIZ {}
#[doc = "`write(|w| ..)` method takes [dch2ssiz::W](dch2ssiz::W) writer structure"]
impl crate::Writable for DCH2SSIZ {}
#[doc = "DCH2SSIZ register"]
pub mod dch2ssiz;
#[doc = "DCH2SSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2ssizclr](dch2ssizclr) module"]
pub type DCH2SSIZCLR = crate::Reg<u32, _DCH2SSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SSIZCLR;
#[doc = "`read()` method returns [dch2ssizclr::R](dch2ssizclr::R) reader structure"]
impl crate::Readable for DCH2SSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch2ssizclr::W](dch2ssizclr::W) writer structure"]
impl crate::Writable for DCH2SSIZCLR {}
#[doc = "DCH2SSIZCLR register"]
pub mod dch2ssizclr;
#[doc = "DCH2SSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2ssizset](dch2ssizset) module"]
pub type DCH2SSIZSET = crate::Reg<u32, _DCH2SSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SSIZSET;
#[doc = "`read()` method returns [dch2ssizset::R](dch2ssizset::R) reader structure"]
impl crate::Readable for DCH2SSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch2ssizset::W](dch2ssizset::W) writer structure"]
impl crate::Writable for DCH2SSIZSET {}
#[doc = "DCH2SSIZSET register"]
pub mod dch2ssizset;
#[doc = "DCH2SSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2ssizinv](dch2ssizinv) module"]
pub type DCH2SSIZINV = crate::Reg<u32, _DCH2SSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SSIZINV;
#[doc = "`read()` method returns [dch2ssizinv::R](dch2ssizinv::R) reader structure"]
impl crate::Readable for DCH2SSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch2ssizinv::W](dch2ssizinv::W) writer structure"]
impl crate::Writable for DCH2SSIZINV {}
#[doc = "DCH2SSIZINV register"]
pub mod dch2ssizinv;
#[doc = "DCH2DSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dsiz](dch2dsiz) module"]
pub type DCH2DSIZ = crate::Reg<u32, _DCH2DSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DSIZ;
#[doc = "`read()` method returns [dch2dsiz::R](dch2dsiz::R) reader structure"]
impl crate::Readable for DCH2DSIZ {}
#[doc = "`write(|w| ..)` method takes [dch2dsiz::W](dch2dsiz::W) writer structure"]
impl crate::Writable for DCH2DSIZ {}
#[doc = "DCH2DSIZ register"]
pub mod dch2dsiz;
#[doc = "DCH2DSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dsizclr](dch2dsizclr) module"]
pub type DCH2DSIZCLR = crate::Reg<u32, _DCH2DSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DSIZCLR;
#[doc = "`read()` method returns [dch2dsizclr::R](dch2dsizclr::R) reader structure"]
impl crate::Readable for DCH2DSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch2dsizclr::W](dch2dsizclr::W) writer structure"]
impl crate::Writable for DCH2DSIZCLR {}
#[doc = "DCH2DSIZCLR register"]
pub mod dch2dsizclr;
#[doc = "DCH2DSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dsizset](dch2dsizset) module"]
pub type DCH2DSIZSET = crate::Reg<u32, _DCH2DSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DSIZSET;
#[doc = "`read()` method returns [dch2dsizset::R](dch2dsizset::R) reader structure"]
impl crate::Readable for DCH2DSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch2dsizset::W](dch2dsizset::W) writer structure"]
impl crate::Writable for DCH2DSIZSET {}
#[doc = "DCH2DSIZSET register"]
pub mod dch2dsizset;
#[doc = "DCH2DSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dsizinv](dch2dsizinv) module"]
pub type DCH2DSIZINV = crate::Reg<u32, _DCH2DSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DSIZINV;
#[doc = "`read()` method returns [dch2dsizinv::R](dch2dsizinv::R) reader structure"]
impl crate::Readable for DCH2DSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch2dsizinv::W](dch2dsizinv::W) writer structure"]
impl crate::Writable for DCH2DSIZINV {}
#[doc = "DCH2DSIZINV register"]
pub mod dch2dsizinv;
#[doc = "DCH2SPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2sptr](dch2sptr) module"]
pub type DCH2SPTR = crate::Reg<u32, _DCH2SPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SPTR;
#[doc = "`read()` method returns [dch2sptr::R](dch2sptr::R) reader structure"]
impl crate::Readable for DCH2SPTR {}
#[doc = "`write(|w| ..)` method takes [dch2sptr::W](dch2sptr::W) writer structure"]
impl crate::Writable for DCH2SPTR {}
#[doc = "DCH2SPTR register"]
pub mod dch2sptr;
#[doc = "DCH2SPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2sptrclr](dch2sptrclr) module"]
pub type DCH2SPTRCLR = crate::Reg<u32, _DCH2SPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SPTRCLR;
#[doc = "`read()` method returns [dch2sptrclr::R](dch2sptrclr::R) reader structure"]
impl crate::Readable for DCH2SPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch2sptrclr::W](dch2sptrclr::W) writer structure"]
impl crate::Writable for DCH2SPTRCLR {}
#[doc = "DCH2SPTRCLR register"]
pub mod dch2sptrclr;
#[doc = "DCH2SPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2sptrset](dch2sptrset) module"]
pub type DCH2SPTRSET = crate::Reg<u32, _DCH2SPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SPTRSET;
#[doc = "`read()` method returns [dch2sptrset::R](dch2sptrset::R) reader structure"]
impl crate::Readable for DCH2SPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch2sptrset::W](dch2sptrset::W) writer structure"]
impl crate::Writable for DCH2SPTRSET {}
#[doc = "DCH2SPTRSET register"]
pub mod dch2sptrset;
#[doc = "DCH2SPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2sptrinv](dch2sptrinv) module"]
pub type DCH2SPTRINV = crate::Reg<u32, _DCH2SPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2SPTRINV;
#[doc = "`read()` method returns [dch2sptrinv::R](dch2sptrinv::R) reader structure"]
impl crate::Readable for DCH2SPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch2sptrinv::W](dch2sptrinv::W) writer structure"]
impl crate::Writable for DCH2SPTRINV {}
#[doc = "DCH2SPTRINV register"]
pub mod dch2sptrinv;
#[doc = "DCH2DPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dptr](dch2dptr) module"]
pub type DCH2DPTR = crate::Reg<u32, _DCH2DPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DPTR;
#[doc = "`read()` method returns [dch2dptr::R](dch2dptr::R) reader structure"]
impl crate::Readable for DCH2DPTR {}
#[doc = "`write(|w| ..)` method takes [dch2dptr::W](dch2dptr::W) writer structure"]
impl crate::Writable for DCH2DPTR {}
#[doc = "DCH2DPTR register"]
pub mod dch2dptr;
#[doc = "DCH2DPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dptrclr](dch2dptrclr) module"]
pub type DCH2DPTRCLR = crate::Reg<u32, _DCH2DPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DPTRCLR;
#[doc = "`read()` method returns [dch2dptrclr::R](dch2dptrclr::R) reader structure"]
impl crate::Readable for DCH2DPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch2dptrclr::W](dch2dptrclr::W) writer structure"]
impl crate::Writable for DCH2DPTRCLR {}
#[doc = "DCH2DPTRCLR register"]
pub mod dch2dptrclr;
#[doc = "DCH2DPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dptrset](dch2dptrset) module"]
pub type DCH2DPTRSET = crate::Reg<u32, _DCH2DPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DPTRSET;
#[doc = "`read()` method returns [dch2dptrset::R](dch2dptrset::R) reader structure"]
impl crate::Readable for DCH2DPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch2dptrset::W](dch2dptrset::W) writer structure"]
impl crate::Writable for DCH2DPTRSET {}
#[doc = "DCH2DPTRSET register"]
pub mod dch2dptrset;
#[doc = "DCH2DPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dptrinv](dch2dptrinv) module"]
pub type DCH2DPTRINV = crate::Reg<u32, _DCH2DPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DPTRINV;
#[doc = "`read()` method returns [dch2dptrinv::R](dch2dptrinv::R) reader structure"]
impl crate::Readable for DCH2DPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch2dptrinv::W](dch2dptrinv::W) writer structure"]
impl crate::Writable for DCH2DPTRINV {}
#[doc = "DCH2DPTRINV register"]
pub mod dch2dptrinv;
#[doc = "DCH2CSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2csiz](dch2csiz) module"]
pub type DCH2CSIZ = crate::Reg<u32, _DCH2CSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CSIZ;
#[doc = "`read()` method returns [dch2csiz::R](dch2csiz::R) reader structure"]
impl crate::Readable for DCH2CSIZ {}
#[doc = "`write(|w| ..)` method takes [dch2csiz::W](dch2csiz::W) writer structure"]
impl crate::Writable for DCH2CSIZ {}
#[doc = "DCH2CSIZ register"]
pub mod dch2csiz;
#[doc = "DCH2CSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2csizclr](dch2csizclr) module"]
pub type DCH2CSIZCLR = crate::Reg<u32, _DCH2CSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CSIZCLR;
#[doc = "`read()` method returns [dch2csizclr::R](dch2csizclr::R) reader structure"]
impl crate::Readable for DCH2CSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch2csizclr::W](dch2csizclr::W) writer structure"]
impl crate::Writable for DCH2CSIZCLR {}
#[doc = "DCH2CSIZCLR register"]
pub mod dch2csizclr;
#[doc = "DCH2CSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2csizset](dch2csizset) module"]
pub type DCH2CSIZSET = crate::Reg<u32, _DCH2CSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CSIZSET;
#[doc = "`read()` method returns [dch2csizset::R](dch2csizset::R) reader structure"]
impl crate::Readable for DCH2CSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch2csizset::W](dch2csizset::W) writer structure"]
impl crate::Writable for DCH2CSIZSET {}
#[doc = "DCH2CSIZSET register"]
pub mod dch2csizset;
#[doc = "DCH2CSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2csizinv](dch2csizinv) module"]
pub type DCH2CSIZINV = crate::Reg<u32, _DCH2CSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CSIZINV;
#[doc = "`read()` method returns [dch2csizinv::R](dch2csizinv::R) reader structure"]
impl crate::Readable for DCH2CSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch2csizinv::W](dch2csizinv::W) writer structure"]
impl crate::Writable for DCH2CSIZINV {}
#[doc = "DCH2CSIZINV register"]
pub mod dch2csizinv;
#[doc = "DCH2CPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2cptr](dch2cptr) module"]
pub type DCH2CPTR = crate::Reg<u32, _DCH2CPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CPTR;
#[doc = "`read()` method returns [dch2cptr::R](dch2cptr::R) reader structure"]
impl crate::Readable for DCH2CPTR {}
#[doc = "`write(|w| ..)` method takes [dch2cptr::W](dch2cptr::W) writer structure"]
impl crate::Writable for DCH2CPTR {}
#[doc = "DCH2CPTR register"]
pub mod dch2cptr;
#[doc = "DCH2CPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2cptrclr](dch2cptrclr) module"]
pub type DCH2CPTRCLR = crate::Reg<u32, _DCH2CPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CPTRCLR;
#[doc = "`read()` method returns [dch2cptrclr::R](dch2cptrclr::R) reader structure"]
impl crate::Readable for DCH2CPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch2cptrclr::W](dch2cptrclr::W) writer structure"]
impl crate::Writable for DCH2CPTRCLR {}
#[doc = "DCH2CPTRCLR register"]
pub mod dch2cptrclr;
#[doc = "DCH2CPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2cptrset](dch2cptrset) module"]
pub type DCH2CPTRSET = crate::Reg<u32, _DCH2CPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CPTRSET;
#[doc = "`read()` method returns [dch2cptrset::R](dch2cptrset::R) reader structure"]
impl crate::Readable for DCH2CPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch2cptrset::W](dch2cptrset::W) writer structure"]
impl crate::Writable for DCH2CPTRSET {}
#[doc = "DCH2CPTRSET register"]
pub mod dch2cptrset;
#[doc = "DCH2CPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2cptrinv](dch2cptrinv) module"]
pub type DCH2CPTRINV = crate::Reg<u32, _DCH2CPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2CPTRINV;
#[doc = "`read()` method returns [dch2cptrinv::R](dch2cptrinv::R) reader structure"]
impl crate::Readable for DCH2CPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch2cptrinv::W](dch2cptrinv::W) writer structure"]
impl crate::Writable for DCH2CPTRINV {}
#[doc = "DCH2CPTRINV register"]
pub mod dch2cptrinv;
#[doc = "DCH2DAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2dat](dch2dat) module"]
pub type DCH2DAT = crate::Reg<u32, _DCH2DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DAT;
#[doc = "`read()` method returns [dch2dat::R](dch2dat::R) reader structure"]
impl crate::Readable for DCH2DAT {}
#[doc = "`write(|w| ..)` method takes [dch2dat::W](dch2dat::W) writer structure"]
impl crate::Writable for DCH2DAT {}
#[doc = "DCH2DAT register"]
pub mod dch2dat;
#[doc = "DCH2DATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2datclr](dch2datclr) module"]
pub type DCH2DATCLR = crate::Reg<u32, _DCH2DATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DATCLR;
#[doc = "`read()` method returns [dch2datclr::R](dch2datclr::R) reader structure"]
impl crate::Readable for DCH2DATCLR {}
#[doc = "`write(|w| ..)` method takes [dch2datclr::W](dch2datclr::W) writer structure"]
impl crate::Writable for DCH2DATCLR {}
#[doc = "DCH2DATCLR register"]
pub mod dch2datclr;
#[doc = "DCH2DATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2datset](dch2datset) module"]
pub type DCH2DATSET = crate::Reg<u32, _DCH2DATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DATSET;
#[doc = "`read()` method returns [dch2datset::R](dch2datset::R) reader structure"]
impl crate::Readable for DCH2DATSET {}
#[doc = "`write(|w| ..)` method takes [dch2datset::W](dch2datset::W) writer structure"]
impl crate::Writable for DCH2DATSET {}
#[doc = "DCH2DATSET register"]
pub mod dch2datset;
#[doc = "DCH2DATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch2datinv](dch2datinv) module"]
pub type DCH2DATINV = crate::Reg<u32, _DCH2DATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH2DATINV;
#[doc = "`read()` method returns [dch2datinv::R](dch2datinv::R) reader structure"]
impl crate::Readable for DCH2DATINV {}
#[doc = "`write(|w| ..)` method takes [dch2datinv::W](dch2datinv::W) writer structure"]
impl crate::Writable for DCH2DATINV {}
#[doc = "DCH2DATINV register"]
pub mod dch2datinv;
