#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTCCON register"]
    pub rtccon: RTCCON,
    #[doc = "0x04 - RTCCONCLR register"]
    pub rtcconclr: RTCCONCLR,
    #[doc = "0x08 - RTCCONSET register"]
    pub rtcconset: RTCCONSET,
    #[doc = "0x0c - RTCCONINV register"]
    pub rtcconinv: RTCCONINV,
    #[doc = "0x10 - RTCALRM register"]
    pub rtcalrm: RTCALRM,
    #[doc = "0x14 - RTCALRMCLR register"]
    pub rtcalrmclr: RTCALRMCLR,
    #[doc = "0x18 - RTCALRMSET register"]
    pub rtcalrmset: RTCALRMSET,
    #[doc = "0x1c - RTCALRMINV register"]
    pub rtcalrminv: RTCALRMINV,
    #[doc = "0x20 - RTCTIME register"]
    pub rtctime: RTCTIME,
    #[doc = "0x24 - RTCTIMECLR register"]
    pub rtctimeclr: RTCTIMECLR,
    #[doc = "0x28 - RTCTIMESET register"]
    pub rtctimeset: RTCTIMESET,
    #[doc = "0x2c - RTCTIMEINV register"]
    pub rtctimeinv: RTCTIMEINV,
    #[doc = "0x30 - RTCDATE register"]
    pub rtcdate: RTCDATE,
    #[doc = "0x34 - RTCDATECLR register"]
    pub rtcdateclr: RTCDATECLR,
    #[doc = "0x38 - RTCDATESET register"]
    pub rtcdateset: RTCDATESET,
    #[doc = "0x3c - RTCDATEINV register"]
    pub rtcdateinv: RTCDATEINV,
    #[doc = "0x40 - ALRMTIME register"]
    pub alrmtime: ALRMTIME,
    #[doc = "0x44 - ALRMTIMECLR register"]
    pub alrmtimeclr: ALRMTIMECLR,
    #[doc = "0x48 - ALRMTIMESET register"]
    pub alrmtimeset: ALRMTIMESET,
    #[doc = "0x4c - ALRMTIMEINV register"]
    pub alrmtimeinv: ALRMTIMEINV,
    #[doc = "0x50 - ALRMDATE register"]
    pub alrmdate: ALRMDATE,
    #[doc = "0x54 - ALRMDATECLR register"]
    pub alrmdateclr: ALRMDATECLR,
    #[doc = "0x58 - ALRMDATESET register"]
    pub alrmdateset: ALRMDATESET,
    #[doc = "0x5c - ALRMDATEINV register"]
    pub alrmdateinv: ALRMDATEINV,
}
#[doc = "RTCCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccon](rtccon) module"]
pub type RTCCON = crate::Reg<u32, _RTCCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCON;
#[doc = "`read()` method returns [rtccon::R](rtccon::R) reader structure"]
impl crate::Readable for RTCCON {}
#[doc = "`write(|w| ..)` method takes [rtccon::W](rtccon::W) writer structure"]
impl crate::Writable for RTCCON {}
#[doc = "RTCCON register"]
pub mod rtccon;
#[doc = "RTCCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcconclr](rtcconclr) module"]
pub type RTCCONCLR = crate::Reg<u32, _RTCCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCONCLR;
#[doc = "`read()` method returns [rtcconclr::R](rtcconclr::R) reader structure"]
impl crate::Readable for RTCCONCLR {}
#[doc = "`write(|w| ..)` method takes [rtcconclr::W](rtcconclr::W) writer structure"]
impl crate::Writable for RTCCONCLR {}
#[doc = "RTCCONCLR register"]
pub mod rtcconclr;
#[doc = "RTCCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcconset](rtcconset) module"]
pub type RTCCONSET = crate::Reg<u32, _RTCCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCONSET;
#[doc = "`read()` method returns [rtcconset::R](rtcconset::R) reader structure"]
impl crate::Readable for RTCCONSET {}
#[doc = "`write(|w| ..)` method takes [rtcconset::W](rtcconset::W) writer structure"]
impl crate::Writable for RTCCONSET {}
#[doc = "RTCCONSET register"]
pub mod rtcconset;
#[doc = "RTCCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcconinv](rtcconinv) module"]
pub type RTCCONINV = crate::Reg<u32, _RTCCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCONINV;
#[doc = "`read()` method returns [rtcconinv::R](rtcconinv::R) reader structure"]
impl crate::Readable for RTCCONINV {}
#[doc = "`write(|w| ..)` method takes [rtcconinv::W](rtcconinv::W) writer structure"]
impl crate::Writable for RTCCONINV {}
#[doc = "RTCCONINV register"]
pub mod rtcconinv;
#[doc = "RTCALRM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcalrm](rtcalrm) module"]
pub type RTCALRM = crate::Reg<u32, _RTCALRM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCALRM;
#[doc = "`read()` method returns [rtcalrm::R](rtcalrm::R) reader structure"]
impl crate::Readable for RTCALRM {}
#[doc = "`write(|w| ..)` method takes [rtcalrm::W](rtcalrm::W) writer structure"]
impl crate::Writable for RTCALRM {}
#[doc = "RTCALRM register"]
pub mod rtcalrm;
#[doc = "RTCALRMCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcalrmclr](rtcalrmclr) module"]
pub type RTCALRMCLR = crate::Reg<u32, _RTCALRMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCALRMCLR;
#[doc = "`read()` method returns [rtcalrmclr::R](rtcalrmclr::R) reader structure"]
impl crate::Readable for RTCALRMCLR {}
#[doc = "`write(|w| ..)` method takes [rtcalrmclr::W](rtcalrmclr::W) writer structure"]
impl crate::Writable for RTCALRMCLR {}
#[doc = "RTCALRMCLR register"]
pub mod rtcalrmclr;
#[doc = "RTCALRMSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcalrmset](rtcalrmset) module"]
pub type RTCALRMSET = crate::Reg<u32, _RTCALRMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCALRMSET;
#[doc = "`read()` method returns [rtcalrmset::R](rtcalrmset::R) reader structure"]
impl crate::Readable for RTCALRMSET {}
#[doc = "`write(|w| ..)` method takes [rtcalrmset::W](rtcalrmset::W) writer structure"]
impl crate::Writable for RTCALRMSET {}
#[doc = "RTCALRMSET register"]
pub mod rtcalrmset;
#[doc = "RTCALRMINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcalrminv](rtcalrminv) module"]
pub type RTCALRMINV = crate::Reg<u32, _RTCALRMINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCALRMINV;
#[doc = "`read()` method returns [rtcalrminv::R](rtcalrminv::R) reader structure"]
impl crate::Readable for RTCALRMINV {}
#[doc = "`write(|w| ..)` method takes [rtcalrminv::W](rtcalrminv::W) writer structure"]
impl crate::Writable for RTCALRMINV {}
#[doc = "RTCALRMINV register"]
pub mod rtcalrminv;
#[doc = "RTCTIME register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctime](rtctime) module"]
pub type RTCTIME = crate::Reg<u32, _RTCTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIME;
#[doc = "`read()` method returns [rtctime::R](rtctime::R) reader structure"]
impl crate::Readable for RTCTIME {}
#[doc = "`write(|w| ..)` method takes [rtctime::W](rtctime::W) writer structure"]
impl crate::Writable for RTCTIME {}
#[doc = "RTCTIME register"]
pub mod rtctime;
#[doc = "RTCTIMECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctimeclr](rtctimeclr) module"]
pub type RTCTIMECLR = crate::Reg<u32, _RTCTIMECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIMECLR;
#[doc = "`read()` method returns [rtctimeclr::R](rtctimeclr::R) reader structure"]
impl crate::Readable for RTCTIMECLR {}
#[doc = "`write(|w| ..)` method takes [rtctimeclr::W](rtctimeclr::W) writer structure"]
impl crate::Writable for RTCTIMECLR {}
#[doc = "RTCTIMECLR register"]
pub mod rtctimeclr;
#[doc = "RTCTIMESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctimeset](rtctimeset) module"]
pub type RTCTIMESET = crate::Reg<u32, _RTCTIMESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIMESET;
#[doc = "`read()` method returns [rtctimeset::R](rtctimeset::R) reader structure"]
impl crate::Readable for RTCTIMESET {}
#[doc = "`write(|w| ..)` method takes [rtctimeset::W](rtctimeset::W) writer structure"]
impl crate::Writable for RTCTIMESET {}
#[doc = "RTCTIMESET register"]
pub mod rtctimeset;
#[doc = "RTCTIMEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctimeinv](rtctimeinv) module"]
pub type RTCTIMEINV = crate::Reg<u32, _RTCTIMEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCTIMEINV;
#[doc = "`read()` method returns [rtctimeinv::R](rtctimeinv::R) reader structure"]
impl crate::Readable for RTCTIMEINV {}
#[doc = "`write(|w| ..)` method takes [rtctimeinv::W](rtctimeinv::W) writer structure"]
impl crate::Writable for RTCTIMEINV {}
#[doc = "RTCTIMEINV register"]
pub mod rtctimeinv;
#[doc = "RTCDATE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdate](rtcdate) module"]
pub type RTCDATE = crate::Reg<u32, _RTCDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDATE;
#[doc = "`read()` method returns [rtcdate::R](rtcdate::R) reader structure"]
impl crate::Readable for RTCDATE {}
#[doc = "`write(|w| ..)` method takes [rtcdate::W](rtcdate::W) writer structure"]
impl crate::Writable for RTCDATE {}
#[doc = "RTCDATE register"]
pub mod rtcdate;
#[doc = "RTCDATECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdateclr](rtcdateclr) module"]
pub type RTCDATECLR = crate::Reg<u32, _RTCDATECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDATECLR;
#[doc = "`read()` method returns [rtcdateclr::R](rtcdateclr::R) reader structure"]
impl crate::Readable for RTCDATECLR {}
#[doc = "`write(|w| ..)` method takes [rtcdateclr::W](rtcdateclr::W) writer structure"]
impl crate::Writable for RTCDATECLR {}
#[doc = "RTCDATECLR register"]
pub mod rtcdateclr;
#[doc = "RTCDATESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdateset](rtcdateset) module"]
pub type RTCDATESET = crate::Reg<u32, _RTCDATESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDATESET;
#[doc = "`read()` method returns [rtcdateset::R](rtcdateset::R) reader structure"]
impl crate::Readable for RTCDATESET {}
#[doc = "`write(|w| ..)` method takes [rtcdateset::W](rtcdateset::W) writer structure"]
impl crate::Writable for RTCDATESET {}
#[doc = "RTCDATESET register"]
pub mod rtcdateset;
#[doc = "RTCDATEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdateinv](rtcdateinv) module"]
pub type RTCDATEINV = crate::Reg<u32, _RTCDATEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCDATEINV;
#[doc = "`read()` method returns [rtcdateinv::R](rtcdateinv::R) reader structure"]
impl crate::Readable for RTCDATEINV {}
#[doc = "`write(|w| ..)` method takes [rtcdateinv::W](rtcdateinv::W) writer structure"]
impl crate::Writable for RTCDATEINV {}
#[doc = "RTCDATEINV register"]
pub mod rtcdateinv;
#[doc = "ALRMTIME register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmtime](alrmtime) module"]
pub type ALRMTIME = crate::Reg<u32, _ALRMTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMTIME;
#[doc = "`read()` method returns [alrmtime::R](alrmtime::R) reader structure"]
impl crate::Readable for ALRMTIME {}
#[doc = "`write(|w| ..)` method takes [alrmtime::W](alrmtime::W) writer structure"]
impl crate::Writable for ALRMTIME {}
#[doc = "ALRMTIME register"]
pub mod alrmtime;
#[doc = "ALRMTIMECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmtimeclr](alrmtimeclr) module"]
pub type ALRMTIMECLR = crate::Reg<u32, _ALRMTIMECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMTIMECLR;
#[doc = "`read()` method returns [alrmtimeclr::R](alrmtimeclr::R) reader structure"]
impl crate::Readable for ALRMTIMECLR {}
#[doc = "`write(|w| ..)` method takes [alrmtimeclr::W](alrmtimeclr::W) writer structure"]
impl crate::Writable for ALRMTIMECLR {}
#[doc = "ALRMTIMECLR register"]
pub mod alrmtimeclr;
#[doc = "ALRMTIMESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmtimeset](alrmtimeset) module"]
pub type ALRMTIMESET = crate::Reg<u32, _ALRMTIMESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMTIMESET;
#[doc = "`read()` method returns [alrmtimeset::R](alrmtimeset::R) reader structure"]
impl crate::Readable for ALRMTIMESET {}
#[doc = "`write(|w| ..)` method takes [alrmtimeset::W](alrmtimeset::W) writer structure"]
impl crate::Writable for ALRMTIMESET {}
#[doc = "ALRMTIMESET register"]
pub mod alrmtimeset;
#[doc = "ALRMTIMEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmtimeinv](alrmtimeinv) module"]
pub type ALRMTIMEINV = crate::Reg<u32, _ALRMTIMEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMTIMEINV;
#[doc = "`read()` method returns [alrmtimeinv::R](alrmtimeinv::R) reader structure"]
impl crate::Readable for ALRMTIMEINV {}
#[doc = "`write(|w| ..)` method takes [alrmtimeinv::W](alrmtimeinv::W) writer structure"]
impl crate::Writable for ALRMTIMEINV {}
#[doc = "ALRMTIMEINV register"]
pub mod alrmtimeinv;
#[doc = "ALRMDATE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmdate](alrmdate) module"]
pub type ALRMDATE = crate::Reg<u32, _ALRMDATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMDATE;
#[doc = "`read()` method returns [alrmdate::R](alrmdate::R) reader structure"]
impl crate::Readable for ALRMDATE {}
#[doc = "`write(|w| ..)` method takes [alrmdate::W](alrmdate::W) writer structure"]
impl crate::Writable for ALRMDATE {}
#[doc = "ALRMDATE register"]
pub mod alrmdate;
#[doc = "ALRMDATECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmdateclr](alrmdateclr) module"]
pub type ALRMDATECLR = crate::Reg<u32, _ALRMDATECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMDATECLR;
#[doc = "`read()` method returns [alrmdateclr::R](alrmdateclr::R) reader structure"]
impl crate::Readable for ALRMDATECLR {}
#[doc = "`write(|w| ..)` method takes [alrmdateclr::W](alrmdateclr::W) writer structure"]
impl crate::Writable for ALRMDATECLR {}
#[doc = "ALRMDATECLR register"]
pub mod alrmdateclr;
#[doc = "ALRMDATESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmdateset](alrmdateset) module"]
pub type ALRMDATESET = crate::Reg<u32, _ALRMDATESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMDATESET;
#[doc = "`read()` method returns [alrmdateset::R](alrmdateset::R) reader structure"]
impl crate::Readable for ALRMDATESET {}
#[doc = "`write(|w| ..)` method takes [alrmdateset::W](alrmdateset::W) writer structure"]
impl crate::Writable for ALRMDATESET {}
#[doc = "ALRMDATESET register"]
pub mod alrmdateset;
#[doc = "ALRMDATEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmdateinv](alrmdateinv) module"]
pub type ALRMDATEINV = crate::Reg<u32, _ALRMDATEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRMDATEINV;
#[doc = "`read()` method returns [alrmdateinv::R](alrmdateinv::R) reader structure"]
impl crate::Readable for ALRMDATEINV {}
#[doc = "`write(|w| ..)` method takes [alrmdateinv::W](alrmdateinv::W) writer structure"]
impl crate::Writable for ALRMDATEINV {}
#[doc = "ALRMDATEINV register"]
pub mod alrmdateinv;
