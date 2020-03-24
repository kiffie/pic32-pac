#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RCON register"]
    pub rcon: RCON,
    #[doc = "0x04 - RCONCLR register"]
    pub rconclr: RCONCLR,
    #[doc = "0x08 - RCONSET register"]
    pub rconset: RCONSET,
    #[doc = "0x0c - RCONINV register"]
    pub rconinv: RCONINV,
    #[doc = "0x10 - RSWRST register"]
    pub rswrst: RSWRST,
    #[doc = "0x14 - RSWRSTCLR register"]
    pub rswrstclr: RSWRSTCLR,
    #[doc = "0x18 - RSWRSTSET register"]
    pub rswrstset: RSWRSTSET,
    #[doc = "0x1c - RSWRSTINV register"]
    pub rswrstinv: RSWRSTINV,
}
#[doc = "RCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcon](rcon) module"]
pub type RCON = crate::Reg<u32, _RCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCON;
#[doc = "`read()` method returns [rcon::R](rcon::R) reader structure"]
impl crate::Readable for RCON {}
#[doc = "`write(|w| ..)` method takes [rcon::W](rcon::W) writer structure"]
impl crate::Writable for RCON {}
#[doc = "RCON register"]
pub mod rcon;
#[doc = "RCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rconclr](rconclr) module"]
pub type RCONCLR = crate::Reg<u32, _RCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCONCLR;
#[doc = "`read()` method returns [rconclr::R](rconclr::R) reader structure"]
impl crate::Readable for RCONCLR {}
#[doc = "`write(|w| ..)` method takes [rconclr::W](rconclr::W) writer structure"]
impl crate::Writable for RCONCLR {}
#[doc = "RCONCLR register"]
pub mod rconclr;
#[doc = "RCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rconset](rconset) module"]
pub type RCONSET = crate::Reg<u32, _RCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCONSET;
#[doc = "`read()` method returns [rconset::R](rconset::R) reader structure"]
impl crate::Readable for RCONSET {}
#[doc = "`write(|w| ..)` method takes [rconset::W](rconset::W) writer structure"]
impl crate::Writable for RCONSET {}
#[doc = "RCONSET register"]
pub mod rconset;
#[doc = "RCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rconinv](rconinv) module"]
pub type RCONINV = crate::Reg<u32, _RCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCONINV;
#[doc = "`read()` method returns [rconinv::R](rconinv::R) reader structure"]
impl crate::Readable for RCONINV {}
#[doc = "`write(|w| ..)` method takes [rconinv::W](rconinv::W) writer structure"]
impl crate::Writable for RCONINV {}
#[doc = "RCONINV register"]
pub mod rconinv;
#[doc = "RSWRST register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrst](rswrst) module"]
pub type RSWRST = crate::Reg<u32, _RSWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWRST;
#[doc = "`read()` method returns [rswrst::R](rswrst::R) reader structure"]
impl crate::Readable for RSWRST {}
#[doc = "`write(|w| ..)` method takes [rswrst::W](rswrst::W) writer structure"]
impl crate::Writable for RSWRST {}
#[doc = "RSWRST register"]
pub mod rswrst;
#[doc = "RSWRSTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrstclr](rswrstclr) module"]
pub type RSWRSTCLR = crate::Reg<u32, _RSWRSTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWRSTCLR;
#[doc = "`read()` method returns [rswrstclr::R](rswrstclr::R) reader structure"]
impl crate::Readable for RSWRSTCLR {}
#[doc = "`write(|w| ..)` method takes [rswrstclr::W](rswrstclr::W) writer structure"]
impl crate::Writable for RSWRSTCLR {}
#[doc = "RSWRSTCLR register"]
pub mod rswrstclr;
#[doc = "RSWRSTSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrstset](rswrstset) module"]
pub type RSWRSTSET = crate::Reg<u32, _RSWRSTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWRSTSET;
#[doc = "`read()` method returns [rswrstset::R](rswrstset::R) reader structure"]
impl crate::Readable for RSWRSTSET {}
#[doc = "`write(|w| ..)` method takes [rswrstset::W](rswrstset::W) writer structure"]
impl crate::Writable for RSWRSTSET {}
#[doc = "RSWRSTSET register"]
pub mod rswrstset;
#[doc = "RSWRSTINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrstinv](rswrstinv) module"]
pub type RSWRSTINV = crate::Reg<u32, _RSWRSTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWRSTINV;
#[doc = "`read()` method returns [rswrstinv::R](rswrstinv::R) reader structure"]
impl crate::Readable for RSWRSTINV {}
#[doc = "`write(|w| ..)` method takes [rswrstinv::W](rswrstinv::W) writer structure"]
impl crate::Writable for RSWRSTINV {}
#[doc = "RSWRSTINV register"]
pub mod rswrstinv;
