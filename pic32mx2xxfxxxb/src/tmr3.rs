#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T3CON register"]
    pub t3con: T3CON,
    #[doc = "0x04 - T3CONCLR register"]
    pub t3conclr: T3CONCLR,
    #[doc = "0x08 - T3CONSET register"]
    pub t3conset: T3CONSET,
    #[doc = "0x0c - T3CONINV register"]
    pub t3coninv: T3CONINV,
    #[doc = "0x10 - TMR3 register"]
    pub tmr3: TMR3,
    #[doc = "0x14 - TMR3CLR register"]
    pub tmr3clr: TMR3CLR,
    #[doc = "0x18 - TMR3SET register"]
    pub tmr3set: TMR3SET,
    #[doc = "0x1c - TMR3INV register"]
    pub tmr3inv: TMR3INV,
    #[doc = "0x20 - PR3 register"]
    pub pr3: PR3,
    #[doc = "0x24 - PR3CLR register"]
    pub pr3clr: PR3CLR,
    #[doc = "0x28 - PR3SET register"]
    pub pr3set: PR3SET,
    #[doc = "0x2c - PR3INV register"]
    pub pr3inv: PR3INV,
}
#[doc = "T3CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3con](t3con) module"]
pub type T3CON = crate::Reg<u32, _T3CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T3CON;
#[doc = "`read()` method returns [t3con::R](t3con::R) reader structure"]
impl crate::Readable for T3CON {}
#[doc = "`write(|w| ..)` method takes [t3con::W](t3con::W) writer structure"]
impl crate::Writable for T3CON {}
#[doc = "T3CON register"]
pub mod t3con;
#[doc = "T3CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3conclr](t3conclr) module"]
pub type T3CONCLR = crate::Reg<u32, _T3CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T3CONCLR;
#[doc = "`read()` method returns [t3conclr::R](t3conclr::R) reader structure"]
impl crate::Readable for T3CONCLR {}
#[doc = "`write(|w| ..)` method takes [t3conclr::W](t3conclr::W) writer structure"]
impl crate::Writable for T3CONCLR {}
#[doc = "T3CONCLR register"]
pub mod t3conclr;
#[doc = "T3CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3conset](t3conset) module"]
pub type T3CONSET = crate::Reg<u32, _T3CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T3CONSET;
#[doc = "`read()` method returns [t3conset::R](t3conset::R) reader structure"]
impl crate::Readable for T3CONSET {}
#[doc = "`write(|w| ..)` method takes [t3conset::W](t3conset::W) writer structure"]
impl crate::Writable for T3CONSET {}
#[doc = "T3CONSET register"]
pub mod t3conset;
#[doc = "T3CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3coninv](t3coninv) module"]
pub type T3CONINV = crate::Reg<u32, _T3CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T3CONINV;
#[doc = "`read()` method returns [t3coninv::R](t3coninv::R) reader structure"]
impl crate::Readable for T3CONINV {}
#[doc = "`write(|w| ..)` method takes [t3coninv::W](t3coninv::W) writer structure"]
impl crate::Writable for T3CONINV {}
#[doc = "T3CONINV register"]
pub mod t3coninv;
#[doc = "TMR3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3](tmr3) module"]
pub type TMR3 = crate::Reg<u32, _TMR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3;
#[doc = "`read()` method returns [tmr3::R](tmr3::R) reader structure"]
impl crate::Readable for TMR3 {}
#[doc = "`write(|w| ..)` method takes [tmr3::W](tmr3::W) writer structure"]
impl crate::Writable for TMR3 {}
#[doc = "TMR3 register"]
pub mod tmr3;
#[doc = "TMR3CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3clr](tmr3clr) module"]
pub type TMR3CLR = crate::Reg<u32, _TMR3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3CLR;
#[doc = "`read()` method returns [tmr3clr::R](tmr3clr::R) reader structure"]
impl crate::Readable for TMR3CLR {}
#[doc = "`write(|w| ..)` method takes [tmr3clr::W](tmr3clr::W) writer structure"]
impl crate::Writable for TMR3CLR {}
#[doc = "TMR3CLR register"]
pub mod tmr3clr;
#[doc = "TMR3SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3set](tmr3set) module"]
pub type TMR3SET = crate::Reg<u32, _TMR3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3SET;
#[doc = "`read()` method returns [tmr3set::R](tmr3set::R) reader structure"]
impl crate::Readable for TMR3SET {}
#[doc = "`write(|w| ..)` method takes [tmr3set::W](tmr3set::W) writer structure"]
impl crate::Writable for TMR3SET {}
#[doc = "TMR3SET register"]
pub mod tmr3set;
#[doc = "TMR3INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3inv](tmr3inv) module"]
pub type TMR3INV = crate::Reg<u32, _TMR3INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR3INV;
#[doc = "`read()` method returns [tmr3inv::R](tmr3inv::R) reader structure"]
impl crate::Readable for TMR3INV {}
#[doc = "`write(|w| ..)` method takes [tmr3inv::W](tmr3inv::W) writer structure"]
impl crate::Writable for TMR3INV {}
#[doc = "TMR3INV register"]
pub mod tmr3inv;
#[doc = "PR3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr3](pr3) module"]
pub type PR3 = crate::Reg<u32, _PR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR3;
#[doc = "`read()` method returns [pr3::R](pr3::R) reader structure"]
impl crate::Readable for PR3 {}
#[doc = "`write(|w| ..)` method takes [pr3::W](pr3::W) writer structure"]
impl crate::Writable for PR3 {}
#[doc = "PR3 register"]
pub mod pr3;
#[doc = "PR3CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr3clr](pr3clr) module"]
pub type PR3CLR = crate::Reg<u32, _PR3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR3CLR;
#[doc = "`read()` method returns [pr3clr::R](pr3clr::R) reader structure"]
impl crate::Readable for PR3CLR {}
#[doc = "`write(|w| ..)` method takes [pr3clr::W](pr3clr::W) writer structure"]
impl crate::Writable for PR3CLR {}
#[doc = "PR3CLR register"]
pub mod pr3clr;
#[doc = "PR3SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr3set](pr3set) module"]
pub type PR3SET = crate::Reg<u32, _PR3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR3SET;
#[doc = "`read()` method returns [pr3set::R](pr3set::R) reader structure"]
impl crate::Readable for PR3SET {}
#[doc = "`write(|w| ..)` method takes [pr3set::W](pr3set::W) writer structure"]
impl crate::Writable for PR3SET {}
#[doc = "PR3SET register"]
pub mod pr3set;
#[doc = "PR3INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr3inv](pr3inv) module"]
pub type PR3INV = crate::Reg<u32, _PR3INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR3INV;
#[doc = "`read()` method returns [pr3inv::R](pr3inv::R) reader structure"]
impl crate::Readable for PR3INV {}
#[doc = "`write(|w| ..)` method takes [pr3inv::W](pr3inv::W) writer structure"]
impl crate::Writable for PR3INV {}
#[doc = "PR3INV register"]
pub mod pr3inv;
