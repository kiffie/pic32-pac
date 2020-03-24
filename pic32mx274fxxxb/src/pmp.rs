#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMCON register"]
    pub pmcon: PMCON,
    #[doc = "0x04 - PMCONCLR register"]
    pub pmconclr: PMCONCLR,
    #[doc = "0x08 - PMCONSET register"]
    pub pmconset: PMCONSET,
    #[doc = "0x0c - PMCONINV register"]
    pub pmconinv: PMCONINV,
    #[doc = "0x10 - PMMODE register"]
    pub pmmode: PMMODE,
    #[doc = "0x14 - PMMODECLR register"]
    pub pmmodeclr: PMMODECLR,
    #[doc = "0x18 - PMMODESET register"]
    pub pmmodeset: PMMODESET,
    #[doc = "0x1c - PMMODEINV register"]
    pub pmmodeinv: PMMODEINV,
    #[doc = "0x20 - PMADDR register"]
    pub pmaddr: PMADDR,
    #[doc = "0x24 - PMADDRCLR register"]
    pub pmaddrclr: PMADDRCLR,
    #[doc = "0x28 - PMADDRSET register"]
    pub pmaddrset: PMADDRSET,
    #[doc = "0x2c - PMADDRINV register"]
    pub pmaddrinv: PMADDRINV,
    #[doc = "0x30 - PMDOUT register"]
    pub pmdout: PMDOUT,
    #[doc = "0x34 - PMDOUTCLR register"]
    pub pmdoutclr: PMDOUTCLR,
    #[doc = "0x38 - PMDOUTSET register"]
    pub pmdoutset: PMDOUTSET,
    #[doc = "0x3c - PMDOUTINV register"]
    pub pmdoutinv: PMDOUTINV,
    #[doc = "0x40 - PMDIN register"]
    pub pmdin: PMDIN,
    #[doc = "0x44 - PMDINCLR register"]
    pub pmdinclr: PMDINCLR,
    #[doc = "0x48 - PMDINSET register"]
    pub pmdinset: PMDINSET,
    #[doc = "0x4c - PMDININV register"]
    pub pmdininv: PMDININV,
    #[doc = "0x50 - PMAEN register"]
    pub pmaen: PMAEN,
    #[doc = "0x54 - PMAENCLR register"]
    pub pmaenclr: PMAENCLR,
    #[doc = "0x58 - PMAENSET register"]
    pub pmaenset: PMAENSET,
    #[doc = "0x5c - PMAENINV register"]
    pub pmaeninv: PMAENINV,
    #[doc = "0x60 - PMSTAT register"]
    pub pmstat: PMSTAT,
    #[doc = "0x64 - PMSTATCLR register"]
    pub pmstatclr: PMSTATCLR,
    #[doc = "0x68 - PMSTATSET register"]
    pub pmstatset: PMSTATSET,
    #[doc = "0x6c - PMSTATINV register"]
    pub pmstatinv: PMSTATINV,
    #[doc = "0x70 - PMWADDR register"]
    pub pmwaddr: PMWADDR,
    #[doc = "0x74 - PMWADDRCLR register"]
    pub pmwaddrclr: PMWADDRCLR,
    #[doc = "0x78 - PMWADDRSET register"]
    pub pmwaddrset: PMWADDRSET,
    #[doc = "0x7c - PMWADDRINV register"]
    pub pmwaddrinv: PMWADDRINV,
    #[doc = "0x80 - PMRADDR register"]
    pub pmraddr: PMRADDR,
    #[doc = "0x84 - PMRADDRCLR register"]
    pub pmraddrclr: PMRADDRCLR,
    #[doc = "0x88 - PMRADDRSET register"]
    pub pmraddrset: PMRADDRSET,
    #[doc = "0x8c - PMRADDRINV register"]
    pub pmraddrinv: PMRADDRINV,
    #[doc = "0x90 - PMRDIN register"]
    pub pmrdin: PMRDIN,
    #[doc = "0x94 - PMRDINCLR register"]
    pub pmrdinclr: PMRDINCLR,
    #[doc = "0x98 - PMRDINSET register"]
    pub pmrdinset: PMRDINSET,
    #[doc = "0x9c - PMRDININV register"]
    pub pmrdininv: PMRDININV,
}
#[doc = "PMCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcon](pmcon) module"]
pub type PMCON = crate::Reg<u32, _PMCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCON;
#[doc = "`read()` method returns [pmcon::R](pmcon::R) reader structure"]
impl crate::Readable for PMCON {}
#[doc = "`write(|w| ..)` method takes [pmcon::W](pmcon::W) writer structure"]
impl crate::Writable for PMCON {}
#[doc = "PMCON register"]
pub mod pmcon;
#[doc = "PMCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmconclr](pmconclr) module"]
pub type PMCONCLR = crate::Reg<u32, _PMCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCONCLR;
#[doc = "`read()` method returns [pmconclr::R](pmconclr::R) reader structure"]
impl crate::Readable for PMCONCLR {}
#[doc = "`write(|w| ..)` method takes [pmconclr::W](pmconclr::W) writer structure"]
impl crate::Writable for PMCONCLR {}
#[doc = "PMCONCLR register"]
pub mod pmconclr;
#[doc = "PMCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmconset](pmconset) module"]
pub type PMCONSET = crate::Reg<u32, _PMCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCONSET;
#[doc = "`read()` method returns [pmconset::R](pmconset::R) reader structure"]
impl crate::Readable for PMCONSET {}
#[doc = "`write(|w| ..)` method takes [pmconset::W](pmconset::W) writer structure"]
impl crate::Writable for PMCONSET {}
#[doc = "PMCONSET register"]
pub mod pmconset;
#[doc = "PMCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmconinv](pmconinv) module"]
pub type PMCONINV = crate::Reg<u32, _PMCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMCONINV;
#[doc = "`read()` method returns [pmconinv::R](pmconinv::R) reader structure"]
impl crate::Readable for PMCONINV {}
#[doc = "`write(|w| ..)` method takes [pmconinv::W](pmconinv::W) writer structure"]
impl crate::Writable for PMCONINV {}
#[doc = "PMCONINV register"]
pub mod pmconinv;
#[doc = "PMMODE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmode](pmmode) module"]
pub type PMMODE = crate::Reg<u32, _PMMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMODE;
#[doc = "`read()` method returns [pmmode::R](pmmode::R) reader structure"]
impl crate::Readable for PMMODE {}
#[doc = "`write(|w| ..)` method takes [pmmode::W](pmmode::W) writer structure"]
impl crate::Writable for PMMODE {}
#[doc = "PMMODE register"]
pub mod pmmode;
#[doc = "PMMODECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmodeclr](pmmodeclr) module"]
pub type PMMODECLR = crate::Reg<u32, _PMMODECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMODECLR;
#[doc = "`read()` method returns [pmmodeclr::R](pmmodeclr::R) reader structure"]
impl crate::Readable for PMMODECLR {}
#[doc = "`write(|w| ..)` method takes [pmmodeclr::W](pmmodeclr::W) writer structure"]
impl crate::Writable for PMMODECLR {}
#[doc = "PMMODECLR register"]
pub mod pmmodeclr;
#[doc = "PMMODESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmodeset](pmmodeset) module"]
pub type PMMODESET = crate::Reg<u32, _PMMODESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMODESET;
#[doc = "`read()` method returns [pmmodeset::R](pmmodeset::R) reader structure"]
impl crate::Readable for PMMODESET {}
#[doc = "`write(|w| ..)` method takes [pmmodeset::W](pmmodeset::W) writer structure"]
impl crate::Writable for PMMODESET {}
#[doc = "PMMODESET register"]
pub mod pmmodeset;
#[doc = "PMMODEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmodeinv](pmmodeinv) module"]
pub type PMMODEINV = crate::Reg<u32, _PMMODEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMMODEINV;
#[doc = "`read()` method returns [pmmodeinv::R](pmmodeinv::R) reader structure"]
impl crate::Readable for PMMODEINV {}
#[doc = "`write(|w| ..)` method takes [pmmodeinv::W](pmmodeinv::W) writer structure"]
impl crate::Writable for PMMODEINV {}
#[doc = "PMMODEINV register"]
pub mod pmmodeinv;
#[doc = "PMADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaddr](pmaddr) module"]
pub type PMADDR = crate::Reg<u32, _PMADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMADDR;
#[doc = "`read()` method returns [pmaddr::R](pmaddr::R) reader structure"]
impl crate::Readable for PMADDR {}
#[doc = "`write(|w| ..)` method takes [pmaddr::W](pmaddr::W) writer structure"]
impl crate::Writable for PMADDR {}
#[doc = "PMADDR register"]
pub mod pmaddr;
#[doc = "PMADDRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaddrclr](pmaddrclr) module"]
pub type PMADDRCLR = crate::Reg<u32, _PMADDRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMADDRCLR;
#[doc = "`read()` method returns [pmaddrclr::R](pmaddrclr::R) reader structure"]
impl crate::Readable for PMADDRCLR {}
#[doc = "`write(|w| ..)` method takes [pmaddrclr::W](pmaddrclr::W) writer structure"]
impl crate::Writable for PMADDRCLR {}
#[doc = "PMADDRCLR register"]
pub mod pmaddrclr;
#[doc = "PMADDRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaddrset](pmaddrset) module"]
pub type PMADDRSET = crate::Reg<u32, _PMADDRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMADDRSET;
#[doc = "`read()` method returns [pmaddrset::R](pmaddrset::R) reader structure"]
impl crate::Readable for PMADDRSET {}
#[doc = "`write(|w| ..)` method takes [pmaddrset::W](pmaddrset::W) writer structure"]
impl crate::Writable for PMADDRSET {}
#[doc = "PMADDRSET register"]
pub mod pmaddrset;
#[doc = "PMADDRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaddrinv](pmaddrinv) module"]
pub type PMADDRINV = crate::Reg<u32, _PMADDRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMADDRINV;
#[doc = "`read()` method returns [pmaddrinv::R](pmaddrinv::R) reader structure"]
impl crate::Readable for PMADDRINV {}
#[doc = "`write(|w| ..)` method takes [pmaddrinv::W](pmaddrinv::W) writer structure"]
impl crate::Writable for PMADDRINV {}
#[doc = "PMADDRINV register"]
pub mod pmaddrinv;
#[doc = "PMDOUT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdout](pmdout) module"]
pub type PMDOUT = crate::Reg<u32, _PMDOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMDOUT;
#[doc = "`read()` method returns [pmdout::R](pmdout::R) reader structure"]
impl crate::Readable for PMDOUT {}
#[doc = "`write(|w| ..)` method takes [pmdout::W](pmdout::W) writer structure"]
impl crate::Writable for PMDOUT {}
#[doc = "PMDOUT register"]
pub mod pmdout;
#[doc = "PMDOUTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdoutclr](pmdoutclr) module"]
pub type PMDOUTCLR = crate::Reg<u32, _PMDOUTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMDOUTCLR;
#[doc = "`read()` method returns [pmdoutclr::R](pmdoutclr::R) reader structure"]
impl crate::Readable for PMDOUTCLR {}
#[doc = "`write(|w| ..)` method takes [pmdoutclr::W](pmdoutclr::W) writer structure"]
impl crate::Writable for PMDOUTCLR {}
#[doc = "PMDOUTCLR register"]
pub mod pmdoutclr;
#[doc = "PMDOUTSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdoutset](pmdoutset) module"]
pub type PMDOUTSET = crate::Reg<u32, _PMDOUTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMDOUTSET;
#[doc = "`read()` method returns [pmdoutset::R](pmdoutset::R) reader structure"]
impl crate::Readable for PMDOUTSET {}
#[doc = "`write(|w| ..)` method takes [pmdoutset::W](pmdoutset::W) writer structure"]
impl crate::Writable for PMDOUTSET {}
#[doc = "PMDOUTSET register"]
pub mod pmdoutset;
#[doc = "PMDOUTINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdoutinv](pmdoutinv) module"]
pub type PMDOUTINV = crate::Reg<u32, _PMDOUTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMDOUTINV;
#[doc = "`read()` method returns [pmdoutinv::R](pmdoutinv::R) reader structure"]
impl crate::Readable for PMDOUTINV {}
#[doc = "`write(|w| ..)` method takes [pmdoutinv::W](pmdoutinv::W) writer structure"]
impl crate::Writable for PMDOUTINV {}
#[doc = "PMDOUTINV register"]
pub mod pmdoutinv;
#[doc = "PMDIN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdin](pmdin) module"]
pub type PMDIN = crate::Reg<u32, _PMDIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMDIN;
#[doc = "`read()` method returns [pmdin::R](pmdin::R) reader structure"]
impl crate::Readable for PMDIN {}
#[doc = "`write(|w| ..)` method takes [pmdin::W](pmdin::W) writer structure"]
impl crate::Writable for PMDIN {}
#[doc = "PMDIN register"]
pub mod pmdin;
#[doc = "PMDINCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdinclr](pmdinclr) module"]
pub type PMDINCLR = crate::Reg<u32, _PMDINCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMDINCLR;
#[doc = "`read()` method returns [pmdinclr::R](pmdinclr::R) reader structure"]
impl crate::Readable for PMDINCLR {}
#[doc = "`write(|w| ..)` method takes [pmdinclr::W](pmdinclr::W) writer structure"]
impl crate::Writable for PMDINCLR {}
#[doc = "PMDINCLR register"]
pub mod pmdinclr;
#[doc = "PMDINSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdinset](pmdinset) module"]
pub type PMDINSET = crate::Reg<u32, _PMDINSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMDINSET;
#[doc = "`read()` method returns [pmdinset::R](pmdinset::R) reader structure"]
impl crate::Readable for PMDINSET {}
#[doc = "`write(|w| ..)` method takes [pmdinset::W](pmdinset::W) writer structure"]
impl crate::Writable for PMDINSET {}
#[doc = "PMDINSET register"]
pub mod pmdinset;
#[doc = "PMDININV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdininv](pmdininv) module"]
pub type PMDININV = crate::Reg<u32, _PMDININV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMDININV;
#[doc = "`read()` method returns [pmdininv::R](pmdininv::R) reader structure"]
impl crate::Readable for PMDININV {}
#[doc = "`write(|w| ..)` method takes [pmdininv::W](pmdininv::W) writer structure"]
impl crate::Writable for PMDININV {}
#[doc = "PMDININV register"]
pub mod pmdininv;
#[doc = "PMAEN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaen](pmaen) module"]
pub type PMAEN = crate::Reg<u32, _PMAEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMAEN;
#[doc = "`read()` method returns [pmaen::R](pmaen::R) reader structure"]
impl crate::Readable for PMAEN {}
#[doc = "`write(|w| ..)` method takes [pmaen::W](pmaen::W) writer structure"]
impl crate::Writable for PMAEN {}
#[doc = "PMAEN register"]
pub mod pmaen;
#[doc = "PMAENCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaenclr](pmaenclr) module"]
pub type PMAENCLR = crate::Reg<u32, _PMAENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMAENCLR;
#[doc = "`read()` method returns [pmaenclr::R](pmaenclr::R) reader structure"]
impl crate::Readable for PMAENCLR {}
#[doc = "`write(|w| ..)` method takes [pmaenclr::W](pmaenclr::W) writer structure"]
impl crate::Writable for PMAENCLR {}
#[doc = "PMAENCLR register"]
pub mod pmaenclr;
#[doc = "PMAENSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaenset](pmaenset) module"]
pub type PMAENSET = crate::Reg<u32, _PMAENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMAENSET;
#[doc = "`read()` method returns [pmaenset::R](pmaenset::R) reader structure"]
impl crate::Readable for PMAENSET {}
#[doc = "`write(|w| ..)` method takes [pmaenset::W](pmaenset::W) writer structure"]
impl crate::Writable for PMAENSET {}
#[doc = "PMAENSET register"]
pub mod pmaenset;
#[doc = "PMAENINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaeninv](pmaeninv) module"]
pub type PMAENINV = crate::Reg<u32, _PMAENINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMAENINV;
#[doc = "`read()` method returns [pmaeninv::R](pmaeninv::R) reader structure"]
impl crate::Readable for PMAENINV {}
#[doc = "`write(|w| ..)` method takes [pmaeninv::W](pmaeninv::W) writer structure"]
impl crate::Writable for PMAENINV {}
#[doc = "PMAENINV register"]
pub mod pmaeninv;
#[doc = "PMSTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmstat](pmstat) module"]
pub type PMSTAT = crate::Reg<u32, _PMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMSTAT;
#[doc = "`read()` method returns [pmstat::R](pmstat::R) reader structure"]
impl crate::Readable for PMSTAT {}
#[doc = "`write(|w| ..)` method takes [pmstat::W](pmstat::W) writer structure"]
impl crate::Writable for PMSTAT {}
#[doc = "PMSTAT register"]
pub mod pmstat;
#[doc = "PMSTATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmstatclr](pmstatclr) module"]
pub type PMSTATCLR = crate::Reg<u32, _PMSTATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMSTATCLR;
#[doc = "`read()` method returns [pmstatclr::R](pmstatclr::R) reader structure"]
impl crate::Readable for PMSTATCLR {}
#[doc = "`write(|w| ..)` method takes [pmstatclr::W](pmstatclr::W) writer structure"]
impl crate::Writable for PMSTATCLR {}
#[doc = "PMSTATCLR register"]
pub mod pmstatclr;
#[doc = "PMSTATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmstatset](pmstatset) module"]
pub type PMSTATSET = crate::Reg<u32, _PMSTATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMSTATSET;
#[doc = "`read()` method returns [pmstatset::R](pmstatset::R) reader structure"]
impl crate::Readable for PMSTATSET {}
#[doc = "`write(|w| ..)` method takes [pmstatset::W](pmstatset::W) writer structure"]
impl crate::Writable for PMSTATSET {}
#[doc = "PMSTATSET register"]
pub mod pmstatset;
#[doc = "PMSTATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmstatinv](pmstatinv) module"]
pub type PMSTATINV = crate::Reg<u32, _PMSTATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMSTATINV;
#[doc = "`read()` method returns [pmstatinv::R](pmstatinv::R) reader structure"]
impl crate::Readable for PMSTATINV {}
#[doc = "`write(|w| ..)` method takes [pmstatinv::W](pmstatinv::W) writer structure"]
impl crate::Writable for PMSTATINV {}
#[doc = "PMSTATINV register"]
pub mod pmstatinv;
#[doc = "PMWADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmwaddr](pmwaddr) module"]
pub type PMWADDR = crate::Reg<u32, _PMWADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMWADDR;
#[doc = "`read()` method returns [pmwaddr::R](pmwaddr::R) reader structure"]
impl crate::Readable for PMWADDR {}
#[doc = "`write(|w| ..)` method takes [pmwaddr::W](pmwaddr::W) writer structure"]
impl crate::Writable for PMWADDR {}
#[doc = "PMWADDR register"]
pub mod pmwaddr;
#[doc = "PMWADDRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmwaddrclr](pmwaddrclr) module"]
pub type PMWADDRCLR = crate::Reg<u32, _PMWADDRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMWADDRCLR;
#[doc = "`read()` method returns [pmwaddrclr::R](pmwaddrclr::R) reader structure"]
impl crate::Readable for PMWADDRCLR {}
#[doc = "`write(|w| ..)` method takes [pmwaddrclr::W](pmwaddrclr::W) writer structure"]
impl crate::Writable for PMWADDRCLR {}
#[doc = "PMWADDRCLR register"]
pub mod pmwaddrclr;
#[doc = "PMWADDRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmwaddrset](pmwaddrset) module"]
pub type PMWADDRSET = crate::Reg<u32, _PMWADDRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMWADDRSET;
#[doc = "`read()` method returns [pmwaddrset::R](pmwaddrset::R) reader structure"]
impl crate::Readable for PMWADDRSET {}
#[doc = "`write(|w| ..)` method takes [pmwaddrset::W](pmwaddrset::W) writer structure"]
impl crate::Writable for PMWADDRSET {}
#[doc = "PMWADDRSET register"]
pub mod pmwaddrset;
#[doc = "PMWADDRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmwaddrinv](pmwaddrinv) module"]
pub type PMWADDRINV = crate::Reg<u32, _PMWADDRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMWADDRINV;
#[doc = "`read()` method returns [pmwaddrinv::R](pmwaddrinv::R) reader structure"]
impl crate::Readable for PMWADDRINV {}
#[doc = "`write(|w| ..)` method takes [pmwaddrinv::W](pmwaddrinv::W) writer structure"]
impl crate::Writable for PMWADDRINV {}
#[doc = "PMWADDRINV register"]
pub mod pmwaddrinv;
#[doc = "PMRADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmraddr](pmraddr) module"]
pub type PMRADDR = crate::Reg<u32, _PMRADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMRADDR;
#[doc = "`read()` method returns [pmraddr::R](pmraddr::R) reader structure"]
impl crate::Readable for PMRADDR {}
#[doc = "`write(|w| ..)` method takes [pmraddr::W](pmraddr::W) writer structure"]
impl crate::Writable for PMRADDR {}
#[doc = "PMRADDR register"]
pub mod pmraddr;
#[doc = "PMRADDRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmraddrclr](pmraddrclr) module"]
pub type PMRADDRCLR = crate::Reg<u32, _PMRADDRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMRADDRCLR;
#[doc = "`read()` method returns [pmraddrclr::R](pmraddrclr::R) reader structure"]
impl crate::Readable for PMRADDRCLR {}
#[doc = "`write(|w| ..)` method takes [pmraddrclr::W](pmraddrclr::W) writer structure"]
impl crate::Writable for PMRADDRCLR {}
#[doc = "PMRADDRCLR register"]
pub mod pmraddrclr;
#[doc = "PMRADDRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmraddrset](pmraddrset) module"]
pub type PMRADDRSET = crate::Reg<u32, _PMRADDRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMRADDRSET;
#[doc = "`read()` method returns [pmraddrset::R](pmraddrset::R) reader structure"]
impl crate::Readable for PMRADDRSET {}
#[doc = "`write(|w| ..)` method takes [pmraddrset::W](pmraddrset::W) writer structure"]
impl crate::Writable for PMRADDRSET {}
#[doc = "PMRADDRSET register"]
pub mod pmraddrset;
#[doc = "PMRADDRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmraddrinv](pmraddrinv) module"]
pub type PMRADDRINV = crate::Reg<u32, _PMRADDRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMRADDRINV;
#[doc = "`read()` method returns [pmraddrinv::R](pmraddrinv::R) reader structure"]
impl crate::Readable for PMRADDRINV {}
#[doc = "`write(|w| ..)` method takes [pmraddrinv::W](pmraddrinv::W) writer structure"]
impl crate::Writable for PMRADDRINV {}
#[doc = "PMRADDRINV register"]
pub mod pmraddrinv;
#[doc = "PMRDIN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmrdin](pmrdin) module"]
pub type PMRDIN = crate::Reg<u32, _PMRDIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMRDIN;
#[doc = "`read()` method returns [pmrdin::R](pmrdin::R) reader structure"]
impl crate::Readable for PMRDIN {}
#[doc = "`write(|w| ..)` method takes [pmrdin::W](pmrdin::W) writer structure"]
impl crate::Writable for PMRDIN {}
#[doc = "PMRDIN register"]
pub mod pmrdin;
#[doc = "PMRDINCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmrdinclr](pmrdinclr) module"]
pub type PMRDINCLR = crate::Reg<u32, _PMRDINCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMRDINCLR;
#[doc = "`read()` method returns [pmrdinclr::R](pmrdinclr::R) reader structure"]
impl crate::Readable for PMRDINCLR {}
#[doc = "`write(|w| ..)` method takes [pmrdinclr::W](pmrdinclr::W) writer structure"]
impl crate::Writable for PMRDINCLR {}
#[doc = "PMRDINCLR register"]
pub mod pmrdinclr;
#[doc = "PMRDINSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmrdinset](pmrdinset) module"]
pub type PMRDINSET = crate::Reg<u32, _PMRDINSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMRDINSET;
#[doc = "`read()` method returns [pmrdinset::R](pmrdinset::R) reader structure"]
impl crate::Readable for PMRDINSET {}
#[doc = "`write(|w| ..)` method takes [pmrdinset::W](pmrdinset::W) writer structure"]
impl crate::Writable for PMRDINSET {}
#[doc = "PMRDINSET register"]
pub mod pmrdinset;
#[doc = "PMRDININV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmrdininv](pmrdininv) module"]
pub type PMRDININV = crate::Reg<u32, _PMRDININV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PMRDININV;
#[doc = "`read()` method returns [pmrdininv::R](pmrdininv::R) reader structure"]
impl crate::Readable for PMRDININV {}
#[doc = "`write(|w| ..)` method takes [pmrdininv::W](pmrdininv::W) writer structure"]
impl crate::Writable for PMRDININV {}
#[doc = "PMRDININV register"]
pub mod pmrdininv;
