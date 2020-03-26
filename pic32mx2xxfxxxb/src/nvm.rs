#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - NVMCON register"]
    pub nvmcon: NVMCON,
    #[doc = "0x04 - NVMCONCLR register"]
    pub nvmconclr: NVMCONCLR,
    #[doc = "0x08 - NVMCONSET register"]
    pub nvmconset: NVMCONSET,
    #[doc = "0x0c - NVMCONINV register"]
    pub nvmconinv: NVMCONINV,
    #[doc = "0x10 - NVMKEY register"]
    pub nvmkey: NVMKEY,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - NVMADDR register"]
    pub nvmaddr: NVMADDR,
    #[doc = "0x24 - NVMADDRCLR register"]
    pub nvmaddrclr: NVMADDRCLR,
    #[doc = "0x28 - NVMADDRSET register"]
    pub nvmaddrset: NVMADDRSET,
    #[doc = "0x2c - NVMADDRINV register"]
    pub nvmaddrinv: NVMADDRINV,
    #[doc = "0x30 - NVMDATA register"]
    pub nvmdata: NVMDATA,
    _reserved10: [u8; 12usize],
    #[doc = "0x40 - NVMSRCADDR register"]
    pub nvmsrcaddr: NVMSRCADDR,
}
#[doc = "NVMCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmcon](nvmcon) module"]
pub type NVMCON = crate::Reg<u32, _NVMCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMCON;
#[doc = "`read()` method returns [nvmcon::R](nvmcon::R) reader structure"]
impl crate::Readable for NVMCON {}
#[doc = "`write(|w| ..)` method takes [nvmcon::W](nvmcon::W) writer structure"]
impl crate::Writable for NVMCON {}
#[doc = "NVMCON register"]
pub mod nvmcon;
#[doc = "NVMCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmconclr](nvmconclr) module"]
pub type NVMCONCLR = crate::Reg<u32, _NVMCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMCONCLR;
#[doc = "`read()` method returns [nvmconclr::R](nvmconclr::R) reader structure"]
impl crate::Readable for NVMCONCLR {}
#[doc = "`write(|w| ..)` method takes [nvmconclr::W](nvmconclr::W) writer structure"]
impl crate::Writable for NVMCONCLR {}
#[doc = "NVMCONCLR register"]
pub mod nvmconclr;
#[doc = "NVMCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmconset](nvmconset) module"]
pub type NVMCONSET = crate::Reg<u32, _NVMCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMCONSET;
#[doc = "`read()` method returns [nvmconset::R](nvmconset::R) reader structure"]
impl crate::Readable for NVMCONSET {}
#[doc = "`write(|w| ..)` method takes [nvmconset::W](nvmconset::W) writer structure"]
impl crate::Writable for NVMCONSET {}
#[doc = "NVMCONSET register"]
pub mod nvmconset;
#[doc = "NVMCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmconinv](nvmconinv) module"]
pub type NVMCONINV = crate::Reg<u32, _NVMCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMCONINV;
#[doc = "`read()` method returns [nvmconinv::R](nvmconinv::R) reader structure"]
impl crate::Readable for NVMCONINV {}
#[doc = "`write(|w| ..)` method takes [nvmconinv::W](nvmconinv::W) writer structure"]
impl crate::Writable for NVMCONINV {}
#[doc = "NVMCONINV register"]
pub mod nvmconinv;
#[doc = "NVMKEY register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmkey](nvmkey) module"]
pub type NVMKEY = crate::Reg<u32, _NVMKEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMKEY;
#[doc = "`read()` method returns [nvmkey::R](nvmkey::R) reader structure"]
impl crate::Readable for NVMKEY {}
#[doc = "`write(|w| ..)` method takes [nvmkey::W](nvmkey::W) writer structure"]
impl crate::Writable for NVMKEY {}
#[doc = "NVMKEY register"]
pub mod nvmkey;
#[doc = "NVMADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmaddr](nvmaddr) module"]
pub type NVMADDR = crate::Reg<u32, _NVMADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMADDR;
#[doc = "`read()` method returns [nvmaddr::R](nvmaddr::R) reader structure"]
impl crate::Readable for NVMADDR {}
#[doc = "`write(|w| ..)` method takes [nvmaddr::W](nvmaddr::W) writer structure"]
impl crate::Writable for NVMADDR {}
#[doc = "NVMADDR register"]
pub mod nvmaddr;
#[doc = "NVMADDRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmaddrclr](nvmaddrclr) module"]
pub type NVMADDRCLR = crate::Reg<u32, _NVMADDRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMADDRCLR;
#[doc = "`read()` method returns [nvmaddrclr::R](nvmaddrclr::R) reader structure"]
impl crate::Readable for NVMADDRCLR {}
#[doc = "`write(|w| ..)` method takes [nvmaddrclr::W](nvmaddrclr::W) writer structure"]
impl crate::Writable for NVMADDRCLR {}
#[doc = "NVMADDRCLR register"]
pub mod nvmaddrclr;
#[doc = "NVMADDRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmaddrset](nvmaddrset) module"]
pub type NVMADDRSET = crate::Reg<u32, _NVMADDRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMADDRSET;
#[doc = "`read()` method returns [nvmaddrset::R](nvmaddrset::R) reader structure"]
impl crate::Readable for NVMADDRSET {}
#[doc = "`write(|w| ..)` method takes [nvmaddrset::W](nvmaddrset::W) writer structure"]
impl crate::Writable for NVMADDRSET {}
#[doc = "NVMADDRSET register"]
pub mod nvmaddrset;
#[doc = "NVMADDRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmaddrinv](nvmaddrinv) module"]
pub type NVMADDRINV = crate::Reg<u32, _NVMADDRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMADDRINV;
#[doc = "`read()` method returns [nvmaddrinv::R](nvmaddrinv::R) reader structure"]
impl crate::Readable for NVMADDRINV {}
#[doc = "`write(|w| ..)` method takes [nvmaddrinv::W](nvmaddrinv::W) writer structure"]
impl crate::Writable for NVMADDRINV {}
#[doc = "NVMADDRINV register"]
pub mod nvmaddrinv;
#[doc = "NVMDATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmdata](nvmdata) module"]
pub type NVMDATA = crate::Reg<u32, _NVMDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMDATA;
#[doc = "`read()` method returns [nvmdata::R](nvmdata::R) reader structure"]
impl crate::Readable for NVMDATA {}
#[doc = "`write(|w| ..)` method takes [nvmdata::W](nvmdata::W) writer structure"]
impl crate::Writable for NVMDATA {}
#[doc = "NVMDATA register"]
pub mod nvmdata;
#[doc = "NVMSRCADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmsrcaddr](nvmsrcaddr) module"]
pub type NVMSRCADDR = crate::Reg<u32, _NVMSRCADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVMSRCADDR;
#[doc = "`read()` method returns [nvmsrcaddr::R](nvmsrcaddr::R) reader structure"]
impl crate::Readable for NVMSRCADDR {}
#[doc = "`write(|w| ..)` method takes [nvmsrcaddr::W](nvmsrcaddr::W) writer structure"]
impl crate::Writable for NVMSRCADDR {}
#[doc = "NVMSRCADDR register"]
pub mod nvmsrcaddr;
