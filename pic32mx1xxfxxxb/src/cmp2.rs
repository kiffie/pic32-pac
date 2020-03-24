#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM2CON register"]
    pub cm2con: CM2CON,
    #[doc = "0x04 - CM2CONCLR register"]
    pub cm2conclr: CM2CONCLR,
    #[doc = "0x08 - CM2CONSET register"]
    pub cm2conset: CM2CONSET,
    #[doc = "0x0c - CM2CONINV register"]
    pub cm2coninv: CM2CONINV,
}
#[doc = "CM2CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm2con](cm2con) module"]
pub type CM2CON = crate::Reg<u32, _CM2CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM2CON;
#[doc = "`read()` method returns [cm2con::R](cm2con::R) reader structure"]
impl crate::Readable for CM2CON {}
#[doc = "`write(|w| ..)` method takes [cm2con::W](cm2con::W) writer structure"]
impl crate::Writable for CM2CON {}
#[doc = "CM2CON register"]
pub mod cm2con;
#[doc = "CM2CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm2conclr](cm2conclr) module"]
pub type CM2CONCLR = crate::Reg<u32, _CM2CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM2CONCLR;
#[doc = "`read()` method returns [cm2conclr::R](cm2conclr::R) reader structure"]
impl crate::Readable for CM2CONCLR {}
#[doc = "`write(|w| ..)` method takes [cm2conclr::W](cm2conclr::W) writer structure"]
impl crate::Writable for CM2CONCLR {}
#[doc = "CM2CONCLR register"]
pub mod cm2conclr;
#[doc = "CM2CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm2conset](cm2conset) module"]
pub type CM2CONSET = crate::Reg<u32, _CM2CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM2CONSET;
#[doc = "`read()` method returns [cm2conset::R](cm2conset::R) reader structure"]
impl crate::Readable for CM2CONSET {}
#[doc = "`write(|w| ..)` method takes [cm2conset::W](cm2conset::W) writer structure"]
impl crate::Writable for CM2CONSET {}
#[doc = "CM2CONSET register"]
pub mod cm2conset;
#[doc = "CM2CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm2coninv](cm2coninv) module"]
pub type CM2CONINV = crate::Reg<u32, _CM2CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM2CONINV;
#[doc = "`read()` method returns [cm2coninv::R](cm2coninv::R) reader structure"]
impl crate::Readable for CM2CONINV {}
#[doc = "`write(|w| ..)` method takes [cm2coninv::W](cm2coninv::W) writer structure"]
impl crate::Writable for CM2CONINV {}
#[doc = "CM2CONINV register"]
pub mod cm2coninv;
