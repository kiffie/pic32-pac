#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMACON register"]
    pub dmacon: DMACON,
    #[doc = "0x04 - DMACONCLR register"]
    pub dmaconclr: DMACONCLR,
    #[doc = "0x08 - DMACONSET register"]
    pub dmaconset: DMACONSET,
    #[doc = "0x0c - DMACONINV register"]
    pub dmaconinv: DMACONINV,
    #[doc = "0x10 - DMASTAT register"]
    pub dmastat: DMASTAT,
    #[doc = "0x14 - DMASTATCLR register"]
    pub dmastatclr: DMASTATCLR,
    #[doc = "0x18 - DMASTATSET register"]
    pub dmastatset: DMASTATSET,
    #[doc = "0x1c - DMASTATINV register"]
    pub dmastatinv: DMASTATINV,
    #[doc = "0x20 - DMAADDR register"]
    pub dmaaddr: DMAADDR,
    #[doc = "0x24 - DMAADDRCLR register"]
    pub dmaaddrclr: DMAADDRCLR,
    #[doc = "0x28 - DMAADDRSET register"]
    pub dmaaddrset: DMAADDRSET,
    #[doc = "0x2c - DMAADDRINV register"]
    pub dmaaddrinv: DMAADDRINV,
    #[doc = "0x30 - DCRCCON register"]
    pub dcrccon: DCRCCON,
    #[doc = "0x34 - DCRCCONCLR register"]
    pub dcrcconclr: DCRCCONCLR,
    #[doc = "0x38 - DCRCCONSET register"]
    pub dcrcconset: DCRCCONSET,
    #[doc = "0x3c - DCRCCONINV register"]
    pub dcrcconinv: DCRCCONINV,
    #[doc = "0x40 - DCRCDATA register"]
    pub dcrcdata: DCRCDATA,
    #[doc = "0x44 - DCRCDATACLR register"]
    pub dcrcdataclr: DCRCDATACLR,
    #[doc = "0x48 - DCRCDATASET register"]
    pub dcrcdataset: DCRCDATASET,
    #[doc = "0x4c - DCRCDATAINV register"]
    pub dcrcdatainv: DCRCDATAINV,
    #[doc = "0x50 - DCRCXOR register"]
    pub dcrcxor: DCRCXOR,
    #[doc = "0x54 - DCRCXORCLR register"]
    pub dcrcxorclr: DCRCXORCLR,
    #[doc = "0x58 - DCRCXORSET register"]
    pub dcrcxorset: DCRCXORSET,
    #[doc = "0x5c - DCRCXORINV register"]
    pub dcrcxorinv: DCRCXORINV,
}
#[doc = "DMACON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacon](dmacon) module"]
pub type DMACON = crate::Reg<u32, _DMACON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACON;
#[doc = "`read()` method returns [dmacon::R](dmacon::R) reader structure"]
impl crate::Readable for DMACON {}
#[doc = "`write(|w| ..)` method takes [dmacon::W](dmacon::W) writer structure"]
impl crate::Writable for DMACON {}
#[doc = "DMACON register"]
pub mod dmacon;
#[doc = "DMACONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaconclr](dmaconclr) module"]
pub type DMACONCLR = crate::Reg<u32, _DMACONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACONCLR;
#[doc = "`read()` method returns [dmaconclr::R](dmaconclr::R) reader structure"]
impl crate::Readable for DMACONCLR {}
#[doc = "`write(|w| ..)` method takes [dmaconclr::W](dmaconclr::W) writer structure"]
impl crate::Writable for DMACONCLR {}
#[doc = "DMACONCLR register"]
pub mod dmaconclr;
#[doc = "DMACONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaconset](dmaconset) module"]
pub type DMACONSET = crate::Reg<u32, _DMACONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACONSET;
#[doc = "`read()` method returns [dmaconset::R](dmaconset::R) reader structure"]
impl crate::Readable for DMACONSET {}
#[doc = "`write(|w| ..)` method takes [dmaconset::W](dmaconset::W) writer structure"]
impl crate::Writable for DMACONSET {}
#[doc = "DMACONSET register"]
pub mod dmaconset;
#[doc = "DMACONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaconinv](dmaconinv) module"]
pub type DMACONINV = crate::Reg<u32, _DMACONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACONINV;
#[doc = "`read()` method returns [dmaconinv::R](dmaconinv::R) reader structure"]
impl crate::Readable for DMACONINV {}
#[doc = "`write(|w| ..)` method takes [dmaconinv::W](dmaconinv::W) writer structure"]
impl crate::Writable for DMACONINV {}
#[doc = "DMACONINV register"]
pub mod dmaconinv;
#[doc = "DMASTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastat](dmastat) module"]
pub type DMASTAT = crate::Reg<u32, _DMASTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASTAT;
#[doc = "`read()` method returns [dmastat::R](dmastat::R) reader structure"]
impl crate::Readable for DMASTAT {}
#[doc = "`write(|w| ..)` method takes [dmastat::W](dmastat::W) writer structure"]
impl crate::Writable for DMASTAT {}
#[doc = "DMASTAT register"]
pub mod dmastat;
#[doc = "DMASTATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastatclr](dmastatclr) module"]
pub type DMASTATCLR = crate::Reg<u32, _DMASTATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASTATCLR;
#[doc = "`read()` method returns [dmastatclr::R](dmastatclr::R) reader structure"]
impl crate::Readable for DMASTATCLR {}
#[doc = "`write(|w| ..)` method takes [dmastatclr::W](dmastatclr::W) writer structure"]
impl crate::Writable for DMASTATCLR {}
#[doc = "DMASTATCLR register"]
pub mod dmastatclr;
#[doc = "DMASTATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastatset](dmastatset) module"]
pub type DMASTATSET = crate::Reg<u32, _DMASTATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASTATSET;
#[doc = "`read()` method returns [dmastatset::R](dmastatset::R) reader structure"]
impl crate::Readable for DMASTATSET {}
#[doc = "`write(|w| ..)` method takes [dmastatset::W](dmastatset::W) writer structure"]
impl crate::Writable for DMASTATSET {}
#[doc = "DMASTATSET register"]
pub mod dmastatset;
#[doc = "DMASTATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmastatinv](dmastatinv) module"]
pub type DMASTATINV = crate::Reg<u32, _DMASTATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMASTATINV;
#[doc = "`read()` method returns [dmastatinv::R](dmastatinv::R) reader structure"]
impl crate::Readable for DMASTATINV {}
#[doc = "`write(|w| ..)` method takes [dmastatinv::W](dmastatinv::W) writer structure"]
impl crate::Writable for DMASTATINV {}
#[doc = "DMASTATINV register"]
pub mod dmastatinv;
#[doc = "DMAADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddr](dmaaddr) module"]
pub type DMAADDR = crate::Reg<u32, _DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDR;
#[doc = "`read()` method returns [dmaaddr::R](dmaaddr::R) reader structure"]
impl crate::Readable for DMAADDR {}
#[doc = "`write(|w| ..)` method takes [dmaaddr::W](dmaaddr::W) writer structure"]
impl crate::Writable for DMAADDR {}
#[doc = "DMAADDR register"]
pub mod dmaaddr;
#[doc = "DMAADDRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddrclr](dmaaddrclr) module"]
pub type DMAADDRCLR = crate::Reg<u32, _DMAADDRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDRCLR;
#[doc = "`read()` method returns [dmaaddrclr::R](dmaaddrclr::R) reader structure"]
impl crate::Readable for DMAADDRCLR {}
#[doc = "`write(|w| ..)` method takes [dmaaddrclr::W](dmaaddrclr::W) writer structure"]
impl crate::Writable for DMAADDRCLR {}
#[doc = "DMAADDRCLR register"]
pub mod dmaaddrclr;
#[doc = "DMAADDRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddrset](dmaaddrset) module"]
pub type DMAADDRSET = crate::Reg<u32, _DMAADDRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDRSET;
#[doc = "`read()` method returns [dmaaddrset::R](dmaaddrset::R) reader structure"]
impl crate::Readable for DMAADDRSET {}
#[doc = "`write(|w| ..)` method takes [dmaaddrset::W](dmaaddrset::W) writer structure"]
impl crate::Writable for DMAADDRSET {}
#[doc = "DMAADDRSET register"]
pub mod dmaaddrset;
#[doc = "DMAADDRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaaddrinv](dmaaddrinv) module"]
pub type DMAADDRINV = crate::Reg<u32, _DMAADDRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAADDRINV;
#[doc = "`read()` method returns [dmaaddrinv::R](dmaaddrinv::R) reader structure"]
impl crate::Readable for DMAADDRINV {}
#[doc = "`write(|w| ..)` method takes [dmaaddrinv::W](dmaaddrinv::W) writer structure"]
impl crate::Writable for DMAADDRINV {}
#[doc = "DMAADDRINV register"]
pub mod dmaaddrinv;
#[doc = "DCRCCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrccon](dcrccon) module"]
pub type DCRCCON = crate::Reg<u32, _DCRCCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCCON;
#[doc = "`read()` method returns [dcrccon::R](dcrccon::R) reader structure"]
impl crate::Readable for DCRCCON {}
#[doc = "`write(|w| ..)` method takes [dcrccon::W](dcrccon::W) writer structure"]
impl crate::Writable for DCRCCON {}
#[doc = "DCRCCON register"]
pub mod dcrccon;
#[doc = "DCRCCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcconclr](dcrcconclr) module"]
pub type DCRCCONCLR = crate::Reg<u32, _DCRCCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCCONCLR;
#[doc = "`read()` method returns [dcrcconclr::R](dcrcconclr::R) reader structure"]
impl crate::Readable for DCRCCONCLR {}
#[doc = "`write(|w| ..)` method takes [dcrcconclr::W](dcrcconclr::W) writer structure"]
impl crate::Writable for DCRCCONCLR {}
#[doc = "DCRCCONCLR register"]
pub mod dcrcconclr;
#[doc = "DCRCCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcconset](dcrcconset) module"]
pub type DCRCCONSET = crate::Reg<u32, _DCRCCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCCONSET;
#[doc = "`read()` method returns [dcrcconset::R](dcrcconset::R) reader structure"]
impl crate::Readable for DCRCCONSET {}
#[doc = "`write(|w| ..)` method takes [dcrcconset::W](dcrcconset::W) writer structure"]
impl crate::Writable for DCRCCONSET {}
#[doc = "DCRCCONSET register"]
pub mod dcrcconset;
#[doc = "DCRCCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcconinv](dcrcconinv) module"]
pub type DCRCCONINV = crate::Reg<u32, _DCRCCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCCONINV;
#[doc = "`read()` method returns [dcrcconinv::R](dcrcconinv::R) reader structure"]
impl crate::Readable for DCRCCONINV {}
#[doc = "`write(|w| ..)` method takes [dcrcconinv::W](dcrcconinv::W) writer structure"]
impl crate::Writable for DCRCCONINV {}
#[doc = "DCRCCONINV register"]
pub mod dcrcconinv;
#[doc = "DCRCDATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcdata](dcrcdata) module"]
pub type DCRCDATA = crate::Reg<u32, _DCRCDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCDATA;
#[doc = "`read()` method returns [dcrcdata::R](dcrcdata::R) reader structure"]
impl crate::Readable for DCRCDATA {}
#[doc = "`write(|w| ..)` method takes [dcrcdata::W](dcrcdata::W) writer structure"]
impl crate::Writable for DCRCDATA {}
#[doc = "DCRCDATA register"]
pub mod dcrcdata;
#[doc = "DCRCDATACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcdataclr](dcrcdataclr) module"]
pub type DCRCDATACLR = crate::Reg<u32, _DCRCDATACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCDATACLR;
#[doc = "`read()` method returns [dcrcdataclr::R](dcrcdataclr::R) reader structure"]
impl crate::Readable for DCRCDATACLR {}
#[doc = "`write(|w| ..)` method takes [dcrcdataclr::W](dcrcdataclr::W) writer structure"]
impl crate::Writable for DCRCDATACLR {}
#[doc = "DCRCDATACLR register"]
pub mod dcrcdataclr;
#[doc = "DCRCDATASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcdataset](dcrcdataset) module"]
pub type DCRCDATASET = crate::Reg<u32, _DCRCDATASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCDATASET;
#[doc = "`read()` method returns [dcrcdataset::R](dcrcdataset::R) reader structure"]
impl crate::Readable for DCRCDATASET {}
#[doc = "`write(|w| ..)` method takes [dcrcdataset::W](dcrcdataset::W) writer structure"]
impl crate::Writable for DCRCDATASET {}
#[doc = "DCRCDATASET register"]
pub mod dcrcdataset;
#[doc = "DCRCDATAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcdatainv](dcrcdatainv) module"]
pub type DCRCDATAINV = crate::Reg<u32, _DCRCDATAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCDATAINV;
#[doc = "`read()` method returns [dcrcdatainv::R](dcrcdatainv::R) reader structure"]
impl crate::Readable for DCRCDATAINV {}
#[doc = "`write(|w| ..)` method takes [dcrcdatainv::W](dcrcdatainv::W) writer structure"]
impl crate::Writable for DCRCDATAINV {}
#[doc = "DCRCDATAINV register"]
pub mod dcrcdatainv;
#[doc = "DCRCXOR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcxor](dcrcxor) module"]
pub type DCRCXOR = crate::Reg<u32, _DCRCXOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCXOR;
#[doc = "`read()` method returns [dcrcxor::R](dcrcxor::R) reader structure"]
impl crate::Readable for DCRCXOR {}
#[doc = "`write(|w| ..)` method takes [dcrcxor::W](dcrcxor::W) writer structure"]
impl crate::Writable for DCRCXOR {}
#[doc = "DCRCXOR register"]
pub mod dcrcxor;
#[doc = "DCRCXORCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcxorclr](dcrcxorclr) module"]
pub type DCRCXORCLR = crate::Reg<u32, _DCRCXORCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCXORCLR;
#[doc = "`read()` method returns [dcrcxorclr::R](dcrcxorclr::R) reader structure"]
impl crate::Readable for DCRCXORCLR {}
#[doc = "`write(|w| ..)` method takes [dcrcxorclr::W](dcrcxorclr::W) writer structure"]
impl crate::Writable for DCRCXORCLR {}
#[doc = "DCRCXORCLR register"]
pub mod dcrcxorclr;
#[doc = "DCRCXORSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcxorset](dcrcxorset) module"]
pub type DCRCXORSET = crate::Reg<u32, _DCRCXORSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCXORSET;
#[doc = "`read()` method returns [dcrcxorset::R](dcrcxorset::R) reader structure"]
impl crate::Readable for DCRCXORSET {}
#[doc = "`write(|w| ..)` method takes [dcrcxorset::W](dcrcxorset::W) writer structure"]
impl crate::Writable for DCRCXORSET {}
#[doc = "DCRCXORSET register"]
pub mod dcrcxorset;
#[doc = "DCRCXORINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcxorinv](dcrcxorinv) module"]
pub type DCRCXORINV = crate::Reg<u32, _DCRCXORINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCRCXORINV;
#[doc = "`read()` method returns [dcrcxorinv::R](dcrcxorinv::R) reader structure"]
impl crate::Readable for DCRCXORINV {}
#[doc = "`write(|w| ..)` method takes [dcrcxorinv::W](dcrcxorinv::W) writer structure"]
impl crate::Writable for DCRCXORINV {}
#[doc = "DCRCXORINV register"]
pub mod dcrcxorinv;
