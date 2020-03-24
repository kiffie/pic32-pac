#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC2CON register"]
    pub ic2con: IC2CON,
    #[doc = "0x04 - IC2CONCLR register"]
    pub ic2conclr: IC2CONCLR,
    #[doc = "0x08 - IC2CONSET register"]
    pub ic2conset: IC2CONSET,
    #[doc = "0x0c - IC2CONINV register"]
    pub ic2coninv: IC2CONINV,
    #[doc = "0x10 - IC2BUF register"]
    pub ic2buf: IC2BUF,
}
#[doc = "IC2CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic2con](ic2con) module"]
pub type IC2CON = crate::Reg<u32, _IC2CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC2CON;
#[doc = "`read()` method returns [ic2con::R](ic2con::R) reader structure"]
impl crate::Readable for IC2CON {}
#[doc = "`write(|w| ..)` method takes [ic2con::W](ic2con::W) writer structure"]
impl crate::Writable for IC2CON {}
#[doc = "IC2CON register"]
pub mod ic2con;
#[doc = "IC2CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic2conclr](ic2conclr) module"]
pub type IC2CONCLR = crate::Reg<u32, _IC2CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC2CONCLR;
#[doc = "`read()` method returns [ic2conclr::R](ic2conclr::R) reader structure"]
impl crate::Readable for IC2CONCLR {}
#[doc = "`write(|w| ..)` method takes [ic2conclr::W](ic2conclr::W) writer structure"]
impl crate::Writable for IC2CONCLR {}
#[doc = "IC2CONCLR register"]
pub mod ic2conclr;
#[doc = "IC2CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic2conset](ic2conset) module"]
pub type IC2CONSET = crate::Reg<u32, _IC2CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC2CONSET;
#[doc = "`read()` method returns [ic2conset::R](ic2conset::R) reader structure"]
impl crate::Readable for IC2CONSET {}
#[doc = "`write(|w| ..)` method takes [ic2conset::W](ic2conset::W) writer structure"]
impl crate::Writable for IC2CONSET {}
#[doc = "IC2CONSET register"]
pub mod ic2conset;
#[doc = "IC2CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic2coninv](ic2coninv) module"]
pub type IC2CONINV = crate::Reg<u32, _IC2CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC2CONINV;
#[doc = "`read()` method returns [ic2coninv::R](ic2coninv::R) reader structure"]
impl crate::Readable for IC2CONINV {}
#[doc = "`write(|w| ..)` method takes [ic2coninv::W](ic2coninv::W) writer structure"]
impl crate::Writable for IC2CONINV {}
#[doc = "IC2CONINV register"]
pub mod ic2coninv;
#[doc = "IC2BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic2buf](ic2buf) module"]
pub type IC2BUF = crate::Reg<u32, _IC2BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC2BUF;
#[doc = "`read()` method returns [ic2buf::R](ic2buf::R) reader structure"]
impl crate::Readable for IC2BUF {}
#[doc = "`write(|w| ..)` method takes [ic2buf::W](ic2buf::W) writer structure"]
impl crate::Writable for IC2BUF {}
#[doc = "IC2BUF register"]
pub mod ic2buf;
