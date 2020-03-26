#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC1CON register"]
    pub ic1con: IC1CON,
    #[doc = "0x04 - IC1CONCLR register"]
    pub ic1conclr: IC1CONCLR,
    #[doc = "0x08 - IC1CONSET register"]
    pub ic1conset: IC1CONSET,
    #[doc = "0x0c - IC1CONINV register"]
    pub ic1coninv: IC1CONINV,
    #[doc = "0x10 - IC1BUF register"]
    pub ic1buf: IC1BUF,
}
#[doc = "IC1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1con](ic1con) module"]
pub type IC1CON = crate::Reg<u32, _IC1CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC1CON;
#[doc = "`read()` method returns [ic1con::R](ic1con::R) reader structure"]
impl crate::Readable for IC1CON {}
#[doc = "`write(|w| ..)` method takes [ic1con::W](ic1con::W) writer structure"]
impl crate::Writable for IC1CON {}
#[doc = "IC1CON register"]
pub mod ic1con;
#[doc = "IC1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1conclr](ic1conclr) module"]
pub type IC1CONCLR = crate::Reg<u32, _IC1CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC1CONCLR;
#[doc = "`read()` method returns [ic1conclr::R](ic1conclr::R) reader structure"]
impl crate::Readable for IC1CONCLR {}
#[doc = "`write(|w| ..)` method takes [ic1conclr::W](ic1conclr::W) writer structure"]
impl crate::Writable for IC1CONCLR {}
#[doc = "IC1CONCLR register"]
pub mod ic1conclr;
#[doc = "IC1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1conset](ic1conset) module"]
pub type IC1CONSET = crate::Reg<u32, _IC1CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC1CONSET;
#[doc = "`read()` method returns [ic1conset::R](ic1conset::R) reader structure"]
impl crate::Readable for IC1CONSET {}
#[doc = "`write(|w| ..)` method takes [ic1conset::W](ic1conset::W) writer structure"]
impl crate::Writable for IC1CONSET {}
#[doc = "IC1CONSET register"]
pub mod ic1conset;
#[doc = "IC1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1coninv](ic1coninv) module"]
pub type IC1CONINV = crate::Reg<u32, _IC1CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC1CONINV;
#[doc = "`read()` method returns [ic1coninv::R](ic1coninv::R) reader structure"]
impl crate::Readable for IC1CONINV {}
#[doc = "`write(|w| ..)` method takes [ic1coninv::W](ic1coninv::W) writer structure"]
impl crate::Writable for IC1CONINV {}
#[doc = "IC1CONINV register"]
pub mod ic1coninv;
#[doc = "IC1BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1buf](ic1buf) module"]
pub type IC1BUF = crate::Reg<u32, _IC1BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC1BUF;
#[doc = "`read()` method returns [ic1buf::R](ic1buf::R) reader structure"]
impl crate::Readable for IC1BUF {}
#[doc = "`write(|w| ..)` method takes [ic1buf::W](ic1buf::W) writer structure"]
impl crate::Writable for IC1BUF {}
#[doc = "IC1BUF register"]
pub mod ic1buf;
