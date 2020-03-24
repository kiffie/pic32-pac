#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH3CON register"]
    pub dch3con: DCH3CON,
    #[doc = "0x04 - DCH3CONCLR register"]
    pub dch3conclr: DCH3CONCLR,
    #[doc = "0x08 - DCH3CONSET register"]
    pub dch3conset: DCH3CONSET,
    #[doc = "0x0c - DCH3CONINV register"]
    pub dch3coninv: DCH3CONINV,
    #[doc = "0x10 - DCH3ECON register"]
    pub dch3econ: DCH3ECON,
    #[doc = "0x14 - DCH3ECONCLR register"]
    pub dch3econclr: DCH3ECONCLR,
    #[doc = "0x18 - DCH3ECONSET register"]
    pub dch3econset: DCH3ECONSET,
    #[doc = "0x1c - DCH3ECONINV register"]
    pub dch3econinv: DCH3ECONINV,
    #[doc = "0x20 - DCH3INT register"]
    pub dch3int: DCH3INT,
    #[doc = "0x24 - DCH3INTCLR register"]
    pub dch3intclr: DCH3INTCLR,
    #[doc = "0x28 - DCH3INTSET register"]
    pub dch3intset: DCH3INTSET,
    #[doc = "0x2c - DCH3INTINV register"]
    pub dch3intinv: DCH3INTINV,
    #[doc = "0x30 - DCH3SSA register"]
    pub dch3ssa: DCH3SSA,
    #[doc = "0x34 - DCH3SSACLR register"]
    pub dch3ssaclr: DCH3SSACLR,
    #[doc = "0x38 - DCH3SSASET register"]
    pub dch3ssaset: DCH3SSASET,
    #[doc = "0x3c - DCH3SSAINV register"]
    pub dch3ssainv: DCH3SSAINV,
    #[doc = "0x40 - DCH3DSA register"]
    pub dch3dsa: DCH3DSA,
    #[doc = "0x44 - DCH3DSACLR register"]
    pub dch3dsaclr: DCH3DSACLR,
    #[doc = "0x48 - DCH3DSASET register"]
    pub dch3dsaset: DCH3DSASET,
    #[doc = "0x4c - DCH3DSAINV register"]
    pub dch3dsainv: DCH3DSAINV,
    #[doc = "0x50 - DCH3SSIZ register"]
    pub dch3ssiz: DCH3SSIZ,
    #[doc = "0x54 - DCH3SSIZCLR register"]
    pub dch3ssizclr: DCH3SSIZCLR,
    #[doc = "0x58 - DCH3SSIZSET register"]
    pub dch3ssizset: DCH3SSIZSET,
    #[doc = "0x5c - DCH3SSIZINV register"]
    pub dch3ssizinv: DCH3SSIZINV,
    #[doc = "0x60 - DCH3DSIZ register"]
    pub dch3dsiz: DCH3DSIZ,
    #[doc = "0x64 - DCH3DSIZCLR register"]
    pub dch3dsizclr: DCH3DSIZCLR,
    #[doc = "0x68 - DCH3DSIZSET register"]
    pub dch3dsizset: DCH3DSIZSET,
    #[doc = "0x6c - DCH3DSIZINV register"]
    pub dch3dsizinv: DCH3DSIZINV,
    #[doc = "0x70 - DCH3SPTR register"]
    pub dch3sptr: DCH3SPTR,
    #[doc = "0x74 - DCH3SPTRCLR register"]
    pub dch3sptrclr: DCH3SPTRCLR,
    #[doc = "0x78 - DCH3SPTRSET register"]
    pub dch3sptrset: DCH3SPTRSET,
    #[doc = "0x7c - DCH3SPTRINV register"]
    pub dch3sptrinv: DCH3SPTRINV,
    #[doc = "0x80 - DCH3DPTR register"]
    pub dch3dptr: DCH3DPTR,
    #[doc = "0x84 - DCH3DPTRCLR register"]
    pub dch3dptrclr: DCH3DPTRCLR,
    #[doc = "0x88 - DCH3DPTRSET register"]
    pub dch3dptrset: DCH3DPTRSET,
    #[doc = "0x8c - DCH3DPTRINV register"]
    pub dch3dptrinv: DCH3DPTRINV,
    #[doc = "0x90 - DCH3CSIZ register"]
    pub dch3csiz: DCH3CSIZ,
    #[doc = "0x94 - DCH3CSIZCLR register"]
    pub dch3csizclr: DCH3CSIZCLR,
    #[doc = "0x98 - DCH3CSIZSET register"]
    pub dch3csizset: DCH3CSIZSET,
    #[doc = "0x9c - DCH3CSIZINV register"]
    pub dch3csizinv: DCH3CSIZINV,
    #[doc = "0xa0 - DCH3CPTR register"]
    pub dch3cptr: DCH3CPTR,
    #[doc = "0xa4 - DCH3CPTRCLR register"]
    pub dch3cptrclr: DCH3CPTRCLR,
    #[doc = "0xa8 - DCH3CPTRSET register"]
    pub dch3cptrset: DCH3CPTRSET,
    #[doc = "0xac - DCH3CPTRINV register"]
    pub dch3cptrinv: DCH3CPTRINV,
    #[doc = "0xb0 - DCH3DAT register"]
    pub dch3dat: DCH3DAT,
    #[doc = "0xb4 - DCH3DATCLR register"]
    pub dch3datclr: DCH3DATCLR,
    #[doc = "0xb8 - DCH3DATSET register"]
    pub dch3datset: DCH3DATSET,
    #[doc = "0xbc - DCH3DATINV register"]
    pub dch3datinv: DCH3DATINV,
}
#[doc = "DCH3CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3con](dch3con) module"]
pub type DCH3CON = crate::Reg<u32, _DCH3CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CON;
#[doc = "`read()` method returns [dch3con::R](dch3con::R) reader structure"]
impl crate::Readable for DCH3CON {}
#[doc = "`write(|w| ..)` method takes [dch3con::W](dch3con::W) writer structure"]
impl crate::Writable for DCH3CON {}
#[doc = "DCH3CON register"]
pub mod dch3con;
#[doc = "DCH3CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3conclr](dch3conclr) module"]
pub type DCH3CONCLR = crate::Reg<u32, _DCH3CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CONCLR;
#[doc = "`read()` method returns [dch3conclr::R](dch3conclr::R) reader structure"]
impl crate::Readable for DCH3CONCLR {}
#[doc = "`write(|w| ..)` method takes [dch3conclr::W](dch3conclr::W) writer structure"]
impl crate::Writable for DCH3CONCLR {}
#[doc = "DCH3CONCLR register"]
pub mod dch3conclr;
#[doc = "DCH3CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3conset](dch3conset) module"]
pub type DCH3CONSET = crate::Reg<u32, _DCH3CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CONSET;
#[doc = "`read()` method returns [dch3conset::R](dch3conset::R) reader structure"]
impl crate::Readable for DCH3CONSET {}
#[doc = "`write(|w| ..)` method takes [dch3conset::W](dch3conset::W) writer structure"]
impl crate::Writable for DCH3CONSET {}
#[doc = "DCH3CONSET register"]
pub mod dch3conset;
#[doc = "DCH3CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3coninv](dch3coninv) module"]
pub type DCH3CONINV = crate::Reg<u32, _DCH3CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CONINV;
#[doc = "`read()` method returns [dch3coninv::R](dch3coninv::R) reader structure"]
impl crate::Readable for DCH3CONINV {}
#[doc = "`write(|w| ..)` method takes [dch3coninv::W](dch3coninv::W) writer structure"]
impl crate::Writable for DCH3CONINV {}
#[doc = "DCH3CONINV register"]
pub mod dch3coninv;
#[doc = "DCH3ECON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3econ](dch3econ) module"]
pub type DCH3ECON = crate::Reg<u32, _DCH3ECON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3ECON;
#[doc = "`read()` method returns [dch3econ::R](dch3econ::R) reader structure"]
impl crate::Readable for DCH3ECON {}
#[doc = "`write(|w| ..)` method takes [dch3econ::W](dch3econ::W) writer structure"]
impl crate::Writable for DCH3ECON {}
#[doc = "DCH3ECON register"]
pub mod dch3econ;
#[doc = "DCH3ECONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3econclr](dch3econclr) module"]
pub type DCH3ECONCLR = crate::Reg<u32, _DCH3ECONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3ECONCLR;
#[doc = "`read()` method returns [dch3econclr::R](dch3econclr::R) reader structure"]
impl crate::Readable for DCH3ECONCLR {}
#[doc = "`write(|w| ..)` method takes [dch3econclr::W](dch3econclr::W) writer structure"]
impl crate::Writable for DCH3ECONCLR {}
#[doc = "DCH3ECONCLR register"]
pub mod dch3econclr;
#[doc = "DCH3ECONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3econset](dch3econset) module"]
pub type DCH3ECONSET = crate::Reg<u32, _DCH3ECONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3ECONSET;
#[doc = "`read()` method returns [dch3econset::R](dch3econset::R) reader structure"]
impl crate::Readable for DCH3ECONSET {}
#[doc = "`write(|w| ..)` method takes [dch3econset::W](dch3econset::W) writer structure"]
impl crate::Writable for DCH3ECONSET {}
#[doc = "DCH3ECONSET register"]
pub mod dch3econset;
#[doc = "DCH3ECONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3econinv](dch3econinv) module"]
pub type DCH3ECONINV = crate::Reg<u32, _DCH3ECONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3ECONINV;
#[doc = "`read()` method returns [dch3econinv::R](dch3econinv::R) reader structure"]
impl crate::Readable for DCH3ECONINV {}
#[doc = "`write(|w| ..)` method takes [dch3econinv::W](dch3econinv::W) writer structure"]
impl crate::Writable for DCH3ECONINV {}
#[doc = "DCH3ECONINV register"]
pub mod dch3econinv;
#[doc = "DCH3INT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3int](dch3int) module"]
pub type DCH3INT = crate::Reg<u32, _DCH3INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3INT;
#[doc = "`read()` method returns [dch3int::R](dch3int::R) reader structure"]
impl crate::Readable for DCH3INT {}
#[doc = "`write(|w| ..)` method takes [dch3int::W](dch3int::W) writer structure"]
impl crate::Writable for DCH3INT {}
#[doc = "DCH3INT register"]
pub mod dch3int;
#[doc = "DCH3INTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3intclr](dch3intclr) module"]
pub type DCH3INTCLR = crate::Reg<u32, _DCH3INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3INTCLR;
#[doc = "`read()` method returns [dch3intclr::R](dch3intclr::R) reader structure"]
impl crate::Readable for DCH3INTCLR {}
#[doc = "`write(|w| ..)` method takes [dch3intclr::W](dch3intclr::W) writer structure"]
impl crate::Writable for DCH3INTCLR {}
#[doc = "DCH3INTCLR register"]
pub mod dch3intclr;
#[doc = "DCH3INTSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3intset](dch3intset) module"]
pub type DCH3INTSET = crate::Reg<u32, _DCH3INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3INTSET;
#[doc = "`read()` method returns [dch3intset::R](dch3intset::R) reader structure"]
impl crate::Readable for DCH3INTSET {}
#[doc = "`write(|w| ..)` method takes [dch3intset::W](dch3intset::W) writer structure"]
impl crate::Writable for DCH3INTSET {}
#[doc = "DCH3INTSET register"]
pub mod dch3intset;
#[doc = "DCH3INTINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3intinv](dch3intinv) module"]
pub type DCH3INTINV = crate::Reg<u32, _DCH3INTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3INTINV;
#[doc = "`read()` method returns [dch3intinv::R](dch3intinv::R) reader structure"]
impl crate::Readable for DCH3INTINV {}
#[doc = "`write(|w| ..)` method takes [dch3intinv::W](dch3intinv::W) writer structure"]
impl crate::Writable for DCH3INTINV {}
#[doc = "DCH3INTINV register"]
pub mod dch3intinv;
#[doc = "DCH3SSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3ssa](dch3ssa) module"]
pub type DCH3SSA = crate::Reg<u32, _DCH3SSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SSA;
#[doc = "`read()` method returns [dch3ssa::R](dch3ssa::R) reader structure"]
impl crate::Readable for DCH3SSA {}
#[doc = "`write(|w| ..)` method takes [dch3ssa::W](dch3ssa::W) writer structure"]
impl crate::Writable for DCH3SSA {}
#[doc = "DCH3SSA register"]
pub mod dch3ssa;
#[doc = "DCH3SSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3ssaclr](dch3ssaclr) module"]
pub type DCH3SSACLR = crate::Reg<u32, _DCH3SSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SSACLR;
#[doc = "`read()` method returns [dch3ssaclr::R](dch3ssaclr::R) reader structure"]
impl crate::Readable for DCH3SSACLR {}
#[doc = "`write(|w| ..)` method takes [dch3ssaclr::W](dch3ssaclr::W) writer structure"]
impl crate::Writable for DCH3SSACLR {}
#[doc = "DCH3SSACLR register"]
pub mod dch3ssaclr;
#[doc = "DCH3SSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3ssaset](dch3ssaset) module"]
pub type DCH3SSASET = crate::Reg<u32, _DCH3SSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SSASET;
#[doc = "`read()` method returns [dch3ssaset::R](dch3ssaset::R) reader structure"]
impl crate::Readable for DCH3SSASET {}
#[doc = "`write(|w| ..)` method takes [dch3ssaset::W](dch3ssaset::W) writer structure"]
impl crate::Writable for DCH3SSASET {}
#[doc = "DCH3SSASET register"]
pub mod dch3ssaset;
#[doc = "DCH3SSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3ssainv](dch3ssainv) module"]
pub type DCH3SSAINV = crate::Reg<u32, _DCH3SSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SSAINV;
#[doc = "`read()` method returns [dch3ssainv::R](dch3ssainv::R) reader structure"]
impl crate::Readable for DCH3SSAINV {}
#[doc = "`write(|w| ..)` method takes [dch3ssainv::W](dch3ssainv::W) writer structure"]
impl crate::Writable for DCH3SSAINV {}
#[doc = "DCH3SSAINV register"]
pub mod dch3ssainv;
#[doc = "DCH3DSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dsa](dch3dsa) module"]
pub type DCH3DSA = crate::Reg<u32, _DCH3DSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DSA;
#[doc = "`read()` method returns [dch3dsa::R](dch3dsa::R) reader structure"]
impl crate::Readable for DCH3DSA {}
#[doc = "`write(|w| ..)` method takes [dch3dsa::W](dch3dsa::W) writer structure"]
impl crate::Writable for DCH3DSA {}
#[doc = "DCH3DSA register"]
pub mod dch3dsa;
#[doc = "DCH3DSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dsaclr](dch3dsaclr) module"]
pub type DCH3DSACLR = crate::Reg<u32, _DCH3DSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DSACLR;
#[doc = "`read()` method returns [dch3dsaclr::R](dch3dsaclr::R) reader structure"]
impl crate::Readable for DCH3DSACLR {}
#[doc = "`write(|w| ..)` method takes [dch3dsaclr::W](dch3dsaclr::W) writer structure"]
impl crate::Writable for DCH3DSACLR {}
#[doc = "DCH3DSACLR register"]
pub mod dch3dsaclr;
#[doc = "DCH3DSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dsaset](dch3dsaset) module"]
pub type DCH3DSASET = crate::Reg<u32, _DCH3DSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DSASET;
#[doc = "`read()` method returns [dch3dsaset::R](dch3dsaset::R) reader structure"]
impl crate::Readable for DCH3DSASET {}
#[doc = "`write(|w| ..)` method takes [dch3dsaset::W](dch3dsaset::W) writer structure"]
impl crate::Writable for DCH3DSASET {}
#[doc = "DCH3DSASET register"]
pub mod dch3dsaset;
#[doc = "DCH3DSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dsainv](dch3dsainv) module"]
pub type DCH3DSAINV = crate::Reg<u32, _DCH3DSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DSAINV;
#[doc = "`read()` method returns [dch3dsainv::R](dch3dsainv::R) reader structure"]
impl crate::Readable for DCH3DSAINV {}
#[doc = "`write(|w| ..)` method takes [dch3dsainv::W](dch3dsainv::W) writer structure"]
impl crate::Writable for DCH3DSAINV {}
#[doc = "DCH3DSAINV register"]
pub mod dch3dsainv;
#[doc = "DCH3SSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3ssiz](dch3ssiz) module"]
pub type DCH3SSIZ = crate::Reg<u32, _DCH3SSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SSIZ;
#[doc = "`read()` method returns [dch3ssiz::R](dch3ssiz::R) reader structure"]
impl crate::Readable for DCH3SSIZ {}
#[doc = "`write(|w| ..)` method takes [dch3ssiz::W](dch3ssiz::W) writer structure"]
impl crate::Writable for DCH3SSIZ {}
#[doc = "DCH3SSIZ register"]
pub mod dch3ssiz;
#[doc = "DCH3SSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3ssizclr](dch3ssizclr) module"]
pub type DCH3SSIZCLR = crate::Reg<u32, _DCH3SSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SSIZCLR;
#[doc = "`read()` method returns [dch3ssizclr::R](dch3ssizclr::R) reader structure"]
impl crate::Readable for DCH3SSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch3ssizclr::W](dch3ssizclr::W) writer structure"]
impl crate::Writable for DCH3SSIZCLR {}
#[doc = "DCH3SSIZCLR register"]
pub mod dch3ssizclr;
#[doc = "DCH3SSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3ssizset](dch3ssizset) module"]
pub type DCH3SSIZSET = crate::Reg<u32, _DCH3SSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SSIZSET;
#[doc = "`read()` method returns [dch3ssizset::R](dch3ssizset::R) reader structure"]
impl crate::Readable for DCH3SSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch3ssizset::W](dch3ssizset::W) writer structure"]
impl crate::Writable for DCH3SSIZSET {}
#[doc = "DCH3SSIZSET register"]
pub mod dch3ssizset;
#[doc = "DCH3SSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3ssizinv](dch3ssizinv) module"]
pub type DCH3SSIZINV = crate::Reg<u32, _DCH3SSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SSIZINV;
#[doc = "`read()` method returns [dch3ssizinv::R](dch3ssizinv::R) reader structure"]
impl crate::Readable for DCH3SSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch3ssizinv::W](dch3ssizinv::W) writer structure"]
impl crate::Writable for DCH3SSIZINV {}
#[doc = "DCH3SSIZINV register"]
pub mod dch3ssizinv;
#[doc = "DCH3DSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dsiz](dch3dsiz) module"]
pub type DCH3DSIZ = crate::Reg<u32, _DCH3DSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DSIZ;
#[doc = "`read()` method returns [dch3dsiz::R](dch3dsiz::R) reader structure"]
impl crate::Readable for DCH3DSIZ {}
#[doc = "`write(|w| ..)` method takes [dch3dsiz::W](dch3dsiz::W) writer structure"]
impl crate::Writable for DCH3DSIZ {}
#[doc = "DCH3DSIZ register"]
pub mod dch3dsiz;
#[doc = "DCH3DSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dsizclr](dch3dsizclr) module"]
pub type DCH3DSIZCLR = crate::Reg<u32, _DCH3DSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DSIZCLR;
#[doc = "`read()` method returns [dch3dsizclr::R](dch3dsizclr::R) reader structure"]
impl crate::Readable for DCH3DSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch3dsizclr::W](dch3dsizclr::W) writer structure"]
impl crate::Writable for DCH3DSIZCLR {}
#[doc = "DCH3DSIZCLR register"]
pub mod dch3dsizclr;
#[doc = "DCH3DSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dsizset](dch3dsizset) module"]
pub type DCH3DSIZSET = crate::Reg<u32, _DCH3DSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DSIZSET;
#[doc = "`read()` method returns [dch3dsizset::R](dch3dsizset::R) reader structure"]
impl crate::Readable for DCH3DSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch3dsizset::W](dch3dsizset::W) writer structure"]
impl crate::Writable for DCH3DSIZSET {}
#[doc = "DCH3DSIZSET register"]
pub mod dch3dsizset;
#[doc = "DCH3DSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dsizinv](dch3dsizinv) module"]
pub type DCH3DSIZINV = crate::Reg<u32, _DCH3DSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DSIZINV;
#[doc = "`read()` method returns [dch3dsizinv::R](dch3dsizinv::R) reader structure"]
impl crate::Readable for DCH3DSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch3dsizinv::W](dch3dsizinv::W) writer structure"]
impl crate::Writable for DCH3DSIZINV {}
#[doc = "DCH3DSIZINV register"]
pub mod dch3dsizinv;
#[doc = "DCH3SPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3sptr](dch3sptr) module"]
pub type DCH3SPTR = crate::Reg<u32, _DCH3SPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SPTR;
#[doc = "`read()` method returns [dch3sptr::R](dch3sptr::R) reader structure"]
impl crate::Readable for DCH3SPTR {}
#[doc = "`write(|w| ..)` method takes [dch3sptr::W](dch3sptr::W) writer structure"]
impl crate::Writable for DCH3SPTR {}
#[doc = "DCH3SPTR register"]
pub mod dch3sptr;
#[doc = "DCH3SPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3sptrclr](dch3sptrclr) module"]
pub type DCH3SPTRCLR = crate::Reg<u32, _DCH3SPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SPTRCLR;
#[doc = "`read()` method returns [dch3sptrclr::R](dch3sptrclr::R) reader structure"]
impl crate::Readable for DCH3SPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch3sptrclr::W](dch3sptrclr::W) writer structure"]
impl crate::Writable for DCH3SPTRCLR {}
#[doc = "DCH3SPTRCLR register"]
pub mod dch3sptrclr;
#[doc = "DCH3SPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3sptrset](dch3sptrset) module"]
pub type DCH3SPTRSET = crate::Reg<u32, _DCH3SPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SPTRSET;
#[doc = "`read()` method returns [dch3sptrset::R](dch3sptrset::R) reader structure"]
impl crate::Readable for DCH3SPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch3sptrset::W](dch3sptrset::W) writer structure"]
impl crate::Writable for DCH3SPTRSET {}
#[doc = "DCH3SPTRSET register"]
pub mod dch3sptrset;
#[doc = "DCH3SPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3sptrinv](dch3sptrinv) module"]
pub type DCH3SPTRINV = crate::Reg<u32, _DCH3SPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3SPTRINV;
#[doc = "`read()` method returns [dch3sptrinv::R](dch3sptrinv::R) reader structure"]
impl crate::Readable for DCH3SPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch3sptrinv::W](dch3sptrinv::W) writer structure"]
impl crate::Writable for DCH3SPTRINV {}
#[doc = "DCH3SPTRINV register"]
pub mod dch3sptrinv;
#[doc = "DCH3DPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dptr](dch3dptr) module"]
pub type DCH3DPTR = crate::Reg<u32, _DCH3DPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DPTR;
#[doc = "`read()` method returns [dch3dptr::R](dch3dptr::R) reader structure"]
impl crate::Readable for DCH3DPTR {}
#[doc = "`write(|w| ..)` method takes [dch3dptr::W](dch3dptr::W) writer structure"]
impl crate::Writable for DCH3DPTR {}
#[doc = "DCH3DPTR register"]
pub mod dch3dptr;
#[doc = "DCH3DPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dptrclr](dch3dptrclr) module"]
pub type DCH3DPTRCLR = crate::Reg<u32, _DCH3DPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DPTRCLR;
#[doc = "`read()` method returns [dch3dptrclr::R](dch3dptrclr::R) reader structure"]
impl crate::Readable for DCH3DPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch3dptrclr::W](dch3dptrclr::W) writer structure"]
impl crate::Writable for DCH3DPTRCLR {}
#[doc = "DCH3DPTRCLR register"]
pub mod dch3dptrclr;
#[doc = "DCH3DPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dptrset](dch3dptrset) module"]
pub type DCH3DPTRSET = crate::Reg<u32, _DCH3DPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DPTRSET;
#[doc = "`read()` method returns [dch3dptrset::R](dch3dptrset::R) reader structure"]
impl crate::Readable for DCH3DPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch3dptrset::W](dch3dptrset::W) writer structure"]
impl crate::Writable for DCH3DPTRSET {}
#[doc = "DCH3DPTRSET register"]
pub mod dch3dptrset;
#[doc = "DCH3DPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dptrinv](dch3dptrinv) module"]
pub type DCH3DPTRINV = crate::Reg<u32, _DCH3DPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DPTRINV;
#[doc = "`read()` method returns [dch3dptrinv::R](dch3dptrinv::R) reader structure"]
impl crate::Readable for DCH3DPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch3dptrinv::W](dch3dptrinv::W) writer structure"]
impl crate::Writable for DCH3DPTRINV {}
#[doc = "DCH3DPTRINV register"]
pub mod dch3dptrinv;
#[doc = "DCH3CSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3csiz](dch3csiz) module"]
pub type DCH3CSIZ = crate::Reg<u32, _DCH3CSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CSIZ;
#[doc = "`read()` method returns [dch3csiz::R](dch3csiz::R) reader structure"]
impl crate::Readable for DCH3CSIZ {}
#[doc = "`write(|w| ..)` method takes [dch3csiz::W](dch3csiz::W) writer structure"]
impl crate::Writable for DCH3CSIZ {}
#[doc = "DCH3CSIZ register"]
pub mod dch3csiz;
#[doc = "DCH3CSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3csizclr](dch3csizclr) module"]
pub type DCH3CSIZCLR = crate::Reg<u32, _DCH3CSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CSIZCLR;
#[doc = "`read()` method returns [dch3csizclr::R](dch3csizclr::R) reader structure"]
impl crate::Readable for DCH3CSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch3csizclr::W](dch3csizclr::W) writer structure"]
impl crate::Writable for DCH3CSIZCLR {}
#[doc = "DCH3CSIZCLR register"]
pub mod dch3csizclr;
#[doc = "DCH3CSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3csizset](dch3csizset) module"]
pub type DCH3CSIZSET = crate::Reg<u32, _DCH3CSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CSIZSET;
#[doc = "`read()` method returns [dch3csizset::R](dch3csizset::R) reader structure"]
impl crate::Readable for DCH3CSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch3csizset::W](dch3csizset::W) writer structure"]
impl crate::Writable for DCH3CSIZSET {}
#[doc = "DCH3CSIZSET register"]
pub mod dch3csizset;
#[doc = "DCH3CSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3csizinv](dch3csizinv) module"]
pub type DCH3CSIZINV = crate::Reg<u32, _DCH3CSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CSIZINV;
#[doc = "`read()` method returns [dch3csizinv::R](dch3csizinv::R) reader structure"]
impl crate::Readable for DCH3CSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch3csizinv::W](dch3csizinv::W) writer structure"]
impl crate::Writable for DCH3CSIZINV {}
#[doc = "DCH3CSIZINV register"]
pub mod dch3csizinv;
#[doc = "DCH3CPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3cptr](dch3cptr) module"]
pub type DCH3CPTR = crate::Reg<u32, _DCH3CPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CPTR;
#[doc = "`read()` method returns [dch3cptr::R](dch3cptr::R) reader structure"]
impl crate::Readable for DCH3CPTR {}
#[doc = "`write(|w| ..)` method takes [dch3cptr::W](dch3cptr::W) writer structure"]
impl crate::Writable for DCH3CPTR {}
#[doc = "DCH3CPTR register"]
pub mod dch3cptr;
#[doc = "DCH3CPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3cptrclr](dch3cptrclr) module"]
pub type DCH3CPTRCLR = crate::Reg<u32, _DCH3CPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CPTRCLR;
#[doc = "`read()` method returns [dch3cptrclr::R](dch3cptrclr::R) reader structure"]
impl crate::Readable for DCH3CPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch3cptrclr::W](dch3cptrclr::W) writer structure"]
impl crate::Writable for DCH3CPTRCLR {}
#[doc = "DCH3CPTRCLR register"]
pub mod dch3cptrclr;
#[doc = "DCH3CPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3cptrset](dch3cptrset) module"]
pub type DCH3CPTRSET = crate::Reg<u32, _DCH3CPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CPTRSET;
#[doc = "`read()` method returns [dch3cptrset::R](dch3cptrset::R) reader structure"]
impl crate::Readable for DCH3CPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch3cptrset::W](dch3cptrset::W) writer structure"]
impl crate::Writable for DCH3CPTRSET {}
#[doc = "DCH3CPTRSET register"]
pub mod dch3cptrset;
#[doc = "DCH3CPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3cptrinv](dch3cptrinv) module"]
pub type DCH3CPTRINV = crate::Reg<u32, _DCH3CPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3CPTRINV;
#[doc = "`read()` method returns [dch3cptrinv::R](dch3cptrinv::R) reader structure"]
impl crate::Readable for DCH3CPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch3cptrinv::W](dch3cptrinv::W) writer structure"]
impl crate::Writable for DCH3CPTRINV {}
#[doc = "DCH3CPTRINV register"]
pub mod dch3cptrinv;
#[doc = "DCH3DAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3dat](dch3dat) module"]
pub type DCH3DAT = crate::Reg<u32, _DCH3DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DAT;
#[doc = "`read()` method returns [dch3dat::R](dch3dat::R) reader structure"]
impl crate::Readable for DCH3DAT {}
#[doc = "`write(|w| ..)` method takes [dch3dat::W](dch3dat::W) writer structure"]
impl crate::Writable for DCH3DAT {}
#[doc = "DCH3DAT register"]
pub mod dch3dat;
#[doc = "DCH3DATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3datclr](dch3datclr) module"]
pub type DCH3DATCLR = crate::Reg<u32, _DCH3DATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DATCLR;
#[doc = "`read()` method returns [dch3datclr::R](dch3datclr::R) reader structure"]
impl crate::Readable for DCH3DATCLR {}
#[doc = "`write(|w| ..)` method takes [dch3datclr::W](dch3datclr::W) writer structure"]
impl crate::Writable for DCH3DATCLR {}
#[doc = "DCH3DATCLR register"]
pub mod dch3datclr;
#[doc = "DCH3DATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3datset](dch3datset) module"]
pub type DCH3DATSET = crate::Reg<u32, _DCH3DATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DATSET;
#[doc = "`read()` method returns [dch3datset::R](dch3datset::R) reader structure"]
impl crate::Readable for DCH3DATSET {}
#[doc = "`write(|w| ..)` method takes [dch3datset::W](dch3datset::W) writer structure"]
impl crate::Writable for DCH3DATSET {}
#[doc = "DCH3DATSET register"]
pub mod dch3datset;
#[doc = "DCH3DATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch3datinv](dch3datinv) module"]
pub type DCH3DATINV = crate::Reg<u32, _DCH3DATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH3DATINV;
#[doc = "`read()` method returns [dch3datinv::R](dch3datinv::R) reader structure"]
impl crate::Readable for DCH3DATINV {}
#[doc = "`write(|w| ..)` method takes [dch3datinv::W](dch3datinv::W) writer structure"]
impl crate::Writable for DCH3DATINV {}
#[doc = "DCH3DATINV register"]
pub mod dch3datinv;
