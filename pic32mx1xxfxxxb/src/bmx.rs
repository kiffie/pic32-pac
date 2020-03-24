#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BMXCON register"]
    pub bmxcon: BMXCON,
    #[doc = "0x04 - BMXCONCLR register"]
    pub bmxconclr: BMXCONCLR,
    #[doc = "0x08 - BMXCONSET register"]
    pub bmxconset: BMXCONSET,
    #[doc = "0x0c - BMXCONINV register"]
    pub bmxconinv: BMXCONINV,
    #[doc = "0x10 - BMXDKPBA register"]
    pub bmxdkpba: BMXDKPBA,
    #[doc = "0x14 - BMXDKPBACLR register"]
    pub bmxdkpbaclr: BMXDKPBACLR,
    #[doc = "0x18 - BMXDKPBASET register"]
    pub bmxdkpbaset: BMXDKPBASET,
    #[doc = "0x1c - BMXDKPBAINV register"]
    pub bmxdkpbainv: BMXDKPBAINV,
    #[doc = "0x20 - BMXDUDBA register"]
    pub bmxdudba: BMXDUDBA,
    #[doc = "0x24 - BMXDUDBACLR register"]
    pub bmxdudbaclr: BMXDUDBACLR,
    #[doc = "0x28 - BMXDUDBASET register"]
    pub bmxdudbaset: BMXDUDBASET,
    #[doc = "0x2c - BMXDUDBAINV register"]
    pub bmxdudbainv: BMXDUDBAINV,
    #[doc = "0x30 - BMXDUPBA register"]
    pub bmxdupba: BMXDUPBA,
    #[doc = "0x34 - BMXDUPBACLR register"]
    pub bmxdupbaclr: BMXDUPBACLR,
    #[doc = "0x38 - BMXDUPBASET register"]
    pub bmxdupbaset: BMXDUPBASET,
    #[doc = "0x3c - BMXDUPBAINV register"]
    pub bmxdupbainv: BMXDUPBAINV,
    #[doc = "0x40 - BMXDRMSZ register"]
    pub bmxdrmsz: BMXDRMSZ,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - BMXPUPBA register"]
    pub bmxpupba: BMXPUPBA,
    #[doc = "0x54 - BMXPUPBACLR register"]
    pub bmxpupbaclr: BMXPUPBACLR,
    #[doc = "0x58 - BMXPUPBASET register"]
    pub bmxpupbaset: BMXPUPBASET,
    #[doc = "0x5c - BMXPUPBAINV register"]
    pub bmxpupbainv: BMXPUPBAINV,
    #[doc = "0x60 - BMXPFMSZ register"]
    pub bmxpfmsz: BMXPFMSZ,
    _reserved22: [u8; 12usize],
    #[doc = "0x70 - BMXBOOTSZ register"]
    pub bmxbootsz: BMXBOOTSZ,
}
#[doc = "BMXCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxcon](bmxcon) module"]
pub type BMXCON = crate::Reg<u32, _BMXCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXCON;
#[doc = "`read()` method returns [bmxcon::R](bmxcon::R) reader structure"]
impl crate::Readable for BMXCON {}
#[doc = "`write(|w| ..)` method takes [bmxcon::W](bmxcon::W) writer structure"]
impl crate::Writable for BMXCON {}
#[doc = "BMXCON register"]
pub mod bmxcon;
#[doc = "BMXCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxconclr](bmxconclr) module"]
pub type BMXCONCLR = crate::Reg<u32, _BMXCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXCONCLR;
#[doc = "`read()` method returns [bmxconclr::R](bmxconclr::R) reader structure"]
impl crate::Readable for BMXCONCLR {}
#[doc = "`write(|w| ..)` method takes [bmxconclr::W](bmxconclr::W) writer structure"]
impl crate::Writable for BMXCONCLR {}
#[doc = "BMXCONCLR register"]
pub mod bmxconclr;
#[doc = "BMXCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxconset](bmxconset) module"]
pub type BMXCONSET = crate::Reg<u32, _BMXCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXCONSET;
#[doc = "`read()` method returns [bmxconset::R](bmxconset::R) reader structure"]
impl crate::Readable for BMXCONSET {}
#[doc = "`write(|w| ..)` method takes [bmxconset::W](bmxconset::W) writer structure"]
impl crate::Writable for BMXCONSET {}
#[doc = "BMXCONSET register"]
pub mod bmxconset;
#[doc = "BMXCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxconinv](bmxconinv) module"]
pub type BMXCONINV = crate::Reg<u32, _BMXCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXCONINV;
#[doc = "`read()` method returns [bmxconinv::R](bmxconinv::R) reader structure"]
impl crate::Readable for BMXCONINV {}
#[doc = "`write(|w| ..)` method takes [bmxconinv::W](bmxconinv::W) writer structure"]
impl crate::Writable for BMXCONINV {}
#[doc = "BMXCONINV register"]
pub mod bmxconinv;
#[doc = "BMXDKPBA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdkpba](bmxdkpba) module"]
pub type BMXDKPBA = crate::Reg<u32, _BMXDKPBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDKPBA;
#[doc = "`read()` method returns [bmxdkpba::R](bmxdkpba::R) reader structure"]
impl crate::Readable for BMXDKPBA {}
#[doc = "`write(|w| ..)` method takes [bmxdkpba::W](bmxdkpba::W) writer structure"]
impl crate::Writable for BMXDKPBA {}
#[doc = "BMXDKPBA register"]
pub mod bmxdkpba;
#[doc = "BMXDKPBACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdkpbaclr](bmxdkpbaclr) module"]
pub type BMXDKPBACLR = crate::Reg<u32, _BMXDKPBACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDKPBACLR;
#[doc = "`read()` method returns [bmxdkpbaclr::R](bmxdkpbaclr::R) reader structure"]
impl crate::Readable for BMXDKPBACLR {}
#[doc = "`write(|w| ..)` method takes [bmxdkpbaclr::W](bmxdkpbaclr::W) writer structure"]
impl crate::Writable for BMXDKPBACLR {}
#[doc = "BMXDKPBACLR register"]
pub mod bmxdkpbaclr;
#[doc = "BMXDKPBASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdkpbaset](bmxdkpbaset) module"]
pub type BMXDKPBASET = crate::Reg<u32, _BMXDKPBASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDKPBASET;
#[doc = "`read()` method returns [bmxdkpbaset::R](bmxdkpbaset::R) reader structure"]
impl crate::Readable for BMXDKPBASET {}
#[doc = "`write(|w| ..)` method takes [bmxdkpbaset::W](bmxdkpbaset::W) writer structure"]
impl crate::Writable for BMXDKPBASET {}
#[doc = "BMXDKPBASET register"]
pub mod bmxdkpbaset;
#[doc = "BMXDKPBAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdkpbainv](bmxdkpbainv) module"]
pub type BMXDKPBAINV = crate::Reg<u32, _BMXDKPBAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDKPBAINV;
#[doc = "`read()` method returns [bmxdkpbainv::R](bmxdkpbainv::R) reader structure"]
impl crate::Readable for BMXDKPBAINV {}
#[doc = "`write(|w| ..)` method takes [bmxdkpbainv::W](bmxdkpbainv::W) writer structure"]
impl crate::Writable for BMXDKPBAINV {}
#[doc = "BMXDKPBAINV register"]
pub mod bmxdkpbainv;
#[doc = "BMXDUDBA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdudba](bmxdudba) module"]
pub type BMXDUDBA = crate::Reg<u32, _BMXDUDBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDUDBA;
#[doc = "`read()` method returns [bmxdudba::R](bmxdudba::R) reader structure"]
impl crate::Readable for BMXDUDBA {}
#[doc = "`write(|w| ..)` method takes [bmxdudba::W](bmxdudba::W) writer structure"]
impl crate::Writable for BMXDUDBA {}
#[doc = "BMXDUDBA register"]
pub mod bmxdudba;
#[doc = "BMXDUDBACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdudbaclr](bmxdudbaclr) module"]
pub type BMXDUDBACLR = crate::Reg<u32, _BMXDUDBACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDUDBACLR;
#[doc = "`read()` method returns [bmxdudbaclr::R](bmxdudbaclr::R) reader structure"]
impl crate::Readable for BMXDUDBACLR {}
#[doc = "`write(|w| ..)` method takes [bmxdudbaclr::W](bmxdudbaclr::W) writer structure"]
impl crate::Writable for BMXDUDBACLR {}
#[doc = "BMXDUDBACLR register"]
pub mod bmxdudbaclr;
#[doc = "BMXDUDBASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdudbaset](bmxdudbaset) module"]
pub type BMXDUDBASET = crate::Reg<u32, _BMXDUDBASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDUDBASET;
#[doc = "`read()` method returns [bmxdudbaset::R](bmxdudbaset::R) reader structure"]
impl crate::Readable for BMXDUDBASET {}
#[doc = "`write(|w| ..)` method takes [bmxdudbaset::W](bmxdudbaset::W) writer structure"]
impl crate::Writable for BMXDUDBASET {}
#[doc = "BMXDUDBASET register"]
pub mod bmxdudbaset;
#[doc = "BMXDUDBAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdudbainv](bmxdudbainv) module"]
pub type BMXDUDBAINV = crate::Reg<u32, _BMXDUDBAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDUDBAINV;
#[doc = "`read()` method returns [bmxdudbainv::R](bmxdudbainv::R) reader structure"]
impl crate::Readable for BMXDUDBAINV {}
#[doc = "`write(|w| ..)` method takes [bmxdudbainv::W](bmxdudbainv::W) writer structure"]
impl crate::Writable for BMXDUDBAINV {}
#[doc = "BMXDUDBAINV register"]
pub mod bmxdudbainv;
#[doc = "BMXDUPBA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdupba](bmxdupba) module"]
pub type BMXDUPBA = crate::Reg<u32, _BMXDUPBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDUPBA;
#[doc = "`read()` method returns [bmxdupba::R](bmxdupba::R) reader structure"]
impl crate::Readable for BMXDUPBA {}
#[doc = "`write(|w| ..)` method takes [bmxdupba::W](bmxdupba::W) writer structure"]
impl crate::Writable for BMXDUPBA {}
#[doc = "BMXDUPBA register"]
pub mod bmxdupba;
#[doc = "BMXDUPBACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdupbaclr](bmxdupbaclr) module"]
pub type BMXDUPBACLR = crate::Reg<u32, _BMXDUPBACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDUPBACLR;
#[doc = "`read()` method returns [bmxdupbaclr::R](bmxdupbaclr::R) reader structure"]
impl crate::Readable for BMXDUPBACLR {}
#[doc = "`write(|w| ..)` method takes [bmxdupbaclr::W](bmxdupbaclr::W) writer structure"]
impl crate::Writable for BMXDUPBACLR {}
#[doc = "BMXDUPBACLR register"]
pub mod bmxdupbaclr;
#[doc = "BMXDUPBASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdupbaset](bmxdupbaset) module"]
pub type BMXDUPBASET = crate::Reg<u32, _BMXDUPBASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDUPBASET;
#[doc = "`read()` method returns [bmxdupbaset::R](bmxdupbaset::R) reader structure"]
impl crate::Readable for BMXDUPBASET {}
#[doc = "`write(|w| ..)` method takes [bmxdupbaset::W](bmxdupbaset::W) writer structure"]
impl crate::Writable for BMXDUPBASET {}
#[doc = "BMXDUPBASET register"]
pub mod bmxdupbaset;
#[doc = "BMXDUPBAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdupbainv](bmxdupbainv) module"]
pub type BMXDUPBAINV = crate::Reg<u32, _BMXDUPBAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDUPBAINV;
#[doc = "`read()` method returns [bmxdupbainv::R](bmxdupbainv::R) reader structure"]
impl crate::Readable for BMXDUPBAINV {}
#[doc = "`write(|w| ..)` method takes [bmxdupbainv::W](bmxdupbainv::W) writer structure"]
impl crate::Writable for BMXDUPBAINV {}
#[doc = "BMXDUPBAINV register"]
pub mod bmxdupbainv;
#[doc = "BMXDRMSZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdrmsz](bmxdrmsz) module"]
pub type BMXDRMSZ = crate::Reg<u32, _BMXDRMSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXDRMSZ;
#[doc = "`read()` method returns [bmxdrmsz::R](bmxdrmsz::R) reader structure"]
impl crate::Readable for BMXDRMSZ {}
#[doc = "`write(|w| ..)` method takes [bmxdrmsz::W](bmxdrmsz::W) writer structure"]
impl crate::Writable for BMXDRMSZ {}
#[doc = "BMXDRMSZ register"]
pub mod bmxdrmsz;
#[doc = "BMXPUPBA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxpupba](bmxpupba) module"]
pub type BMXPUPBA = crate::Reg<u32, _BMXPUPBA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXPUPBA;
#[doc = "`read()` method returns [bmxpupba::R](bmxpupba::R) reader structure"]
impl crate::Readable for BMXPUPBA {}
#[doc = "`write(|w| ..)` method takes [bmxpupba::W](bmxpupba::W) writer structure"]
impl crate::Writable for BMXPUPBA {}
#[doc = "BMXPUPBA register"]
pub mod bmxpupba;
#[doc = "BMXPUPBACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxpupbaclr](bmxpupbaclr) module"]
pub type BMXPUPBACLR = crate::Reg<u32, _BMXPUPBACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXPUPBACLR;
#[doc = "`read()` method returns [bmxpupbaclr::R](bmxpupbaclr::R) reader structure"]
impl crate::Readable for BMXPUPBACLR {}
#[doc = "`write(|w| ..)` method takes [bmxpupbaclr::W](bmxpupbaclr::W) writer structure"]
impl crate::Writable for BMXPUPBACLR {}
#[doc = "BMXPUPBACLR register"]
pub mod bmxpupbaclr;
#[doc = "BMXPUPBASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxpupbaset](bmxpupbaset) module"]
pub type BMXPUPBASET = crate::Reg<u32, _BMXPUPBASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXPUPBASET;
#[doc = "`read()` method returns [bmxpupbaset::R](bmxpupbaset::R) reader structure"]
impl crate::Readable for BMXPUPBASET {}
#[doc = "`write(|w| ..)` method takes [bmxpupbaset::W](bmxpupbaset::W) writer structure"]
impl crate::Writable for BMXPUPBASET {}
#[doc = "BMXPUPBASET register"]
pub mod bmxpupbaset;
#[doc = "BMXPUPBAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxpupbainv](bmxpupbainv) module"]
pub type BMXPUPBAINV = crate::Reg<u32, _BMXPUPBAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXPUPBAINV;
#[doc = "`read()` method returns [bmxpupbainv::R](bmxpupbainv::R) reader structure"]
impl crate::Readable for BMXPUPBAINV {}
#[doc = "`write(|w| ..)` method takes [bmxpupbainv::W](bmxpupbainv::W) writer structure"]
impl crate::Writable for BMXPUPBAINV {}
#[doc = "BMXPUPBAINV register"]
pub mod bmxpupbainv;
#[doc = "BMXPFMSZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxpfmsz](bmxpfmsz) module"]
pub type BMXPFMSZ = crate::Reg<u32, _BMXPFMSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXPFMSZ;
#[doc = "`read()` method returns [bmxpfmsz::R](bmxpfmsz::R) reader structure"]
impl crate::Readable for BMXPFMSZ {}
#[doc = "`write(|w| ..)` method takes [bmxpfmsz::W](bmxpfmsz::W) writer structure"]
impl crate::Writable for BMXPFMSZ {}
#[doc = "BMXPFMSZ register"]
pub mod bmxpfmsz;
#[doc = "BMXBOOTSZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxbootsz](bmxbootsz) module"]
pub type BMXBOOTSZ = crate::Reg<u32, _BMXBOOTSZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMXBOOTSZ;
#[doc = "`read()` method returns [bmxbootsz::R](bmxbootsz::R) reader structure"]
impl crate::Readable for BMXBOOTSZ {}
#[doc = "`write(|w| ..)` method takes [bmxbootsz::W](bmxbootsz::W) writer structure"]
impl crate::Writable for BMXBOOTSZ {}
#[doc = "BMXBOOTSZ register"]
pub mod bmxbootsz;
