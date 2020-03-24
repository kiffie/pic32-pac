#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH0CON register"]
    pub dch0con: DCH0CON,
    #[doc = "0x04 - DCH0CONCLR register"]
    pub dch0conclr: DCH0CONCLR,
    #[doc = "0x08 - DCH0CONSET register"]
    pub dch0conset: DCH0CONSET,
    #[doc = "0x0c - DCH0CONINV register"]
    pub dch0coninv: DCH0CONINV,
    #[doc = "0x10 - DCH0ECON register"]
    pub dch0econ: DCH0ECON,
    #[doc = "0x14 - DCH0ECONCLR register"]
    pub dch0econclr: DCH0ECONCLR,
    #[doc = "0x18 - DCH0ECONSET register"]
    pub dch0econset: DCH0ECONSET,
    #[doc = "0x1c - DCH0ECONINV register"]
    pub dch0econinv: DCH0ECONINV,
    #[doc = "0x20 - DCH0INT register"]
    pub dch0int: DCH0INT,
    #[doc = "0x24 - DCH0INTCLR register"]
    pub dch0intclr: DCH0INTCLR,
    #[doc = "0x28 - DCH0INTSET register"]
    pub dch0intset: DCH0INTSET,
    #[doc = "0x2c - DCH0INTINV register"]
    pub dch0intinv: DCH0INTINV,
    #[doc = "0x30 - DCH0SSA register"]
    pub dch0ssa: DCH0SSA,
    #[doc = "0x34 - DCH0SSACLR register"]
    pub dch0ssaclr: DCH0SSACLR,
    #[doc = "0x38 - DCH0SSASET register"]
    pub dch0ssaset: DCH0SSASET,
    #[doc = "0x3c - DCH0SSAINV register"]
    pub dch0ssainv: DCH0SSAINV,
    #[doc = "0x40 - DCH0DSA register"]
    pub dch0dsa: DCH0DSA,
    #[doc = "0x44 - DCH0DSACLR register"]
    pub dch0dsaclr: DCH0DSACLR,
    #[doc = "0x48 - DCH0DSASET register"]
    pub dch0dsaset: DCH0DSASET,
    #[doc = "0x4c - DCH0DSAINV register"]
    pub dch0dsainv: DCH0DSAINV,
    #[doc = "0x50 - DCH0SSIZ register"]
    pub dch0ssiz: DCH0SSIZ,
    #[doc = "0x54 - DCH0SSIZCLR register"]
    pub dch0ssizclr: DCH0SSIZCLR,
    #[doc = "0x58 - DCH0SSIZSET register"]
    pub dch0ssizset: DCH0SSIZSET,
    #[doc = "0x5c - DCH0SSIZINV register"]
    pub dch0ssizinv: DCH0SSIZINV,
    #[doc = "0x60 - DCH0DSIZ register"]
    pub dch0dsiz: DCH0DSIZ,
    #[doc = "0x64 - DCH0DSIZCLR register"]
    pub dch0dsizclr: DCH0DSIZCLR,
    #[doc = "0x68 - DCH0DSIZSET register"]
    pub dch0dsizset: DCH0DSIZSET,
    #[doc = "0x6c - DCH0DSIZINV register"]
    pub dch0dsizinv: DCH0DSIZINV,
    #[doc = "0x70 - DCH0SPTR register"]
    pub dch0sptr: DCH0SPTR,
    #[doc = "0x74 - DCH0SPTRCLR register"]
    pub dch0sptrclr: DCH0SPTRCLR,
    #[doc = "0x78 - DCH0SPTRSET register"]
    pub dch0sptrset: DCH0SPTRSET,
    #[doc = "0x7c - DCH0SPTRINV register"]
    pub dch0sptrinv: DCH0SPTRINV,
    #[doc = "0x80 - DCH0DPTR register"]
    pub dch0dptr: DCH0DPTR,
    #[doc = "0x84 - DCH0DPTRCLR register"]
    pub dch0dptrclr: DCH0DPTRCLR,
    #[doc = "0x88 - DCH0DPTRSET register"]
    pub dch0dptrset: DCH0DPTRSET,
    #[doc = "0x8c - DCH0DPTRINV register"]
    pub dch0dptrinv: DCH0DPTRINV,
    #[doc = "0x90 - DCH0CSIZ register"]
    pub dch0csiz: DCH0CSIZ,
    #[doc = "0x94 - DCH0CSIZCLR register"]
    pub dch0csizclr: DCH0CSIZCLR,
    #[doc = "0x98 - DCH0CSIZSET register"]
    pub dch0csizset: DCH0CSIZSET,
    #[doc = "0x9c - DCH0CSIZINV register"]
    pub dch0csizinv: DCH0CSIZINV,
    #[doc = "0xa0 - DCH0CPTR register"]
    pub dch0cptr: DCH0CPTR,
    #[doc = "0xa4 - DCH0CPTRCLR register"]
    pub dch0cptrclr: DCH0CPTRCLR,
    #[doc = "0xa8 - DCH0CPTRSET register"]
    pub dch0cptrset: DCH0CPTRSET,
    #[doc = "0xac - DCH0CPTRINV register"]
    pub dch0cptrinv: DCH0CPTRINV,
    #[doc = "0xb0 - DCH0DAT register"]
    pub dch0dat: DCH0DAT,
    #[doc = "0xb4 - DCH0DATCLR register"]
    pub dch0datclr: DCH0DATCLR,
    #[doc = "0xb8 - DCH0DATSET register"]
    pub dch0datset: DCH0DATSET,
    #[doc = "0xbc - DCH0DATINV register"]
    pub dch0datinv: DCH0DATINV,
}
#[doc = "DCH0CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0con](dch0con) module"]
pub type DCH0CON = crate::Reg<u32, _DCH0CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CON;
#[doc = "`read()` method returns [dch0con::R](dch0con::R) reader structure"]
impl crate::Readable for DCH0CON {}
#[doc = "`write(|w| ..)` method takes [dch0con::W](dch0con::W) writer structure"]
impl crate::Writable for DCH0CON {}
#[doc = "DCH0CON register"]
pub mod dch0con;
#[doc = "DCH0CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0conclr](dch0conclr) module"]
pub type DCH0CONCLR = crate::Reg<u32, _DCH0CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CONCLR;
#[doc = "`read()` method returns [dch0conclr::R](dch0conclr::R) reader structure"]
impl crate::Readable for DCH0CONCLR {}
#[doc = "`write(|w| ..)` method takes [dch0conclr::W](dch0conclr::W) writer structure"]
impl crate::Writable for DCH0CONCLR {}
#[doc = "DCH0CONCLR register"]
pub mod dch0conclr;
#[doc = "DCH0CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0conset](dch0conset) module"]
pub type DCH0CONSET = crate::Reg<u32, _DCH0CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CONSET;
#[doc = "`read()` method returns [dch0conset::R](dch0conset::R) reader structure"]
impl crate::Readable for DCH0CONSET {}
#[doc = "`write(|w| ..)` method takes [dch0conset::W](dch0conset::W) writer structure"]
impl crate::Writable for DCH0CONSET {}
#[doc = "DCH0CONSET register"]
pub mod dch0conset;
#[doc = "DCH0CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0coninv](dch0coninv) module"]
pub type DCH0CONINV = crate::Reg<u32, _DCH0CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CONINV;
#[doc = "`read()` method returns [dch0coninv::R](dch0coninv::R) reader structure"]
impl crate::Readable for DCH0CONINV {}
#[doc = "`write(|w| ..)` method takes [dch0coninv::W](dch0coninv::W) writer structure"]
impl crate::Writable for DCH0CONINV {}
#[doc = "DCH0CONINV register"]
pub mod dch0coninv;
#[doc = "DCH0ECON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0econ](dch0econ) module"]
pub type DCH0ECON = crate::Reg<u32, _DCH0ECON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0ECON;
#[doc = "`read()` method returns [dch0econ::R](dch0econ::R) reader structure"]
impl crate::Readable for DCH0ECON {}
#[doc = "`write(|w| ..)` method takes [dch0econ::W](dch0econ::W) writer structure"]
impl crate::Writable for DCH0ECON {}
#[doc = "DCH0ECON register"]
pub mod dch0econ;
#[doc = "DCH0ECONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0econclr](dch0econclr) module"]
pub type DCH0ECONCLR = crate::Reg<u32, _DCH0ECONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0ECONCLR;
#[doc = "`read()` method returns [dch0econclr::R](dch0econclr::R) reader structure"]
impl crate::Readable for DCH0ECONCLR {}
#[doc = "`write(|w| ..)` method takes [dch0econclr::W](dch0econclr::W) writer structure"]
impl crate::Writable for DCH0ECONCLR {}
#[doc = "DCH0ECONCLR register"]
pub mod dch0econclr;
#[doc = "DCH0ECONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0econset](dch0econset) module"]
pub type DCH0ECONSET = crate::Reg<u32, _DCH0ECONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0ECONSET;
#[doc = "`read()` method returns [dch0econset::R](dch0econset::R) reader structure"]
impl crate::Readable for DCH0ECONSET {}
#[doc = "`write(|w| ..)` method takes [dch0econset::W](dch0econset::W) writer structure"]
impl crate::Writable for DCH0ECONSET {}
#[doc = "DCH0ECONSET register"]
pub mod dch0econset;
#[doc = "DCH0ECONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0econinv](dch0econinv) module"]
pub type DCH0ECONINV = crate::Reg<u32, _DCH0ECONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0ECONINV;
#[doc = "`read()` method returns [dch0econinv::R](dch0econinv::R) reader structure"]
impl crate::Readable for DCH0ECONINV {}
#[doc = "`write(|w| ..)` method takes [dch0econinv::W](dch0econinv::W) writer structure"]
impl crate::Writable for DCH0ECONINV {}
#[doc = "DCH0ECONINV register"]
pub mod dch0econinv;
#[doc = "DCH0INT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0int](dch0int) module"]
pub type DCH0INT = crate::Reg<u32, _DCH0INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0INT;
#[doc = "`read()` method returns [dch0int::R](dch0int::R) reader structure"]
impl crate::Readable for DCH0INT {}
#[doc = "`write(|w| ..)` method takes [dch0int::W](dch0int::W) writer structure"]
impl crate::Writable for DCH0INT {}
#[doc = "DCH0INT register"]
pub mod dch0int;
#[doc = "DCH0INTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0intclr](dch0intclr) module"]
pub type DCH0INTCLR = crate::Reg<u32, _DCH0INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0INTCLR;
#[doc = "`read()` method returns [dch0intclr::R](dch0intclr::R) reader structure"]
impl crate::Readable for DCH0INTCLR {}
#[doc = "`write(|w| ..)` method takes [dch0intclr::W](dch0intclr::W) writer structure"]
impl crate::Writable for DCH0INTCLR {}
#[doc = "DCH0INTCLR register"]
pub mod dch0intclr;
#[doc = "DCH0INTSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0intset](dch0intset) module"]
pub type DCH0INTSET = crate::Reg<u32, _DCH0INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0INTSET;
#[doc = "`read()` method returns [dch0intset::R](dch0intset::R) reader structure"]
impl crate::Readable for DCH0INTSET {}
#[doc = "`write(|w| ..)` method takes [dch0intset::W](dch0intset::W) writer structure"]
impl crate::Writable for DCH0INTSET {}
#[doc = "DCH0INTSET register"]
pub mod dch0intset;
#[doc = "DCH0INTINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0intinv](dch0intinv) module"]
pub type DCH0INTINV = crate::Reg<u32, _DCH0INTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0INTINV;
#[doc = "`read()` method returns [dch0intinv::R](dch0intinv::R) reader structure"]
impl crate::Readable for DCH0INTINV {}
#[doc = "`write(|w| ..)` method takes [dch0intinv::W](dch0intinv::W) writer structure"]
impl crate::Writable for DCH0INTINV {}
#[doc = "DCH0INTINV register"]
pub mod dch0intinv;
#[doc = "DCH0SSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0ssa](dch0ssa) module"]
pub type DCH0SSA = crate::Reg<u32, _DCH0SSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SSA;
#[doc = "`read()` method returns [dch0ssa::R](dch0ssa::R) reader structure"]
impl crate::Readable for DCH0SSA {}
#[doc = "`write(|w| ..)` method takes [dch0ssa::W](dch0ssa::W) writer structure"]
impl crate::Writable for DCH0SSA {}
#[doc = "DCH0SSA register"]
pub mod dch0ssa;
#[doc = "DCH0SSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0ssaclr](dch0ssaclr) module"]
pub type DCH0SSACLR = crate::Reg<u32, _DCH0SSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SSACLR;
#[doc = "`read()` method returns [dch0ssaclr::R](dch0ssaclr::R) reader structure"]
impl crate::Readable for DCH0SSACLR {}
#[doc = "`write(|w| ..)` method takes [dch0ssaclr::W](dch0ssaclr::W) writer structure"]
impl crate::Writable for DCH0SSACLR {}
#[doc = "DCH0SSACLR register"]
pub mod dch0ssaclr;
#[doc = "DCH0SSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0ssaset](dch0ssaset) module"]
pub type DCH0SSASET = crate::Reg<u32, _DCH0SSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SSASET;
#[doc = "`read()` method returns [dch0ssaset::R](dch0ssaset::R) reader structure"]
impl crate::Readable for DCH0SSASET {}
#[doc = "`write(|w| ..)` method takes [dch0ssaset::W](dch0ssaset::W) writer structure"]
impl crate::Writable for DCH0SSASET {}
#[doc = "DCH0SSASET register"]
pub mod dch0ssaset;
#[doc = "DCH0SSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0ssainv](dch0ssainv) module"]
pub type DCH0SSAINV = crate::Reg<u32, _DCH0SSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SSAINV;
#[doc = "`read()` method returns [dch0ssainv::R](dch0ssainv::R) reader structure"]
impl crate::Readable for DCH0SSAINV {}
#[doc = "`write(|w| ..)` method takes [dch0ssainv::W](dch0ssainv::W) writer structure"]
impl crate::Writable for DCH0SSAINV {}
#[doc = "DCH0SSAINV register"]
pub mod dch0ssainv;
#[doc = "DCH0DSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dsa](dch0dsa) module"]
pub type DCH0DSA = crate::Reg<u32, _DCH0DSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DSA;
#[doc = "`read()` method returns [dch0dsa::R](dch0dsa::R) reader structure"]
impl crate::Readable for DCH0DSA {}
#[doc = "`write(|w| ..)` method takes [dch0dsa::W](dch0dsa::W) writer structure"]
impl crate::Writable for DCH0DSA {}
#[doc = "DCH0DSA register"]
pub mod dch0dsa;
#[doc = "DCH0DSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dsaclr](dch0dsaclr) module"]
pub type DCH0DSACLR = crate::Reg<u32, _DCH0DSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DSACLR;
#[doc = "`read()` method returns [dch0dsaclr::R](dch0dsaclr::R) reader structure"]
impl crate::Readable for DCH0DSACLR {}
#[doc = "`write(|w| ..)` method takes [dch0dsaclr::W](dch0dsaclr::W) writer structure"]
impl crate::Writable for DCH0DSACLR {}
#[doc = "DCH0DSACLR register"]
pub mod dch0dsaclr;
#[doc = "DCH0DSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dsaset](dch0dsaset) module"]
pub type DCH0DSASET = crate::Reg<u32, _DCH0DSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DSASET;
#[doc = "`read()` method returns [dch0dsaset::R](dch0dsaset::R) reader structure"]
impl crate::Readable for DCH0DSASET {}
#[doc = "`write(|w| ..)` method takes [dch0dsaset::W](dch0dsaset::W) writer structure"]
impl crate::Writable for DCH0DSASET {}
#[doc = "DCH0DSASET register"]
pub mod dch0dsaset;
#[doc = "DCH0DSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dsainv](dch0dsainv) module"]
pub type DCH0DSAINV = crate::Reg<u32, _DCH0DSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DSAINV;
#[doc = "`read()` method returns [dch0dsainv::R](dch0dsainv::R) reader structure"]
impl crate::Readable for DCH0DSAINV {}
#[doc = "`write(|w| ..)` method takes [dch0dsainv::W](dch0dsainv::W) writer structure"]
impl crate::Writable for DCH0DSAINV {}
#[doc = "DCH0DSAINV register"]
pub mod dch0dsainv;
#[doc = "DCH0SSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0ssiz](dch0ssiz) module"]
pub type DCH0SSIZ = crate::Reg<u32, _DCH0SSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SSIZ;
#[doc = "`read()` method returns [dch0ssiz::R](dch0ssiz::R) reader structure"]
impl crate::Readable for DCH0SSIZ {}
#[doc = "`write(|w| ..)` method takes [dch0ssiz::W](dch0ssiz::W) writer structure"]
impl crate::Writable for DCH0SSIZ {}
#[doc = "DCH0SSIZ register"]
pub mod dch0ssiz;
#[doc = "DCH0SSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0ssizclr](dch0ssizclr) module"]
pub type DCH0SSIZCLR = crate::Reg<u32, _DCH0SSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SSIZCLR;
#[doc = "`read()` method returns [dch0ssizclr::R](dch0ssizclr::R) reader structure"]
impl crate::Readable for DCH0SSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch0ssizclr::W](dch0ssizclr::W) writer structure"]
impl crate::Writable for DCH0SSIZCLR {}
#[doc = "DCH0SSIZCLR register"]
pub mod dch0ssizclr;
#[doc = "DCH0SSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0ssizset](dch0ssizset) module"]
pub type DCH0SSIZSET = crate::Reg<u32, _DCH0SSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SSIZSET;
#[doc = "`read()` method returns [dch0ssizset::R](dch0ssizset::R) reader structure"]
impl crate::Readable for DCH0SSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch0ssizset::W](dch0ssizset::W) writer structure"]
impl crate::Writable for DCH0SSIZSET {}
#[doc = "DCH0SSIZSET register"]
pub mod dch0ssizset;
#[doc = "DCH0SSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0ssizinv](dch0ssizinv) module"]
pub type DCH0SSIZINV = crate::Reg<u32, _DCH0SSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SSIZINV;
#[doc = "`read()` method returns [dch0ssizinv::R](dch0ssizinv::R) reader structure"]
impl crate::Readable for DCH0SSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch0ssizinv::W](dch0ssizinv::W) writer structure"]
impl crate::Writable for DCH0SSIZINV {}
#[doc = "DCH0SSIZINV register"]
pub mod dch0ssizinv;
#[doc = "DCH0DSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dsiz](dch0dsiz) module"]
pub type DCH0DSIZ = crate::Reg<u32, _DCH0DSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DSIZ;
#[doc = "`read()` method returns [dch0dsiz::R](dch0dsiz::R) reader structure"]
impl crate::Readable for DCH0DSIZ {}
#[doc = "`write(|w| ..)` method takes [dch0dsiz::W](dch0dsiz::W) writer structure"]
impl crate::Writable for DCH0DSIZ {}
#[doc = "DCH0DSIZ register"]
pub mod dch0dsiz;
#[doc = "DCH0DSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dsizclr](dch0dsizclr) module"]
pub type DCH0DSIZCLR = crate::Reg<u32, _DCH0DSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DSIZCLR;
#[doc = "`read()` method returns [dch0dsizclr::R](dch0dsizclr::R) reader structure"]
impl crate::Readable for DCH0DSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch0dsizclr::W](dch0dsizclr::W) writer structure"]
impl crate::Writable for DCH0DSIZCLR {}
#[doc = "DCH0DSIZCLR register"]
pub mod dch0dsizclr;
#[doc = "DCH0DSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dsizset](dch0dsizset) module"]
pub type DCH0DSIZSET = crate::Reg<u32, _DCH0DSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DSIZSET;
#[doc = "`read()` method returns [dch0dsizset::R](dch0dsizset::R) reader structure"]
impl crate::Readable for DCH0DSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch0dsizset::W](dch0dsizset::W) writer structure"]
impl crate::Writable for DCH0DSIZSET {}
#[doc = "DCH0DSIZSET register"]
pub mod dch0dsizset;
#[doc = "DCH0DSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dsizinv](dch0dsizinv) module"]
pub type DCH0DSIZINV = crate::Reg<u32, _DCH0DSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DSIZINV;
#[doc = "`read()` method returns [dch0dsizinv::R](dch0dsizinv::R) reader structure"]
impl crate::Readable for DCH0DSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch0dsizinv::W](dch0dsizinv::W) writer structure"]
impl crate::Writable for DCH0DSIZINV {}
#[doc = "DCH0DSIZINV register"]
pub mod dch0dsizinv;
#[doc = "DCH0SPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0sptr](dch0sptr) module"]
pub type DCH0SPTR = crate::Reg<u32, _DCH0SPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SPTR;
#[doc = "`read()` method returns [dch0sptr::R](dch0sptr::R) reader structure"]
impl crate::Readable for DCH0SPTR {}
#[doc = "`write(|w| ..)` method takes [dch0sptr::W](dch0sptr::W) writer structure"]
impl crate::Writable for DCH0SPTR {}
#[doc = "DCH0SPTR register"]
pub mod dch0sptr;
#[doc = "DCH0SPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0sptrclr](dch0sptrclr) module"]
pub type DCH0SPTRCLR = crate::Reg<u32, _DCH0SPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SPTRCLR;
#[doc = "`read()` method returns [dch0sptrclr::R](dch0sptrclr::R) reader structure"]
impl crate::Readable for DCH0SPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch0sptrclr::W](dch0sptrclr::W) writer structure"]
impl crate::Writable for DCH0SPTRCLR {}
#[doc = "DCH0SPTRCLR register"]
pub mod dch0sptrclr;
#[doc = "DCH0SPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0sptrset](dch0sptrset) module"]
pub type DCH0SPTRSET = crate::Reg<u32, _DCH0SPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SPTRSET;
#[doc = "`read()` method returns [dch0sptrset::R](dch0sptrset::R) reader structure"]
impl crate::Readable for DCH0SPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch0sptrset::W](dch0sptrset::W) writer structure"]
impl crate::Writable for DCH0SPTRSET {}
#[doc = "DCH0SPTRSET register"]
pub mod dch0sptrset;
#[doc = "DCH0SPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0sptrinv](dch0sptrinv) module"]
pub type DCH0SPTRINV = crate::Reg<u32, _DCH0SPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0SPTRINV;
#[doc = "`read()` method returns [dch0sptrinv::R](dch0sptrinv::R) reader structure"]
impl crate::Readable for DCH0SPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch0sptrinv::W](dch0sptrinv::W) writer structure"]
impl crate::Writable for DCH0SPTRINV {}
#[doc = "DCH0SPTRINV register"]
pub mod dch0sptrinv;
#[doc = "DCH0DPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dptr](dch0dptr) module"]
pub type DCH0DPTR = crate::Reg<u32, _DCH0DPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DPTR;
#[doc = "`read()` method returns [dch0dptr::R](dch0dptr::R) reader structure"]
impl crate::Readable for DCH0DPTR {}
#[doc = "`write(|w| ..)` method takes [dch0dptr::W](dch0dptr::W) writer structure"]
impl crate::Writable for DCH0DPTR {}
#[doc = "DCH0DPTR register"]
pub mod dch0dptr;
#[doc = "DCH0DPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dptrclr](dch0dptrclr) module"]
pub type DCH0DPTRCLR = crate::Reg<u32, _DCH0DPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DPTRCLR;
#[doc = "`read()` method returns [dch0dptrclr::R](dch0dptrclr::R) reader structure"]
impl crate::Readable for DCH0DPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch0dptrclr::W](dch0dptrclr::W) writer structure"]
impl crate::Writable for DCH0DPTRCLR {}
#[doc = "DCH0DPTRCLR register"]
pub mod dch0dptrclr;
#[doc = "DCH0DPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dptrset](dch0dptrset) module"]
pub type DCH0DPTRSET = crate::Reg<u32, _DCH0DPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DPTRSET;
#[doc = "`read()` method returns [dch0dptrset::R](dch0dptrset::R) reader structure"]
impl crate::Readable for DCH0DPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch0dptrset::W](dch0dptrset::W) writer structure"]
impl crate::Writable for DCH0DPTRSET {}
#[doc = "DCH0DPTRSET register"]
pub mod dch0dptrset;
#[doc = "DCH0DPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dptrinv](dch0dptrinv) module"]
pub type DCH0DPTRINV = crate::Reg<u32, _DCH0DPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DPTRINV;
#[doc = "`read()` method returns [dch0dptrinv::R](dch0dptrinv::R) reader structure"]
impl crate::Readable for DCH0DPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch0dptrinv::W](dch0dptrinv::W) writer structure"]
impl crate::Writable for DCH0DPTRINV {}
#[doc = "DCH0DPTRINV register"]
pub mod dch0dptrinv;
#[doc = "DCH0CSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0csiz](dch0csiz) module"]
pub type DCH0CSIZ = crate::Reg<u32, _DCH0CSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CSIZ;
#[doc = "`read()` method returns [dch0csiz::R](dch0csiz::R) reader structure"]
impl crate::Readable for DCH0CSIZ {}
#[doc = "`write(|w| ..)` method takes [dch0csiz::W](dch0csiz::W) writer structure"]
impl crate::Writable for DCH0CSIZ {}
#[doc = "DCH0CSIZ register"]
pub mod dch0csiz;
#[doc = "DCH0CSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0csizclr](dch0csizclr) module"]
pub type DCH0CSIZCLR = crate::Reg<u32, _DCH0CSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CSIZCLR;
#[doc = "`read()` method returns [dch0csizclr::R](dch0csizclr::R) reader structure"]
impl crate::Readable for DCH0CSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch0csizclr::W](dch0csizclr::W) writer structure"]
impl crate::Writable for DCH0CSIZCLR {}
#[doc = "DCH0CSIZCLR register"]
pub mod dch0csizclr;
#[doc = "DCH0CSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0csizset](dch0csizset) module"]
pub type DCH0CSIZSET = crate::Reg<u32, _DCH0CSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CSIZSET;
#[doc = "`read()` method returns [dch0csizset::R](dch0csizset::R) reader structure"]
impl crate::Readable for DCH0CSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch0csizset::W](dch0csizset::W) writer structure"]
impl crate::Writable for DCH0CSIZSET {}
#[doc = "DCH0CSIZSET register"]
pub mod dch0csizset;
#[doc = "DCH0CSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0csizinv](dch0csizinv) module"]
pub type DCH0CSIZINV = crate::Reg<u32, _DCH0CSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CSIZINV;
#[doc = "`read()` method returns [dch0csizinv::R](dch0csizinv::R) reader structure"]
impl crate::Readable for DCH0CSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch0csizinv::W](dch0csizinv::W) writer structure"]
impl crate::Writable for DCH0CSIZINV {}
#[doc = "DCH0CSIZINV register"]
pub mod dch0csizinv;
#[doc = "DCH0CPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0cptr](dch0cptr) module"]
pub type DCH0CPTR = crate::Reg<u32, _DCH0CPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CPTR;
#[doc = "`read()` method returns [dch0cptr::R](dch0cptr::R) reader structure"]
impl crate::Readable for DCH0CPTR {}
#[doc = "`write(|w| ..)` method takes [dch0cptr::W](dch0cptr::W) writer structure"]
impl crate::Writable for DCH0CPTR {}
#[doc = "DCH0CPTR register"]
pub mod dch0cptr;
#[doc = "DCH0CPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0cptrclr](dch0cptrclr) module"]
pub type DCH0CPTRCLR = crate::Reg<u32, _DCH0CPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CPTRCLR;
#[doc = "`read()` method returns [dch0cptrclr::R](dch0cptrclr::R) reader structure"]
impl crate::Readable for DCH0CPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch0cptrclr::W](dch0cptrclr::W) writer structure"]
impl crate::Writable for DCH0CPTRCLR {}
#[doc = "DCH0CPTRCLR register"]
pub mod dch0cptrclr;
#[doc = "DCH0CPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0cptrset](dch0cptrset) module"]
pub type DCH0CPTRSET = crate::Reg<u32, _DCH0CPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CPTRSET;
#[doc = "`read()` method returns [dch0cptrset::R](dch0cptrset::R) reader structure"]
impl crate::Readable for DCH0CPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch0cptrset::W](dch0cptrset::W) writer structure"]
impl crate::Writable for DCH0CPTRSET {}
#[doc = "DCH0CPTRSET register"]
pub mod dch0cptrset;
#[doc = "DCH0CPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0cptrinv](dch0cptrinv) module"]
pub type DCH0CPTRINV = crate::Reg<u32, _DCH0CPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0CPTRINV;
#[doc = "`read()` method returns [dch0cptrinv::R](dch0cptrinv::R) reader structure"]
impl crate::Readable for DCH0CPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch0cptrinv::W](dch0cptrinv::W) writer structure"]
impl crate::Writable for DCH0CPTRINV {}
#[doc = "DCH0CPTRINV register"]
pub mod dch0cptrinv;
#[doc = "DCH0DAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0dat](dch0dat) module"]
pub type DCH0DAT = crate::Reg<u32, _DCH0DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DAT;
#[doc = "`read()` method returns [dch0dat::R](dch0dat::R) reader structure"]
impl crate::Readable for DCH0DAT {}
#[doc = "`write(|w| ..)` method takes [dch0dat::W](dch0dat::W) writer structure"]
impl crate::Writable for DCH0DAT {}
#[doc = "DCH0DAT register"]
pub mod dch0dat;
#[doc = "DCH0DATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0datclr](dch0datclr) module"]
pub type DCH0DATCLR = crate::Reg<u32, _DCH0DATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DATCLR;
#[doc = "`read()` method returns [dch0datclr::R](dch0datclr::R) reader structure"]
impl crate::Readable for DCH0DATCLR {}
#[doc = "`write(|w| ..)` method takes [dch0datclr::W](dch0datclr::W) writer structure"]
impl crate::Writable for DCH0DATCLR {}
#[doc = "DCH0DATCLR register"]
pub mod dch0datclr;
#[doc = "DCH0DATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0datset](dch0datset) module"]
pub type DCH0DATSET = crate::Reg<u32, _DCH0DATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DATSET;
#[doc = "`read()` method returns [dch0datset::R](dch0datset::R) reader structure"]
impl crate::Readable for DCH0DATSET {}
#[doc = "`write(|w| ..)` method takes [dch0datset::W](dch0datset::W) writer structure"]
impl crate::Writable for DCH0DATSET {}
#[doc = "DCH0DATSET register"]
pub mod dch0datset;
#[doc = "DCH0DATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch0datinv](dch0datinv) module"]
pub type DCH0DATINV = crate::Reg<u32, _DCH0DATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH0DATINV;
#[doc = "`read()` method returns [dch0datinv::R](dch0datinv::R) reader structure"]
impl crate::Readable for DCH0DATINV {}
#[doc = "`write(|w| ..)` method takes [dch0datinv::W](dch0datinv::W) writer structure"]
impl crate::Writable for DCH0DATINV {}
#[doc = "DCH0DATINV register"]
pub mod dch0datinv;
