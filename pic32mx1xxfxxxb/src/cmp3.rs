#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CM3CON register"]
    pub cm3con: CM3CON,
    #[doc = "0x04 - CM3CONCLR register"]
    pub cm3conclr: CM3CONCLR,
    #[doc = "0x08 - CM3CONSET register"]
    pub cm3conset: CM3CONSET,
    #[doc = "0x0c - CM3CONINV register"]
    pub cm3coninv: CM3CONINV,
}
#[doc = "CM3CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm3con](cm3con) module"]
pub type CM3CON = crate::Reg<u32, _CM3CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM3CON;
#[doc = "`read()` method returns [cm3con::R](cm3con::R) reader structure"]
impl crate::Readable for CM3CON {}
#[doc = "`write(|w| ..)` method takes [cm3con::W](cm3con::W) writer structure"]
impl crate::Writable for CM3CON {}
#[doc = "CM3CON register"]
pub mod cm3con;
#[doc = "CM3CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm3conclr](cm3conclr) module"]
pub type CM3CONCLR = crate::Reg<u32, _CM3CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM3CONCLR;
#[doc = "`read()` method returns [cm3conclr::R](cm3conclr::R) reader structure"]
impl crate::Readable for CM3CONCLR {}
#[doc = "`write(|w| ..)` method takes [cm3conclr::W](cm3conclr::W) writer structure"]
impl crate::Writable for CM3CONCLR {}
#[doc = "CM3CONCLR register"]
pub mod cm3conclr;
#[doc = "CM3CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm3conset](cm3conset) module"]
pub type CM3CONSET = crate::Reg<u32, _CM3CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM3CONSET;
#[doc = "`read()` method returns [cm3conset::R](cm3conset::R) reader structure"]
impl crate::Readable for CM3CONSET {}
#[doc = "`write(|w| ..)` method takes [cm3conset::W](cm3conset::W) writer structure"]
impl crate::Writable for CM3CONSET {}
#[doc = "CM3CONSET register"]
pub mod cm3conset;
#[doc = "CM3CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm3coninv](cm3coninv) module"]
pub type CM3CONINV = crate::Reg<u32, _CM3CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CM3CONINV;
#[doc = "`read()` method returns [cm3coninv::R](cm3coninv::R) reader structure"]
impl crate::Readable for CM3CONINV {}
#[doc = "`write(|w| ..)` method takes [cm3coninv::W](cm3coninv::W) writer structure"]
impl crate::Writable for CM3CONINV {}
#[doc = "CM3CONINV register"]
pub mod cm3coninv;
