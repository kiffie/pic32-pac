#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HLVDCON register"]
    pub hlvdcon: HLVDCON,
    #[doc = "0x04 - HLVDCONCLR register"]
    pub hlvdconclr: HLVDCONCLR,
    #[doc = "0x08 - HLVDCONSET register"]
    pub hlvdconset: HLVDCONSET,
    #[doc = "0x0c - HLVDCONINV register"]
    pub hlvdconinv: HLVDCONINV,
}
#[doc = "HLVDCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hlvdcon](hlvdcon) module"]
pub type HLVDCON = crate::Reg<u32, _HLVDCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HLVDCON;
#[doc = "`read()` method returns [hlvdcon::R](hlvdcon::R) reader structure"]
impl crate::Readable for HLVDCON {}
#[doc = "`write(|w| ..)` method takes [hlvdcon::W](hlvdcon::W) writer structure"]
impl crate::Writable for HLVDCON {}
#[doc = "HLVDCON register"]
pub mod hlvdcon;
#[doc = "HLVDCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hlvdconclr](hlvdconclr) module"]
pub type HLVDCONCLR = crate::Reg<u32, _HLVDCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HLVDCONCLR;
#[doc = "`read()` method returns [hlvdconclr::R](hlvdconclr::R) reader structure"]
impl crate::Readable for HLVDCONCLR {}
#[doc = "`write(|w| ..)` method takes [hlvdconclr::W](hlvdconclr::W) writer structure"]
impl crate::Writable for HLVDCONCLR {}
#[doc = "HLVDCONCLR register"]
pub mod hlvdconclr;
#[doc = "HLVDCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hlvdconset](hlvdconset) module"]
pub type HLVDCONSET = crate::Reg<u32, _HLVDCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HLVDCONSET;
#[doc = "`read()` method returns [hlvdconset::R](hlvdconset::R) reader structure"]
impl crate::Readable for HLVDCONSET {}
#[doc = "`write(|w| ..)` method takes [hlvdconset::W](hlvdconset::W) writer structure"]
impl crate::Writable for HLVDCONSET {}
#[doc = "HLVDCONSET register"]
pub mod hlvdconset;
#[doc = "HLVDCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hlvdconinv](hlvdconinv) module"]
pub type HLVDCONINV = crate::Reg<u32, _HLVDCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HLVDCONINV;
#[doc = "`read()` method returns [hlvdconinv::R](hlvdconinv::R) reader structure"]
impl crate::Readable for HLVDCONINV {}
#[doc = "`write(|w| ..)` method takes [hlvdconinv::W](hlvdconinv::W) writer structure"]
impl crate::Writable for HLVDCONINV {}
#[doc = "HLVDCONINV register"]
pub mod hlvdconinv;
