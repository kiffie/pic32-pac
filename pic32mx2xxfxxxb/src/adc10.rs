#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AD1CON1 register"]
    pub ad1con1: AD1CON1,
    #[doc = "0x04 - AD1CON1CLR register"]
    pub ad1con1clr: AD1CON1CLR,
    #[doc = "0x08 - AD1CON1SET register"]
    pub ad1con1set: AD1CON1SET,
    #[doc = "0x0c - AD1CON1INV register"]
    pub ad1con1inv: AD1CON1INV,
    #[doc = "0x10 - AD1CON2 register"]
    pub ad1con2: AD1CON2,
    #[doc = "0x14 - AD1CON2CLR register"]
    pub ad1con2clr: AD1CON2CLR,
    #[doc = "0x18 - AD1CON2SET register"]
    pub ad1con2set: AD1CON2SET,
    #[doc = "0x1c - AD1CON2INV register"]
    pub ad1con2inv: AD1CON2INV,
    #[doc = "0x20 - AD1CON3 register"]
    pub ad1con3: AD1CON3,
    #[doc = "0x24 - AD1CON3CLR register"]
    pub ad1con3clr: AD1CON3CLR,
    #[doc = "0x28 - AD1CON3SET register"]
    pub ad1con3set: AD1CON3SET,
    #[doc = "0x2c - AD1CON3INV register"]
    pub ad1con3inv: AD1CON3INV,
    _reserved12: [u8; 16usize],
    #[doc = "0x40 - AD1CHS register"]
    pub ad1chs: AD1CHS,
    #[doc = "0x44 - AD1CHSCLR register"]
    pub ad1chsclr: AD1CHSCLR,
    #[doc = "0x48 - AD1CHSSET register"]
    pub ad1chsset: AD1CHSSET,
    #[doc = "0x4c - AD1CHSINV register"]
    pub ad1chsinv: AD1CHSINV,
    #[doc = "0x50 - AD1CSSL register"]
    pub ad1cssl: AD1CSSL,
    #[doc = "0x54 - AD1CSSLCLR register"]
    pub ad1csslclr: AD1CSSLCLR,
    #[doc = "0x58 - AD1CSSLSET register"]
    pub ad1csslset: AD1CSSLSET,
    #[doc = "0x5c - AD1CSSLINV register"]
    pub ad1csslinv: AD1CSSLINV,
    _reserved20: [u8; 16usize],
    #[doc = "0x70 - ADC1BUF0 register"]
    pub adc1buf0: ADC1BUF0,
    _reserved21: [u8; 12usize],
    #[doc = "0x80 - ADC1BUF1 register"]
    pub adc1buf1: ADC1BUF1,
    _reserved22: [u8; 12usize],
    #[doc = "0x90 - ADC1BUF2 register"]
    pub adc1buf2: ADC1BUF2,
    _reserved23: [u8; 12usize],
    #[doc = "0xa0 - ADC1BUF3 register"]
    pub adc1buf3: ADC1BUF3,
    _reserved24: [u8; 12usize],
    #[doc = "0xb0 - ADC1BUF4 register"]
    pub adc1buf4: ADC1BUF4,
    _reserved25: [u8; 12usize],
    #[doc = "0xc0 - ADC1BUF5 register"]
    pub adc1buf5: ADC1BUF5,
    _reserved26: [u8; 12usize],
    #[doc = "0xd0 - ADC1BUF6 register"]
    pub adc1buf6: ADC1BUF6,
    _reserved27: [u8; 12usize],
    #[doc = "0xe0 - ADC1BUF7 register"]
    pub adc1buf7: ADC1BUF7,
    _reserved28: [u8; 12usize],
    #[doc = "0xf0 - ADC1BUF8 register"]
    pub adc1buf8: ADC1BUF8,
    _reserved29: [u8; 12usize],
    #[doc = "0x100 - ADC1BUF9 register"]
    pub adc1buf9: ADC1BUF9,
    _reserved30: [u8; 12usize],
    #[doc = "0x110 - ADC1BUFA register"]
    pub adc1bufa: ADC1BUFA,
    _reserved31: [u8; 12usize],
    #[doc = "0x120 - ADC1BUFB register"]
    pub adc1bufb: ADC1BUFB,
    _reserved32: [u8; 12usize],
    #[doc = "0x130 - ADC1BUFC register"]
    pub adc1bufc: ADC1BUFC,
    _reserved33: [u8; 12usize],
    #[doc = "0x140 - ADC1BUFD register"]
    pub adc1bufd: ADC1BUFD,
    _reserved34: [u8; 12usize],
    #[doc = "0x150 - ADC1BUFE register"]
    pub adc1bufe: ADC1BUFE,
    _reserved35: [u8; 12usize],
    #[doc = "0x160 - ADC1BUFF register"]
    pub adc1buff: ADC1BUFF,
}
#[doc = "AD1CON1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con1](ad1con1) module"]
pub type AD1CON1 = crate::Reg<u32, _AD1CON1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON1;
#[doc = "`read()` method returns [ad1con1::R](ad1con1::R) reader structure"]
impl crate::Readable for AD1CON1 {}
#[doc = "`write(|w| ..)` method takes [ad1con1::W](ad1con1::W) writer structure"]
impl crate::Writable for AD1CON1 {}
#[doc = "AD1CON1 register"]
pub mod ad1con1;
#[doc = "AD1CON1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con1clr](ad1con1clr) module"]
pub type AD1CON1CLR = crate::Reg<u32, _AD1CON1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON1CLR;
#[doc = "`read()` method returns [ad1con1clr::R](ad1con1clr::R) reader structure"]
impl crate::Readable for AD1CON1CLR {}
#[doc = "`write(|w| ..)` method takes [ad1con1clr::W](ad1con1clr::W) writer structure"]
impl crate::Writable for AD1CON1CLR {}
#[doc = "AD1CON1CLR register"]
pub mod ad1con1clr;
#[doc = "AD1CON1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con1set](ad1con1set) module"]
pub type AD1CON1SET = crate::Reg<u32, _AD1CON1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON1SET;
#[doc = "`read()` method returns [ad1con1set::R](ad1con1set::R) reader structure"]
impl crate::Readable for AD1CON1SET {}
#[doc = "`write(|w| ..)` method takes [ad1con1set::W](ad1con1set::W) writer structure"]
impl crate::Writable for AD1CON1SET {}
#[doc = "AD1CON1SET register"]
pub mod ad1con1set;
#[doc = "AD1CON1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con1inv](ad1con1inv) module"]
pub type AD1CON1INV = crate::Reg<u32, _AD1CON1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON1INV;
#[doc = "`read()` method returns [ad1con1inv::R](ad1con1inv::R) reader structure"]
impl crate::Readable for AD1CON1INV {}
#[doc = "`write(|w| ..)` method takes [ad1con1inv::W](ad1con1inv::W) writer structure"]
impl crate::Writable for AD1CON1INV {}
#[doc = "AD1CON1INV register"]
pub mod ad1con1inv;
#[doc = "AD1CON2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con2](ad1con2) module"]
pub type AD1CON2 = crate::Reg<u32, _AD1CON2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON2;
#[doc = "`read()` method returns [ad1con2::R](ad1con2::R) reader structure"]
impl crate::Readable for AD1CON2 {}
#[doc = "`write(|w| ..)` method takes [ad1con2::W](ad1con2::W) writer structure"]
impl crate::Writable for AD1CON2 {}
#[doc = "AD1CON2 register"]
pub mod ad1con2;
#[doc = "AD1CON2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con2clr](ad1con2clr) module"]
pub type AD1CON2CLR = crate::Reg<u32, _AD1CON2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON2CLR;
#[doc = "`read()` method returns [ad1con2clr::R](ad1con2clr::R) reader structure"]
impl crate::Readable for AD1CON2CLR {}
#[doc = "`write(|w| ..)` method takes [ad1con2clr::W](ad1con2clr::W) writer structure"]
impl crate::Writable for AD1CON2CLR {}
#[doc = "AD1CON2CLR register"]
pub mod ad1con2clr;
#[doc = "AD1CON2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con2set](ad1con2set) module"]
pub type AD1CON2SET = crate::Reg<u32, _AD1CON2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON2SET;
#[doc = "`read()` method returns [ad1con2set::R](ad1con2set::R) reader structure"]
impl crate::Readable for AD1CON2SET {}
#[doc = "`write(|w| ..)` method takes [ad1con2set::W](ad1con2set::W) writer structure"]
impl crate::Writable for AD1CON2SET {}
#[doc = "AD1CON2SET register"]
pub mod ad1con2set;
#[doc = "AD1CON2INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con2inv](ad1con2inv) module"]
pub type AD1CON2INV = crate::Reg<u32, _AD1CON2INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON2INV;
#[doc = "`read()` method returns [ad1con2inv::R](ad1con2inv::R) reader structure"]
impl crate::Readable for AD1CON2INV {}
#[doc = "`write(|w| ..)` method takes [ad1con2inv::W](ad1con2inv::W) writer structure"]
impl crate::Writable for AD1CON2INV {}
#[doc = "AD1CON2INV register"]
pub mod ad1con2inv;
#[doc = "AD1CON3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con3](ad1con3) module"]
pub type AD1CON3 = crate::Reg<u32, _AD1CON3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON3;
#[doc = "`read()` method returns [ad1con3::R](ad1con3::R) reader structure"]
impl crate::Readable for AD1CON3 {}
#[doc = "`write(|w| ..)` method takes [ad1con3::W](ad1con3::W) writer structure"]
impl crate::Writable for AD1CON3 {}
#[doc = "AD1CON3 register"]
pub mod ad1con3;
#[doc = "AD1CON3CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con3clr](ad1con3clr) module"]
pub type AD1CON3CLR = crate::Reg<u32, _AD1CON3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON3CLR;
#[doc = "`read()` method returns [ad1con3clr::R](ad1con3clr::R) reader structure"]
impl crate::Readable for AD1CON3CLR {}
#[doc = "`write(|w| ..)` method takes [ad1con3clr::W](ad1con3clr::W) writer structure"]
impl crate::Writable for AD1CON3CLR {}
#[doc = "AD1CON3CLR register"]
pub mod ad1con3clr;
#[doc = "AD1CON3SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con3set](ad1con3set) module"]
pub type AD1CON3SET = crate::Reg<u32, _AD1CON3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON3SET;
#[doc = "`read()` method returns [ad1con3set::R](ad1con3set::R) reader structure"]
impl crate::Readable for AD1CON3SET {}
#[doc = "`write(|w| ..)` method takes [ad1con3set::W](ad1con3set::W) writer structure"]
impl crate::Writable for AD1CON3SET {}
#[doc = "AD1CON3SET register"]
pub mod ad1con3set;
#[doc = "AD1CON3INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con3inv](ad1con3inv) module"]
pub type AD1CON3INV = crate::Reg<u32, _AD1CON3INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CON3INV;
#[doc = "`read()` method returns [ad1con3inv::R](ad1con3inv::R) reader structure"]
impl crate::Readable for AD1CON3INV {}
#[doc = "`write(|w| ..)` method takes [ad1con3inv::W](ad1con3inv::W) writer structure"]
impl crate::Writable for AD1CON3INV {}
#[doc = "AD1CON3INV register"]
pub mod ad1con3inv;
#[doc = "AD1CHS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1chs](ad1chs) module"]
pub type AD1CHS = crate::Reg<u32, _AD1CHS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CHS;
#[doc = "`read()` method returns [ad1chs::R](ad1chs::R) reader structure"]
impl crate::Readable for AD1CHS {}
#[doc = "`write(|w| ..)` method takes [ad1chs::W](ad1chs::W) writer structure"]
impl crate::Writable for AD1CHS {}
#[doc = "AD1CHS register"]
pub mod ad1chs;
#[doc = "AD1CHSCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1chsclr](ad1chsclr) module"]
pub type AD1CHSCLR = crate::Reg<u32, _AD1CHSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CHSCLR;
#[doc = "`read()` method returns [ad1chsclr::R](ad1chsclr::R) reader structure"]
impl crate::Readable for AD1CHSCLR {}
#[doc = "`write(|w| ..)` method takes [ad1chsclr::W](ad1chsclr::W) writer structure"]
impl crate::Writable for AD1CHSCLR {}
#[doc = "AD1CHSCLR register"]
pub mod ad1chsclr;
#[doc = "AD1CHSSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1chsset](ad1chsset) module"]
pub type AD1CHSSET = crate::Reg<u32, _AD1CHSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CHSSET;
#[doc = "`read()` method returns [ad1chsset::R](ad1chsset::R) reader structure"]
impl crate::Readable for AD1CHSSET {}
#[doc = "`write(|w| ..)` method takes [ad1chsset::W](ad1chsset::W) writer structure"]
impl crate::Writable for AD1CHSSET {}
#[doc = "AD1CHSSET register"]
pub mod ad1chsset;
#[doc = "AD1CHSINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1chsinv](ad1chsinv) module"]
pub type AD1CHSINV = crate::Reg<u32, _AD1CHSINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CHSINV;
#[doc = "`read()` method returns [ad1chsinv::R](ad1chsinv::R) reader structure"]
impl crate::Readable for AD1CHSINV {}
#[doc = "`write(|w| ..)` method takes [ad1chsinv::W](ad1chsinv::W) writer structure"]
impl crate::Writable for AD1CHSINV {}
#[doc = "AD1CHSINV register"]
pub mod ad1chsinv;
#[doc = "AD1CSSL register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1cssl](ad1cssl) module"]
pub type AD1CSSL = crate::Reg<u32, _AD1CSSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CSSL;
#[doc = "`read()` method returns [ad1cssl::R](ad1cssl::R) reader structure"]
impl crate::Readable for AD1CSSL {}
#[doc = "`write(|w| ..)` method takes [ad1cssl::W](ad1cssl::W) writer structure"]
impl crate::Writable for AD1CSSL {}
#[doc = "AD1CSSL register"]
pub mod ad1cssl;
#[doc = "AD1CSSLCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1csslclr](ad1csslclr) module"]
pub type AD1CSSLCLR = crate::Reg<u32, _AD1CSSLCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CSSLCLR;
#[doc = "`read()` method returns [ad1csslclr::R](ad1csslclr::R) reader structure"]
impl crate::Readable for AD1CSSLCLR {}
#[doc = "`write(|w| ..)` method takes [ad1csslclr::W](ad1csslclr::W) writer structure"]
impl crate::Writable for AD1CSSLCLR {}
#[doc = "AD1CSSLCLR register"]
pub mod ad1csslclr;
#[doc = "AD1CSSLSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1csslset](ad1csslset) module"]
pub type AD1CSSLSET = crate::Reg<u32, _AD1CSSLSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CSSLSET;
#[doc = "`read()` method returns [ad1csslset::R](ad1csslset::R) reader structure"]
impl crate::Readable for AD1CSSLSET {}
#[doc = "`write(|w| ..)` method takes [ad1csslset::W](ad1csslset::W) writer structure"]
impl crate::Writable for AD1CSSLSET {}
#[doc = "AD1CSSLSET register"]
pub mod ad1csslset;
#[doc = "AD1CSSLINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1csslinv](ad1csslinv) module"]
pub type AD1CSSLINV = crate::Reg<u32, _AD1CSSLINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AD1CSSLINV;
#[doc = "`read()` method returns [ad1csslinv::R](ad1csslinv::R) reader structure"]
impl crate::Readable for AD1CSSLINV {}
#[doc = "`write(|w| ..)` method takes [ad1csslinv::W](ad1csslinv::W) writer structure"]
impl crate::Writable for AD1CSSLINV {}
#[doc = "AD1CSSLINV register"]
pub mod ad1csslinv;
#[doc = "ADC1BUF0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf0](adc1buf0) module"]
pub type ADC1BUF0 = crate::Reg<u32, _ADC1BUF0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF0;
#[doc = "`read()` method returns [adc1buf0::R](adc1buf0::R) reader structure"]
impl crate::Readable for ADC1BUF0 {}
#[doc = "`write(|w| ..)` method takes [adc1buf0::W](adc1buf0::W) writer structure"]
impl crate::Writable for ADC1BUF0 {}
#[doc = "ADC1BUF0 register"]
pub mod adc1buf0;
#[doc = "ADC1BUF1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf1](adc1buf1) module"]
pub type ADC1BUF1 = crate::Reg<u32, _ADC1BUF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF1;
#[doc = "`read()` method returns [adc1buf1::R](adc1buf1::R) reader structure"]
impl crate::Readable for ADC1BUF1 {}
#[doc = "`write(|w| ..)` method takes [adc1buf1::W](adc1buf1::W) writer structure"]
impl crate::Writable for ADC1BUF1 {}
#[doc = "ADC1BUF1 register"]
pub mod adc1buf1;
#[doc = "ADC1BUF2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf2](adc1buf2) module"]
pub type ADC1BUF2 = crate::Reg<u32, _ADC1BUF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF2;
#[doc = "`read()` method returns [adc1buf2::R](adc1buf2::R) reader structure"]
impl crate::Readable for ADC1BUF2 {}
#[doc = "`write(|w| ..)` method takes [adc1buf2::W](adc1buf2::W) writer structure"]
impl crate::Writable for ADC1BUF2 {}
#[doc = "ADC1BUF2 register"]
pub mod adc1buf2;
#[doc = "ADC1BUF3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf3](adc1buf3) module"]
pub type ADC1BUF3 = crate::Reg<u32, _ADC1BUF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF3;
#[doc = "`read()` method returns [adc1buf3::R](adc1buf3::R) reader structure"]
impl crate::Readable for ADC1BUF3 {}
#[doc = "`write(|w| ..)` method takes [adc1buf3::W](adc1buf3::W) writer structure"]
impl crate::Writable for ADC1BUF3 {}
#[doc = "ADC1BUF3 register"]
pub mod adc1buf3;
#[doc = "ADC1BUF4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf4](adc1buf4) module"]
pub type ADC1BUF4 = crate::Reg<u32, _ADC1BUF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF4;
#[doc = "`read()` method returns [adc1buf4::R](adc1buf4::R) reader structure"]
impl crate::Readable for ADC1BUF4 {}
#[doc = "`write(|w| ..)` method takes [adc1buf4::W](adc1buf4::W) writer structure"]
impl crate::Writable for ADC1BUF4 {}
#[doc = "ADC1BUF4 register"]
pub mod adc1buf4;
#[doc = "ADC1BUF5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf5](adc1buf5) module"]
pub type ADC1BUF5 = crate::Reg<u32, _ADC1BUF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF5;
#[doc = "`read()` method returns [adc1buf5::R](adc1buf5::R) reader structure"]
impl crate::Readable for ADC1BUF5 {}
#[doc = "`write(|w| ..)` method takes [adc1buf5::W](adc1buf5::W) writer structure"]
impl crate::Writable for ADC1BUF5 {}
#[doc = "ADC1BUF5 register"]
pub mod adc1buf5;
#[doc = "ADC1BUF6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf6](adc1buf6) module"]
pub type ADC1BUF6 = crate::Reg<u32, _ADC1BUF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF6;
#[doc = "`read()` method returns [adc1buf6::R](adc1buf6::R) reader structure"]
impl crate::Readable for ADC1BUF6 {}
#[doc = "`write(|w| ..)` method takes [adc1buf6::W](adc1buf6::W) writer structure"]
impl crate::Writable for ADC1BUF6 {}
#[doc = "ADC1BUF6 register"]
pub mod adc1buf6;
#[doc = "ADC1BUF7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf7](adc1buf7) module"]
pub type ADC1BUF7 = crate::Reg<u32, _ADC1BUF7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF7;
#[doc = "`read()` method returns [adc1buf7::R](adc1buf7::R) reader structure"]
impl crate::Readable for ADC1BUF7 {}
#[doc = "`write(|w| ..)` method takes [adc1buf7::W](adc1buf7::W) writer structure"]
impl crate::Writable for ADC1BUF7 {}
#[doc = "ADC1BUF7 register"]
pub mod adc1buf7;
#[doc = "ADC1BUF8 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf8](adc1buf8) module"]
pub type ADC1BUF8 = crate::Reg<u32, _ADC1BUF8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF8;
#[doc = "`read()` method returns [adc1buf8::R](adc1buf8::R) reader structure"]
impl crate::Readable for ADC1BUF8 {}
#[doc = "`write(|w| ..)` method takes [adc1buf8::W](adc1buf8::W) writer structure"]
impl crate::Writable for ADC1BUF8 {}
#[doc = "ADC1BUF8 register"]
pub mod adc1buf8;
#[doc = "ADC1BUF9 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buf9](adc1buf9) module"]
pub type ADC1BUF9 = crate::Reg<u32, _ADC1BUF9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUF9;
#[doc = "`read()` method returns [adc1buf9::R](adc1buf9::R) reader structure"]
impl crate::Readable for ADC1BUF9 {}
#[doc = "`write(|w| ..)` method takes [adc1buf9::W](adc1buf9::W) writer structure"]
impl crate::Writable for ADC1BUF9 {}
#[doc = "ADC1BUF9 register"]
pub mod adc1buf9;
#[doc = "ADC1BUFA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1bufa](adc1bufa) module"]
pub type ADC1BUFA = crate::Reg<u32, _ADC1BUFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUFA;
#[doc = "`read()` method returns [adc1bufa::R](adc1bufa::R) reader structure"]
impl crate::Readable for ADC1BUFA {}
#[doc = "`write(|w| ..)` method takes [adc1bufa::W](adc1bufa::W) writer structure"]
impl crate::Writable for ADC1BUFA {}
#[doc = "ADC1BUFA register"]
pub mod adc1bufa;
#[doc = "ADC1BUFB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1bufb](adc1bufb) module"]
pub type ADC1BUFB = crate::Reg<u32, _ADC1BUFB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUFB;
#[doc = "`read()` method returns [adc1bufb::R](adc1bufb::R) reader structure"]
impl crate::Readable for ADC1BUFB {}
#[doc = "`write(|w| ..)` method takes [adc1bufb::W](adc1bufb::W) writer structure"]
impl crate::Writable for ADC1BUFB {}
#[doc = "ADC1BUFB register"]
pub mod adc1bufb;
#[doc = "ADC1BUFC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1bufc](adc1bufc) module"]
pub type ADC1BUFC = crate::Reg<u32, _ADC1BUFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUFC;
#[doc = "`read()` method returns [adc1bufc::R](adc1bufc::R) reader structure"]
impl crate::Readable for ADC1BUFC {}
#[doc = "`write(|w| ..)` method takes [adc1bufc::W](adc1bufc::W) writer structure"]
impl crate::Writable for ADC1BUFC {}
#[doc = "ADC1BUFC register"]
pub mod adc1bufc;
#[doc = "ADC1BUFD register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1bufd](adc1bufd) module"]
pub type ADC1BUFD = crate::Reg<u32, _ADC1BUFD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUFD;
#[doc = "`read()` method returns [adc1bufd::R](adc1bufd::R) reader structure"]
impl crate::Readable for ADC1BUFD {}
#[doc = "`write(|w| ..)` method takes [adc1bufd::W](adc1bufd::W) writer structure"]
impl crate::Writable for ADC1BUFD {}
#[doc = "ADC1BUFD register"]
pub mod adc1bufd;
#[doc = "ADC1BUFE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1bufe](adc1bufe) module"]
pub type ADC1BUFE = crate::Reg<u32, _ADC1BUFE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUFE;
#[doc = "`read()` method returns [adc1bufe::R](adc1bufe::R) reader structure"]
impl crate::Readable for ADC1BUFE {}
#[doc = "`write(|w| ..)` method takes [adc1bufe::W](adc1bufe::W) writer structure"]
impl crate::Writable for ADC1BUFE {}
#[doc = "ADC1BUFE register"]
pub mod adc1bufe;
#[doc = "ADC1BUFF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1buff](adc1buff) module"]
pub type ADC1BUFF = crate::Reg<u32, _ADC1BUFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1BUFF;
#[doc = "`read()` method returns [adc1buff::R](adc1buff::R) reader structure"]
impl crate::Readable for ADC1BUFF {}
#[doc = "`write(|w| ..)` method takes [adc1buff::W](adc1buff::W) writer structure"]
impl crate::Writable for ADC1BUFF {}
#[doc = "ADC1BUFF register"]
pub mod adc1buff;
