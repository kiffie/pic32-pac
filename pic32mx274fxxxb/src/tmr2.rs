#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T2CON register"]
    pub t2con: T2CON,
    #[doc = "0x04 - T2CONCLR register"]
    pub t2conclr: T2CONCLR,
    #[doc = "0x08 - T2CONSET register"]
    pub t2conset: T2CONSET,
    #[doc = "0x0c - T2CONINV register"]
    pub t2coninv: T2CONINV,
    #[doc = "0x10 - TMR2 register"]
    pub tmr2: TMR2,
    #[doc = "0x14 - TMR2CLR register"]
    pub tmr2clr: TMR2CLR,
    #[doc = "0x18 - TMR2SET register"]
    pub tmr2set: TMR2SET,
    #[doc = "0x1c - TMR2INV register"]
    pub tmr2inv: TMR2INV,
    #[doc = "0x20 - PR2 register"]
    pub pr2: PR2,
    #[doc = "0x24 - PR2CLR register"]
    pub pr2clr: PR2CLR,
    #[doc = "0x28 - PR2SET register"]
    pub pr2set: PR2SET,
    #[doc = "0x2c - PR2INV register"]
    pub pr2inv: PR2INV,
}
#[doc = "T2CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2con](t2con) module"]
pub type T2CON = crate::Reg<u32, _T2CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T2CON;
#[doc = "`read()` method returns [t2con::R](t2con::R) reader structure"]
impl crate::Readable for T2CON {}
#[doc = "`write(|w| ..)` method takes [t2con::W](t2con::W) writer structure"]
impl crate::Writable for T2CON {}
#[doc = "T2CON register"]
pub mod t2con;
#[doc = "T2CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2conclr](t2conclr) module"]
pub type T2CONCLR = crate::Reg<u32, _T2CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T2CONCLR;
#[doc = "`read()` method returns [t2conclr::R](t2conclr::R) reader structure"]
impl crate::Readable for T2CONCLR {}
#[doc = "`write(|w| ..)` method takes [t2conclr::W](t2conclr::W) writer structure"]
impl crate::Writable for T2CONCLR {}
#[doc = "T2CONCLR register"]
pub mod t2conclr;
#[doc = "T2CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2conset](t2conset) module"]
pub type T2CONSET = crate::Reg<u32, _T2CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T2CONSET;
#[doc = "`read()` method returns [t2conset::R](t2conset::R) reader structure"]
impl crate::Readable for T2CONSET {}
#[doc = "`write(|w| ..)` method takes [t2conset::W](t2conset::W) writer structure"]
impl crate::Writable for T2CONSET {}
#[doc = "T2CONSET register"]
pub mod t2conset;
#[doc = "T2CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2coninv](t2coninv) module"]
pub type T2CONINV = crate::Reg<u32, _T2CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T2CONINV;
#[doc = "`read()` method returns [t2coninv::R](t2coninv::R) reader structure"]
impl crate::Readable for T2CONINV {}
#[doc = "`write(|w| ..)` method takes [t2coninv::W](t2coninv::W) writer structure"]
impl crate::Writable for T2CONINV {}
#[doc = "T2CONINV register"]
pub mod t2coninv;
#[doc = "TMR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2](tmr2) module"]
pub type TMR2 = crate::Reg<u32, _TMR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2;
#[doc = "`read()` method returns [tmr2::R](tmr2::R) reader structure"]
impl crate::Readable for TMR2 {}
#[doc = "`write(|w| ..)` method takes [tmr2::W](tmr2::W) writer structure"]
impl crate::Writable for TMR2 {}
#[doc = "TMR2 register"]
pub mod tmr2;
#[doc = "TMR2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2clr](tmr2clr) module"]
pub type TMR2CLR = crate::Reg<u32, _TMR2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2CLR;
#[doc = "`read()` method returns [tmr2clr::R](tmr2clr::R) reader structure"]
impl crate::Readable for TMR2CLR {}
#[doc = "`write(|w| ..)` method takes [tmr2clr::W](tmr2clr::W) writer structure"]
impl crate::Writable for TMR2CLR {}
#[doc = "TMR2CLR register"]
pub mod tmr2clr;
#[doc = "TMR2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2set](tmr2set) module"]
pub type TMR2SET = crate::Reg<u32, _TMR2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2SET;
#[doc = "`read()` method returns [tmr2set::R](tmr2set::R) reader structure"]
impl crate::Readable for TMR2SET {}
#[doc = "`write(|w| ..)` method takes [tmr2set::W](tmr2set::W) writer structure"]
impl crate::Writable for TMR2SET {}
#[doc = "TMR2SET register"]
pub mod tmr2set;
#[doc = "TMR2INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2inv](tmr2inv) module"]
pub type TMR2INV = crate::Reg<u32, _TMR2INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR2INV;
#[doc = "`read()` method returns [tmr2inv::R](tmr2inv::R) reader structure"]
impl crate::Readable for TMR2INV {}
#[doc = "`write(|w| ..)` method takes [tmr2inv::W](tmr2inv::W) writer structure"]
impl crate::Writable for TMR2INV {}
#[doc = "TMR2INV register"]
pub mod tmr2inv;
#[doc = "PR2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2](pr2) module"]
pub type PR2 = crate::Reg<u32, _PR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR2;
#[doc = "`read()` method returns [pr2::R](pr2::R) reader structure"]
impl crate::Readable for PR2 {}
#[doc = "`write(|w| ..)` method takes [pr2::W](pr2::W) writer structure"]
impl crate::Writable for PR2 {}
#[doc = "PR2 register"]
pub mod pr2;
#[doc = "PR2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2clr](pr2clr) module"]
pub type PR2CLR = crate::Reg<u32, _PR2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR2CLR;
#[doc = "`read()` method returns [pr2clr::R](pr2clr::R) reader structure"]
impl crate::Readable for PR2CLR {}
#[doc = "`write(|w| ..)` method takes [pr2clr::W](pr2clr::W) writer structure"]
impl crate::Writable for PR2CLR {}
#[doc = "PR2CLR register"]
pub mod pr2clr;
#[doc = "PR2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2set](pr2set) module"]
pub type PR2SET = crate::Reg<u32, _PR2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR2SET;
#[doc = "`read()` method returns [pr2set::R](pr2set::R) reader structure"]
impl crate::Readable for PR2SET {}
#[doc = "`write(|w| ..)` method takes [pr2set::W](pr2set::W) writer structure"]
impl crate::Writable for PR2SET {}
#[doc = "PR2SET register"]
pub mod pr2set;
#[doc = "PR2INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr2inv](pr2inv) module"]
pub type PR2INV = crate::Reg<u32, _PR2INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PR2INV;
#[doc = "`read()` method returns [pr2inv::R](pr2inv::R) reader structure"]
impl crate::Readable for PR2INV {}
#[doc = "`write(|w| ..)` method takes [pr2inv::W](pr2inv::W) writer structure"]
impl crate::Writable for PR2INV {}
#[doc = "PR2INV register"]
pub mod pr2inv;
