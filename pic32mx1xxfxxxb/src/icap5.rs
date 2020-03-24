#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC5CON register"]
    pub ic5con: IC5CON,
    #[doc = "0x04 - IC5CONCLR register"]
    pub ic5conclr: IC5CONCLR,
    #[doc = "0x08 - IC5CONSET register"]
    pub ic5conset: IC5CONSET,
    #[doc = "0x0c - IC5CONINV register"]
    pub ic5coninv: IC5CONINV,
    #[doc = "0x10 - IC5BUF register"]
    pub ic5buf: IC5BUF,
}
#[doc = "IC5CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic5con](ic5con) module"]
pub type IC5CON = crate::Reg<u32, _IC5CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC5CON;
#[doc = "`read()` method returns [ic5con::R](ic5con::R) reader structure"]
impl crate::Readable for IC5CON {}
#[doc = "`write(|w| ..)` method takes [ic5con::W](ic5con::W) writer structure"]
impl crate::Writable for IC5CON {}
#[doc = "IC5CON register"]
pub mod ic5con;
#[doc = "IC5CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic5conclr](ic5conclr) module"]
pub type IC5CONCLR = crate::Reg<u32, _IC5CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC5CONCLR;
#[doc = "`read()` method returns [ic5conclr::R](ic5conclr::R) reader structure"]
impl crate::Readable for IC5CONCLR {}
#[doc = "`write(|w| ..)` method takes [ic5conclr::W](ic5conclr::W) writer structure"]
impl crate::Writable for IC5CONCLR {}
#[doc = "IC5CONCLR register"]
pub mod ic5conclr;
#[doc = "IC5CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic5conset](ic5conset) module"]
pub type IC5CONSET = crate::Reg<u32, _IC5CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC5CONSET;
#[doc = "`read()` method returns [ic5conset::R](ic5conset::R) reader structure"]
impl crate::Readable for IC5CONSET {}
#[doc = "`write(|w| ..)` method takes [ic5conset::W](ic5conset::W) writer structure"]
impl crate::Writable for IC5CONSET {}
#[doc = "IC5CONSET register"]
pub mod ic5conset;
#[doc = "IC5CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic5coninv](ic5coninv) module"]
pub type IC5CONINV = crate::Reg<u32, _IC5CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC5CONINV;
#[doc = "`read()` method returns [ic5coninv::R](ic5coninv::R) reader structure"]
impl crate::Readable for IC5CONINV {}
#[doc = "`write(|w| ..)` method takes [ic5coninv::W](ic5coninv::W) writer structure"]
impl crate::Writable for IC5CONINV {}
#[doc = "IC5CONINV register"]
pub mod ic5coninv;
#[doc = "IC5BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic5buf](ic5buf) module"]
pub type IC5BUF = crate::Reg<u32, _IC5BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC5BUF;
#[doc = "`read()` method returns [ic5buf::R](ic5buf::R) reader structure"]
impl crate::Readable for IC5BUF {}
#[doc = "`write(|w| ..)` method takes [ic5buf::W](ic5buf::W) writer structure"]
impl crate::Writable for IC5BUF {}
#[doc = "IC5BUF register"]
pub mod ic5buf;
