#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC5CON register"]
    pub oc5con: OC5CON,
    #[doc = "0x04 - OC5CONCLR register"]
    pub oc5conclr: OC5CONCLR,
    #[doc = "0x08 - OC5CONSET register"]
    pub oc5conset: OC5CONSET,
    #[doc = "0x0c - OC5CONINV register"]
    pub oc5coninv: OC5CONINV,
    #[doc = "0x10 - OC5R register"]
    pub oc5r: OC5R,
    #[doc = "0x14 - OC5RCLR register"]
    pub oc5rclr: OC5RCLR,
    #[doc = "0x18 - OC5RSET register"]
    pub oc5rset: OC5RSET,
    #[doc = "0x1c - OC5RINV register"]
    pub oc5rinv: OC5RINV,
    #[doc = "0x20 - OC5RS register"]
    pub oc5rs: OC5RS,
    #[doc = "0x24 - OC5RSCLR register"]
    pub oc5rsclr: OC5RSCLR,
    #[doc = "0x28 - OC5RSSET register"]
    pub oc5rsset: OC5RSSET,
    #[doc = "0x2c - OC5RSINV register"]
    pub oc5rsinv: OC5RSINV,
}
#[doc = "OC5CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5con](oc5con) module"]
pub type OC5CON = crate::Reg<u32, _OC5CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5CON;
#[doc = "`read()` method returns [oc5con::R](oc5con::R) reader structure"]
impl crate::Readable for OC5CON {}
#[doc = "`write(|w| ..)` method takes [oc5con::W](oc5con::W) writer structure"]
impl crate::Writable for OC5CON {}
#[doc = "OC5CON register"]
pub mod oc5con;
#[doc = "OC5CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5conclr](oc5conclr) module"]
pub type OC5CONCLR = crate::Reg<u32, _OC5CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5CONCLR;
#[doc = "`read()` method returns [oc5conclr::R](oc5conclr::R) reader structure"]
impl crate::Readable for OC5CONCLR {}
#[doc = "`write(|w| ..)` method takes [oc5conclr::W](oc5conclr::W) writer structure"]
impl crate::Writable for OC5CONCLR {}
#[doc = "OC5CONCLR register"]
pub mod oc5conclr;
#[doc = "OC5CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5conset](oc5conset) module"]
pub type OC5CONSET = crate::Reg<u32, _OC5CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5CONSET;
#[doc = "`read()` method returns [oc5conset::R](oc5conset::R) reader structure"]
impl crate::Readable for OC5CONSET {}
#[doc = "`write(|w| ..)` method takes [oc5conset::W](oc5conset::W) writer structure"]
impl crate::Writable for OC5CONSET {}
#[doc = "OC5CONSET register"]
pub mod oc5conset;
#[doc = "OC5CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5coninv](oc5coninv) module"]
pub type OC5CONINV = crate::Reg<u32, _OC5CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5CONINV;
#[doc = "`read()` method returns [oc5coninv::R](oc5coninv::R) reader structure"]
impl crate::Readable for OC5CONINV {}
#[doc = "`write(|w| ..)` method takes [oc5coninv::W](oc5coninv::W) writer structure"]
impl crate::Writable for OC5CONINV {}
#[doc = "OC5CONINV register"]
pub mod oc5coninv;
#[doc = "OC5R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5r](oc5r) module"]
pub type OC5R = crate::Reg<u32, _OC5R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5R;
#[doc = "`read()` method returns [oc5r::R](oc5r::R) reader structure"]
impl crate::Readable for OC5R {}
#[doc = "`write(|w| ..)` method takes [oc5r::W](oc5r::W) writer structure"]
impl crate::Writable for OC5R {}
#[doc = "OC5R register"]
pub mod oc5r;
#[doc = "OC5RCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5rclr](oc5rclr) module"]
pub type OC5RCLR = crate::Reg<u32, _OC5RCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5RCLR;
#[doc = "`read()` method returns [oc5rclr::R](oc5rclr::R) reader structure"]
impl crate::Readable for OC5RCLR {}
#[doc = "`write(|w| ..)` method takes [oc5rclr::W](oc5rclr::W) writer structure"]
impl crate::Writable for OC5RCLR {}
#[doc = "OC5RCLR register"]
pub mod oc5rclr;
#[doc = "OC5RSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5rset](oc5rset) module"]
pub type OC5RSET = crate::Reg<u32, _OC5RSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5RSET;
#[doc = "`read()` method returns [oc5rset::R](oc5rset::R) reader structure"]
impl crate::Readable for OC5RSET {}
#[doc = "`write(|w| ..)` method takes [oc5rset::W](oc5rset::W) writer structure"]
impl crate::Writable for OC5RSET {}
#[doc = "OC5RSET register"]
pub mod oc5rset;
#[doc = "OC5RINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5rinv](oc5rinv) module"]
pub type OC5RINV = crate::Reg<u32, _OC5RINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5RINV;
#[doc = "`read()` method returns [oc5rinv::R](oc5rinv::R) reader structure"]
impl crate::Readable for OC5RINV {}
#[doc = "`write(|w| ..)` method takes [oc5rinv::W](oc5rinv::W) writer structure"]
impl crate::Writable for OC5RINV {}
#[doc = "OC5RINV register"]
pub mod oc5rinv;
#[doc = "OC5RS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5rs](oc5rs) module"]
pub type OC5RS = crate::Reg<u32, _OC5RS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5RS;
#[doc = "`read()` method returns [oc5rs::R](oc5rs::R) reader structure"]
impl crate::Readable for OC5RS {}
#[doc = "`write(|w| ..)` method takes [oc5rs::W](oc5rs::W) writer structure"]
impl crate::Writable for OC5RS {}
#[doc = "OC5RS register"]
pub mod oc5rs;
#[doc = "OC5RSCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5rsclr](oc5rsclr) module"]
pub type OC5RSCLR = crate::Reg<u32, _OC5RSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5RSCLR;
#[doc = "`read()` method returns [oc5rsclr::R](oc5rsclr::R) reader structure"]
impl crate::Readable for OC5RSCLR {}
#[doc = "`write(|w| ..)` method takes [oc5rsclr::W](oc5rsclr::W) writer structure"]
impl crate::Writable for OC5RSCLR {}
#[doc = "OC5RSCLR register"]
pub mod oc5rsclr;
#[doc = "OC5RSSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5rsset](oc5rsset) module"]
pub type OC5RSSET = crate::Reg<u32, _OC5RSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5RSSET;
#[doc = "`read()` method returns [oc5rsset::R](oc5rsset::R) reader structure"]
impl crate::Readable for OC5RSSET {}
#[doc = "`write(|w| ..)` method takes [oc5rsset::W](oc5rsset::W) writer structure"]
impl crate::Writable for OC5RSSET {}
#[doc = "OC5RSSET register"]
pub mod oc5rsset;
#[doc = "OC5RSINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc5rsinv](oc5rsinv) module"]
pub type OC5RSINV = crate::Reg<u32, _OC5RSINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC5RSINV;
#[doc = "`read()` method returns [oc5rsinv::R](oc5rsinv::R) reader structure"]
impl crate::Readable for OC5RSINV {}
#[doc = "`write(|w| ..)` method takes [oc5rsinv::W](oc5rsinv::W) writer structure"]
impl crate::Writable for OC5RSINV {}
#[doc = "OC5RSINV register"]
pub mod oc5rsinv;
