#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMSTAT register"]
    pub cmstat: CMSTAT,
    #[doc = "0x04 - CMSTATCLR register"]
    pub cmstatclr: CMSTATCLR,
    #[doc = "0x08 - CMSTATSET register"]
    pub cmstatset: CMSTATSET,
    #[doc = "0x0c - CMSTATINV register"]
    pub cmstatinv: CMSTATINV,
}
#[doc = "CMSTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmstat](cmstat) module"]
pub type CMSTAT = crate::Reg<u32, _CMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMSTAT;
#[doc = "`read()` method returns [cmstat::R](cmstat::R) reader structure"]
impl crate::Readable for CMSTAT {}
#[doc = "`write(|w| ..)` method takes [cmstat::W](cmstat::W) writer structure"]
impl crate::Writable for CMSTAT {}
#[doc = "CMSTAT register"]
pub mod cmstat;
#[doc = "CMSTATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmstatclr](cmstatclr) module"]
pub type CMSTATCLR = crate::Reg<u32, _CMSTATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMSTATCLR;
#[doc = "`read()` method returns [cmstatclr::R](cmstatclr::R) reader structure"]
impl crate::Readable for CMSTATCLR {}
#[doc = "`write(|w| ..)` method takes [cmstatclr::W](cmstatclr::W) writer structure"]
impl crate::Writable for CMSTATCLR {}
#[doc = "CMSTATCLR register"]
pub mod cmstatclr;
#[doc = "CMSTATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmstatset](cmstatset) module"]
pub type CMSTATSET = crate::Reg<u32, _CMSTATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMSTATSET;
#[doc = "`read()` method returns [cmstatset::R](cmstatset::R) reader structure"]
impl crate::Readable for CMSTATSET {}
#[doc = "`write(|w| ..)` method takes [cmstatset::W](cmstatset::W) writer structure"]
impl crate::Writable for CMSTATSET {}
#[doc = "CMSTATSET register"]
pub mod cmstatset;
#[doc = "CMSTATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmstatinv](cmstatinv) module"]
pub type CMSTATINV = crate::Reg<u32, _CMSTATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMSTATINV;
#[doc = "`read()` method returns [cmstatinv::R](cmstatinv::R) reader structure"]
impl crate::Readable for CMSTATINV {}
#[doc = "`write(|w| ..)` method takes [cmstatinv::W](cmstatinv::W) writer structure"]
impl crate::Writable for CMSTATINV {}
#[doc = "CMSTATINV register"]
pub mod cmstatinv;
