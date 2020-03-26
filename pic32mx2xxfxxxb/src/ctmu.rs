#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTMUCON register"]
    pub ctmucon: CTMUCON,
    #[doc = "0x04 - CTMUCONCLR register"]
    pub ctmuconclr: CTMUCONCLR,
    #[doc = "0x08 - CTMUCONSET register"]
    pub ctmuconset: CTMUCONSET,
    #[doc = "0x0c - CTMUCONINV register"]
    pub ctmuconinv: CTMUCONINV,
}
#[doc = "CTMUCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctmucon](ctmucon) module"]
pub type CTMUCON = crate::Reg<u32, _CTMUCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTMUCON;
#[doc = "`read()` method returns [ctmucon::R](ctmucon::R) reader structure"]
impl crate::Readable for CTMUCON {}
#[doc = "`write(|w| ..)` method takes [ctmucon::W](ctmucon::W) writer structure"]
impl crate::Writable for CTMUCON {}
#[doc = "CTMUCON register"]
pub mod ctmucon;
#[doc = "CTMUCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctmuconclr](ctmuconclr) module"]
pub type CTMUCONCLR = crate::Reg<u32, _CTMUCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTMUCONCLR;
#[doc = "`read()` method returns [ctmuconclr::R](ctmuconclr::R) reader structure"]
impl crate::Readable for CTMUCONCLR {}
#[doc = "`write(|w| ..)` method takes [ctmuconclr::W](ctmuconclr::W) writer structure"]
impl crate::Writable for CTMUCONCLR {}
#[doc = "CTMUCONCLR register"]
pub mod ctmuconclr;
#[doc = "CTMUCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctmuconset](ctmuconset) module"]
pub type CTMUCONSET = crate::Reg<u32, _CTMUCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTMUCONSET;
#[doc = "`read()` method returns [ctmuconset::R](ctmuconset::R) reader structure"]
impl crate::Readable for CTMUCONSET {}
#[doc = "`write(|w| ..)` method takes [ctmuconset::W](ctmuconset::W) writer structure"]
impl crate::Writable for CTMUCONSET {}
#[doc = "CTMUCONSET register"]
pub mod ctmuconset;
#[doc = "CTMUCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctmuconinv](ctmuconinv) module"]
pub type CTMUCONINV = crate::Reg<u32, _CTMUCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTMUCONINV;
#[doc = "`read()` method returns [ctmuconinv::R](ctmuconinv::R) reader structure"]
impl crate::Readable for CTMUCONINV {}
#[doc = "`write(|w| ..)` method takes [ctmuconinv::W](ctmuconinv::W) writer structure"]
impl crate::Writable for CTMUCONINV {}
#[doc = "CTMUCONINV register"]
pub mod ctmuconinv;
