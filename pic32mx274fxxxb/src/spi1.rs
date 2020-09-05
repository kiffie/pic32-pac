#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPI1CON register"]
    pub con1: CON1,
    #[doc = "0x04 - SPI1CONCLR register"]
    pub conclr: CONCLR,
    #[doc = "0x08 - SPI1CONSET register"]
    pub conset: CONSET,
    #[doc = "0x0c - SPI1CONINV register"]
    pub coninv: CONINV,
    #[doc = "0x10 - SPI1STAT register"]
    pub stat: STAT,
    #[doc = "0x14 - SPI1STATCLR register"]
    pub statclr: STATCLR,
    #[doc = "0x18 - SPI1STATSET register"]
    pub statset: STATSET,
    #[doc = "0x1c - SPI1STATINV register"]
    pub statinv: STATINV,
    #[doc = "0x20 - SPI1BUF register"]
    pub buf: BUF,
    _reserved9: [u8; 12usize],
    #[doc = "0x30 - SPI1BRG register"]
    pub brg: BRG,
    #[doc = "0x34 - SPI1BRGCLR register"]
    pub brgclr: BRGCLR,
    #[doc = "0x38 - SPI1BRGSET register"]
    pub brgset: BRGSET,
    #[doc = "0x3c - SPI1BRGINV register"]
    pub brginv: BRGINV,
    #[doc = "0x40 - SPI1CON2 register"]
    pub con2: CON2,
    #[doc = "0x44 - SPI1CON2CLR register"]
    pub con2clr: CON2CLR,
    #[doc = "0x48 - SPI1CON2SET register"]
    pub con2set: CON2SET,
    #[doc = "0x4c - SPI1CON2INV register"]
    pub con2inv: CON2INV,
}
#[doc = "SPI1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con1](con1) module"]
pub type CON1 = crate::Reg<u32, _CON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON1;
#[doc = "`read()` method returns [con1::R](con1::R) reader structure"]
impl crate::Readable for CON1 {}
#[doc = "`write(|w| ..)` method takes [con1::W](con1::W) writer structure"]
impl crate::Writable for CON1 {}
#[doc = "SPI1CON register"]
pub mod con1;
#[doc = "SPI1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conclr](conclr) module"]
pub type CONCLR = crate::Reg<u32, _CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONCLR;
#[doc = "`read()` method returns [conclr::R](conclr::R) reader structure"]
impl crate::Readable for CONCLR {}
#[doc = "`write(|w| ..)` method takes [conclr::W](conclr::W) writer structure"]
impl crate::Writable for CONCLR {}
#[doc = "SPI1CONCLR register"]
pub mod conclr;
#[doc = "SPI1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conset](conset) module"]
pub type CONSET = crate::Reg<u32, _CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONSET;
#[doc = "`read()` method returns [conset::R](conset::R) reader structure"]
impl crate::Readable for CONSET {}
#[doc = "`write(|w| ..)` method takes [conset::W](conset::W) writer structure"]
impl crate::Writable for CONSET {}
#[doc = "SPI1CONSET register"]
pub mod conset;
#[doc = "SPI1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coninv](coninv) module"]
pub type CONINV = crate::Reg<u32, _CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONINV;
#[doc = "`read()` method returns [coninv::R](coninv::R) reader structure"]
impl crate::Readable for CONINV {}
#[doc = "`write(|w| ..)` method takes [coninv::W](coninv::W) writer structure"]
impl crate::Writable for CONINV {}
#[doc = "SPI1CONINV register"]
pub mod coninv;
#[doc = "SPI1STAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "SPI1STAT register"]
pub mod stat;
#[doc = "SPI1STATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statclr](statclr) module"]
pub type STATCLR = crate::Reg<u32, _STATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATCLR;
#[doc = "`read()` method returns [statclr::R](statclr::R) reader structure"]
impl crate::Readable for STATCLR {}
#[doc = "`write(|w| ..)` method takes [statclr::W](statclr::W) writer structure"]
impl crate::Writable for STATCLR {}
#[doc = "SPI1STATCLR register"]
pub mod statclr;
#[doc = "SPI1STATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statset](statset) module"]
pub type STATSET = crate::Reg<u32, _STATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATSET;
#[doc = "`read()` method returns [statset::R](statset::R) reader structure"]
impl crate::Readable for STATSET {}
#[doc = "`write(|w| ..)` method takes [statset::W](statset::W) writer structure"]
impl crate::Writable for STATSET {}
#[doc = "SPI1STATSET register"]
pub mod statset;
#[doc = "SPI1STATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statinv](statinv) module"]
pub type STATINV = crate::Reg<u32, _STATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATINV;
#[doc = "`read()` method returns [statinv::R](statinv::R) reader structure"]
impl crate::Readable for STATINV {}
#[doc = "`write(|w| ..)` method takes [statinv::W](statinv::W) writer structure"]
impl crate::Writable for STATINV {}
#[doc = "SPI1STATINV register"]
pub mod statinv;
#[doc = "SPI1BUF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf](buf) module"]
pub type BUF = crate::Reg<u32, _BUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUF;
#[doc = "`read()` method returns [buf::R](buf::R) reader structure"]
impl crate::Readable for BUF {}
#[doc = "`write(|w| ..)` method takes [buf::W](buf::W) writer structure"]
impl crate::Writable for BUF {}
#[doc = "SPI1BUF register"]
pub mod buf;
#[doc = "SPI1BRG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brg](brg) module"]
pub type BRG = crate::Reg<u32, _BRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRG;
#[doc = "`read()` method returns [brg::R](brg::R) reader structure"]
impl crate::Readable for BRG {}
#[doc = "`write(|w| ..)` method takes [brg::W](brg::W) writer structure"]
impl crate::Writable for BRG {}
#[doc = "SPI1BRG register"]
pub mod brg;
#[doc = "SPI1BRGCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgclr](brgclr) module"]
pub type BRGCLR = crate::Reg<u32, _BRGCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGCLR;
#[doc = "`read()` method returns [brgclr::R](brgclr::R) reader structure"]
impl crate::Readable for BRGCLR {}
#[doc = "`write(|w| ..)` method takes [brgclr::W](brgclr::W) writer structure"]
impl crate::Writable for BRGCLR {}
#[doc = "SPI1BRGCLR register"]
pub mod brgclr;
#[doc = "SPI1BRGSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgset](brgset) module"]
pub type BRGSET = crate::Reg<u32, _BRGSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGSET;
#[doc = "`read()` method returns [brgset::R](brgset::R) reader structure"]
impl crate::Readable for BRGSET {}
#[doc = "`write(|w| ..)` method takes [brgset::W](brgset::W) writer structure"]
impl crate::Writable for BRGSET {}
#[doc = "SPI1BRGSET register"]
pub mod brgset;
#[doc = "SPI1BRGINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brginv](brginv) module"]
pub type BRGINV = crate::Reg<u32, _BRGINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGINV;
#[doc = "`read()` method returns [brginv::R](brginv::R) reader structure"]
impl crate::Readable for BRGINV {}
#[doc = "`write(|w| ..)` method takes [brginv::W](brginv::W) writer structure"]
impl crate::Writable for BRGINV {}
#[doc = "SPI1BRGINV register"]
pub mod brginv;
#[doc = "SPI1CON2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con2](con2) module"]
pub type CON2 = crate::Reg<u32, _CON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON2;
#[doc = "`read()` method returns [con2::R](con2::R) reader structure"]
impl crate::Readable for CON2 {}
#[doc = "`write(|w| ..)` method takes [con2::W](con2::W) writer structure"]
impl crate::Writable for CON2 {}
#[doc = "SPI1CON2 register"]
pub mod con2;
#[doc = "SPI1CON2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con2clr](con2clr) module"]
pub type CON2CLR = crate::Reg<u32, _CON2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON2CLR;
#[doc = "`read()` method returns [con2clr::R](con2clr::R) reader structure"]
impl crate::Readable for CON2CLR {}
#[doc = "`write(|w| ..)` method takes [con2clr::W](con2clr::W) writer structure"]
impl crate::Writable for CON2CLR {}
#[doc = "SPI1CON2CLR register"]
pub mod con2clr;
#[doc = "SPI1CON2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con2set](con2set) module"]
pub type CON2SET = crate::Reg<u32, _CON2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON2SET;
#[doc = "`read()` method returns [con2set::R](con2set::R) reader structure"]
impl crate::Readable for CON2SET {}
#[doc = "`write(|w| ..)` method takes [con2set::W](con2set::W) writer structure"]
impl crate::Writable for CON2SET {}
#[doc = "SPI1CON2SET register"]
pub mod con2set;
#[doc = "SPI1CON2INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con2inv](con2inv) module"]
pub type CON2INV = crate::Reg<u32, _CON2INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON2INV;
#[doc = "`read()` method returns [con2inv::R](con2inv::R) reader structure"]
impl crate::Readable for CON2INV {}
#[doc = "`write(|w| ..)` method takes [con2inv::W](con2inv::W) writer structure"]
impl crate::Writable for CON2INV {}
#[doc = "SPI1CON2INV register"]
pub mod con2inv;
