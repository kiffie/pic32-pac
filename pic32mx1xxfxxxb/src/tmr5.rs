#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T5CON register"]
    pub t5con: T5CON,
    #[doc = "0x04 - T5CONCLR register"]
    pub t5conclr: T5CONCLR,
    #[doc = "0x08 - T5CONSET register"]
    pub t5conset: T5CONSET,
    #[doc = "0x0c - T5CONINV register"]
    pub t5coninv: T5CONINV,
    #[doc = "0x10 - TMR5 register"]
    pub tmr5: TMR5,
    #[doc = "0x14 - TMR5CLR register"]
    pub tmr5clr: TMR5CLR,
    #[doc = "0x18 - TMR5SET register"]
    pub tmr5set: TMR5SET,
    #[doc = "0x1c - TMR5INV register"]
    pub tmr5inv: TMR5INV,
    #[doc = "0x20 - PR5 register"]
    pub pr5: PR5,
    #[doc = "0x24 - PR5CLR register"]
    pub pr5clr: PR5CLR,
    #[doc = "0x28 - PR5SET register"]
    pub pr5set: PR5SET,
    #[doc = "0x2c - PR5INV register"]
    pub pr5inv: PR5INV,
}
#[doc = "T5CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t5con](t5con) module"]
pub type T5CON = crate::Reg<u32, _T5CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T5CON;
#[doc = "`read()` method returns [t5con::R](t5con::R) reader structure"]
impl crate::Readable for T5CON {}
#[doc = "`write(|w| ..)` method takes [t5con::W](t5con::W) writer structure"]
impl crate::Writable for T5CON {}
#[doc = "T5CON register"]
pub mod t5con;
#[doc = "T5CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t5conclr](t5conclr) module"]
pub type T5CONCLR = crate::Reg<u32, _T5CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T5CONCLR;
#[doc = "`read()` method returns [t5conclr::R](t5conclr::R) reader structure"]
impl crate::Readable for T5CONCLR {}
#[doc = "`write(|w| ..)` method takes [t5conclr::W](t5conclr::W) writer structure"]
impl crate::Writable for T5CONCLR {}
#[doc = "T5CONCLR register"]
pub mod t5conclr;
#[doc = "T5CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t5conset](t5conset) module"]
pub type T5CONSET = crate::Reg<u32, _T5CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T5CONSET;
#[doc = "`read()` method returns [t5conset::R](t5conset::R) reader structure"]
impl crate::Readable for T5CONSET {}
#[doc = "`write(|w| ..)` method takes [t5conset::W](t5conset::W) writer structure"]
impl crate::Writable for T5CONSET {}
#[doc = "T5CONSET register"]
pub mod t5conset;
#[doc = "T5CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t5coninv](t5coninv) module"]
pub type T5CONINV = crate::Reg<u32, _T5CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T5CONINV;
#[doc = "`read()` method returns [t5coninv::R](t5coninv::R) reader structure"]
impl crate::Readable for T5CONINV {}
#[doc = "`write(|w| ..)` method takes [t5coninv::W](t5coninv::W) writer structure"]
impl crate::Writable for T5CONINV {}
#[doc = "T5CONINV register"]
pub mod t5coninv;
#[doc = "TMR5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr5](tmr5) module"]
pub type TMR5 = crate::Reg<u32, _TMR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR5;
#[doc = "`read()` method returns [tmr5::R](tmr5::R) reader structure"]
impl crate::Readable for TMR5 {}
#[doc = "`write(|w| ..)` method takes [tmr5::W](tmr5::W) writer structure"]
impl crate::Writable for TMR5 {}
#[doc = "TMR5 register"]
pub mod tmr5;
#[doc = "TMR5CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr5clr](tmr5clr) module"]
pub type TMR5CLR = crate::Reg<u32, _TMR5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR5CLR;
#[doc = "`read()` method returns [tmr5clr::R](tmr5clr::R) reader structure"]
impl crate::Readable for TMR5CLR {}
#[doc = "`write(|w| ..)` method takes [tmr5clr::W](tmr5clr::W) writer structure"]
impl crate::Writable for TMR5CLR {}
#[doc = "TMR5CLR register"]
pub mod tmr5clr;
#[doc = "TMR5SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr5set](tmr5set) module"]
pub type TMR5SET = crate::Reg<u32, _TMR5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR5SET;
#[doc = "`read()` method returns [tmr5set::R](tmr5set::R) reader structure"]
impl crate::Readable for TMR5SET {}
#[doc = "`write(|w| ..)` method takes [tmr5set::W](tmr5set::W) writer structure"]
impl crate::Writable for TMR5SET {}
#[doc = "TMR5SET register"]
pub mod tmr5set;
#[doc = "TMR5INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr5inv](tmr5inv) module"]
pub type TMR5INV = crate::Reg<u32, _TMR5INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR5INV;
#[doc = "`read()` method returns [tmr5inv::R](tmr5inv::R) reader structure"]
impl crate::Readable for TMR5INV {}
#[doc = "`write(|w| ..)` method takes [tmr5inv::W](tmr5inv::W) writer structure"]
impl crate::Writable for TMR5INV {}
#[doc = "TMR5INV register"]
pub mod tmr5inv;
#[doc = "PR5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr5](pr5) module"]
pub type PR5 = crate::Reg<u32, _PR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR5;
#[doc = "`read()` method returns [pr5::R](pr5::R) reader structure"]
impl crate::Readable for PR5 {}
#[doc = "`write(|w| ..)` method takes [pr5::W](pr5::W) writer structure"]
impl crate::Writable for PR5 {}
#[doc = "PR5 register"]
pub mod pr5;
#[doc = "PR5CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr5clr](pr5clr) module"]
pub type PR5CLR = crate::Reg<u32, _PR5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR5CLR;
#[doc = "`read()` method returns [pr5clr::R](pr5clr::R) reader structure"]
impl crate::Readable for PR5CLR {}
#[doc = "`write(|w| ..)` method takes [pr5clr::W](pr5clr::W) writer structure"]
impl crate::Writable for PR5CLR {}
#[doc = "PR5CLR register"]
pub mod pr5clr;
#[doc = "PR5SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr5set](pr5set) module"]
pub type PR5SET = crate::Reg<u32, _PR5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR5SET;
#[doc = "`read()` method returns [pr5set::R](pr5set::R) reader structure"]
impl crate::Readable for PR5SET {}
#[doc = "`write(|w| ..)` method takes [pr5set::W](pr5set::W) writer structure"]
impl crate::Writable for PR5SET {}
#[doc = "PR5SET register"]
pub mod pr5set;
#[doc = "PR5INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr5inv](pr5inv) module"]
pub type PR5INV = crate::Reg<u32, _PR5INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR5INV;
#[doc = "`read()` method returns [pr5inv::R](pr5inv::R) reader structure"]
impl crate::Readable for PR5INV {}
#[doc = "`write(|w| ..)` method takes [pr5inv::W](pr5inv::W) writer structure"]
impl crate::Writable for PR5INV {}
#[doc = "PR5INV register"]
pub mod pr5inv;
