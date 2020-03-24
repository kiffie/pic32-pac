#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T4CON register"]
    pub t4con: T4CON,
    #[doc = "0x04 - T4CONCLR register"]
    pub t4conclr: T4CONCLR,
    #[doc = "0x08 - T4CONSET register"]
    pub t4conset: T4CONSET,
    #[doc = "0x0c - T4CONINV register"]
    pub t4coninv: T4CONINV,
    #[doc = "0x10 - TMR4 register"]
    pub tmr4: TMR4,
    #[doc = "0x14 - TMR4CLR register"]
    pub tmr4clr: TMR4CLR,
    #[doc = "0x18 - TMR4SET register"]
    pub tmr4set: TMR4SET,
    #[doc = "0x1c - TMR4INV register"]
    pub tmr4inv: TMR4INV,
    #[doc = "0x20 - PR4 register"]
    pub pr4: PR4,
    #[doc = "0x24 - PR4CLR register"]
    pub pr4clr: PR4CLR,
    #[doc = "0x28 - PR4SET register"]
    pub pr4set: PR4SET,
    #[doc = "0x2c - PR4INV register"]
    pub pr4inv: PR4INV,
}
#[doc = "T4CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t4con](t4con) module"]
pub type T4CON = crate::Reg<u32, _T4CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T4CON;
#[doc = "`read()` method returns [t4con::R](t4con::R) reader structure"]
impl crate::Readable for T4CON {}
#[doc = "`write(|w| ..)` method takes [t4con::W](t4con::W) writer structure"]
impl crate::Writable for T4CON {}
#[doc = "T4CON register"]
pub mod t4con;
#[doc = "T4CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t4conclr](t4conclr) module"]
pub type T4CONCLR = crate::Reg<u32, _T4CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T4CONCLR;
#[doc = "`read()` method returns [t4conclr::R](t4conclr::R) reader structure"]
impl crate::Readable for T4CONCLR {}
#[doc = "`write(|w| ..)` method takes [t4conclr::W](t4conclr::W) writer structure"]
impl crate::Writable for T4CONCLR {}
#[doc = "T4CONCLR register"]
pub mod t4conclr;
#[doc = "T4CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t4conset](t4conset) module"]
pub type T4CONSET = crate::Reg<u32, _T4CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T4CONSET;
#[doc = "`read()` method returns [t4conset::R](t4conset::R) reader structure"]
impl crate::Readable for T4CONSET {}
#[doc = "`write(|w| ..)` method takes [t4conset::W](t4conset::W) writer structure"]
impl crate::Writable for T4CONSET {}
#[doc = "T4CONSET register"]
pub mod t4conset;
#[doc = "T4CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t4coninv](t4coninv) module"]
pub type T4CONINV = crate::Reg<u32, _T4CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T4CONINV;
#[doc = "`read()` method returns [t4coninv::R](t4coninv::R) reader structure"]
impl crate::Readable for T4CONINV {}
#[doc = "`write(|w| ..)` method takes [t4coninv::W](t4coninv::W) writer structure"]
impl crate::Writable for T4CONINV {}
#[doc = "T4CONINV register"]
pub mod t4coninv;
#[doc = "TMR4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr4](tmr4) module"]
pub type TMR4 = crate::Reg<u32, _TMR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR4;
#[doc = "`read()` method returns [tmr4::R](tmr4::R) reader structure"]
impl crate::Readable for TMR4 {}
#[doc = "`write(|w| ..)` method takes [tmr4::W](tmr4::W) writer structure"]
impl crate::Writable for TMR4 {}
#[doc = "TMR4 register"]
pub mod tmr4;
#[doc = "TMR4CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr4clr](tmr4clr) module"]
pub type TMR4CLR = crate::Reg<u32, _TMR4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR4CLR;
#[doc = "`read()` method returns [tmr4clr::R](tmr4clr::R) reader structure"]
impl crate::Readable for TMR4CLR {}
#[doc = "`write(|w| ..)` method takes [tmr4clr::W](tmr4clr::W) writer structure"]
impl crate::Writable for TMR4CLR {}
#[doc = "TMR4CLR register"]
pub mod tmr4clr;
#[doc = "TMR4SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr4set](tmr4set) module"]
pub type TMR4SET = crate::Reg<u32, _TMR4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR4SET;
#[doc = "`read()` method returns [tmr4set::R](tmr4set::R) reader structure"]
impl crate::Readable for TMR4SET {}
#[doc = "`write(|w| ..)` method takes [tmr4set::W](tmr4set::W) writer structure"]
impl crate::Writable for TMR4SET {}
#[doc = "TMR4SET register"]
pub mod tmr4set;
#[doc = "TMR4INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr4inv](tmr4inv) module"]
pub type TMR4INV = crate::Reg<u32, _TMR4INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR4INV;
#[doc = "`read()` method returns [tmr4inv::R](tmr4inv::R) reader structure"]
impl crate::Readable for TMR4INV {}
#[doc = "`write(|w| ..)` method takes [tmr4inv::W](tmr4inv::W) writer structure"]
impl crate::Writable for TMR4INV {}
#[doc = "TMR4INV register"]
pub mod tmr4inv;
#[doc = "PR4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr4](pr4) module"]
pub type PR4 = crate::Reg<u32, _PR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR4;
#[doc = "`read()` method returns [pr4::R](pr4::R) reader structure"]
impl crate::Readable for PR4 {}
#[doc = "`write(|w| ..)` method takes [pr4::W](pr4::W) writer structure"]
impl crate::Writable for PR4 {}
#[doc = "PR4 register"]
pub mod pr4;
#[doc = "PR4CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr4clr](pr4clr) module"]
pub type PR4CLR = crate::Reg<u32, _PR4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR4CLR;
#[doc = "`read()` method returns [pr4clr::R](pr4clr::R) reader structure"]
impl crate::Readable for PR4CLR {}
#[doc = "`write(|w| ..)` method takes [pr4clr::W](pr4clr::W) writer structure"]
impl crate::Writable for PR4CLR {}
#[doc = "PR4CLR register"]
pub mod pr4clr;
#[doc = "PR4SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr4set](pr4set) module"]
pub type PR4SET = crate::Reg<u32, _PR4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR4SET;
#[doc = "`read()` method returns [pr4set::R](pr4set::R) reader structure"]
impl crate::Readable for PR4SET {}
#[doc = "`write(|w| ..)` method takes [pr4set::W](pr4set::W) writer structure"]
impl crate::Writable for PR4SET {}
#[doc = "PR4SET register"]
pub mod pr4set;
#[doc = "PR4INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr4inv](pr4inv) module"]
pub type PR4INV = crate::Reg<u32, _PR4INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR4INV;
#[doc = "`read()` method returns [pr4inv::R](pr4inv::R) reader structure"]
impl crate::Readable for PR4INV {}
#[doc = "`write(|w| ..)` method takes [pr4inv::W](pr4inv::W) writer structure"]
impl crate::Writable for PR4INV {}
#[doc = "PR4INV register"]
pub mod pr4inv;
