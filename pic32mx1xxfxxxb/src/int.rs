#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - INTCON register"]
    pub intcon: INTCON,
    #[doc = "0x04 - INTCONCLR register"]
    pub intconclr: INTCONCLR,
    #[doc = "0x08 - INTCONSET register"]
    pub intconset: INTCONSET,
    #[doc = "0x0c - INTCONINV register"]
    pub intconinv: INTCONINV,
    #[doc = "0x10 - INTSTAT register"]
    pub intstat: INTSTAT,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - IPTMR register"]
    pub iptmr: IPTMR,
    #[doc = "0x24 - IPTMRCLR register"]
    pub iptmrclr: IPTMRCLR,
    #[doc = "0x28 - IPTMRSET register"]
    pub iptmrset: IPTMRSET,
    #[doc = "0x2c - IPTMRINV register"]
    pub iptmrinv: IPTMRINV,
    #[doc = "0x30 - IFS0 register"]
    pub ifs0: IFS0,
    #[doc = "0x34 - IFS0CLR register"]
    pub ifs0clr: IFS0CLR,
    #[doc = "0x38 - IFS0SET register"]
    pub ifs0set: IFS0SET,
    #[doc = "0x3c - IFS0INV register"]
    pub ifs0inv: IFS0INV,
    #[doc = "0x40 - IFS1 register"]
    pub ifs1: IFS1,
    #[doc = "0x44 - IFS1CLR register"]
    pub ifs1clr: IFS1CLR,
    #[doc = "0x48 - IFS1SET register"]
    pub ifs1set: IFS1SET,
    #[doc = "0x4c - IFS1INV register"]
    pub ifs1inv: IFS1INV,
    _reserved17: [u8; 16usize],
    #[doc = "0x60 - IEC0 register"]
    pub iec0: IEC0,
    #[doc = "0x64 - IEC0CLR register"]
    pub iec0clr: IEC0CLR,
    #[doc = "0x68 - IEC0SET register"]
    pub iec0set: IEC0SET,
    #[doc = "0x6c - IEC0INV register"]
    pub iec0inv: IEC0INV,
    #[doc = "0x70 - IEC1 register"]
    pub iec1: IEC1,
    #[doc = "0x74 - IEC1CLR register"]
    pub iec1clr: IEC1CLR,
    #[doc = "0x78 - IEC1SET register"]
    pub iec1set: IEC1SET,
    #[doc = "0x7c - IEC1INV register"]
    pub iec1inv: IEC1INV,
    _reserved25: [u8; 16usize],
    #[doc = "0x90 - IPC0 register"]
    pub ipc0: IPC0,
    #[doc = "0x94 - IPC0CLR register"]
    pub ipc0clr: IPC0CLR,
    #[doc = "0x98 - IPC0SET register"]
    pub ipc0set: IPC0SET,
    #[doc = "0x9c - IPC0INV register"]
    pub ipc0inv: IPC0INV,
    #[doc = "0xa0 - IPC1 register"]
    pub ipc1: IPC1,
    #[doc = "0xa4 - IPC1CLR register"]
    pub ipc1clr: IPC1CLR,
    #[doc = "0xa8 - IPC1SET register"]
    pub ipc1set: IPC1SET,
    #[doc = "0xac - IPC1INV register"]
    pub ipc1inv: IPC1INV,
    #[doc = "0xb0 - IPC2 register"]
    pub ipc2: IPC2,
    #[doc = "0xb4 - IPC2CLR register"]
    pub ipc2clr: IPC2CLR,
    #[doc = "0xb8 - IPC2SET register"]
    pub ipc2set: IPC2SET,
    #[doc = "0xbc - IPC2INV register"]
    pub ipc2inv: IPC2INV,
    #[doc = "0xc0 - IPC3 register"]
    pub ipc3: IPC3,
    #[doc = "0xc4 - IPC3CLR register"]
    pub ipc3clr: IPC3CLR,
    #[doc = "0xc8 - IPC3SET register"]
    pub ipc3set: IPC3SET,
    #[doc = "0xcc - IPC3INV register"]
    pub ipc3inv: IPC3INV,
    #[doc = "0xd0 - IPC4 register"]
    pub ipc4: IPC4,
    #[doc = "0xd4 - IPC4CLR register"]
    pub ipc4clr: IPC4CLR,
    #[doc = "0xd8 - IPC4SET register"]
    pub ipc4set: IPC4SET,
    #[doc = "0xdc - IPC4INV register"]
    pub ipc4inv: IPC4INV,
    #[doc = "0xe0 - IPC5 register"]
    pub ipc5: IPC5,
    #[doc = "0xe4 - IPC5CLR register"]
    pub ipc5clr: IPC5CLR,
    #[doc = "0xe8 - IPC5SET register"]
    pub ipc5set: IPC5SET,
    #[doc = "0xec - IPC5INV register"]
    pub ipc5inv: IPC5INV,
    #[doc = "0xf0 - IPC6 register"]
    pub ipc6: IPC6,
    #[doc = "0xf4 - IPC6CLR register"]
    pub ipc6clr: IPC6CLR,
    #[doc = "0xf8 - IPC6SET register"]
    pub ipc6set: IPC6SET,
    #[doc = "0xfc - IPC6INV register"]
    pub ipc6inv: IPC6INV,
    #[doc = "0x100 - IPC7 register"]
    pub ipc7: IPC7,
    #[doc = "0x104 - IPC7CLR register"]
    pub ipc7clr: IPC7CLR,
    #[doc = "0x108 - IPC7SET register"]
    pub ipc7set: IPC7SET,
    #[doc = "0x10c - IPC7INV register"]
    pub ipc7inv: IPC7INV,
    #[doc = "0x110 - IPC8 register"]
    pub ipc8: IPC8,
    #[doc = "0x114 - IPC8CLR register"]
    pub ipc8clr: IPC8CLR,
    #[doc = "0x118 - IPC8SET register"]
    pub ipc8set: IPC8SET,
    #[doc = "0x11c - IPC8INV register"]
    pub ipc8inv: IPC8INV,
    #[doc = "0x120 - IPC9 register"]
    pub ipc9: IPC9,
    #[doc = "0x124 - IPC9CLR register"]
    pub ipc9clr: IPC9CLR,
    #[doc = "0x128 - IPC9SET register"]
    pub ipc9set: IPC9SET,
    #[doc = "0x12c - IPC9INV register"]
    pub ipc9inv: IPC9INV,
    #[doc = "0x130 - IPC10 register"]
    pub ipc10: IPC10,
    #[doc = "0x134 - IPC10CLR register"]
    pub ipc10clr: IPC10CLR,
    #[doc = "0x138 - IPC10SET register"]
    pub ipc10set: IPC10SET,
    #[doc = "0x13c - IPC10INV register"]
    pub ipc10inv: IPC10INV,
}
#[doc = "INTCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intcon](intcon) module"]
pub type INTCON = crate::Reg<u32, _INTCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCON;
#[doc = "`read()` method returns [intcon::R](intcon::R) reader structure"]
impl crate::Readable for INTCON {}
#[doc = "`write(|w| ..)` method takes [intcon::W](intcon::W) writer structure"]
impl crate::Writable for INTCON {}
#[doc = "INTCON register"]
pub mod intcon;
#[doc = "INTCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intconclr](intconclr) module"]
pub type INTCONCLR = crate::Reg<u32, _INTCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCONCLR;
#[doc = "`read()` method returns [intconclr::R](intconclr::R) reader structure"]
impl crate::Readable for INTCONCLR {}
#[doc = "`write(|w| ..)` method takes [intconclr::W](intconclr::W) writer structure"]
impl crate::Writable for INTCONCLR {}
#[doc = "INTCONCLR register"]
pub mod intconclr;
#[doc = "INTCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intconset](intconset) module"]
pub type INTCONSET = crate::Reg<u32, _INTCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCONSET;
#[doc = "`read()` method returns [intconset::R](intconset::R) reader structure"]
impl crate::Readable for INTCONSET {}
#[doc = "`write(|w| ..)` method takes [intconset::W](intconset::W) writer structure"]
impl crate::Writable for INTCONSET {}
#[doc = "INTCONSET register"]
pub mod intconset;
#[doc = "INTCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intconinv](intconinv) module"]
pub type INTCONINV = crate::Reg<u32, _INTCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCONINV;
#[doc = "`read()` method returns [intconinv::R](intconinv::R) reader structure"]
impl crate::Readable for INTCONINV {}
#[doc = "`write(|w| ..)` method takes [intconinv::W](intconinv::W) writer structure"]
impl crate::Writable for INTCONINV {}
#[doc = "INTCONINV register"]
pub mod intconinv;
#[doc = "INTSTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "`write(|w| ..)` method takes [intstat::W](intstat::W) writer structure"]
impl crate::Writable for INTSTAT {}
#[doc = "INTSTAT register"]
pub mod intstat;
#[doc = "IPTMR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptmr](iptmr) module"]
pub type IPTMR = crate::Reg<u32, _IPTMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTMR;
#[doc = "`read()` method returns [iptmr::R](iptmr::R) reader structure"]
impl crate::Readable for IPTMR {}
#[doc = "`write(|w| ..)` method takes [iptmr::W](iptmr::W) writer structure"]
impl crate::Writable for IPTMR {}
#[doc = "IPTMR register"]
pub mod iptmr;
#[doc = "IPTMRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptmrclr](iptmrclr) module"]
pub type IPTMRCLR = crate::Reg<u32, _IPTMRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTMRCLR;
#[doc = "`read()` method returns [iptmrclr::R](iptmrclr::R) reader structure"]
impl crate::Readable for IPTMRCLR {}
#[doc = "`write(|w| ..)` method takes [iptmrclr::W](iptmrclr::W) writer structure"]
impl crate::Writable for IPTMRCLR {}
#[doc = "IPTMRCLR register"]
pub mod iptmrclr;
#[doc = "IPTMRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptmrset](iptmrset) module"]
pub type IPTMRSET = crate::Reg<u32, _IPTMRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTMRSET;
#[doc = "`read()` method returns [iptmrset::R](iptmrset::R) reader structure"]
impl crate::Readable for IPTMRSET {}
#[doc = "`write(|w| ..)` method takes [iptmrset::W](iptmrset::W) writer structure"]
impl crate::Writable for IPTMRSET {}
#[doc = "IPTMRSET register"]
pub mod iptmrset;
#[doc = "IPTMRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptmrinv](iptmrinv) module"]
pub type IPTMRINV = crate::Reg<u32, _IPTMRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPTMRINV;
#[doc = "`read()` method returns [iptmrinv::R](iptmrinv::R) reader structure"]
impl crate::Readable for IPTMRINV {}
#[doc = "`write(|w| ..)` method takes [iptmrinv::W](iptmrinv::W) writer structure"]
impl crate::Writable for IPTMRINV {}
#[doc = "IPTMRINV register"]
pub mod iptmrinv;
#[doc = "IFS0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs0](ifs0) module"]
pub type IFS0 = crate::Reg<u32, _IFS0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS0;
#[doc = "`read()` method returns [ifs0::R](ifs0::R) reader structure"]
impl crate::Readable for IFS0 {}
#[doc = "`write(|w| ..)` method takes [ifs0::W](ifs0::W) writer structure"]
impl crate::Writable for IFS0 {}
#[doc = "IFS0 register"]
pub mod ifs0;
#[doc = "IFS0CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs0clr](ifs0clr) module"]
pub type IFS0CLR = crate::Reg<u32, _IFS0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS0CLR;
#[doc = "`read()` method returns [ifs0clr::R](ifs0clr::R) reader structure"]
impl crate::Readable for IFS0CLR {}
#[doc = "`write(|w| ..)` method takes [ifs0clr::W](ifs0clr::W) writer structure"]
impl crate::Writable for IFS0CLR {}
#[doc = "IFS0CLR register"]
pub mod ifs0clr;
#[doc = "IFS0SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs0set](ifs0set) module"]
pub type IFS0SET = crate::Reg<u32, _IFS0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS0SET;
#[doc = "`read()` method returns [ifs0set::R](ifs0set::R) reader structure"]
impl crate::Readable for IFS0SET {}
#[doc = "`write(|w| ..)` method takes [ifs0set::W](ifs0set::W) writer structure"]
impl crate::Writable for IFS0SET {}
#[doc = "IFS0SET register"]
pub mod ifs0set;
#[doc = "IFS0INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs0inv](ifs0inv) module"]
pub type IFS0INV = crate::Reg<u32, _IFS0INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS0INV;
#[doc = "`read()` method returns [ifs0inv::R](ifs0inv::R) reader structure"]
impl crate::Readable for IFS0INV {}
#[doc = "`write(|w| ..)` method takes [ifs0inv::W](ifs0inv::W) writer structure"]
impl crate::Writable for IFS0INV {}
#[doc = "IFS0INV register"]
pub mod ifs0inv;
#[doc = "IFS1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs1](ifs1) module"]
pub type IFS1 = crate::Reg<u32, _IFS1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS1;
#[doc = "`read()` method returns [ifs1::R](ifs1::R) reader structure"]
impl crate::Readable for IFS1 {}
#[doc = "`write(|w| ..)` method takes [ifs1::W](ifs1::W) writer structure"]
impl crate::Writable for IFS1 {}
#[doc = "IFS1 register"]
pub mod ifs1;
#[doc = "IFS1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs1clr](ifs1clr) module"]
pub type IFS1CLR = crate::Reg<u32, _IFS1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS1CLR;
#[doc = "`read()` method returns [ifs1clr::R](ifs1clr::R) reader structure"]
impl crate::Readable for IFS1CLR {}
#[doc = "`write(|w| ..)` method takes [ifs1clr::W](ifs1clr::W) writer structure"]
impl crate::Writable for IFS1CLR {}
#[doc = "IFS1CLR register"]
pub mod ifs1clr;
#[doc = "IFS1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs1set](ifs1set) module"]
pub type IFS1SET = crate::Reg<u32, _IFS1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS1SET;
#[doc = "`read()` method returns [ifs1set::R](ifs1set::R) reader structure"]
impl crate::Readable for IFS1SET {}
#[doc = "`write(|w| ..)` method takes [ifs1set::W](ifs1set::W) writer structure"]
impl crate::Writable for IFS1SET {}
#[doc = "IFS1SET register"]
pub mod ifs1set;
#[doc = "IFS1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs1inv](ifs1inv) module"]
pub type IFS1INV = crate::Reg<u32, _IFS1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS1INV;
#[doc = "`read()` method returns [ifs1inv::R](ifs1inv::R) reader structure"]
impl crate::Readable for IFS1INV {}
#[doc = "`write(|w| ..)` method takes [ifs1inv::W](ifs1inv::W) writer structure"]
impl crate::Writable for IFS1INV {}
#[doc = "IFS1INV register"]
pub mod ifs1inv;
#[doc = "IEC0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec0](iec0) module"]
pub type IEC0 = crate::Reg<u32, _IEC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC0;
#[doc = "`read()` method returns [iec0::R](iec0::R) reader structure"]
impl crate::Readable for IEC0 {}
#[doc = "`write(|w| ..)` method takes [iec0::W](iec0::W) writer structure"]
impl crate::Writable for IEC0 {}
#[doc = "IEC0 register"]
pub mod iec0;
#[doc = "IEC0CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec0clr](iec0clr) module"]
pub type IEC0CLR = crate::Reg<u32, _IEC0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC0CLR;
#[doc = "`read()` method returns [iec0clr::R](iec0clr::R) reader structure"]
impl crate::Readable for IEC0CLR {}
#[doc = "`write(|w| ..)` method takes [iec0clr::W](iec0clr::W) writer structure"]
impl crate::Writable for IEC0CLR {}
#[doc = "IEC0CLR register"]
pub mod iec0clr;
#[doc = "IEC0SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec0set](iec0set) module"]
pub type IEC0SET = crate::Reg<u32, _IEC0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC0SET;
#[doc = "`read()` method returns [iec0set::R](iec0set::R) reader structure"]
impl crate::Readable for IEC0SET {}
#[doc = "`write(|w| ..)` method takes [iec0set::W](iec0set::W) writer structure"]
impl crate::Writable for IEC0SET {}
#[doc = "IEC0SET register"]
pub mod iec0set;
#[doc = "IEC0INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec0inv](iec0inv) module"]
pub type IEC0INV = crate::Reg<u32, _IEC0INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC0INV;
#[doc = "`read()` method returns [iec0inv::R](iec0inv::R) reader structure"]
impl crate::Readable for IEC0INV {}
#[doc = "`write(|w| ..)` method takes [iec0inv::W](iec0inv::W) writer structure"]
impl crate::Writable for IEC0INV {}
#[doc = "IEC0INV register"]
pub mod iec0inv;
#[doc = "IEC1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec1](iec1) module"]
pub type IEC1 = crate::Reg<u32, _IEC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC1;
#[doc = "`read()` method returns [iec1::R](iec1::R) reader structure"]
impl crate::Readable for IEC1 {}
#[doc = "`write(|w| ..)` method takes [iec1::W](iec1::W) writer structure"]
impl crate::Writable for IEC1 {}
#[doc = "IEC1 register"]
pub mod iec1;
#[doc = "IEC1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec1clr](iec1clr) module"]
pub type IEC1CLR = crate::Reg<u32, _IEC1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC1CLR;
#[doc = "`read()` method returns [iec1clr::R](iec1clr::R) reader structure"]
impl crate::Readable for IEC1CLR {}
#[doc = "`write(|w| ..)` method takes [iec1clr::W](iec1clr::W) writer structure"]
impl crate::Writable for IEC1CLR {}
#[doc = "IEC1CLR register"]
pub mod iec1clr;
#[doc = "IEC1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec1set](iec1set) module"]
pub type IEC1SET = crate::Reg<u32, _IEC1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC1SET;
#[doc = "`read()` method returns [iec1set::R](iec1set::R) reader structure"]
impl crate::Readable for IEC1SET {}
#[doc = "`write(|w| ..)` method takes [iec1set::W](iec1set::W) writer structure"]
impl crate::Writable for IEC1SET {}
#[doc = "IEC1SET register"]
pub mod iec1set;
#[doc = "IEC1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec1inv](iec1inv) module"]
pub type IEC1INV = crate::Reg<u32, _IEC1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEC1INV;
#[doc = "`read()` method returns [iec1inv::R](iec1inv::R) reader structure"]
impl crate::Readable for IEC1INV {}
#[doc = "`write(|w| ..)` method takes [iec1inv::W](iec1inv::W) writer structure"]
impl crate::Writable for IEC1INV {}
#[doc = "IEC1INV register"]
pub mod iec1inv;
#[doc = "IPC0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc0](ipc0) module"]
pub type IPC0 = crate::Reg<u32, _IPC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC0;
#[doc = "`read()` method returns [ipc0::R](ipc0::R) reader structure"]
impl crate::Readable for IPC0 {}
#[doc = "`write(|w| ..)` method takes [ipc0::W](ipc0::W) writer structure"]
impl crate::Writable for IPC0 {}
#[doc = "IPC0 register"]
pub mod ipc0;
#[doc = "IPC0CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc0clr](ipc0clr) module"]
pub type IPC0CLR = crate::Reg<u32, _IPC0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC0CLR;
#[doc = "`read()` method returns [ipc0clr::R](ipc0clr::R) reader structure"]
impl crate::Readable for IPC0CLR {}
#[doc = "`write(|w| ..)` method takes [ipc0clr::W](ipc0clr::W) writer structure"]
impl crate::Writable for IPC0CLR {}
#[doc = "IPC0CLR register"]
pub mod ipc0clr;
#[doc = "IPC0SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc0set](ipc0set) module"]
pub type IPC0SET = crate::Reg<u32, _IPC0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC0SET;
#[doc = "`read()` method returns [ipc0set::R](ipc0set::R) reader structure"]
impl crate::Readable for IPC0SET {}
#[doc = "`write(|w| ..)` method takes [ipc0set::W](ipc0set::W) writer structure"]
impl crate::Writable for IPC0SET {}
#[doc = "IPC0SET register"]
pub mod ipc0set;
#[doc = "IPC0INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc0inv](ipc0inv) module"]
pub type IPC0INV = crate::Reg<u32, _IPC0INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC0INV;
#[doc = "`read()` method returns [ipc0inv::R](ipc0inv::R) reader structure"]
impl crate::Readable for IPC0INV {}
#[doc = "`write(|w| ..)` method takes [ipc0inv::W](ipc0inv::W) writer structure"]
impl crate::Writable for IPC0INV {}
#[doc = "IPC0INV register"]
pub mod ipc0inv;
#[doc = "IPC1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc1](ipc1) module"]
pub type IPC1 = crate::Reg<u32, _IPC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC1;
#[doc = "`read()` method returns [ipc1::R](ipc1::R) reader structure"]
impl crate::Readable for IPC1 {}
#[doc = "`write(|w| ..)` method takes [ipc1::W](ipc1::W) writer structure"]
impl crate::Writable for IPC1 {}
#[doc = "IPC1 register"]
pub mod ipc1;
#[doc = "IPC1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc1clr](ipc1clr) module"]
pub type IPC1CLR = crate::Reg<u32, _IPC1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC1CLR;
#[doc = "`read()` method returns [ipc1clr::R](ipc1clr::R) reader structure"]
impl crate::Readable for IPC1CLR {}
#[doc = "`write(|w| ..)` method takes [ipc1clr::W](ipc1clr::W) writer structure"]
impl crate::Writable for IPC1CLR {}
#[doc = "IPC1CLR register"]
pub mod ipc1clr;
#[doc = "IPC1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc1set](ipc1set) module"]
pub type IPC1SET = crate::Reg<u32, _IPC1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC1SET;
#[doc = "`read()` method returns [ipc1set::R](ipc1set::R) reader structure"]
impl crate::Readable for IPC1SET {}
#[doc = "`write(|w| ..)` method takes [ipc1set::W](ipc1set::W) writer structure"]
impl crate::Writable for IPC1SET {}
#[doc = "IPC1SET register"]
pub mod ipc1set;
#[doc = "IPC1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc1inv](ipc1inv) module"]
pub type IPC1INV = crate::Reg<u32, _IPC1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC1INV;
#[doc = "`read()` method returns [ipc1inv::R](ipc1inv::R) reader structure"]
impl crate::Readable for IPC1INV {}
#[doc = "`write(|w| ..)` method takes [ipc1inv::W](ipc1inv::W) writer structure"]
impl crate::Writable for IPC1INV {}
#[doc = "IPC1INV register"]
pub mod ipc1inv;
#[doc = "IPC2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc2](ipc2) module"]
pub type IPC2 = crate::Reg<u32, _IPC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC2;
#[doc = "`read()` method returns [ipc2::R](ipc2::R) reader structure"]
impl crate::Readable for IPC2 {}
#[doc = "`write(|w| ..)` method takes [ipc2::W](ipc2::W) writer structure"]
impl crate::Writable for IPC2 {}
#[doc = "IPC2 register"]
pub mod ipc2;
#[doc = "IPC2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc2clr](ipc2clr) module"]
pub type IPC2CLR = crate::Reg<u32, _IPC2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC2CLR;
#[doc = "`read()` method returns [ipc2clr::R](ipc2clr::R) reader structure"]
impl crate::Readable for IPC2CLR {}
#[doc = "`write(|w| ..)` method takes [ipc2clr::W](ipc2clr::W) writer structure"]
impl crate::Writable for IPC2CLR {}
#[doc = "IPC2CLR register"]
pub mod ipc2clr;
#[doc = "IPC2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc2set](ipc2set) module"]
pub type IPC2SET = crate::Reg<u32, _IPC2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC2SET;
#[doc = "`read()` method returns [ipc2set::R](ipc2set::R) reader structure"]
impl crate::Readable for IPC2SET {}
#[doc = "`write(|w| ..)` method takes [ipc2set::W](ipc2set::W) writer structure"]
impl crate::Writable for IPC2SET {}
#[doc = "IPC2SET register"]
pub mod ipc2set;
#[doc = "IPC2INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc2inv](ipc2inv) module"]
pub type IPC2INV = crate::Reg<u32, _IPC2INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC2INV;
#[doc = "`read()` method returns [ipc2inv::R](ipc2inv::R) reader structure"]
impl crate::Readable for IPC2INV {}
#[doc = "`write(|w| ..)` method takes [ipc2inv::W](ipc2inv::W) writer structure"]
impl crate::Writable for IPC2INV {}
#[doc = "IPC2INV register"]
pub mod ipc2inv;
#[doc = "IPC3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc3](ipc3) module"]
pub type IPC3 = crate::Reg<u32, _IPC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC3;
#[doc = "`read()` method returns [ipc3::R](ipc3::R) reader structure"]
impl crate::Readable for IPC3 {}
#[doc = "`write(|w| ..)` method takes [ipc3::W](ipc3::W) writer structure"]
impl crate::Writable for IPC3 {}
#[doc = "IPC3 register"]
pub mod ipc3;
#[doc = "IPC3CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc3clr](ipc3clr) module"]
pub type IPC3CLR = crate::Reg<u32, _IPC3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC3CLR;
#[doc = "`read()` method returns [ipc3clr::R](ipc3clr::R) reader structure"]
impl crate::Readable for IPC3CLR {}
#[doc = "`write(|w| ..)` method takes [ipc3clr::W](ipc3clr::W) writer structure"]
impl crate::Writable for IPC3CLR {}
#[doc = "IPC3CLR register"]
pub mod ipc3clr;
#[doc = "IPC3SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc3set](ipc3set) module"]
pub type IPC3SET = crate::Reg<u32, _IPC3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC3SET;
#[doc = "`read()` method returns [ipc3set::R](ipc3set::R) reader structure"]
impl crate::Readable for IPC3SET {}
#[doc = "`write(|w| ..)` method takes [ipc3set::W](ipc3set::W) writer structure"]
impl crate::Writable for IPC3SET {}
#[doc = "IPC3SET register"]
pub mod ipc3set;
#[doc = "IPC3INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc3inv](ipc3inv) module"]
pub type IPC3INV = crate::Reg<u32, _IPC3INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC3INV;
#[doc = "`read()` method returns [ipc3inv::R](ipc3inv::R) reader structure"]
impl crate::Readable for IPC3INV {}
#[doc = "`write(|w| ..)` method takes [ipc3inv::W](ipc3inv::W) writer structure"]
impl crate::Writable for IPC3INV {}
#[doc = "IPC3INV register"]
pub mod ipc3inv;
#[doc = "IPC4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc4](ipc4) module"]
pub type IPC4 = crate::Reg<u32, _IPC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC4;
#[doc = "`read()` method returns [ipc4::R](ipc4::R) reader structure"]
impl crate::Readable for IPC4 {}
#[doc = "`write(|w| ..)` method takes [ipc4::W](ipc4::W) writer structure"]
impl crate::Writable for IPC4 {}
#[doc = "IPC4 register"]
pub mod ipc4;
#[doc = "IPC4CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc4clr](ipc4clr) module"]
pub type IPC4CLR = crate::Reg<u32, _IPC4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC4CLR;
#[doc = "`read()` method returns [ipc4clr::R](ipc4clr::R) reader structure"]
impl crate::Readable for IPC4CLR {}
#[doc = "`write(|w| ..)` method takes [ipc4clr::W](ipc4clr::W) writer structure"]
impl crate::Writable for IPC4CLR {}
#[doc = "IPC4CLR register"]
pub mod ipc4clr;
#[doc = "IPC4SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc4set](ipc4set) module"]
pub type IPC4SET = crate::Reg<u32, _IPC4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC4SET;
#[doc = "`read()` method returns [ipc4set::R](ipc4set::R) reader structure"]
impl crate::Readable for IPC4SET {}
#[doc = "`write(|w| ..)` method takes [ipc4set::W](ipc4set::W) writer structure"]
impl crate::Writable for IPC4SET {}
#[doc = "IPC4SET register"]
pub mod ipc4set;
#[doc = "IPC4INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc4inv](ipc4inv) module"]
pub type IPC4INV = crate::Reg<u32, _IPC4INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC4INV;
#[doc = "`read()` method returns [ipc4inv::R](ipc4inv::R) reader structure"]
impl crate::Readable for IPC4INV {}
#[doc = "`write(|w| ..)` method takes [ipc4inv::W](ipc4inv::W) writer structure"]
impl crate::Writable for IPC4INV {}
#[doc = "IPC4INV register"]
pub mod ipc4inv;
#[doc = "IPC5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc5](ipc5) module"]
pub type IPC5 = crate::Reg<u32, _IPC5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC5;
#[doc = "`read()` method returns [ipc5::R](ipc5::R) reader structure"]
impl crate::Readable for IPC5 {}
#[doc = "`write(|w| ..)` method takes [ipc5::W](ipc5::W) writer structure"]
impl crate::Writable for IPC5 {}
#[doc = "IPC5 register"]
pub mod ipc5;
#[doc = "IPC5CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc5clr](ipc5clr) module"]
pub type IPC5CLR = crate::Reg<u32, _IPC5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC5CLR;
#[doc = "`read()` method returns [ipc5clr::R](ipc5clr::R) reader structure"]
impl crate::Readable for IPC5CLR {}
#[doc = "`write(|w| ..)` method takes [ipc5clr::W](ipc5clr::W) writer structure"]
impl crate::Writable for IPC5CLR {}
#[doc = "IPC5CLR register"]
pub mod ipc5clr;
#[doc = "IPC5SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc5set](ipc5set) module"]
pub type IPC5SET = crate::Reg<u32, _IPC5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC5SET;
#[doc = "`read()` method returns [ipc5set::R](ipc5set::R) reader structure"]
impl crate::Readable for IPC5SET {}
#[doc = "`write(|w| ..)` method takes [ipc5set::W](ipc5set::W) writer structure"]
impl crate::Writable for IPC5SET {}
#[doc = "IPC5SET register"]
pub mod ipc5set;
#[doc = "IPC5INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc5inv](ipc5inv) module"]
pub type IPC5INV = crate::Reg<u32, _IPC5INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC5INV;
#[doc = "`read()` method returns [ipc5inv::R](ipc5inv::R) reader structure"]
impl crate::Readable for IPC5INV {}
#[doc = "`write(|w| ..)` method takes [ipc5inv::W](ipc5inv::W) writer structure"]
impl crate::Writable for IPC5INV {}
#[doc = "IPC5INV register"]
pub mod ipc5inv;
#[doc = "IPC6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc6](ipc6) module"]
pub type IPC6 = crate::Reg<u32, _IPC6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC6;
#[doc = "`read()` method returns [ipc6::R](ipc6::R) reader structure"]
impl crate::Readable for IPC6 {}
#[doc = "`write(|w| ..)` method takes [ipc6::W](ipc6::W) writer structure"]
impl crate::Writable for IPC6 {}
#[doc = "IPC6 register"]
pub mod ipc6;
#[doc = "IPC6CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc6clr](ipc6clr) module"]
pub type IPC6CLR = crate::Reg<u32, _IPC6CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC6CLR;
#[doc = "`read()` method returns [ipc6clr::R](ipc6clr::R) reader structure"]
impl crate::Readable for IPC6CLR {}
#[doc = "`write(|w| ..)` method takes [ipc6clr::W](ipc6clr::W) writer structure"]
impl crate::Writable for IPC6CLR {}
#[doc = "IPC6CLR register"]
pub mod ipc6clr;
#[doc = "IPC6SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc6set](ipc6set) module"]
pub type IPC6SET = crate::Reg<u32, _IPC6SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC6SET;
#[doc = "`read()` method returns [ipc6set::R](ipc6set::R) reader structure"]
impl crate::Readable for IPC6SET {}
#[doc = "`write(|w| ..)` method takes [ipc6set::W](ipc6set::W) writer structure"]
impl crate::Writable for IPC6SET {}
#[doc = "IPC6SET register"]
pub mod ipc6set;
#[doc = "IPC6INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc6inv](ipc6inv) module"]
pub type IPC6INV = crate::Reg<u32, _IPC6INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC6INV;
#[doc = "`read()` method returns [ipc6inv::R](ipc6inv::R) reader structure"]
impl crate::Readable for IPC6INV {}
#[doc = "`write(|w| ..)` method takes [ipc6inv::W](ipc6inv::W) writer structure"]
impl crate::Writable for IPC6INV {}
#[doc = "IPC6INV register"]
pub mod ipc6inv;
#[doc = "IPC7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc7](ipc7) module"]
pub type IPC7 = crate::Reg<u32, _IPC7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC7;
#[doc = "`read()` method returns [ipc7::R](ipc7::R) reader structure"]
impl crate::Readable for IPC7 {}
#[doc = "`write(|w| ..)` method takes [ipc7::W](ipc7::W) writer structure"]
impl crate::Writable for IPC7 {}
#[doc = "IPC7 register"]
pub mod ipc7;
#[doc = "IPC7CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc7clr](ipc7clr) module"]
pub type IPC7CLR = crate::Reg<u32, _IPC7CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC7CLR;
#[doc = "`read()` method returns [ipc7clr::R](ipc7clr::R) reader structure"]
impl crate::Readable for IPC7CLR {}
#[doc = "`write(|w| ..)` method takes [ipc7clr::W](ipc7clr::W) writer structure"]
impl crate::Writable for IPC7CLR {}
#[doc = "IPC7CLR register"]
pub mod ipc7clr;
#[doc = "IPC7SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc7set](ipc7set) module"]
pub type IPC7SET = crate::Reg<u32, _IPC7SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC7SET;
#[doc = "`read()` method returns [ipc7set::R](ipc7set::R) reader structure"]
impl crate::Readable for IPC7SET {}
#[doc = "`write(|w| ..)` method takes [ipc7set::W](ipc7set::W) writer structure"]
impl crate::Writable for IPC7SET {}
#[doc = "IPC7SET register"]
pub mod ipc7set;
#[doc = "IPC7INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc7inv](ipc7inv) module"]
pub type IPC7INV = crate::Reg<u32, _IPC7INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC7INV;
#[doc = "`read()` method returns [ipc7inv::R](ipc7inv::R) reader structure"]
impl crate::Readable for IPC7INV {}
#[doc = "`write(|w| ..)` method takes [ipc7inv::W](ipc7inv::W) writer structure"]
impl crate::Writable for IPC7INV {}
#[doc = "IPC7INV register"]
pub mod ipc7inv;
#[doc = "IPC8 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc8](ipc8) module"]
pub type IPC8 = crate::Reg<u32, _IPC8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC8;
#[doc = "`read()` method returns [ipc8::R](ipc8::R) reader structure"]
impl crate::Readable for IPC8 {}
#[doc = "`write(|w| ..)` method takes [ipc8::W](ipc8::W) writer structure"]
impl crate::Writable for IPC8 {}
#[doc = "IPC8 register"]
pub mod ipc8;
#[doc = "IPC8CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc8clr](ipc8clr) module"]
pub type IPC8CLR = crate::Reg<u32, _IPC8CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC8CLR;
#[doc = "`read()` method returns [ipc8clr::R](ipc8clr::R) reader structure"]
impl crate::Readable for IPC8CLR {}
#[doc = "`write(|w| ..)` method takes [ipc8clr::W](ipc8clr::W) writer structure"]
impl crate::Writable for IPC8CLR {}
#[doc = "IPC8CLR register"]
pub mod ipc8clr;
#[doc = "IPC8SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc8set](ipc8set) module"]
pub type IPC8SET = crate::Reg<u32, _IPC8SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC8SET;
#[doc = "`read()` method returns [ipc8set::R](ipc8set::R) reader structure"]
impl crate::Readable for IPC8SET {}
#[doc = "`write(|w| ..)` method takes [ipc8set::W](ipc8set::W) writer structure"]
impl crate::Writable for IPC8SET {}
#[doc = "IPC8SET register"]
pub mod ipc8set;
#[doc = "IPC8INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc8inv](ipc8inv) module"]
pub type IPC8INV = crate::Reg<u32, _IPC8INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC8INV;
#[doc = "`read()` method returns [ipc8inv::R](ipc8inv::R) reader structure"]
impl crate::Readable for IPC8INV {}
#[doc = "`write(|w| ..)` method takes [ipc8inv::W](ipc8inv::W) writer structure"]
impl crate::Writable for IPC8INV {}
#[doc = "IPC8INV register"]
pub mod ipc8inv;
#[doc = "IPC9 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc9](ipc9) module"]
pub type IPC9 = crate::Reg<u32, _IPC9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC9;
#[doc = "`read()` method returns [ipc9::R](ipc9::R) reader structure"]
impl crate::Readable for IPC9 {}
#[doc = "`write(|w| ..)` method takes [ipc9::W](ipc9::W) writer structure"]
impl crate::Writable for IPC9 {}
#[doc = "IPC9 register"]
pub mod ipc9;
#[doc = "IPC9CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc9clr](ipc9clr) module"]
pub type IPC9CLR = crate::Reg<u32, _IPC9CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC9CLR;
#[doc = "`read()` method returns [ipc9clr::R](ipc9clr::R) reader structure"]
impl crate::Readable for IPC9CLR {}
#[doc = "`write(|w| ..)` method takes [ipc9clr::W](ipc9clr::W) writer structure"]
impl crate::Writable for IPC9CLR {}
#[doc = "IPC9CLR register"]
pub mod ipc9clr;
#[doc = "IPC9SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc9set](ipc9set) module"]
pub type IPC9SET = crate::Reg<u32, _IPC9SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC9SET;
#[doc = "`read()` method returns [ipc9set::R](ipc9set::R) reader structure"]
impl crate::Readable for IPC9SET {}
#[doc = "`write(|w| ..)` method takes [ipc9set::W](ipc9set::W) writer structure"]
impl crate::Writable for IPC9SET {}
#[doc = "IPC9SET register"]
pub mod ipc9set;
#[doc = "IPC9INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc9inv](ipc9inv) module"]
pub type IPC9INV = crate::Reg<u32, _IPC9INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC9INV;
#[doc = "`read()` method returns [ipc9inv::R](ipc9inv::R) reader structure"]
impl crate::Readable for IPC9INV {}
#[doc = "`write(|w| ..)` method takes [ipc9inv::W](ipc9inv::W) writer structure"]
impl crate::Writable for IPC9INV {}
#[doc = "IPC9INV register"]
pub mod ipc9inv;
#[doc = "IPC10 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc10](ipc10) module"]
pub type IPC10 = crate::Reg<u32, _IPC10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC10;
#[doc = "`read()` method returns [ipc10::R](ipc10::R) reader structure"]
impl crate::Readable for IPC10 {}
#[doc = "`write(|w| ..)` method takes [ipc10::W](ipc10::W) writer structure"]
impl crate::Writable for IPC10 {}
#[doc = "IPC10 register"]
pub mod ipc10;
#[doc = "IPC10CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc10clr](ipc10clr) module"]
pub type IPC10CLR = crate::Reg<u32, _IPC10CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC10CLR;
#[doc = "`read()` method returns [ipc10clr::R](ipc10clr::R) reader structure"]
impl crate::Readable for IPC10CLR {}
#[doc = "`write(|w| ..)` method takes [ipc10clr::W](ipc10clr::W) writer structure"]
impl crate::Writable for IPC10CLR {}
#[doc = "IPC10CLR register"]
pub mod ipc10clr;
#[doc = "IPC10SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc10set](ipc10set) module"]
pub type IPC10SET = crate::Reg<u32, _IPC10SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC10SET;
#[doc = "`read()` method returns [ipc10set::R](ipc10set::R) reader structure"]
impl crate::Readable for IPC10SET {}
#[doc = "`write(|w| ..)` method takes [ipc10set::W](ipc10set::W) writer structure"]
impl crate::Writable for IPC10SET {}
#[doc = "IPC10SET register"]
pub mod ipc10set;
#[doc = "IPC10INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc10inv](ipc10inv) module"]
pub type IPC10INV = crate::Reg<u32, _IPC10INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPC10INV;
#[doc = "`read()` method returns [ipc10inv::R](ipc10inv::R) reader structure"]
impl crate::Readable for IPC10INV {}
#[doc = "`write(|w| ..)` method takes [ipc10inv::W](ipc10inv::W) writer structure"]
impl crate::Writable for IPC10INV {}
#[doc = "IPC10INV register"]
pub mod ipc10inv;
