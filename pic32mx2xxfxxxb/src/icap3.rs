#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IC3CON register"]
    pub ic3con: IC3CON,
    #[doc = "0x04 - IC3CONCLR register"]
    pub ic3conclr: IC3CONCLR,
    #[doc = "0x08 - IC3CONSET register"]
    pub ic3conset: IC3CONSET,
    #[doc = "0x0c - IC3CONINV register"]
    pub ic3coninv: IC3CONINV,
    #[doc = "0x10 - IC3BUF register"]
    pub ic3buf: IC3BUF,
}
#[doc = "IC3CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic3con](ic3con) module"]
pub type IC3CON = crate::Reg<u32, _IC3CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC3CON;
#[doc = "`read()` method returns [ic3con::R](ic3con::R) reader structure"]
impl crate::Readable for IC3CON {}
#[doc = "`write(|w| ..)` method takes [ic3con::W](ic3con::W) writer structure"]
impl crate::Writable for IC3CON {}
#[doc = "IC3CON register"]
pub mod ic3con;
#[doc = "IC3CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic3conclr](ic3conclr) module"]
pub type IC3CONCLR = crate::Reg<u32, _IC3CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC3CONCLR;
#[doc = "`read()` method returns [ic3conclr::R](ic3conclr::R) reader structure"]
impl crate::Readable for IC3CONCLR {}
#[doc = "`write(|w| ..)` method takes [ic3conclr::W](ic3conclr::W) writer structure"]
impl crate::Writable for IC3CONCLR {}
#[doc = "IC3CONCLR register"]
pub mod ic3conclr;
#[doc = "IC3CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic3conset](ic3conset) module"]
pub type IC3CONSET = crate::Reg<u32, _IC3CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC3CONSET;
#[doc = "`read()` method returns [ic3conset::R](ic3conset::R) reader structure"]
impl crate::Readable for IC3CONSET {}
#[doc = "`write(|w| ..)` method takes [ic3conset::W](ic3conset::W) writer structure"]
impl crate::Writable for IC3CONSET {}
#[doc = "IC3CONSET register"]
pub mod ic3conset;
#[doc = "IC3CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic3coninv](ic3coninv) module"]
pub type IC3CONINV = crate::Reg<u32, _IC3CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC3CONINV;
#[doc = "`read()` method returns [ic3coninv::R](ic3coninv::R) reader structure"]
impl crate::Readable for IC3CONINV {}
#[doc = "`write(|w| ..)` method takes [ic3coninv::W](ic3coninv::W) writer structure"]
impl crate::Writable for IC3CONINV {}
#[doc = "IC3CONINV register"]
pub mod ic3coninv;
#[doc = "IC3BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic3buf](ic3buf) module"]
pub type IC3BUF = crate::Reg<u32, _IC3BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC3BUF;
#[doc = "`read()` method returns [ic3buf::R](ic3buf::R) reader structure"]
impl crate::Readable for IC3BUF {}
#[doc = "`write(|w| ..)` method takes [ic3buf::W](ic3buf::W) writer structure"]
impl crate::Writable for IC3BUF {}
#[doc = "IC3BUF register"]
pub mod ic3buf;
