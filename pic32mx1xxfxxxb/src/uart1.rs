#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U1MODE register"]
    pub mode: MODE,
    #[doc = "0x04 - U1MODECLR register"]
    pub modeclr: MODECLR,
    #[doc = "0x08 - U1MODESET register"]
    pub modeset: MODESET,
    #[doc = "0x0c - U1MODEINV register"]
    pub modeinv: MODEINV,
    #[doc = "0x10 - U1STA register"]
    pub sta: STA,
    #[doc = "0x14 - U1STACLR register"]
    pub staclr: STACLR,
    #[doc = "0x18 - U1STASET register"]
    pub staset: STASET,
    #[doc = "0x1c - U1STAINV register"]
    pub stainv: STAINV,
    #[doc = "0x20 - U1TXREG register"]
    pub txreg: TXREG,
    _reserved9: [u8; 12usize],
    #[doc = "0x30 - U1RXREG register"]
    pub rxreg: RXREG,
    _reserved10: [u8; 12usize],
    #[doc = "0x40 - U1BRG register"]
    pub brg: BRG,
    #[doc = "0x44 - U1BRGCLR register"]
    pub brgclr: BRGCLR,
    #[doc = "0x48 - U1BRGSET register"]
    pub brgset: BRGSET,
    #[doc = "0x4c - U1BRGINV register"]
    pub brginv: BRGINV,
}
#[doc = "U1MODE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "U1MODE register"]
pub mod mode;
#[doc = "U1MODECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modeclr](modeclr) module"]
pub type MODECLR = crate::Reg<u32, _MODECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODECLR;
#[doc = "`read()` method returns [modeclr::R](modeclr::R) reader structure"]
impl crate::Readable for MODECLR {}
#[doc = "`write(|w| ..)` method takes [modeclr::W](modeclr::W) writer structure"]
impl crate::Writable for MODECLR {}
#[doc = "U1MODECLR register"]
pub mod modeclr;
#[doc = "U1MODESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modeset](modeset) module"]
pub type MODESET = crate::Reg<u32, _MODESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODESET;
#[doc = "`read()` method returns [modeset::R](modeset::R) reader structure"]
impl crate::Readable for MODESET {}
#[doc = "`write(|w| ..)` method takes [modeset::W](modeset::W) writer structure"]
impl crate::Writable for MODESET {}
#[doc = "U1MODESET register"]
pub mod modeset;
#[doc = "U1MODEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modeinv](modeinv) module"]
pub type MODEINV = crate::Reg<u32, _MODEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODEINV;
#[doc = "`read()` method returns [modeinv::R](modeinv::R) reader structure"]
impl crate::Readable for MODEINV {}
#[doc = "`write(|w| ..)` method takes [modeinv::W](modeinv::W) writer structure"]
impl crate::Writable for MODEINV {}
#[doc = "U1MODEINV register"]
pub mod modeinv;
#[doc = "U1STA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](sta) module"]
pub type STA = crate::Reg<u32, _STA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STA;
#[doc = "`read()` method returns [sta::R](sta::R) reader structure"]
impl crate::Readable for STA {}
#[doc = "`write(|w| ..)` method takes [sta::W](sta::W) writer structure"]
impl crate::Writable for STA {}
#[doc = "U1STA register"]
pub mod sta;
#[doc = "U1STACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staclr](staclr) module"]
pub type STACLR = crate::Reg<u32, _STACLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STACLR;
#[doc = "`read()` method returns [staclr::R](staclr::R) reader structure"]
impl crate::Readable for STACLR {}
#[doc = "`write(|w| ..)` method takes [staclr::W](staclr::W) writer structure"]
impl crate::Writable for STACLR {}
#[doc = "U1STACLR register"]
pub mod staclr;
#[doc = "U1STASET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staset](staset) module"]
pub type STASET = crate::Reg<u32, _STASET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STASET;
#[doc = "`read()` method returns [staset::R](staset::R) reader structure"]
impl crate::Readable for STASET {}
#[doc = "`write(|w| ..)` method takes [staset::W](staset::W) writer structure"]
impl crate::Writable for STASET {}
#[doc = "U1STASET register"]
pub mod staset;
#[doc = "U1STAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stainv](stainv) module"]
pub type STAINV = crate::Reg<u32, _STAINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAINV;
#[doc = "`read()` method returns [stainv::R](stainv::R) reader structure"]
impl crate::Readable for STAINV {}
#[doc = "`write(|w| ..)` method takes [stainv::W](stainv::W) writer structure"]
impl crate::Writable for STAINV {}
#[doc = "U1STAINV register"]
pub mod stainv;
#[doc = "U1TXREG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txreg](txreg) module"]
pub type TXREG = crate::Reg<u32, _TXREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXREG;
#[doc = "`read()` method returns [txreg::R](txreg::R) reader structure"]
impl crate::Readable for TXREG {}
#[doc = "`write(|w| ..)` method takes [txreg::W](txreg::W) writer structure"]
impl crate::Writable for TXREG {}
#[doc = "U1TXREG register"]
pub mod txreg;
#[doc = "U1RXREG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxreg](rxreg) module"]
pub type RXREG = crate::Reg<u32, _RXREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXREG;
#[doc = "`read()` method returns [rxreg::R](rxreg::R) reader structure"]
impl crate::Readable for RXREG {}
#[doc = "`write(|w| ..)` method takes [rxreg::W](rxreg::W) writer structure"]
impl crate::Writable for RXREG {}
#[doc = "U1RXREG register"]
pub mod rxreg;
#[doc = "U1BRG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brg](brg) module"]
pub type BRG = crate::Reg<u32, _BRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRG;
#[doc = "`read()` method returns [brg::R](brg::R) reader structure"]
impl crate::Readable for BRG {}
#[doc = "`write(|w| ..)` method takes [brg::W](brg::W) writer structure"]
impl crate::Writable for BRG {}
#[doc = "U1BRG register"]
pub mod brg;
#[doc = "U1BRGCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgclr](brgclr) module"]
pub type BRGCLR = crate::Reg<u32, _BRGCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGCLR;
#[doc = "`read()` method returns [brgclr::R](brgclr::R) reader structure"]
impl crate::Readable for BRGCLR {}
#[doc = "`write(|w| ..)` method takes [brgclr::W](brgclr::W) writer structure"]
impl crate::Writable for BRGCLR {}
#[doc = "U1BRGCLR register"]
pub mod brgclr;
#[doc = "U1BRGSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgset](brgset) module"]
pub type BRGSET = crate::Reg<u32, _BRGSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGSET;
#[doc = "`read()` method returns [brgset::R](brgset::R) reader structure"]
impl crate::Readable for BRGSET {}
#[doc = "`write(|w| ..)` method takes [brgset::W](brgset::W) writer structure"]
impl crate::Writable for BRGSET {}
#[doc = "U1BRGSET register"]
pub mod brgset;
#[doc = "U1BRGINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brginv](brginv) module"]
pub type BRGINV = crate::Reg<u32, _BRGINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGINV;
#[doc = "`read()` method returns [brginv::R](brginv::R) reader structure"]
impl crate::Readable for BRGINV {}
#[doc = "`write(|w| ..)` method takes [brginv::W](brginv::W) writer structure"]
impl crate::Writable for BRGINV {}
#[doc = "U1BRGINV register"]
pub mod brginv;
