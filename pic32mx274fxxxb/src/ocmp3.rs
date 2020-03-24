#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC3CON register"]
    pub oc3con: OC3CON,
    #[doc = "0x04 - OC3CONCLR register"]
    pub oc3conclr: OC3CONCLR,
    #[doc = "0x08 - OC3CONSET register"]
    pub oc3conset: OC3CONSET,
    #[doc = "0x0c - OC3CONINV register"]
    pub oc3coninv: OC3CONINV,
    #[doc = "0x10 - OC3R register"]
    pub oc3r: OC3R,
    #[doc = "0x14 - OC3RCLR register"]
    pub oc3rclr: OC3RCLR,
    #[doc = "0x18 - OC3RSET register"]
    pub oc3rset: OC3RSET,
    #[doc = "0x1c - OC3RINV register"]
    pub oc3rinv: OC3RINV,
    #[doc = "0x20 - OC3RS register"]
    pub oc3rs: OC3RS,
    #[doc = "0x24 - OC3RSCLR register"]
    pub oc3rsclr: OC3RSCLR,
    #[doc = "0x28 - OC3RSSET register"]
    pub oc3rsset: OC3RSSET,
    #[doc = "0x2c - OC3RSINV register"]
    pub oc3rsinv: OC3RSINV,
}
#[doc = "OC3CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3con](oc3con) module"]
pub type OC3CON = crate::Reg<u32, _OC3CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3CON;
#[doc = "`read()` method returns [oc3con::R](oc3con::R) reader structure"]
impl crate::Readable for OC3CON {}
#[doc = "`write(|w| ..)` method takes [oc3con::W](oc3con::W) writer structure"]
impl crate::Writable for OC3CON {}
#[doc = "OC3CON register"]
pub mod oc3con;
#[doc = "OC3CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3conclr](oc3conclr) module"]
pub type OC3CONCLR = crate::Reg<u32, _OC3CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3CONCLR;
#[doc = "`read()` method returns [oc3conclr::R](oc3conclr::R) reader structure"]
impl crate::Readable for OC3CONCLR {}
#[doc = "`write(|w| ..)` method takes [oc3conclr::W](oc3conclr::W) writer structure"]
impl crate::Writable for OC3CONCLR {}
#[doc = "OC3CONCLR register"]
pub mod oc3conclr;
#[doc = "OC3CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3conset](oc3conset) module"]
pub type OC3CONSET = crate::Reg<u32, _OC3CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3CONSET;
#[doc = "`read()` method returns [oc3conset::R](oc3conset::R) reader structure"]
impl crate::Readable for OC3CONSET {}
#[doc = "`write(|w| ..)` method takes [oc3conset::W](oc3conset::W) writer structure"]
impl crate::Writable for OC3CONSET {}
#[doc = "OC3CONSET register"]
pub mod oc3conset;
#[doc = "OC3CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3coninv](oc3coninv) module"]
pub type OC3CONINV = crate::Reg<u32, _OC3CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3CONINV;
#[doc = "`read()` method returns [oc3coninv::R](oc3coninv::R) reader structure"]
impl crate::Readable for OC3CONINV {}
#[doc = "`write(|w| ..)` method takes [oc3coninv::W](oc3coninv::W) writer structure"]
impl crate::Writable for OC3CONINV {}
#[doc = "OC3CONINV register"]
pub mod oc3coninv;
#[doc = "OC3R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3r](oc3r) module"]
pub type OC3R = crate::Reg<u32, _OC3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3R;
#[doc = "`read()` method returns [oc3r::R](oc3r::R) reader structure"]
impl crate::Readable for OC3R {}
#[doc = "`write(|w| ..)` method takes [oc3r::W](oc3r::W) writer structure"]
impl crate::Writable for OC3R {}
#[doc = "OC3R register"]
pub mod oc3r;
#[doc = "OC3RCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3rclr](oc3rclr) module"]
pub type OC3RCLR = crate::Reg<u32, _OC3RCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3RCLR;
#[doc = "`read()` method returns [oc3rclr::R](oc3rclr::R) reader structure"]
impl crate::Readable for OC3RCLR {}
#[doc = "`write(|w| ..)` method takes [oc3rclr::W](oc3rclr::W) writer structure"]
impl crate::Writable for OC3RCLR {}
#[doc = "OC3RCLR register"]
pub mod oc3rclr;
#[doc = "OC3RSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3rset](oc3rset) module"]
pub type OC3RSET = crate::Reg<u32, _OC3RSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3RSET;
#[doc = "`read()` method returns [oc3rset::R](oc3rset::R) reader structure"]
impl crate::Readable for OC3RSET {}
#[doc = "`write(|w| ..)` method takes [oc3rset::W](oc3rset::W) writer structure"]
impl crate::Writable for OC3RSET {}
#[doc = "OC3RSET register"]
pub mod oc3rset;
#[doc = "OC3RINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3rinv](oc3rinv) module"]
pub type OC3RINV = crate::Reg<u32, _OC3RINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3RINV;
#[doc = "`read()` method returns [oc3rinv::R](oc3rinv::R) reader structure"]
impl crate::Readable for OC3RINV {}
#[doc = "`write(|w| ..)` method takes [oc3rinv::W](oc3rinv::W) writer structure"]
impl crate::Writable for OC3RINV {}
#[doc = "OC3RINV register"]
pub mod oc3rinv;
#[doc = "OC3RS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3rs](oc3rs) module"]
pub type OC3RS = crate::Reg<u32, _OC3RS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3RS;
#[doc = "`read()` method returns [oc3rs::R](oc3rs::R) reader structure"]
impl crate::Readable for OC3RS {}
#[doc = "`write(|w| ..)` method takes [oc3rs::W](oc3rs::W) writer structure"]
impl crate::Writable for OC3RS {}
#[doc = "OC3RS register"]
pub mod oc3rs;
#[doc = "OC3RSCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3rsclr](oc3rsclr) module"]
pub type OC3RSCLR = crate::Reg<u32, _OC3RSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3RSCLR;
#[doc = "`read()` method returns [oc3rsclr::R](oc3rsclr::R) reader structure"]
impl crate::Readable for OC3RSCLR {}
#[doc = "`write(|w| ..)` method takes [oc3rsclr::W](oc3rsclr::W) writer structure"]
impl crate::Writable for OC3RSCLR {}
#[doc = "OC3RSCLR register"]
pub mod oc3rsclr;
#[doc = "OC3RSSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3rsset](oc3rsset) module"]
pub type OC3RSSET = crate::Reg<u32, _OC3RSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3RSSET;
#[doc = "`read()` method returns [oc3rsset::R](oc3rsset::R) reader structure"]
impl crate::Readable for OC3RSSET {}
#[doc = "`write(|w| ..)` method takes [oc3rsset::W](oc3rsset::W) writer structure"]
impl crate::Writable for OC3RSSET {}
#[doc = "OC3RSSET register"]
pub mod oc3rsset;
#[doc = "OC3RSINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc3rsinv](oc3rsinv) module"]
pub type OC3RSINV = crate::Reg<u32, _OC3RSINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC3RSINV;
#[doc = "`read()` method returns [oc3rsinv::R](oc3rsinv::R) reader structure"]
impl crate::Readable for OC3RSINV {}
#[doc = "`write(|w| ..)` method takes [oc3rsinv::W](oc3rsinv::W) writer structure"]
impl crate::Writable for OC3RSINV {}
#[doc = "OC3RSINV register"]
pub mod oc3rsinv;
