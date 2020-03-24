#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC2CON register"]
    pub oc2con: OC2CON,
    #[doc = "0x04 - OC2CONCLR register"]
    pub oc2conclr: OC2CONCLR,
    #[doc = "0x08 - OC2CONSET register"]
    pub oc2conset: OC2CONSET,
    #[doc = "0x0c - OC2CONINV register"]
    pub oc2coninv: OC2CONINV,
    #[doc = "0x10 - OC2R register"]
    pub oc2r: OC2R,
    #[doc = "0x14 - OC2RCLR register"]
    pub oc2rclr: OC2RCLR,
    #[doc = "0x18 - OC2RSET register"]
    pub oc2rset: OC2RSET,
    #[doc = "0x1c - OC2RINV register"]
    pub oc2rinv: OC2RINV,
    #[doc = "0x20 - OC2RS register"]
    pub oc2rs: OC2RS,
    #[doc = "0x24 - OC2RSCLR register"]
    pub oc2rsclr: OC2RSCLR,
    #[doc = "0x28 - OC2RSSET register"]
    pub oc2rsset: OC2RSSET,
    #[doc = "0x2c - OC2RSINV register"]
    pub oc2rsinv: OC2RSINV,
}
#[doc = "OC2CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2con](oc2con) module"]
pub type OC2CON = crate::Reg<u32, _OC2CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2CON;
#[doc = "`read()` method returns [oc2con::R](oc2con::R) reader structure"]
impl crate::Readable for OC2CON {}
#[doc = "`write(|w| ..)` method takes [oc2con::W](oc2con::W) writer structure"]
impl crate::Writable for OC2CON {}
#[doc = "OC2CON register"]
pub mod oc2con;
#[doc = "OC2CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2conclr](oc2conclr) module"]
pub type OC2CONCLR = crate::Reg<u32, _OC2CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2CONCLR;
#[doc = "`read()` method returns [oc2conclr::R](oc2conclr::R) reader structure"]
impl crate::Readable for OC2CONCLR {}
#[doc = "`write(|w| ..)` method takes [oc2conclr::W](oc2conclr::W) writer structure"]
impl crate::Writable for OC2CONCLR {}
#[doc = "OC2CONCLR register"]
pub mod oc2conclr;
#[doc = "OC2CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2conset](oc2conset) module"]
pub type OC2CONSET = crate::Reg<u32, _OC2CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2CONSET;
#[doc = "`read()` method returns [oc2conset::R](oc2conset::R) reader structure"]
impl crate::Readable for OC2CONSET {}
#[doc = "`write(|w| ..)` method takes [oc2conset::W](oc2conset::W) writer structure"]
impl crate::Writable for OC2CONSET {}
#[doc = "OC2CONSET register"]
pub mod oc2conset;
#[doc = "OC2CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2coninv](oc2coninv) module"]
pub type OC2CONINV = crate::Reg<u32, _OC2CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2CONINV;
#[doc = "`read()` method returns [oc2coninv::R](oc2coninv::R) reader structure"]
impl crate::Readable for OC2CONINV {}
#[doc = "`write(|w| ..)` method takes [oc2coninv::W](oc2coninv::W) writer structure"]
impl crate::Writable for OC2CONINV {}
#[doc = "OC2CONINV register"]
pub mod oc2coninv;
#[doc = "OC2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2r](oc2r) module"]
pub type OC2R = crate::Reg<u32, _OC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2R;
#[doc = "`read()` method returns [oc2r::R](oc2r::R) reader structure"]
impl crate::Readable for OC2R {}
#[doc = "`write(|w| ..)` method takes [oc2r::W](oc2r::W) writer structure"]
impl crate::Writable for OC2R {}
#[doc = "OC2R register"]
pub mod oc2r;
#[doc = "OC2RCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2rclr](oc2rclr) module"]
pub type OC2RCLR = crate::Reg<u32, _OC2RCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2RCLR;
#[doc = "`read()` method returns [oc2rclr::R](oc2rclr::R) reader structure"]
impl crate::Readable for OC2RCLR {}
#[doc = "`write(|w| ..)` method takes [oc2rclr::W](oc2rclr::W) writer structure"]
impl crate::Writable for OC2RCLR {}
#[doc = "OC2RCLR register"]
pub mod oc2rclr;
#[doc = "OC2RSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2rset](oc2rset) module"]
pub type OC2RSET = crate::Reg<u32, _OC2RSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2RSET;
#[doc = "`read()` method returns [oc2rset::R](oc2rset::R) reader structure"]
impl crate::Readable for OC2RSET {}
#[doc = "`write(|w| ..)` method takes [oc2rset::W](oc2rset::W) writer structure"]
impl crate::Writable for OC2RSET {}
#[doc = "OC2RSET register"]
pub mod oc2rset;
#[doc = "OC2RINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2rinv](oc2rinv) module"]
pub type OC2RINV = crate::Reg<u32, _OC2RINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2RINV;
#[doc = "`read()` method returns [oc2rinv::R](oc2rinv::R) reader structure"]
impl crate::Readable for OC2RINV {}
#[doc = "`write(|w| ..)` method takes [oc2rinv::W](oc2rinv::W) writer structure"]
impl crate::Writable for OC2RINV {}
#[doc = "OC2RINV register"]
pub mod oc2rinv;
#[doc = "OC2RS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2rs](oc2rs) module"]
pub type OC2RS = crate::Reg<u32, _OC2RS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2RS;
#[doc = "`read()` method returns [oc2rs::R](oc2rs::R) reader structure"]
impl crate::Readable for OC2RS {}
#[doc = "`write(|w| ..)` method takes [oc2rs::W](oc2rs::W) writer structure"]
impl crate::Writable for OC2RS {}
#[doc = "OC2RS register"]
pub mod oc2rs;
#[doc = "OC2RSCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2rsclr](oc2rsclr) module"]
pub type OC2RSCLR = crate::Reg<u32, _OC2RSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2RSCLR;
#[doc = "`read()` method returns [oc2rsclr::R](oc2rsclr::R) reader structure"]
impl crate::Readable for OC2RSCLR {}
#[doc = "`write(|w| ..)` method takes [oc2rsclr::W](oc2rsclr::W) writer structure"]
impl crate::Writable for OC2RSCLR {}
#[doc = "OC2RSCLR register"]
pub mod oc2rsclr;
#[doc = "OC2RSSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2rsset](oc2rsset) module"]
pub type OC2RSSET = crate::Reg<u32, _OC2RSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2RSSET;
#[doc = "`read()` method returns [oc2rsset::R](oc2rsset::R) reader structure"]
impl crate::Readable for OC2RSSET {}
#[doc = "`write(|w| ..)` method takes [oc2rsset::W](oc2rsset::W) writer structure"]
impl crate::Writable for OC2RSSET {}
#[doc = "OC2RSSET register"]
pub mod oc2rsset;
#[doc = "OC2RSINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2rsinv](oc2rsinv) module"]
pub type OC2RSINV = crate::Reg<u32, _OC2RSINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC2RSINV;
#[doc = "`read()` method returns [oc2rsinv::R](oc2rsinv::R) reader structure"]
impl crate::Readable for OC2RSINV {}
#[doc = "`write(|w| ..)` method takes [oc2rsinv::W](oc2rsinv::W) writer structure"]
impl crate::Writable for OC2RSINV {}
#[doc = "OC2RSINV register"]
pub mod oc2rsinv;
