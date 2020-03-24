#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM1CON register"]
    pub cm1con: CM1CON,
    #[doc = "0x04 - CM1CONCLR register"]
    pub cm1conclr: CM1CONCLR,
    #[doc = "0x08 - CM1CONSET register"]
    pub cm1conset: CM1CONSET,
    #[doc = "0x0c - CM1CONINV register"]
    pub cm1coninv: CM1CONINV,
}
#[doc = "CM1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1con](cm1con) module"]
pub type CM1CON = crate::Reg<u32, _CM1CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1CON;
#[doc = "`read()` method returns [cm1con::R](cm1con::R) reader structure"]
impl crate::Readable for CM1CON {}
#[doc = "`write(|w| ..)` method takes [cm1con::W](cm1con::W) writer structure"]
impl crate::Writable for CM1CON {}
#[doc = "CM1CON register"]
pub mod cm1con;
#[doc = "CM1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1conclr](cm1conclr) module"]
pub type CM1CONCLR = crate::Reg<u32, _CM1CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1CONCLR;
#[doc = "`read()` method returns [cm1conclr::R](cm1conclr::R) reader structure"]
impl crate::Readable for CM1CONCLR {}
#[doc = "`write(|w| ..)` method takes [cm1conclr::W](cm1conclr::W) writer structure"]
impl crate::Writable for CM1CONCLR {}
#[doc = "CM1CONCLR register"]
pub mod cm1conclr;
#[doc = "CM1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1conset](cm1conset) module"]
pub type CM1CONSET = crate::Reg<u32, _CM1CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1CONSET;
#[doc = "`read()` method returns [cm1conset::R](cm1conset::R) reader structure"]
impl crate::Readable for CM1CONSET {}
#[doc = "`write(|w| ..)` method takes [cm1conset::W](cm1conset::W) writer structure"]
impl crate::Writable for CM1CONSET {}
#[doc = "CM1CONSET register"]
pub mod cm1conset;
#[doc = "CM1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1coninv](cm1coninv) module"]
pub type CM1CONINV = crate::Reg<u32, _CM1CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM1CONINV;
#[doc = "`read()` method returns [cm1coninv::R](cm1coninv::R) reader structure"]
impl crate::Readable for CM1CONINV {}
#[doc = "`write(|w| ..)` method takes [cm1coninv::W](cm1coninv::W) writer structure"]
impl crate::Writable for CM1CONINV {}
#[doc = "CM1CONINV register"]
pub mod cm1coninv;
