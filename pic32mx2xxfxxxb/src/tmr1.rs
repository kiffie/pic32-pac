#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T1CON register"]
    pub t1con: T1CON,
    #[doc = "0x04 - T1CONCLR register"]
    pub t1conclr: T1CONCLR,
    #[doc = "0x08 - T1CONSET register"]
    pub t1conset: T1CONSET,
    #[doc = "0x0c - T1CONINV register"]
    pub t1coninv: T1CONINV,
    #[doc = "0x10 - TMR1 register"]
    pub tmr1: TMR1,
    #[doc = "0x14 - TMR1CLR register"]
    pub tmr1clr: TMR1CLR,
    #[doc = "0x18 - TMR1SET register"]
    pub tmr1set: TMR1SET,
    #[doc = "0x1c - TMR1INV register"]
    pub tmr1inv: TMR1INV,
    #[doc = "0x20 - PR1 register"]
    pub pr1: PR1,
    #[doc = "0x24 - PR1CLR register"]
    pub pr1clr: PR1CLR,
    #[doc = "0x28 - PR1SET register"]
    pub pr1set: PR1SET,
    #[doc = "0x2c - PR1INV register"]
    pub pr1inv: PR1INV,
}
#[doc = "T1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1con](t1con) module"]
pub type T1CON = crate::Reg<u32, _T1CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CON;
#[doc = "`read()` method returns [t1con::R](t1con::R) reader structure"]
impl crate::Readable for T1CON {}
#[doc = "`write(|w| ..)` method takes [t1con::W](t1con::W) writer structure"]
impl crate::Writable for T1CON {}
#[doc = "T1CON register"]
pub mod t1con;
#[doc = "T1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1conclr](t1conclr) module"]
pub type T1CONCLR = crate::Reg<u32, _T1CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CONCLR;
#[doc = "`read()` method returns [t1conclr::R](t1conclr::R) reader structure"]
impl crate::Readable for T1CONCLR {}
#[doc = "`write(|w| ..)` method takes [t1conclr::W](t1conclr::W) writer structure"]
impl crate::Writable for T1CONCLR {}
#[doc = "T1CONCLR register"]
pub mod t1conclr;
#[doc = "T1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1conset](t1conset) module"]
pub type T1CONSET = crate::Reg<u32, _T1CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CONSET;
#[doc = "`read()` method returns [t1conset::R](t1conset::R) reader structure"]
impl crate::Readable for T1CONSET {}
#[doc = "`write(|w| ..)` method takes [t1conset::W](t1conset::W) writer structure"]
impl crate::Writable for T1CONSET {}
#[doc = "T1CONSET register"]
pub mod t1conset;
#[doc = "T1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1coninv](t1coninv) module"]
pub type T1CONINV = crate::Reg<u32, _T1CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1CONINV;
#[doc = "`read()` method returns [t1coninv::R](t1coninv::R) reader structure"]
impl crate::Readable for T1CONINV {}
#[doc = "`write(|w| ..)` method takes [t1coninv::W](t1coninv::W) writer structure"]
impl crate::Writable for T1CONINV {}
#[doc = "T1CONINV register"]
pub mod t1coninv;
#[doc = "TMR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr1](tmr1) module"]
pub type TMR1 = crate::Reg<u32, _TMR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR1;
#[doc = "`read()` method returns [tmr1::R](tmr1::R) reader structure"]
impl crate::Readable for TMR1 {}
#[doc = "`write(|w| ..)` method takes [tmr1::W](tmr1::W) writer structure"]
impl crate::Writable for TMR1 {}
#[doc = "TMR1 register"]
pub mod tmr1;
#[doc = "TMR1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr1clr](tmr1clr) module"]
pub type TMR1CLR = crate::Reg<u32, _TMR1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR1CLR;
#[doc = "`read()` method returns [tmr1clr::R](tmr1clr::R) reader structure"]
impl crate::Readable for TMR1CLR {}
#[doc = "`write(|w| ..)` method takes [tmr1clr::W](tmr1clr::W) writer structure"]
impl crate::Writable for TMR1CLR {}
#[doc = "TMR1CLR register"]
pub mod tmr1clr;
#[doc = "TMR1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr1set](tmr1set) module"]
pub type TMR1SET = crate::Reg<u32, _TMR1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR1SET;
#[doc = "`read()` method returns [tmr1set::R](tmr1set::R) reader structure"]
impl crate::Readable for TMR1SET {}
#[doc = "`write(|w| ..)` method takes [tmr1set::W](tmr1set::W) writer structure"]
impl crate::Writable for TMR1SET {}
#[doc = "TMR1SET register"]
pub mod tmr1set;
#[doc = "TMR1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr1inv](tmr1inv) module"]
pub type TMR1INV = crate::Reg<u32, _TMR1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR1INV;
#[doc = "`read()` method returns [tmr1inv::R](tmr1inv::R) reader structure"]
impl crate::Readable for TMR1INV {}
#[doc = "`write(|w| ..)` method takes [tmr1inv::W](tmr1inv::W) writer structure"]
impl crate::Writable for TMR1INV {}
#[doc = "TMR1INV register"]
pub mod tmr1inv;
#[doc = "PR1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1](pr1) module"]
pub type PR1 = crate::Reg<u32, _PR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR1;
#[doc = "`read()` method returns [pr1::R](pr1::R) reader structure"]
impl crate::Readable for PR1 {}
#[doc = "`write(|w| ..)` method takes [pr1::W](pr1::W) writer structure"]
impl crate::Writable for PR1 {}
#[doc = "PR1 register"]
pub mod pr1;
#[doc = "PR1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1clr](pr1clr) module"]
pub type PR1CLR = crate::Reg<u32, _PR1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR1CLR;
#[doc = "`read()` method returns [pr1clr::R](pr1clr::R) reader structure"]
impl crate::Readable for PR1CLR {}
#[doc = "`write(|w| ..)` method takes [pr1clr::W](pr1clr::W) writer structure"]
impl crate::Writable for PR1CLR {}
#[doc = "PR1CLR register"]
pub mod pr1clr;
#[doc = "PR1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1set](pr1set) module"]
pub type PR1SET = crate::Reg<u32, _PR1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR1SET;
#[doc = "`read()` method returns [pr1set::R](pr1set::R) reader structure"]
impl crate::Readable for PR1SET {}
#[doc = "`write(|w| ..)` method takes [pr1set::W](pr1set::W) writer structure"]
impl crate::Writable for PR1SET {}
#[doc = "PR1SET register"]
pub mod pr1set;
#[doc = "PR1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1inv](pr1inv) module"]
pub type PR1INV = crate::Reg<u32, _PR1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR1INV;
#[doc = "`read()` method returns [pr1inv::R](pr1inv::R) reader structure"]
impl crate::Readable for PR1INV {}
#[doc = "`write(|w| ..)` method takes [pr1inv::W](pr1inv::W) writer structure"]
impl crate::Writable for PR1INV {}
#[doc = "PR1INV register"]
pub mod pr1inv;
