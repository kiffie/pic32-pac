#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC1CON register"]
    pub oc1con: OC1CON,
    #[doc = "0x04 - OC1CONCLR register"]
    pub oc1conclr: OC1CONCLR,
    #[doc = "0x08 - OC1CONSET register"]
    pub oc1conset: OC1CONSET,
    #[doc = "0x0c - OC1CONINV register"]
    pub oc1coninv: OC1CONINV,
    #[doc = "0x10 - OC1R register"]
    pub oc1r: OC1R,
    #[doc = "0x14 - OC1RCLR register"]
    pub oc1rclr: OC1RCLR,
    #[doc = "0x18 - OC1RSET register"]
    pub oc1rset: OC1RSET,
    #[doc = "0x1c - OC1RINV register"]
    pub oc1rinv: OC1RINV,
    #[doc = "0x20 - OC1RS register"]
    pub oc1rs: OC1RS,
    #[doc = "0x24 - OC1RSCLR register"]
    pub oc1rsclr: OC1RSCLR,
    #[doc = "0x28 - OC1RSSET register"]
    pub oc1rsset: OC1RSSET,
    #[doc = "0x2c - OC1RSINV register"]
    pub oc1rsinv: OC1RSINV,
}
#[doc = "OC1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1con](oc1con) module"]
pub type OC1CON = crate::Reg<u32, _OC1CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1CON;
#[doc = "`read()` method returns [oc1con::R](oc1con::R) reader structure"]
impl crate::Readable for OC1CON {}
#[doc = "`write(|w| ..)` method takes [oc1con::W](oc1con::W) writer structure"]
impl crate::Writable for OC1CON {}
#[doc = "OC1CON register"]
pub mod oc1con;
#[doc = "OC1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1conclr](oc1conclr) module"]
pub type OC1CONCLR = crate::Reg<u32, _OC1CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1CONCLR;
#[doc = "`read()` method returns [oc1conclr::R](oc1conclr::R) reader structure"]
impl crate::Readable for OC1CONCLR {}
#[doc = "`write(|w| ..)` method takes [oc1conclr::W](oc1conclr::W) writer structure"]
impl crate::Writable for OC1CONCLR {}
#[doc = "OC1CONCLR register"]
pub mod oc1conclr;
#[doc = "OC1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1conset](oc1conset) module"]
pub type OC1CONSET = crate::Reg<u32, _OC1CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1CONSET;
#[doc = "`read()` method returns [oc1conset::R](oc1conset::R) reader structure"]
impl crate::Readable for OC1CONSET {}
#[doc = "`write(|w| ..)` method takes [oc1conset::W](oc1conset::W) writer structure"]
impl crate::Writable for OC1CONSET {}
#[doc = "OC1CONSET register"]
pub mod oc1conset;
#[doc = "OC1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1coninv](oc1coninv) module"]
pub type OC1CONINV = crate::Reg<u32, _OC1CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1CONINV;
#[doc = "`read()` method returns [oc1coninv::R](oc1coninv::R) reader structure"]
impl crate::Readable for OC1CONINV {}
#[doc = "`write(|w| ..)` method takes [oc1coninv::W](oc1coninv::W) writer structure"]
impl crate::Writable for OC1CONINV {}
#[doc = "OC1CONINV register"]
pub mod oc1coninv;
#[doc = "OC1R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1r](oc1r) module"]
pub type OC1R = crate::Reg<u32, _OC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1R;
#[doc = "`read()` method returns [oc1r::R](oc1r::R) reader structure"]
impl crate::Readable for OC1R {}
#[doc = "`write(|w| ..)` method takes [oc1r::W](oc1r::W) writer structure"]
impl crate::Writable for OC1R {}
#[doc = "OC1R register"]
pub mod oc1r;
#[doc = "OC1RCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1rclr](oc1rclr) module"]
pub type OC1RCLR = crate::Reg<u32, _OC1RCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1RCLR;
#[doc = "`read()` method returns [oc1rclr::R](oc1rclr::R) reader structure"]
impl crate::Readable for OC1RCLR {}
#[doc = "`write(|w| ..)` method takes [oc1rclr::W](oc1rclr::W) writer structure"]
impl crate::Writable for OC1RCLR {}
#[doc = "OC1RCLR register"]
pub mod oc1rclr;
#[doc = "OC1RSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1rset](oc1rset) module"]
pub type OC1RSET = crate::Reg<u32, _OC1RSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1RSET;
#[doc = "`read()` method returns [oc1rset::R](oc1rset::R) reader structure"]
impl crate::Readable for OC1RSET {}
#[doc = "`write(|w| ..)` method takes [oc1rset::W](oc1rset::W) writer structure"]
impl crate::Writable for OC1RSET {}
#[doc = "OC1RSET register"]
pub mod oc1rset;
#[doc = "OC1RINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1rinv](oc1rinv) module"]
pub type OC1RINV = crate::Reg<u32, _OC1RINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1RINV;
#[doc = "`read()` method returns [oc1rinv::R](oc1rinv::R) reader structure"]
impl crate::Readable for OC1RINV {}
#[doc = "`write(|w| ..)` method takes [oc1rinv::W](oc1rinv::W) writer structure"]
impl crate::Writable for OC1RINV {}
#[doc = "OC1RINV register"]
pub mod oc1rinv;
#[doc = "OC1RS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1rs](oc1rs) module"]
pub type OC1RS = crate::Reg<u32, _OC1RS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1RS;
#[doc = "`read()` method returns [oc1rs::R](oc1rs::R) reader structure"]
impl crate::Readable for OC1RS {}
#[doc = "`write(|w| ..)` method takes [oc1rs::W](oc1rs::W) writer structure"]
impl crate::Writable for OC1RS {}
#[doc = "OC1RS register"]
pub mod oc1rs;
#[doc = "OC1RSCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1rsclr](oc1rsclr) module"]
pub type OC1RSCLR = crate::Reg<u32, _OC1RSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1RSCLR;
#[doc = "`read()` method returns [oc1rsclr::R](oc1rsclr::R) reader structure"]
impl crate::Readable for OC1RSCLR {}
#[doc = "`write(|w| ..)` method takes [oc1rsclr::W](oc1rsclr::W) writer structure"]
impl crate::Writable for OC1RSCLR {}
#[doc = "OC1RSCLR register"]
pub mod oc1rsclr;
#[doc = "OC1RSSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1rsset](oc1rsset) module"]
pub type OC1RSSET = crate::Reg<u32, _OC1RSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1RSSET;
#[doc = "`read()` method returns [oc1rsset::R](oc1rsset::R) reader structure"]
impl crate::Readable for OC1RSSET {}
#[doc = "`write(|w| ..)` method takes [oc1rsset::W](oc1rsset::W) writer structure"]
impl crate::Writable for OC1RSSET {}
#[doc = "OC1RSSET register"]
pub mod oc1rsset;
#[doc = "OC1RSINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc1rsinv](oc1rsinv) module"]
pub type OC1RSINV = crate::Reg<u32, _OC1RSINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC1RSINV;
#[doc = "`read()` method returns [oc1rsinv::R](oc1rsinv::R) reader structure"]
impl crate::Readable for OC1RSINV {}
#[doc = "`write(|w| ..)` method takes [oc1rsinv::W](oc1rsinv::W) writer structure"]
impl crate::Writable for OC1RSINV {}
#[doc = "OC1RSINV register"]
pub mod oc1rsinv;
