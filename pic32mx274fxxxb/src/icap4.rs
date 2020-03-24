#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC4CON register"]
    pub ic4con: IC4CON,
    #[doc = "0x04 - IC4CONCLR register"]
    pub ic4conclr: IC4CONCLR,
    #[doc = "0x08 - IC4CONSET register"]
    pub ic4conset: IC4CONSET,
    #[doc = "0x0c - IC4CONINV register"]
    pub ic4coninv: IC4CONINV,
    #[doc = "0x10 - IC4BUF register"]
    pub ic4buf: IC4BUF,
}
#[doc = "IC4CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic4con](ic4con) module"]
pub type IC4CON = crate::Reg<u32, _IC4CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC4CON;
#[doc = "`read()` method returns [ic4con::R](ic4con::R) reader structure"]
impl crate::Readable for IC4CON {}
#[doc = "`write(|w| ..)` method takes [ic4con::W](ic4con::W) writer structure"]
impl crate::Writable for IC4CON {}
#[doc = "IC4CON register"]
pub mod ic4con;
#[doc = "IC4CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic4conclr](ic4conclr) module"]
pub type IC4CONCLR = crate::Reg<u32, _IC4CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC4CONCLR;
#[doc = "`read()` method returns [ic4conclr::R](ic4conclr::R) reader structure"]
impl crate::Readable for IC4CONCLR {}
#[doc = "`write(|w| ..)` method takes [ic4conclr::W](ic4conclr::W) writer structure"]
impl crate::Writable for IC4CONCLR {}
#[doc = "IC4CONCLR register"]
pub mod ic4conclr;
#[doc = "IC4CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic4conset](ic4conset) module"]
pub type IC4CONSET = crate::Reg<u32, _IC4CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC4CONSET;
#[doc = "`read()` method returns [ic4conset::R](ic4conset::R) reader structure"]
impl crate::Readable for IC4CONSET {}
#[doc = "`write(|w| ..)` method takes [ic4conset::W](ic4conset::W) writer structure"]
impl crate::Writable for IC4CONSET {}
#[doc = "IC4CONSET register"]
pub mod ic4conset;
#[doc = "IC4CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic4coninv](ic4coninv) module"]
pub type IC4CONINV = crate::Reg<u32, _IC4CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC4CONINV;
#[doc = "`read()` method returns [ic4coninv::R](ic4coninv::R) reader structure"]
impl crate::Readable for IC4CONINV {}
#[doc = "`write(|w| ..)` method takes [ic4coninv::W](ic4coninv::W) writer structure"]
impl crate::Writable for IC4CONINV {}
#[doc = "IC4CONINV register"]
pub mod ic4coninv;
#[doc = "IC4BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic4buf](ic4buf) module"]
pub type IC4BUF = crate::Reg<u32, _IC4BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC4BUF;
#[doc = "`read()` method returns [ic4buf::R](ic4buf::R) reader structure"]
impl crate::Readable for IC4BUF {}
#[doc = "`write(|w| ..)` method takes [ic4buf::W](ic4buf::W) writer structure"]
impl crate::Writable for IC4BUF {}
#[doc = "IC4BUF register"]
pub mod ic4buf;
