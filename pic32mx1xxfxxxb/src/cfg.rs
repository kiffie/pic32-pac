#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CFGCON register"]
    pub cfgcon: CFGCON,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - DEVID register"]
    pub devid: DEVID,
    _reserved2: [u8; 12usize],
    #[doc = "0x30 - SYSKEY register"]
    pub syskey: SYSKEY,
    #[doc = "0x34 - SYSKEYCLR register"]
    pub syskeyclr: SYSKEYCLR,
    #[doc = "0x38 - SYSKEYSET register"]
    pub syskeyset: SYSKEYSET,
    #[doc = "0x3c - SYSKEYINV register"]
    pub syskeyinv: SYSKEYINV,
    #[doc = "0x40 - PMD1 register"]
    pub pmd1: PMD1,
    #[doc = "0x44 - PMD1CLR register"]
    pub pmd1clr: PMD1CLR,
    #[doc = "0x48 - PMD1SET register"]
    pub pmd1set: PMD1SET,
    #[doc = "0x4c - PMD1INV register"]
    pub pmd1inv: PMD1INV,
    #[doc = "0x50 - PMD2 register"]
    pub pmd2: PMD2,
    #[doc = "0x54 - PMD2CLR register"]
    pub pmd2clr: PMD2CLR,
    #[doc = "0x58 - PMD2SET register"]
    pub pmd2set: PMD2SET,
    #[doc = "0x5c - PMD2INV register"]
    pub pmd2inv: PMD2INV,
    #[doc = "0x60 - PMD3 register"]
    pub pmd3: PMD3,
    #[doc = "0x64 - PMD3CLR register"]
    pub pmd3clr: PMD3CLR,
    #[doc = "0x68 - PMD3SET register"]
    pub pmd3set: PMD3SET,
    #[doc = "0x6c - PMD3INV register"]
    pub pmd3inv: PMD3INV,
    #[doc = "0x70 - PMD4 register"]
    pub pmd4: PMD4,
    #[doc = "0x74 - PMD4CLR register"]
    pub pmd4clr: PMD4CLR,
    #[doc = "0x78 - PMD4SET register"]
    pub pmd4set: PMD4SET,
    #[doc = "0x7c - PMD4INV register"]
    pub pmd4inv: PMD4INV,
    #[doc = "0x80 - PMD5 register"]
    pub pmd5: PMD5,
    #[doc = "0x84 - PMD5CLR register"]
    pub pmd5clr: PMD5CLR,
    #[doc = "0x88 - PMD5SET register"]
    pub pmd5set: PMD5SET,
    #[doc = "0x8c - PMD5INV register"]
    pub pmd5inv: PMD5INV,
    #[doc = "0x90 - PMD6 register"]
    pub pmd6: PMD6,
    #[doc = "0x94 - PMD6CLR register"]
    pub pmd6clr: PMD6CLR,
    #[doc = "0x98 - PMD6SET register"]
    pub pmd6set: PMD6SET,
    #[doc = "0x9c - PMD6INV register"]
    pub pmd6inv: PMD6INV,
}
#[doc = "CFGCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgcon](cfgcon) module"]
pub type CFGCON = crate::Reg<u32, _CFGCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGCON;
#[doc = "`read()` method returns [cfgcon::R](cfgcon::R) reader structure"]
impl crate::Readable for CFGCON {}
#[doc = "`write(|w| ..)` method takes [cfgcon::W](cfgcon::W) writer structure"]
impl crate::Writable for CFGCON {}
#[doc = "CFGCON register"]
pub mod cfgcon;
#[doc = "DEVID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devid](devid) module"]
pub type DEVID = crate::Reg<u32, _DEVID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVID;
#[doc = "`read()` method returns [devid::R](devid::R) reader structure"]
impl crate::Readable for DEVID {}
#[doc = "`write(|w| ..)` method takes [devid::W](devid::W) writer structure"]
impl crate::Writable for DEVID {}
#[doc = "DEVID register"]
pub mod devid;
#[doc = "SYSKEY register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syskey](syskey) module"]
pub type SYSKEY = crate::Reg<u32, _SYSKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSKEY;
#[doc = "`read()` method returns [syskey::R](syskey::R) reader structure"]
impl crate::Readable for SYSKEY {}
#[doc = "`write(|w| ..)` method takes [syskey::W](syskey::W) writer structure"]
impl crate::Writable for SYSKEY {}
#[doc = "SYSKEY register"]
pub mod syskey;
#[doc = "SYSKEYCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syskeyclr](syskeyclr) module"]
pub type SYSKEYCLR = crate::Reg<u32, _SYSKEYCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSKEYCLR;
#[doc = "`read()` method returns [syskeyclr::R](syskeyclr::R) reader structure"]
impl crate::Readable for SYSKEYCLR {}
#[doc = "`write(|w| ..)` method takes [syskeyclr::W](syskeyclr::W) writer structure"]
impl crate::Writable for SYSKEYCLR {}
#[doc = "SYSKEYCLR register"]
pub mod syskeyclr;
#[doc = "SYSKEYSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syskeyset](syskeyset) module"]
pub type SYSKEYSET = crate::Reg<u32, _SYSKEYSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSKEYSET;
#[doc = "`read()` method returns [syskeyset::R](syskeyset::R) reader structure"]
impl crate::Readable for SYSKEYSET {}
#[doc = "`write(|w| ..)` method takes [syskeyset::W](syskeyset::W) writer structure"]
impl crate::Writable for SYSKEYSET {}
#[doc = "SYSKEYSET register"]
pub mod syskeyset;
#[doc = "SYSKEYINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syskeyinv](syskeyinv) module"]
pub type SYSKEYINV = crate::Reg<u32, _SYSKEYINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSKEYINV;
#[doc = "`read()` method returns [syskeyinv::R](syskeyinv::R) reader structure"]
impl crate::Readable for SYSKEYINV {}
#[doc = "`write(|w| ..)` method takes [syskeyinv::W](syskeyinv::W) writer structure"]
impl crate::Writable for SYSKEYINV {}
#[doc = "SYSKEYINV register"]
pub mod syskeyinv;
#[doc = "PMD1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd1](pmd1) module"]
pub type PMD1 = crate::Reg<u32, _PMD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD1;
#[doc = "`read()` method returns [pmd1::R](pmd1::R) reader structure"]
impl crate::Readable for PMD1 {}
#[doc = "`write(|w| ..)` method takes [pmd1::W](pmd1::W) writer structure"]
impl crate::Writable for PMD1 {}
#[doc = "PMD1 register"]
pub mod pmd1;
#[doc = "PMD1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd1clr](pmd1clr) module"]
pub type PMD1CLR = crate::Reg<u32, _PMD1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD1CLR;
#[doc = "`read()` method returns [pmd1clr::R](pmd1clr::R) reader structure"]
impl crate::Readable for PMD1CLR {}
#[doc = "`write(|w| ..)` method takes [pmd1clr::W](pmd1clr::W) writer structure"]
impl crate::Writable for PMD1CLR {}
#[doc = "PMD1CLR register"]
pub mod pmd1clr;
#[doc = "PMD1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd1set](pmd1set) module"]
pub type PMD1SET = crate::Reg<u32, _PMD1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD1SET;
#[doc = "`read()` method returns [pmd1set::R](pmd1set::R) reader structure"]
impl crate::Readable for PMD1SET {}
#[doc = "`write(|w| ..)` method takes [pmd1set::W](pmd1set::W) writer structure"]
impl crate::Writable for PMD1SET {}
#[doc = "PMD1SET register"]
pub mod pmd1set;
#[doc = "PMD1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd1inv](pmd1inv) module"]
pub type PMD1INV = crate::Reg<u32, _PMD1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD1INV;
#[doc = "`read()` method returns [pmd1inv::R](pmd1inv::R) reader structure"]
impl crate::Readable for PMD1INV {}
#[doc = "`write(|w| ..)` method takes [pmd1inv::W](pmd1inv::W) writer structure"]
impl crate::Writable for PMD1INV {}
#[doc = "PMD1INV register"]
pub mod pmd1inv;
#[doc = "PMD2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd2](pmd2) module"]
pub type PMD2 = crate::Reg<u32, _PMD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD2;
#[doc = "`read()` method returns [pmd2::R](pmd2::R) reader structure"]
impl crate::Readable for PMD2 {}
#[doc = "`write(|w| ..)` method takes [pmd2::W](pmd2::W) writer structure"]
impl crate::Writable for PMD2 {}
#[doc = "PMD2 register"]
pub mod pmd2;
#[doc = "PMD2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd2clr](pmd2clr) module"]
pub type PMD2CLR = crate::Reg<u32, _PMD2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD2CLR;
#[doc = "`read()` method returns [pmd2clr::R](pmd2clr::R) reader structure"]
impl crate::Readable for PMD2CLR {}
#[doc = "`write(|w| ..)` method takes [pmd2clr::W](pmd2clr::W) writer structure"]
impl crate::Writable for PMD2CLR {}
#[doc = "PMD2CLR register"]
pub mod pmd2clr;
#[doc = "PMD2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd2set](pmd2set) module"]
pub type PMD2SET = crate::Reg<u32, _PMD2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD2SET;
#[doc = "`read()` method returns [pmd2set::R](pmd2set::R) reader structure"]
impl crate::Readable for PMD2SET {}
#[doc = "`write(|w| ..)` method takes [pmd2set::W](pmd2set::W) writer structure"]
impl crate::Writable for PMD2SET {}
#[doc = "PMD2SET register"]
pub mod pmd2set;
#[doc = "PMD2INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd2inv](pmd2inv) module"]
pub type PMD2INV = crate::Reg<u32, _PMD2INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD2INV;
#[doc = "`read()` method returns [pmd2inv::R](pmd2inv::R) reader structure"]
impl crate::Readable for PMD2INV {}
#[doc = "`write(|w| ..)` method takes [pmd2inv::W](pmd2inv::W) writer structure"]
impl crate::Writable for PMD2INV {}
#[doc = "PMD2INV register"]
pub mod pmd2inv;
#[doc = "PMD3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd3](pmd3) module"]
pub type PMD3 = crate::Reg<u32, _PMD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD3;
#[doc = "`read()` method returns [pmd3::R](pmd3::R) reader structure"]
impl crate::Readable for PMD3 {}
#[doc = "`write(|w| ..)` method takes [pmd3::W](pmd3::W) writer structure"]
impl crate::Writable for PMD3 {}
#[doc = "PMD3 register"]
pub mod pmd3;
#[doc = "PMD3CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd3clr](pmd3clr) module"]
pub type PMD3CLR = crate::Reg<u32, _PMD3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD3CLR;
#[doc = "`read()` method returns [pmd3clr::R](pmd3clr::R) reader structure"]
impl crate::Readable for PMD3CLR {}
#[doc = "`write(|w| ..)` method takes [pmd3clr::W](pmd3clr::W) writer structure"]
impl crate::Writable for PMD3CLR {}
#[doc = "PMD3CLR register"]
pub mod pmd3clr;
#[doc = "PMD3SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd3set](pmd3set) module"]
pub type PMD3SET = crate::Reg<u32, _PMD3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD3SET;
#[doc = "`read()` method returns [pmd3set::R](pmd3set::R) reader structure"]
impl crate::Readable for PMD3SET {}
#[doc = "`write(|w| ..)` method takes [pmd3set::W](pmd3set::W) writer structure"]
impl crate::Writable for PMD3SET {}
#[doc = "PMD3SET register"]
pub mod pmd3set;
#[doc = "PMD3INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd3inv](pmd3inv) module"]
pub type PMD3INV = crate::Reg<u32, _PMD3INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD3INV;
#[doc = "`read()` method returns [pmd3inv::R](pmd3inv::R) reader structure"]
impl crate::Readable for PMD3INV {}
#[doc = "`write(|w| ..)` method takes [pmd3inv::W](pmd3inv::W) writer structure"]
impl crate::Writable for PMD3INV {}
#[doc = "PMD3INV register"]
pub mod pmd3inv;
#[doc = "PMD4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd4](pmd4) module"]
pub type PMD4 = crate::Reg<u32, _PMD4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD4;
#[doc = "`read()` method returns [pmd4::R](pmd4::R) reader structure"]
impl crate::Readable for PMD4 {}
#[doc = "`write(|w| ..)` method takes [pmd4::W](pmd4::W) writer structure"]
impl crate::Writable for PMD4 {}
#[doc = "PMD4 register"]
pub mod pmd4;
#[doc = "PMD4CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd4clr](pmd4clr) module"]
pub type PMD4CLR = crate::Reg<u32, _PMD4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD4CLR;
#[doc = "`read()` method returns [pmd4clr::R](pmd4clr::R) reader structure"]
impl crate::Readable for PMD4CLR {}
#[doc = "`write(|w| ..)` method takes [pmd4clr::W](pmd4clr::W) writer structure"]
impl crate::Writable for PMD4CLR {}
#[doc = "PMD4CLR register"]
pub mod pmd4clr;
#[doc = "PMD4SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd4set](pmd4set) module"]
pub type PMD4SET = crate::Reg<u32, _PMD4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD4SET;
#[doc = "`read()` method returns [pmd4set::R](pmd4set::R) reader structure"]
impl crate::Readable for PMD4SET {}
#[doc = "`write(|w| ..)` method takes [pmd4set::W](pmd4set::W) writer structure"]
impl crate::Writable for PMD4SET {}
#[doc = "PMD4SET register"]
pub mod pmd4set;
#[doc = "PMD4INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd4inv](pmd4inv) module"]
pub type PMD4INV = crate::Reg<u32, _PMD4INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD4INV;
#[doc = "`read()` method returns [pmd4inv::R](pmd4inv::R) reader structure"]
impl crate::Readable for PMD4INV {}
#[doc = "`write(|w| ..)` method takes [pmd4inv::W](pmd4inv::W) writer structure"]
impl crate::Writable for PMD4INV {}
#[doc = "PMD4INV register"]
pub mod pmd4inv;
#[doc = "PMD5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd5](pmd5) module"]
pub type PMD5 = crate::Reg<u32, _PMD5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD5;
#[doc = "`read()` method returns [pmd5::R](pmd5::R) reader structure"]
impl crate::Readable for PMD5 {}
#[doc = "`write(|w| ..)` method takes [pmd5::W](pmd5::W) writer structure"]
impl crate::Writable for PMD5 {}
#[doc = "PMD5 register"]
pub mod pmd5;
#[doc = "PMD5CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd5clr](pmd5clr) module"]
pub type PMD5CLR = crate::Reg<u32, _PMD5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD5CLR;
#[doc = "`read()` method returns [pmd5clr::R](pmd5clr::R) reader structure"]
impl crate::Readable for PMD5CLR {}
#[doc = "`write(|w| ..)` method takes [pmd5clr::W](pmd5clr::W) writer structure"]
impl crate::Writable for PMD5CLR {}
#[doc = "PMD5CLR register"]
pub mod pmd5clr;
#[doc = "PMD5SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd5set](pmd5set) module"]
pub type PMD5SET = crate::Reg<u32, _PMD5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD5SET;
#[doc = "`read()` method returns [pmd5set::R](pmd5set::R) reader structure"]
impl crate::Readable for PMD5SET {}
#[doc = "`write(|w| ..)` method takes [pmd5set::W](pmd5set::W) writer structure"]
impl crate::Writable for PMD5SET {}
#[doc = "PMD5SET register"]
pub mod pmd5set;
#[doc = "PMD5INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd5inv](pmd5inv) module"]
pub type PMD5INV = crate::Reg<u32, _PMD5INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD5INV;
#[doc = "`read()` method returns [pmd5inv::R](pmd5inv::R) reader structure"]
impl crate::Readable for PMD5INV {}
#[doc = "`write(|w| ..)` method takes [pmd5inv::W](pmd5inv::W) writer structure"]
impl crate::Writable for PMD5INV {}
#[doc = "PMD5INV register"]
pub mod pmd5inv;
#[doc = "PMD6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd6](pmd6) module"]
pub type PMD6 = crate::Reg<u32, _PMD6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD6;
#[doc = "`read()` method returns [pmd6::R](pmd6::R) reader structure"]
impl crate::Readable for PMD6 {}
#[doc = "`write(|w| ..)` method takes [pmd6::W](pmd6::W) writer structure"]
impl crate::Writable for PMD6 {}
#[doc = "PMD6 register"]
pub mod pmd6;
#[doc = "PMD6CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd6clr](pmd6clr) module"]
pub type PMD6CLR = crate::Reg<u32, _PMD6CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD6CLR;
#[doc = "`read()` method returns [pmd6clr::R](pmd6clr::R) reader structure"]
impl crate::Readable for PMD6CLR {}
#[doc = "`write(|w| ..)` method takes [pmd6clr::W](pmd6clr::W) writer structure"]
impl crate::Writable for PMD6CLR {}
#[doc = "PMD6CLR register"]
pub mod pmd6clr;
#[doc = "PMD6SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd6set](pmd6set) module"]
pub type PMD6SET = crate::Reg<u32, _PMD6SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD6SET;
#[doc = "`read()` method returns [pmd6set::R](pmd6set::R) reader structure"]
impl crate::Readable for PMD6SET {}
#[doc = "`write(|w| ..)` method takes [pmd6set::W](pmd6set::W) writer structure"]
impl crate::Writable for PMD6SET {}
#[doc = "PMD6SET register"]
pub mod pmd6set;
#[doc = "PMD6INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd6inv](pmd6inv) module"]
pub type PMD6INV = crate::Reg<u32, _PMD6INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMD6INV;
#[doc = "`read()` method returns [pmd6inv::R](pmd6inv::R) reader structure"]
impl crate::Readable for PMD6INV {}
#[doc = "`write(|w| ..)` method takes [pmd6inv::W](pmd6inv::W) writer structure"]
impl crate::Writable for PMD6INV {}
#[doc = "PMD6INV register"]
pub mod pmd6inv;
