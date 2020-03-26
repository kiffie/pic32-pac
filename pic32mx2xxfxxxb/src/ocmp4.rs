#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OC4CON register"]
    pub oc4con: OC4CON,
    #[doc = "0x04 - OC4CONCLR register"]
    pub oc4conclr: OC4CONCLR,
    #[doc = "0x08 - OC4CONSET register"]
    pub oc4conset: OC4CONSET,
    #[doc = "0x0c - OC4CONINV register"]
    pub oc4coninv: OC4CONINV,
    #[doc = "0x10 - OC4R register"]
    pub oc4r: OC4R,
    #[doc = "0x14 - OC4RCLR register"]
    pub oc4rclr: OC4RCLR,
    #[doc = "0x18 - OC4RSET register"]
    pub oc4rset: OC4RSET,
    #[doc = "0x1c - OC4RINV register"]
    pub oc4rinv: OC4RINV,
    #[doc = "0x20 - OC4RS register"]
    pub oc4rs: OC4RS,
    #[doc = "0x24 - OC4RSCLR register"]
    pub oc4rsclr: OC4RSCLR,
    #[doc = "0x28 - OC4RSSET register"]
    pub oc4rsset: OC4RSSET,
    #[doc = "0x2c - OC4RSINV register"]
    pub oc4rsinv: OC4RSINV,
}
#[doc = "OC4CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4con](oc4con) module"]
pub type OC4CON = crate::Reg<u32, _OC4CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4CON;
#[doc = "`read()` method returns [oc4con::R](oc4con::R) reader structure"]
impl crate::Readable for OC4CON {}
#[doc = "`write(|w| ..)` method takes [oc4con::W](oc4con::W) writer structure"]
impl crate::Writable for OC4CON {}
#[doc = "OC4CON register"]
pub mod oc4con;
#[doc = "OC4CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4conclr](oc4conclr) module"]
pub type OC4CONCLR = crate::Reg<u32, _OC4CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4CONCLR;
#[doc = "`read()` method returns [oc4conclr::R](oc4conclr::R) reader structure"]
impl crate::Readable for OC4CONCLR {}
#[doc = "`write(|w| ..)` method takes [oc4conclr::W](oc4conclr::W) writer structure"]
impl crate::Writable for OC4CONCLR {}
#[doc = "OC4CONCLR register"]
pub mod oc4conclr;
#[doc = "OC4CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4conset](oc4conset) module"]
pub type OC4CONSET = crate::Reg<u32, _OC4CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4CONSET;
#[doc = "`read()` method returns [oc4conset::R](oc4conset::R) reader structure"]
impl crate::Readable for OC4CONSET {}
#[doc = "`write(|w| ..)` method takes [oc4conset::W](oc4conset::W) writer structure"]
impl crate::Writable for OC4CONSET {}
#[doc = "OC4CONSET register"]
pub mod oc4conset;
#[doc = "OC4CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4coninv](oc4coninv) module"]
pub type OC4CONINV = crate::Reg<u32, _OC4CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4CONINV;
#[doc = "`read()` method returns [oc4coninv::R](oc4coninv::R) reader structure"]
impl crate::Readable for OC4CONINV {}
#[doc = "`write(|w| ..)` method takes [oc4coninv::W](oc4coninv::W) writer structure"]
impl crate::Writable for OC4CONINV {}
#[doc = "OC4CONINV register"]
pub mod oc4coninv;
#[doc = "OC4R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4r](oc4r) module"]
pub type OC4R = crate::Reg<u32, _OC4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4R;
#[doc = "`read()` method returns [oc4r::R](oc4r::R) reader structure"]
impl crate::Readable for OC4R {}
#[doc = "`write(|w| ..)` method takes [oc4r::W](oc4r::W) writer structure"]
impl crate::Writable for OC4R {}
#[doc = "OC4R register"]
pub mod oc4r;
#[doc = "OC4RCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4rclr](oc4rclr) module"]
pub type OC4RCLR = crate::Reg<u32, _OC4RCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4RCLR;
#[doc = "`read()` method returns [oc4rclr::R](oc4rclr::R) reader structure"]
impl crate::Readable for OC4RCLR {}
#[doc = "`write(|w| ..)` method takes [oc4rclr::W](oc4rclr::W) writer structure"]
impl crate::Writable for OC4RCLR {}
#[doc = "OC4RCLR register"]
pub mod oc4rclr;
#[doc = "OC4RSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4rset](oc4rset) module"]
pub type OC4RSET = crate::Reg<u32, _OC4RSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4RSET;
#[doc = "`read()` method returns [oc4rset::R](oc4rset::R) reader structure"]
impl crate::Readable for OC4RSET {}
#[doc = "`write(|w| ..)` method takes [oc4rset::W](oc4rset::W) writer structure"]
impl crate::Writable for OC4RSET {}
#[doc = "OC4RSET register"]
pub mod oc4rset;
#[doc = "OC4RINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4rinv](oc4rinv) module"]
pub type OC4RINV = crate::Reg<u32, _OC4RINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4RINV;
#[doc = "`read()` method returns [oc4rinv::R](oc4rinv::R) reader structure"]
impl crate::Readable for OC4RINV {}
#[doc = "`write(|w| ..)` method takes [oc4rinv::W](oc4rinv::W) writer structure"]
impl crate::Writable for OC4RINV {}
#[doc = "OC4RINV register"]
pub mod oc4rinv;
#[doc = "OC4RS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4rs](oc4rs) module"]
pub type OC4RS = crate::Reg<u32, _OC4RS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4RS;
#[doc = "`read()` method returns [oc4rs::R](oc4rs::R) reader structure"]
impl crate::Readable for OC4RS {}
#[doc = "`write(|w| ..)` method takes [oc4rs::W](oc4rs::W) writer structure"]
impl crate::Writable for OC4RS {}
#[doc = "OC4RS register"]
pub mod oc4rs;
#[doc = "OC4RSCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4rsclr](oc4rsclr) module"]
pub type OC4RSCLR = crate::Reg<u32, _OC4RSCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4RSCLR;
#[doc = "`read()` method returns [oc4rsclr::R](oc4rsclr::R) reader structure"]
impl crate::Readable for OC4RSCLR {}
#[doc = "`write(|w| ..)` method takes [oc4rsclr::W](oc4rsclr::W) writer structure"]
impl crate::Writable for OC4RSCLR {}
#[doc = "OC4RSCLR register"]
pub mod oc4rsclr;
#[doc = "OC4RSSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4rsset](oc4rsset) module"]
pub type OC4RSSET = crate::Reg<u32, _OC4RSSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4RSSET;
#[doc = "`read()` method returns [oc4rsset::R](oc4rsset::R) reader structure"]
impl crate::Readable for OC4RSSET {}
#[doc = "`write(|w| ..)` method takes [oc4rsset::W](oc4rsset::W) writer structure"]
impl crate::Writable for OC4RSSET {}
#[doc = "OC4RSSET register"]
pub mod oc4rsset;
#[doc = "OC4RSINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4rsinv](oc4rsinv) module"]
pub type OC4RSINV = crate::Reg<u32, _OC4RSINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OC4RSINV;
#[doc = "`read()` method returns [oc4rsinv::R](oc4rsinv::R) reader structure"]
impl crate::Readable for OC4RSINV {}
#[doc = "`write(|w| ..)` method takes [oc4rsinv::W](oc4rsinv::W) writer structure"]
impl crate::Writable for OC4RSINV {}
#[doc = "OC4RSINV register"]
pub mod oc4rsinv;
