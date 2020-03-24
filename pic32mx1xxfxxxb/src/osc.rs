#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSCCON register"]
    pub osccon: OSCCON,
    #[doc = "0x04 - OSCCONCLR register"]
    pub oscconclr: OSCCONCLR,
    #[doc = "0x08 - OSCCONSET register"]
    pub oscconset: OSCCONSET,
    #[doc = "0x0c - OSCCONINV register"]
    pub oscconinv: OSCCONINV,
    #[doc = "0x10 - OSCTUN register"]
    pub osctun: OSCTUN,
    #[doc = "0x14 - OSCTUNCLR register"]
    pub osctunclr: OSCTUNCLR,
    #[doc = "0x18 - OSCTUNSET register"]
    pub osctunset: OSCTUNSET,
    #[doc = "0x1c - OSCTUNINV register"]
    pub osctuninv: OSCTUNINV,
    #[doc = "0x20 - REFOCON register"]
    pub refocon: REFOCON,
    #[doc = "0x24 - REFOCONCLR register"]
    pub refoconclr: REFOCONCLR,
    #[doc = "0x28 - REFOCONSET register"]
    pub refoconset: REFOCONSET,
    #[doc = "0x2c - REFOCONINV register"]
    pub refoconinv: REFOCONINV,
    #[doc = "0x30 - REFOTRIM register"]
    pub refotrim: REFOTRIM,
    #[doc = "0x34 - REFOTRIMCLR register"]
    pub refotrimclr: REFOTRIMCLR,
    #[doc = "0x38 - REFOTRIMSET register"]
    pub refotrimset: REFOTRIMSET,
    #[doc = "0x3c - REFOTRIMINV register"]
    pub refotriminv: REFOTRIMINV,
}
#[doc = "OSCCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccon](osccon) module"]
pub type OSCCON = crate::Reg<u32, _OSCCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCON;
#[doc = "`read()` method returns [osccon::R](osccon::R) reader structure"]
impl crate::Readable for OSCCON {}
#[doc = "`write(|w| ..)` method takes [osccon::W](osccon::W) writer structure"]
impl crate::Writable for OSCCON {}
#[doc = "OSCCON register"]
pub mod osccon;
#[doc = "OSCCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscconclr](oscconclr) module"]
pub type OSCCONCLR = crate::Reg<u32, _OSCCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCONCLR;
#[doc = "`read()` method returns [oscconclr::R](oscconclr::R) reader structure"]
impl crate::Readable for OSCCONCLR {}
#[doc = "`write(|w| ..)` method takes [oscconclr::W](oscconclr::W) writer structure"]
impl crate::Writable for OSCCONCLR {}
#[doc = "OSCCONCLR register"]
pub mod oscconclr;
#[doc = "OSCCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscconset](oscconset) module"]
pub type OSCCONSET = crate::Reg<u32, _OSCCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCONSET;
#[doc = "`read()` method returns [oscconset::R](oscconset::R) reader structure"]
impl crate::Readable for OSCCONSET {}
#[doc = "`write(|w| ..)` method takes [oscconset::W](oscconset::W) writer structure"]
impl crate::Writable for OSCCONSET {}
#[doc = "OSCCONSET register"]
pub mod oscconset;
#[doc = "OSCCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscconinv](oscconinv) module"]
pub type OSCCONINV = crate::Reg<u32, _OSCCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCONINV;
#[doc = "`read()` method returns [oscconinv::R](oscconinv::R) reader structure"]
impl crate::Readable for OSCCONINV {}
#[doc = "`write(|w| ..)` method takes [oscconinv::W](oscconinv::W) writer structure"]
impl crate::Writable for OSCCONINV {}
#[doc = "OSCCONINV register"]
pub mod oscconinv;
#[doc = "OSCTUN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctun](osctun) module"]
pub type OSCTUN = crate::Reg<u32, _OSCTUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCTUN;
#[doc = "`read()` method returns [osctun::R](osctun::R) reader structure"]
impl crate::Readable for OSCTUN {}
#[doc = "`write(|w| ..)` method takes [osctun::W](osctun::W) writer structure"]
impl crate::Writable for OSCTUN {}
#[doc = "OSCTUN register"]
pub mod osctun;
#[doc = "OSCTUNCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctunclr](osctunclr) module"]
pub type OSCTUNCLR = crate::Reg<u32, _OSCTUNCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCTUNCLR;
#[doc = "`read()` method returns [osctunclr::R](osctunclr::R) reader structure"]
impl crate::Readable for OSCTUNCLR {}
#[doc = "`write(|w| ..)` method takes [osctunclr::W](osctunclr::W) writer structure"]
impl crate::Writable for OSCTUNCLR {}
#[doc = "OSCTUNCLR register"]
pub mod osctunclr;
#[doc = "OSCTUNSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctunset](osctunset) module"]
pub type OSCTUNSET = crate::Reg<u32, _OSCTUNSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCTUNSET;
#[doc = "`read()` method returns [osctunset::R](osctunset::R) reader structure"]
impl crate::Readable for OSCTUNSET {}
#[doc = "`write(|w| ..)` method takes [osctunset::W](osctunset::W) writer structure"]
impl crate::Writable for OSCTUNSET {}
#[doc = "OSCTUNSET register"]
pub mod osctunset;
#[doc = "OSCTUNINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctuninv](osctuninv) module"]
pub type OSCTUNINV = crate::Reg<u32, _OSCTUNINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCTUNINV;
#[doc = "`read()` method returns [osctuninv::R](osctuninv::R) reader structure"]
impl crate::Readable for OSCTUNINV {}
#[doc = "`write(|w| ..)` method takes [osctuninv::W](osctuninv::W) writer structure"]
impl crate::Writable for OSCTUNINV {}
#[doc = "OSCTUNINV register"]
pub mod osctuninv;
#[doc = "REFOCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refocon](refocon) module"]
pub type REFOCON = crate::Reg<u32, _REFOCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFOCON;
#[doc = "`read()` method returns [refocon::R](refocon::R) reader structure"]
impl crate::Readable for REFOCON {}
#[doc = "`write(|w| ..)` method takes [refocon::W](refocon::W) writer structure"]
impl crate::Writable for REFOCON {}
#[doc = "REFOCON register"]
pub mod refocon;
#[doc = "REFOCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refoconclr](refoconclr) module"]
pub type REFOCONCLR = crate::Reg<u32, _REFOCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFOCONCLR;
#[doc = "`read()` method returns [refoconclr::R](refoconclr::R) reader structure"]
impl crate::Readable for REFOCONCLR {}
#[doc = "`write(|w| ..)` method takes [refoconclr::W](refoconclr::W) writer structure"]
impl crate::Writable for REFOCONCLR {}
#[doc = "REFOCONCLR register"]
pub mod refoconclr;
#[doc = "REFOCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refoconset](refoconset) module"]
pub type REFOCONSET = crate::Reg<u32, _REFOCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFOCONSET;
#[doc = "`read()` method returns [refoconset::R](refoconset::R) reader structure"]
impl crate::Readable for REFOCONSET {}
#[doc = "`write(|w| ..)` method takes [refoconset::W](refoconset::W) writer structure"]
impl crate::Writable for REFOCONSET {}
#[doc = "REFOCONSET register"]
pub mod refoconset;
#[doc = "REFOCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refoconinv](refoconinv) module"]
pub type REFOCONINV = crate::Reg<u32, _REFOCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFOCONINV;
#[doc = "`read()` method returns [refoconinv::R](refoconinv::R) reader structure"]
impl crate::Readable for REFOCONINV {}
#[doc = "`write(|w| ..)` method takes [refoconinv::W](refoconinv::W) writer structure"]
impl crate::Writable for REFOCONINV {}
#[doc = "REFOCONINV register"]
pub mod refoconinv;
#[doc = "REFOTRIM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refotrim](refotrim) module"]
pub type REFOTRIM = crate::Reg<u32, _REFOTRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFOTRIM;
#[doc = "`read()` method returns [refotrim::R](refotrim::R) reader structure"]
impl crate::Readable for REFOTRIM {}
#[doc = "`write(|w| ..)` method takes [refotrim::W](refotrim::W) writer structure"]
impl crate::Writable for REFOTRIM {}
#[doc = "REFOTRIM register"]
pub mod refotrim;
#[doc = "REFOTRIMCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refotrimclr](refotrimclr) module"]
pub type REFOTRIMCLR = crate::Reg<u32, _REFOTRIMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFOTRIMCLR;
#[doc = "`read()` method returns [refotrimclr::R](refotrimclr::R) reader structure"]
impl crate::Readable for REFOTRIMCLR {}
#[doc = "`write(|w| ..)` method takes [refotrimclr::W](refotrimclr::W) writer structure"]
impl crate::Writable for REFOTRIMCLR {}
#[doc = "REFOTRIMCLR register"]
pub mod refotrimclr;
#[doc = "REFOTRIMSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refotrimset](refotrimset) module"]
pub type REFOTRIMSET = crate::Reg<u32, _REFOTRIMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFOTRIMSET;
#[doc = "`read()` method returns [refotrimset::R](refotrimset::R) reader structure"]
impl crate::Readable for REFOTRIMSET {}
#[doc = "`write(|w| ..)` method takes [refotrimset::W](refotrimset::W) writer structure"]
impl crate::Writable for REFOTRIMSET {}
#[doc = "REFOTRIMSET register"]
pub mod refotrimset;
#[doc = "REFOTRIMINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refotriminv](refotriminv) module"]
pub type REFOTRIMINV = crate::Reg<u32, _REFOTRIMINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFOTRIMINV;
#[doc = "`read()` method returns [refotriminv::R](refotriminv::R) reader structure"]
impl crate::Readable for REFOTRIMINV {}
#[doc = "`write(|w| ..)` method takes [refotriminv::W](refotriminv::W) writer structure"]
impl crate::Writable for REFOTRIMINV {}
#[doc = "REFOTRIMINV register"]
pub mod refotriminv;
