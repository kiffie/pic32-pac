#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDTCON register"]
    pub wdtcon: WDTCON,
    #[doc = "0x04 - WDTCONCLR register"]
    pub wdtconclr: WDTCONCLR,
    #[doc = "0x08 - WDTCONSET register"]
    pub wdtconset: WDTCONSET,
    #[doc = "0x0c - WDTCONINV register"]
    pub wdtconinv: WDTCONINV,
}
#[doc = "WDTCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtcon](wdtcon) module"]
pub type WDTCON = crate::Reg<u32, _WDTCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCON;
#[doc = "`read()` method returns [wdtcon::R](wdtcon::R) reader structure"]
impl crate::Readable for WDTCON {}
#[doc = "`write(|w| ..)` method takes [wdtcon::W](wdtcon::W) writer structure"]
impl crate::Writable for WDTCON {}
#[doc = "WDTCON register"]
pub mod wdtcon;
#[doc = "WDTCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconclr](wdtconclr) module"]
pub type WDTCONCLR = crate::Reg<u32, _WDTCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONCLR;
#[doc = "`read()` method returns [wdtconclr::R](wdtconclr::R) reader structure"]
impl crate::Readable for WDTCONCLR {}
#[doc = "`write(|w| ..)` method takes [wdtconclr::W](wdtconclr::W) writer structure"]
impl crate::Writable for WDTCONCLR {}
#[doc = "WDTCONCLR register"]
pub mod wdtconclr;
#[doc = "WDTCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconset](wdtconset) module"]
pub type WDTCONSET = crate::Reg<u32, _WDTCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONSET;
#[doc = "`read()` method returns [wdtconset::R](wdtconset::R) reader structure"]
impl crate::Readable for WDTCONSET {}
#[doc = "`write(|w| ..)` method takes [wdtconset::W](wdtconset::W) writer structure"]
impl crate::Writable for WDTCONSET {}
#[doc = "WDTCONSET register"]
pub mod wdtconset;
#[doc = "WDTCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconinv](wdtconinv) module"]
pub type WDTCONINV = crate::Reg<u32, _WDTCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDTCONINV;
#[doc = "`read()` method returns [wdtconinv::R](wdtconinv::R) reader structure"]
impl crate::Readable for WDTCONINV {}
#[doc = "`write(|w| ..)` method takes [wdtconinv::W](wdtconinv::W) writer structure"]
impl crate::Writable for WDTCONINV {}
#[doc = "WDTCONINV register"]
pub mod wdtconinv;
