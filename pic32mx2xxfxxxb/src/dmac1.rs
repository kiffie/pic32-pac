#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH1CON register"]
    pub dch1con: DCH1CON,
    #[doc = "0x04 - DCH1CONCLR register"]
    pub dch1conclr: DCH1CONCLR,
    #[doc = "0x08 - DCH1CONSET register"]
    pub dch1conset: DCH1CONSET,
    #[doc = "0x0c - DCH1CONINV register"]
    pub dch1coninv: DCH1CONINV,
    #[doc = "0x10 - DCH1ECON register"]
    pub dch1econ: DCH1ECON,
    #[doc = "0x14 - DCH1ECONCLR register"]
    pub dch1econclr: DCH1ECONCLR,
    #[doc = "0x18 - DCH1ECONSET register"]
    pub dch1econset: DCH1ECONSET,
    #[doc = "0x1c - DCH1ECONINV register"]
    pub dch1econinv: DCH1ECONINV,
    #[doc = "0x20 - DCH1INT register"]
    pub dch1int: DCH1INT,
    #[doc = "0x24 - DCH1INTCLR register"]
    pub dch1intclr: DCH1INTCLR,
    #[doc = "0x28 - DCH1INTSET register"]
    pub dch1intset: DCH1INTSET,
    #[doc = "0x2c - DCH1INTINV register"]
    pub dch1intinv: DCH1INTINV,
    #[doc = "0x30 - DCH1SSA register"]
    pub dch1ssa: DCH1SSA,
    #[doc = "0x34 - DCH1SSACLR register"]
    pub dch1ssaclr: DCH1SSACLR,
    #[doc = "0x38 - DCH1SSASET register"]
    pub dch1ssaset: DCH1SSASET,
    #[doc = "0x3c - DCH1SSAINV register"]
    pub dch1ssainv: DCH1SSAINV,
    #[doc = "0x40 - DCH1DSA register"]
    pub dch1dsa: DCH1DSA,
    #[doc = "0x44 - DCH1DSACLR register"]
    pub dch1dsaclr: DCH1DSACLR,
    #[doc = "0x48 - DCH1DSASET register"]
    pub dch1dsaset: DCH1DSASET,
    #[doc = "0x4c - DCH1DSAINV register"]
    pub dch1dsainv: DCH1DSAINV,
    #[doc = "0x50 - DCH1SSIZ register"]
    pub dch1ssiz: DCH1SSIZ,
    #[doc = "0x54 - DCH1SSIZCLR register"]
    pub dch1ssizclr: DCH1SSIZCLR,
    #[doc = "0x58 - DCH1SSIZSET register"]
    pub dch1ssizset: DCH1SSIZSET,
    #[doc = "0x5c - DCH1SSIZINV register"]
    pub dch1ssizinv: DCH1SSIZINV,
    #[doc = "0x60 - DCH1DSIZ register"]
    pub dch1dsiz: DCH1DSIZ,
    #[doc = "0x64 - DCH1DSIZCLR register"]
    pub dch1dsizclr: DCH1DSIZCLR,
    #[doc = "0x68 - DCH1DSIZSET register"]
    pub dch1dsizset: DCH1DSIZSET,
    #[doc = "0x6c - DCH1DSIZINV register"]
    pub dch1dsizinv: DCH1DSIZINV,
    #[doc = "0x70 - DCH1SPTR register"]
    pub dch1sptr: DCH1SPTR,
    #[doc = "0x74 - DCH1SPTRCLR register"]
    pub dch1sptrclr: DCH1SPTRCLR,
    #[doc = "0x78 - DCH1SPTRSET register"]
    pub dch1sptrset: DCH1SPTRSET,
    #[doc = "0x7c - DCH1SPTRINV register"]
    pub dch1sptrinv: DCH1SPTRINV,
    #[doc = "0x80 - DCH1DPTR register"]
    pub dch1dptr: DCH1DPTR,
    #[doc = "0x84 - DCH1DPTRCLR register"]
    pub dch1dptrclr: DCH1DPTRCLR,
    #[doc = "0x88 - DCH1DPTRSET register"]
    pub dch1dptrset: DCH1DPTRSET,
    #[doc = "0x8c - DCH1DPTRINV register"]
    pub dch1dptrinv: DCH1DPTRINV,
    #[doc = "0x90 - DCH1CSIZ register"]
    pub dch1csiz: DCH1CSIZ,
    #[doc = "0x94 - DCH1CSIZCLR register"]
    pub dch1csizclr: DCH1CSIZCLR,
    #[doc = "0x98 - DCH1CSIZSET register"]
    pub dch1csizset: DCH1CSIZSET,
    #[doc = "0x9c - DCH1CSIZINV register"]
    pub dch1csizinv: DCH1CSIZINV,
    #[doc = "0xa0 - DCH1CPTR register"]
    pub dch1cptr: DCH1CPTR,
    #[doc = "0xa4 - DCH1CPTRCLR register"]
    pub dch1cptrclr: DCH1CPTRCLR,
    #[doc = "0xa8 - DCH1CPTRSET register"]
    pub dch1cptrset: DCH1CPTRSET,
    #[doc = "0xac - DCH1CPTRINV register"]
    pub dch1cptrinv: DCH1CPTRINV,
    #[doc = "0xb0 - DCH1DAT register"]
    pub dch1dat: DCH1DAT,
    #[doc = "0xb4 - DCH1DATCLR register"]
    pub dch1datclr: DCH1DATCLR,
    #[doc = "0xb8 - DCH1DATSET register"]
    pub dch1datset: DCH1DATSET,
    #[doc = "0xbc - DCH1DATINV register"]
    pub dch1datinv: DCH1DATINV,
}
#[doc = "DCH1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1con](dch1con) module"]
pub type DCH1CON = crate::Reg<u32, _DCH1CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CON;
#[doc = "`read()` method returns [dch1con::R](dch1con::R) reader structure"]
impl crate::Readable for DCH1CON {}
#[doc = "`write(|w| ..)` method takes [dch1con::W](dch1con::W) writer structure"]
impl crate::Writable for DCH1CON {}
#[doc = "DCH1CON register"]
pub mod dch1con;
#[doc = "DCH1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1conclr](dch1conclr) module"]
pub type DCH1CONCLR = crate::Reg<u32, _DCH1CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CONCLR;
#[doc = "`read()` method returns [dch1conclr::R](dch1conclr::R) reader structure"]
impl crate::Readable for DCH1CONCLR {}
#[doc = "`write(|w| ..)` method takes [dch1conclr::W](dch1conclr::W) writer structure"]
impl crate::Writable for DCH1CONCLR {}
#[doc = "DCH1CONCLR register"]
pub mod dch1conclr;
#[doc = "DCH1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1conset](dch1conset) module"]
pub type DCH1CONSET = crate::Reg<u32, _DCH1CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CONSET;
#[doc = "`read()` method returns [dch1conset::R](dch1conset::R) reader structure"]
impl crate::Readable for DCH1CONSET {}
#[doc = "`write(|w| ..)` method takes [dch1conset::W](dch1conset::W) writer structure"]
impl crate::Writable for DCH1CONSET {}
#[doc = "DCH1CONSET register"]
pub mod dch1conset;
#[doc = "DCH1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1coninv](dch1coninv) module"]
pub type DCH1CONINV = crate::Reg<u32, _DCH1CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CONINV;
#[doc = "`read()` method returns [dch1coninv::R](dch1coninv::R) reader structure"]
impl crate::Readable for DCH1CONINV {}
#[doc = "`write(|w| ..)` method takes [dch1coninv::W](dch1coninv::W) writer structure"]
impl crate::Writable for DCH1CONINV {}
#[doc = "DCH1CONINV register"]
pub mod dch1coninv;
#[doc = "DCH1ECON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1econ](dch1econ) module"]
pub type DCH1ECON = crate::Reg<u32, _DCH1ECON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1ECON;
#[doc = "`read()` method returns [dch1econ::R](dch1econ::R) reader structure"]
impl crate::Readable for DCH1ECON {}
#[doc = "`write(|w| ..)` method takes [dch1econ::W](dch1econ::W) writer structure"]
impl crate::Writable for DCH1ECON {}
#[doc = "DCH1ECON register"]
pub mod dch1econ;
#[doc = "DCH1ECONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1econclr](dch1econclr) module"]
pub type DCH1ECONCLR = crate::Reg<u32, _DCH1ECONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1ECONCLR;
#[doc = "`read()` method returns [dch1econclr::R](dch1econclr::R) reader structure"]
impl crate::Readable for DCH1ECONCLR {}
#[doc = "`write(|w| ..)` method takes [dch1econclr::W](dch1econclr::W) writer structure"]
impl crate::Writable for DCH1ECONCLR {}
#[doc = "DCH1ECONCLR register"]
pub mod dch1econclr;
#[doc = "DCH1ECONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1econset](dch1econset) module"]
pub type DCH1ECONSET = crate::Reg<u32, _DCH1ECONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1ECONSET;
#[doc = "`read()` method returns [dch1econset::R](dch1econset::R) reader structure"]
impl crate::Readable for DCH1ECONSET {}
#[doc = "`write(|w| ..)` method takes [dch1econset::W](dch1econset::W) writer structure"]
impl crate::Writable for DCH1ECONSET {}
#[doc = "DCH1ECONSET register"]
pub mod dch1econset;
#[doc = "DCH1ECONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1econinv](dch1econinv) module"]
pub type DCH1ECONINV = crate::Reg<u32, _DCH1ECONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1ECONINV;
#[doc = "`read()` method returns [dch1econinv::R](dch1econinv::R) reader structure"]
impl crate::Readable for DCH1ECONINV {}
#[doc = "`write(|w| ..)` method takes [dch1econinv::W](dch1econinv::W) writer structure"]
impl crate::Writable for DCH1ECONINV {}
#[doc = "DCH1ECONINV register"]
pub mod dch1econinv;
#[doc = "DCH1INT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1int](dch1int) module"]
pub type DCH1INT = crate::Reg<u32, _DCH1INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1INT;
#[doc = "`read()` method returns [dch1int::R](dch1int::R) reader structure"]
impl crate::Readable for DCH1INT {}
#[doc = "`write(|w| ..)` method takes [dch1int::W](dch1int::W) writer structure"]
impl crate::Writable for DCH1INT {}
#[doc = "DCH1INT register"]
pub mod dch1int;
#[doc = "DCH1INTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1intclr](dch1intclr) module"]
pub type DCH1INTCLR = crate::Reg<u32, _DCH1INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1INTCLR;
#[doc = "`read()` method returns [dch1intclr::R](dch1intclr::R) reader structure"]
impl crate::Readable for DCH1INTCLR {}
#[doc = "`write(|w| ..)` method takes [dch1intclr::W](dch1intclr::W) writer structure"]
impl crate::Writable for DCH1INTCLR {}
#[doc = "DCH1INTCLR register"]
pub mod dch1intclr;
#[doc = "DCH1INTSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1intset](dch1intset) module"]
pub type DCH1INTSET = crate::Reg<u32, _DCH1INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1INTSET;
#[doc = "`read()` method returns [dch1intset::R](dch1intset::R) reader structure"]
impl crate::Readable for DCH1INTSET {}
#[doc = "`write(|w| ..)` method takes [dch1intset::W](dch1intset::W) writer structure"]
impl crate::Writable for DCH1INTSET {}
#[doc = "DCH1INTSET register"]
pub mod dch1intset;
#[doc = "DCH1INTINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1intinv](dch1intinv) module"]
pub type DCH1INTINV = crate::Reg<u32, _DCH1INTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1INTINV;
#[doc = "`read()` method returns [dch1intinv::R](dch1intinv::R) reader structure"]
impl crate::Readable for DCH1INTINV {}
#[doc = "`write(|w| ..)` method takes [dch1intinv::W](dch1intinv::W) writer structure"]
impl crate::Writable for DCH1INTINV {}
#[doc = "DCH1INTINV register"]
pub mod dch1intinv;
#[doc = "DCH1SSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1ssa](dch1ssa) module"]
pub type DCH1SSA = crate::Reg<u32, _DCH1SSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SSA;
#[doc = "`read()` method returns [dch1ssa::R](dch1ssa::R) reader structure"]
impl crate::Readable for DCH1SSA {}
#[doc = "`write(|w| ..)` method takes [dch1ssa::W](dch1ssa::W) writer structure"]
impl crate::Writable for DCH1SSA {}
#[doc = "DCH1SSA register"]
pub mod dch1ssa;
#[doc = "DCH1SSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1ssaclr](dch1ssaclr) module"]
pub type DCH1SSACLR = crate::Reg<u32, _DCH1SSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SSACLR;
#[doc = "`read()` method returns [dch1ssaclr::R](dch1ssaclr::R) reader structure"]
impl crate::Readable for DCH1SSACLR {}
#[doc = "`write(|w| ..)` method takes [dch1ssaclr::W](dch1ssaclr::W) writer structure"]
impl crate::Writable for DCH1SSACLR {}
#[doc = "DCH1SSACLR register"]
pub mod dch1ssaclr;
#[doc = "DCH1SSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1ssaset](dch1ssaset) module"]
pub type DCH1SSASET = crate::Reg<u32, _DCH1SSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SSASET;
#[doc = "`read()` method returns [dch1ssaset::R](dch1ssaset::R) reader structure"]
impl crate::Readable for DCH1SSASET {}
#[doc = "`write(|w| ..)` method takes [dch1ssaset::W](dch1ssaset::W) writer structure"]
impl crate::Writable for DCH1SSASET {}
#[doc = "DCH1SSASET register"]
pub mod dch1ssaset;
#[doc = "DCH1SSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1ssainv](dch1ssainv) module"]
pub type DCH1SSAINV = crate::Reg<u32, _DCH1SSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SSAINV;
#[doc = "`read()` method returns [dch1ssainv::R](dch1ssainv::R) reader structure"]
impl crate::Readable for DCH1SSAINV {}
#[doc = "`write(|w| ..)` method takes [dch1ssainv::W](dch1ssainv::W) writer structure"]
impl crate::Writable for DCH1SSAINV {}
#[doc = "DCH1SSAINV register"]
pub mod dch1ssainv;
#[doc = "DCH1DSA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dsa](dch1dsa) module"]
pub type DCH1DSA = crate::Reg<u32, _DCH1DSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DSA;
#[doc = "`read()` method returns [dch1dsa::R](dch1dsa::R) reader structure"]
impl crate::Readable for DCH1DSA {}
#[doc = "`write(|w| ..)` method takes [dch1dsa::W](dch1dsa::W) writer structure"]
impl crate::Writable for DCH1DSA {}
#[doc = "DCH1DSA register"]
pub mod dch1dsa;
#[doc = "DCH1DSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dsaclr](dch1dsaclr) module"]
pub type DCH1DSACLR = crate::Reg<u32, _DCH1DSACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DSACLR;
#[doc = "`read()` method returns [dch1dsaclr::R](dch1dsaclr::R) reader structure"]
impl crate::Readable for DCH1DSACLR {}
#[doc = "`write(|w| ..)` method takes [dch1dsaclr::W](dch1dsaclr::W) writer structure"]
impl crate::Writable for DCH1DSACLR {}
#[doc = "DCH1DSACLR register"]
pub mod dch1dsaclr;
#[doc = "DCH1DSASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dsaset](dch1dsaset) module"]
pub type DCH1DSASET = crate::Reg<u32, _DCH1DSASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DSASET;
#[doc = "`read()` method returns [dch1dsaset::R](dch1dsaset::R) reader structure"]
impl crate::Readable for DCH1DSASET {}
#[doc = "`write(|w| ..)` method takes [dch1dsaset::W](dch1dsaset::W) writer structure"]
impl crate::Writable for DCH1DSASET {}
#[doc = "DCH1DSASET register"]
pub mod dch1dsaset;
#[doc = "DCH1DSAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dsainv](dch1dsainv) module"]
pub type DCH1DSAINV = crate::Reg<u32, _DCH1DSAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DSAINV;
#[doc = "`read()` method returns [dch1dsainv::R](dch1dsainv::R) reader structure"]
impl crate::Readable for DCH1DSAINV {}
#[doc = "`write(|w| ..)` method takes [dch1dsainv::W](dch1dsainv::W) writer structure"]
impl crate::Writable for DCH1DSAINV {}
#[doc = "DCH1DSAINV register"]
pub mod dch1dsainv;
#[doc = "DCH1SSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1ssiz](dch1ssiz) module"]
pub type DCH1SSIZ = crate::Reg<u32, _DCH1SSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SSIZ;
#[doc = "`read()` method returns [dch1ssiz::R](dch1ssiz::R) reader structure"]
impl crate::Readable for DCH1SSIZ {}
#[doc = "`write(|w| ..)` method takes [dch1ssiz::W](dch1ssiz::W) writer structure"]
impl crate::Writable for DCH1SSIZ {}
#[doc = "DCH1SSIZ register"]
pub mod dch1ssiz;
#[doc = "DCH1SSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1ssizclr](dch1ssizclr) module"]
pub type DCH1SSIZCLR = crate::Reg<u32, _DCH1SSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SSIZCLR;
#[doc = "`read()` method returns [dch1ssizclr::R](dch1ssizclr::R) reader structure"]
impl crate::Readable for DCH1SSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch1ssizclr::W](dch1ssizclr::W) writer structure"]
impl crate::Writable for DCH1SSIZCLR {}
#[doc = "DCH1SSIZCLR register"]
pub mod dch1ssizclr;
#[doc = "DCH1SSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1ssizset](dch1ssizset) module"]
pub type DCH1SSIZSET = crate::Reg<u32, _DCH1SSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SSIZSET;
#[doc = "`read()` method returns [dch1ssizset::R](dch1ssizset::R) reader structure"]
impl crate::Readable for DCH1SSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch1ssizset::W](dch1ssizset::W) writer structure"]
impl crate::Writable for DCH1SSIZSET {}
#[doc = "DCH1SSIZSET register"]
pub mod dch1ssizset;
#[doc = "DCH1SSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1ssizinv](dch1ssizinv) module"]
pub type DCH1SSIZINV = crate::Reg<u32, _DCH1SSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SSIZINV;
#[doc = "`read()` method returns [dch1ssizinv::R](dch1ssizinv::R) reader structure"]
impl crate::Readable for DCH1SSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch1ssizinv::W](dch1ssizinv::W) writer structure"]
impl crate::Writable for DCH1SSIZINV {}
#[doc = "DCH1SSIZINV register"]
pub mod dch1ssizinv;
#[doc = "DCH1DSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dsiz](dch1dsiz) module"]
pub type DCH1DSIZ = crate::Reg<u32, _DCH1DSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DSIZ;
#[doc = "`read()` method returns [dch1dsiz::R](dch1dsiz::R) reader structure"]
impl crate::Readable for DCH1DSIZ {}
#[doc = "`write(|w| ..)` method takes [dch1dsiz::W](dch1dsiz::W) writer structure"]
impl crate::Writable for DCH1DSIZ {}
#[doc = "DCH1DSIZ register"]
pub mod dch1dsiz;
#[doc = "DCH1DSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dsizclr](dch1dsizclr) module"]
pub type DCH1DSIZCLR = crate::Reg<u32, _DCH1DSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DSIZCLR;
#[doc = "`read()` method returns [dch1dsizclr::R](dch1dsizclr::R) reader structure"]
impl crate::Readable for DCH1DSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch1dsizclr::W](dch1dsizclr::W) writer structure"]
impl crate::Writable for DCH1DSIZCLR {}
#[doc = "DCH1DSIZCLR register"]
pub mod dch1dsizclr;
#[doc = "DCH1DSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dsizset](dch1dsizset) module"]
pub type DCH1DSIZSET = crate::Reg<u32, _DCH1DSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DSIZSET;
#[doc = "`read()` method returns [dch1dsizset::R](dch1dsizset::R) reader structure"]
impl crate::Readable for DCH1DSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch1dsizset::W](dch1dsizset::W) writer structure"]
impl crate::Writable for DCH1DSIZSET {}
#[doc = "DCH1DSIZSET register"]
pub mod dch1dsizset;
#[doc = "DCH1DSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dsizinv](dch1dsizinv) module"]
pub type DCH1DSIZINV = crate::Reg<u32, _DCH1DSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DSIZINV;
#[doc = "`read()` method returns [dch1dsizinv::R](dch1dsizinv::R) reader structure"]
impl crate::Readable for DCH1DSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch1dsizinv::W](dch1dsizinv::W) writer structure"]
impl crate::Writable for DCH1DSIZINV {}
#[doc = "DCH1DSIZINV register"]
pub mod dch1dsizinv;
#[doc = "DCH1SPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1sptr](dch1sptr) module"]
pub type DCH1SPTR = crate::Reg<u32, _DCH1SPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SPTR;
#[doc = "`read()` method returns [dch1sptr::R](dch1sptr::R) reader structure"]
impl crate::Readable for DCH1SPTR {}
#[doc = "`write(|w| ..)` method takes [dch1sptr::W](dch1sptr::W) writer structure"]
impl crate::Writable for DCH1SPTR {}
#[doc = "DCH1SPTR register"]
pub mod dch1sptr;
#[doc = "DCH1SPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1sptrclr](dch1sptrclr) module"]
pub type DCH1SPTRCLR = crate::Reg<u32, _DCH1SPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SPTRCLR;
#[doc = "`read()` method returns [dch1sptrclr::R](dch1sptrclr::R) reader structure"]
impl crate::Readable for DCH1SPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch1sptrclr::W](dch1sptrclr::W) writer structure"]
impl crate::Writable for DCH1SPTRCLR {}
#[doc = "DCH1SPTRCLR register"]
pub mod dch1sptrclr;
#[doc = "DCH1SPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1sptrset](dch1sptrset) module"]
pub type DCH1SPTRSET = crate::Reg<u32, _DCH1SPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SPTRSET;
#[doc = "`read()` method returns [dch1sptrset::R](dch1sptrset::R) reader structure"]
impl crate::Readable for DCH1SPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch1sptrset::W](dch1sptrset::W) writer structure"]
impl crate::Writable for DCH1SPTRSET {}
#[doc = "DCH1SPTRSET register"]
pub mod dch1sptrset;
#[doc = "DCH1SPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1sptrinv](dch1sptrinv) module"]
pub type DCH1SPTRINV = crate::Reg<u32, _DCH1SPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1SPTRINV;
#[doc = "`read()` method returns [dch1sptrinv::R](dch1sptrinv::R) reader structure"]
impl crate::Readable for DCH1SPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch1sptrinv::W](dch1sptrinv::W) writer structure"]
impl crate::Writable for DCH1SPTRINV {}
#[doc = "DCH1SPTRINV register"]
pub mod dch1sptrinv;
#[doc = "DCH1DPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dptr](dch1dptr) module"]
pub type DCH1DPTR = crate::Reg<u32, _DCH1DPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DPTR;
#[doc = "`read()` method returns [dch1dptr::R](dch1dptr::R) reader structure"]
impl crate::Readable for DCH1DPTR {}
#[doc = "`write(|w| ..)` method takes [dch1dptr::W](dch1dptr::W) writer structure"]
impl crate::Writable for DCH1DPTR {}
#[doc = "DCH1DPTR register"]
pub mod dch1dptr;
#[doc = "DCH1DPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dptrclr](dch1dptrclr) module"]
pub type DCH1DPTRCLR = crate::Reg<u32, _DCH1DPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DPTRCLR;
#[doc = "`read()` method returns [dch1dptrclr::R](dch1dptrclr::R) reader structure"]
impl crate::Readable for DCH1DPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch1dptrclr::W](dch1dptrclr::W) writer structure"]
impl crate::Writable for DCH1DPTRCLR {}
#[doc = "DCH1DPTRCLR register"]
pub mod dch1dptrclr;
#[doc = "DCH1DPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dptrset](dch1dptrset) module"]
pub type DCH1DPTRSET = crate::Reg<u32, _DCH1DPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DPTRSET;
#[doc = "`read()` method returns [dch1dptrset::R](dch1dptrset::R) reader structure"]
impl crate::Readable for DCH1DPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch1dptrset::W](dch1dptrset::W) writer structure"]
impl crate::Writable for DCH1DPTRSET {}
#[doc = "DCH1DPTRSET register"]
pub mod dch1dptrset;
#[doc = "DCH1DPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dptrinv](dch1dptrinv) module"]
pub type DCH1DPTRINV = crate::Reg<u32, _DCH1DPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DPTRINV;
#[doc = "`read()` method returns [dch1dptrinv::R](dch1dptrinv::R) reader structure"]
impl crate::Readable for DCH1DPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch1dptrinv::W](dch1dptrinv::W) writer structure"]
impl crate::Writable for DCH1DPTRINV {}
#[doc = "DCH1DPTRINV register"]
pub mod dch1dptrinv;
#[doc = "DCH1CSIZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1csiz](dch1csiz) module"]
pub type DCH1CSIZ = crate::Reg<u32, _DCH1CSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CSIZ;
#[doc = "`read()` method returns [dch1csiz::R](dch1csiz::R) reader structure"]
impl crate::Readable for DCH1CSIZ {}
#[doc = "`write(|w| ..)` method takes [dch1csiz::W](dch1csiz::W) writer structure"]
impl crate::Writable for DCH1CSIZ {}
#[doc = "DCH1CSIZ register"]
pub mod dch1csiz;
#[doc = "DCH1CSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1csizclr](dch1csizclr) module"]
pub type DCH1CSIZCLR = crate::Reg<u32, _DCH1CSIZCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CSIZCLR;
#[doc = "`read()` method returns [dch1csizclr::R](dch1csizclr::R) reader structure"]
impl crate::Readable for DCH1CSIZCLR {}
#[doc = "`write(|w| ..)` method takes [dch1csizclr::W](dch1csizclr::W) writer structure"]
impl crate::Writable for DCH1CSIZCLR {}
#[doc = "DCH1CSIZCLR register"]
pub mod dch1csizclr;
#[doc = "DCH1CSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1csizset](dch1csizset) module"]
pub type DCH1CSIZSET = crate::Reg<u32, _DCH1CSIZSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CSIZSET;
#[doc = "`read()` method returns [dch1csizset::R](dch1csizset::R) reader structure"]
impl crate::Readable for DCH1CSIZSET {}
#[doc = "`write(|w| ..)` method takes [dch1csizset::W](dch1csizset::W) writer structure"]
impl crate::Writable for DCH1CSIZSET {}
#[doc = "DCH1CSIZSET register"]
pub mod dch1csizset;
#[doc = "DCH1CSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1csizinv](dch1csizinv) module"]
pub type DCH1CSIZINV = crate::Reg<u32, _DCH1CSIZINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CSIZINV;
#[doc = "`read()` method returns [dch1csizinv::R](dch1csizinv::R) reader structure"]
impl crate::Readable for DCH1CSIZINV {}
#[doc = "`write(|w| ..)` method takes [dch1csizinv::W](dch1csizinv::W) writer structure"]
impl crate::Writable for DCH1CSIZINV {}
#[doc = "DCH1CSIZINV register"]
pub mod dch1csizinv;
#[doc = "DCH1CPTR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1cptr](dch1cptr) module"]
pub type DCH1CPTR = crate::Reg<u32, _DCH1CPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CPTR;
#[doc = "`read()` method returns [dch1cptr::R](dch1cptr::R) reader structure"]
impl crate::Readable for DCH1CPTR {}
#[doc = "`write(|w| ..)` method takes [dch1cptr::W](dch1cptr::W) writer structure"]
impl crate::Writable for DCH1CPTR {}
#[doc = "DCH1CPTR register"]
pub mod dch1cptr;
#[doc = "DCH1CPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1cptrclr](dch1cptrclr) module"]
pub type DCH1CPTRCLR = crate::Reg<u32, _DCH1CPTRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CPTRCLR;
#[doc = "`read()` method returns [dch1cptrclr::R](dch1cptrclr::R) reader structure"]
impl crate::Readable for DCH1CPTRCLR {}
#[doc = "`write(|w| ..)` method takes [dch1cptrclr::W](dch1cptrclr::W) writer structure"]
impl crate::Writable for DCH1CPTRCLR {}
#[doc = "DCH1CPTRCLR register"]
pub mod dch1cptrclr;
#[doc = "DCH1CPTRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1cptrset](dch1cptrset) module"]
pub type DCH1CPTRSET = crate::Reg<u32, _DCH1CPTRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CPTRSET;
#[doc = "`read()` method returns [dch1cptrset::R](dch1cptrset::R) reader structure"]
impl crate::Readable for DCH1CPTRSET {}
#[doc = "`write(|w| ..)` method takes [dch1cptrset::W](dch1cptrset::W) writer structure"]
impl crate::Writable for DCH1CPTRSET {}
#[doc = "DCH1CPTRSET register"]
pub mod dch1cptrset;
#[doc = "DCH1CPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1cptrinv](dch1cptrinv) module"]
pub type DCH1CPTRINV = crate::Reg<u32, _DCH1CPTRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1CPTRINV;
#[doc = "`read()` method returns [dch1cptrinv::R](dch1cptrinv::R) reader structure"]
impl crate::Readable for DCH1CPTRINV {}
#[doc = "`write(|w| ..)` method takes [dch1cptrinv::W](dch1cptrinv::W) writer structure"]
impl crate::Writable for DCH1CPTRINV {}
#[doc = "DCH1CPTRINV register"]
pub mod dch1cptrinv;
#[doc = "DCH1DAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1dat](dch1dat) module"]
pub type DCH1DAT = crate::Reg<u32, _DCH1DAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DAT;
#[doc = "`read()` method returns [dch1dat::R](dch1dat::R) reader structure"]
impl crate::Readable for DCH1DAT {}
#[doc = "`write(|w| ..)` method takes [dch1dat::W](dch1dat::W) writer structure"]
impl crate::Writable for DCH1DAT {}
#[doc = "DCH1DAT register"]
pub mod dch1dat;
#[doc = "DCH1DATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1datclr](dch1datclr) module"]
pub type DCH1DATCLR = crate::Reg<u32, _DCH1DATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DATCLR;
#[doc = "`read()` method returns [dch1datclr::R](dch1datclr::R) reader structure"]
impl crate::Readable for DCH1DATCLR {}
#[doc = "`write(|w| ..)` method takes [dch1datclr::W](dch1datclr::W) writer structure"]
impl crate::Writable for DCH1DATCLR {}
#[doc = "DCH1DATCLR register"]
pub mod dch1datclr;
#[doc = "DCH1DATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1datset](dch1datset) module"]
pub type DCH1DATSET = crate::Reg<u32, _DCH1DATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DATSET;
#[doc = "`read()` method returns [dch1datset::R](dch1datset::R) reader structure"]
impl crate::Readable for DCH1DATSET {}
#[doc = "`write(|w| ..)` method takes [dch1datset::W](dch1datset::W) writer structure"]
impl crate::Writable for DCH1DATSET {}
#[doc = "DCH1DATSET register"]
pub mod dch1datset;
#[doc = "DCH1DATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dch1datinv](dch1datinv) module"]
pub type DCH1DATINV = crate::Reg<u32, _DCH1DATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCH1DATINV;
#[doc = "`read()` method returns [dch1datinv::R](dch1datinv::R) reader structure"]
impl crate::Readable for DCH1DATINV {}
#[doc = "`write(|w| ..)` method takes [dch1datinv::W](dch1datinv::W) writer structure"]
impl crate::Writable for DCH1DATINV {}
#[doc = "DCH1DATINV register"]
pub mod dch1datinv;
