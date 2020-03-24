#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CVRCON register"]
    pub cvrcon: CVRCON,
    #[doc = "0x04 - CVRCONCLR register"]
    pub cvrconclr: CVRCONCLR,
    #[doc = "0x08 - CVRCONSET register"]
    pub cvrconset: CVRCONSET,
    #[doc = "0x0c - CVRCONINV register"]
    pub cvrconinv: CVRCONINV,
}
#[doc = "CVRCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvrcon](cvrcon) module"]
pub type CVRCON = crate::Reg<u32, _CVRCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVRCON;
#[doc = "`read()` method returns [cvrcon::R](cvrcon::R) reader structure"]
impl crate::Readable for CVRCON {}
#[doc = "`write(|w| ..)` method takes [cvrcon::W](cvrcon::W) writer structure"]
impl crate::Writable for CVRCON {}
#[doc = "CVRCON register"]
pub mod cvrcon;
#[doc = "CVRCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvrconclr](cvrconclr) module"]
pub type CVRCONCLR = crate::Reg<u32, _CVRCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVRCONCLR;
#[doc = "`read()` method returns [cvrconclr::R](cvrconclr::R) reader structure"]
impl crate::Readable for CVRCONCLR {}
#[doc = "`write(|w| ..)` method takes [cvrconclr::W](cvrconclr::W) writer structure"]
impl crate::Writable for CVRCONCLR {}
#[doc = "CVRCONCLR register"]
pub mod cvrconclr;
#[doc = "CVRCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvrconset](cvrconset) module"]
pub type CVRCONSET = crate::Reg<u32, _CVRCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVRCONSET;
#[doc = "`read()` method returns [cvrconset::R](cvrconset::R) reader structure"]
impl crate::Readable for CVRCONSET {}
#[doc = "`write(|w| ..)` method takes [cvrconset::W](cvrconset::W) writer structure"]
impl crate::Writable for CVRCONSET {}
#[doc = "CVRCONSET register"]
pub mod cvrconset;
#[doc = "CVRCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvrconinv](cvrconinv) module"]
pub type CVRCONINV = crate::Reg<u32, _CVRCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CVRCONINV;
#[doc = "`read()` method returns [cvrconinv::R](cvrconinv::R) reader structure"]
impl crate::Readable for CVRCONINV {}
#[doc = "`write(|w| ..)` method takes [cvrconinv::W](cvrconinv::W) writer structure"]
impl crate::Writable for CVRCONINV {}
#[doc = "CVRCONINV register"]
pub mod cvrconinv;
