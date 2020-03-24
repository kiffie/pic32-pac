#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OSCCON register"]
    pub osccon: OSCCON,
    #[doc = "0x04 - OSCCONCLR register"]
    pub oscconclr: OSCCONCLR,
    #[doc = "0x08 - OSCCONSET register"]
    pub oscconset: OSCCONSET,
    #[doc = "0x0c - OSCCONINV register"]
    pub oscconinv: OSCCONINV,
    #[doc = "0x10 - OSCTUN register"]
    pub osctun: OSCTUN,
    #[doc = "0x14 - OSCTUNCLR register"]
    pub osctunclr: OSCTUNCLR,
    #[doc = "0x18 - OSCTUNSET register"]
    pub osctunset: OSCTUNSET,
    #[doc = "0x1c - OSCTUNINV register"]
    pub osctuninv: OSCTUNINV,
    #[doc = "0x20 - SPLLCON register"]
    pub spllcon: SPLLCON,
    #[doc = "0x24 - SPLLCONCLR register"]
    pub spllconclr: SPLLCONCLR,
    #[doc = "0x28 - SPLLCONSET register"]
    pub spllconset: SPLLCONSET,
    #[doc = "0x2c - SPLLCONINV register"]
    pub spllconinv: SPLLCONINV,
    #[doc = "0x30 - UPLLCON register"]
    pub upllcon: UPLLCON,
    #[doc = "0x34 - UPLLCONCLR register"]
    pub upllconclr: UPLLCONCLR,
    #[doc = "0x38 - UPLLCONSET register"]
    pub upllconset: UPLLCONSET,
    #[doc = "0x3c - UPLLCONINV register"]
    pub upllconinv: UPLLCONINV,
    #[doc = "0x40 - RCON register"]
    pub rcon: RCON,
    #[doc = "0x44 - RCONCLR register"]
    pub rconclr: RCONCLR,
    #[doc = "0x48 - RCONSET register"]
    pub rconset: RCONSET,
    #[doc = "0x4c - RCONINV register"]
    pub rconinv: RCONINV,
    #[doc = "0x50 - RSWRST register"]
    pub rswrst: RSWRST,
    #[doc = "0x54 - RSWRSTCLR register"]
    pub rswrstclr: RSWRSTCLR,
    #[doc = "0x58 - RSWRSTSET register"]
    pub rswrstset: RSWRSTSET,
    #[doc = "0x5c - RSWRSTINV register"]
    pub rswrstinv: RSWRSTINV,
    #[doc = "0x60 - RNMICON register"]
    pub rnmicon: RNMICON,
    #[doc = "0x64 - RNMICONCLR register"]
    pub rnmiconclr: RNMICONCLR,
    #[doc = "0x68 - RNMICONSET register"]
    pub rnmiconset: RNMICONSET,
    #[doc = "0x6c - RNMICONINV register"]
    pub rnmiconinv: RNMICONINV,
    #[doc = "0x70 - PWRCON register"]
    pub pwrcon: PWRCON,
    #[doc = "0x74 - PWRCONCLR register"]
    pub pwrconclr: PWRCONCLR,
    #[doc = "0x78 - PWRCONSET register"]
    pub pwrconset: PWRCONSET,
    #[doc = "0x7c - PWRCONINV register"]
    pub pwrconinv: PWRCONINV,
    #[doc = "0x80 - REFO1CON register"]
    pub refo1con: REFO1CON,
    #[doc = "0x84 - REFO1CONCLR register"]
    pub refo1conclr: REFO1CONCLR,
    #[doc = "0x88 - REFO1CONSET register"]
    pub refo1conset: REFO1CONSET,
    #[doc = "0x8c - REFO1CONINV register"]
    pub refo1coninv: REFO1CONINV,
    #[doc = "0x90 - REFO1TRIM register"]
    pub refo1trim: REFO1TRIM,
    #[doc = "0x94 - REFO1TRIMCLR register"]
    pub refo1trimclr: REFO1TRIMCLR,
    #[doc = "0x98 - REFO1TRIMSET register"]
    pub refo1trimset: REFO1TRIMSET,
    #[doc = "0x9c - REFO1TRIMINV register"]
    pub refo1triminv: REFO1TRIMINV,
    _reserved40: [u8; 96usize],
    #[doc = "0x100 - PB1DIV register"]
    pub pb1div: PB1DIV,
    #[doc = "0x104 - PB1DIVCLR register"]
    pub pb1divclr: PB1DIVCLR,
    #[doc = "0x108 - PB1DIVSET register"]
    pub pb1divset: PB1DIVSET,
    #[doc = "0x10c - PB1DIVINV register"]
    pub pb1divinv: PB1DIVINV,
    _reserved44: [u8; 192usize],
    #[doc = "0x1d0 - CLKSTAT register"]
    pub clkstat: CLKSTAT,
    #[doc = "0x1d4 - CLKSTATCLR register"]
    pub clkstatclr: CLKSTATCLR,
    #[doc = "0x1d8 - CLKSTATSET register"]
    pub clkstatset: CLKSTATSET,
    #[doc = "0x1dc - CLKSTATINV register"]
    pub clkstatinv: CLKSTATINV,
}
#[doc = "OSCCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccon](osccon) module"]
pub type OSCCON = crate::Reg<u32, _OSCCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCON;
#[doc = "`read()` method returns [osccon::R](osccon::R) reader structure"]
impl crate::Readable for OSCCON {}
#[doc = "`write(|w| ..)` method takes [osccon::W](osccon::W) writer structure"]
impl crate::Writable for OSCCON {}
#[doc = "OSCCON register"]
pub mod osccon;
#[doc = "OSCCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscconclr](oscconclr) module"]
pub type OSCCONCLR = crate::Reg<u32, _OSCCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCONCLR;
#[doc = "`read()` method returns [oscconclr::R](oscconclr::R) reader structure"]
impl crate::Readable for OSCCONCLR {}
#[doc = "`write(|w| ..)` method takes [oscconclr::W](oscconclr::W) writer structure"]
impl crate::Writable for OSCCONCLR {}
#[doc = "OSCCONCLR register"]
pub mod oscconclr;
#[doc = "OSCCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscconset](oscconset) module"]
pub type OSCCONSET = crate::Reg<u32, _OSCCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCONSET;
#[doc = "`read()` method returns [oscconset::R](oscconset::R) reader structure"]
impl crate::Readable for OSCCONSET {}
#[doc = "`write(|w| ..)` method takes [oscconset::W](oscconset::W) writer structure"]
impl crate::Writable for OSCCONSET {}
#[doc = "OSCCONSET register"]
pub mod oscconset;
#[doc = "OSCCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscconinv](oscconinv) module"]
pub type OSCCONINV = crate::Reg<u32, _OSCCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCCONINV;
#[doc = "`read()` method returns [oscconinv::R](oscconinv::R) reader structure"]
impl crate::Readable for OSCCONINV {}
#[doc = "`write(|w| ..)` method takes [oscconinv::W](oscconinv::W) writer structure"]
impl crate::Writable for OSCCONINV {}
#[doc = "OSCCONINV register"]
pub mod oscconinv;
#[doc = "OSCTUN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctun](osctun) module"]
pub type OSCTUN = crate::Reg<u32, _OSCTUN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCTUN;
#[doc = "`read()` method returns [osctun::R](osctun::R) reader structure"]
impl crate::Readable for OSCTUN {}
#[doc = "`write(|w| ..)` method takes [osctun::W](osctun::W) writer structure"]
impl crate::Writable for OSCTUN {}
#[doc = "OSCTUN register"]
pub mod osctun;
#[doc = "OSCTUNCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctunclr](osctunclr) module"]
pub type OSCTUNCLR = crate::Reg<u32, _OSCTUNCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCTUNCLR;
#[doc = "`read()` method returns [osctunclr::R](osctunclr::R) reader structure"]
impl crate::Readable for OSCTUNCLR {}
#[doc = "`write(|w| ..)` method takes [osctunclr::W](osctunclr::W) writer structure"]
impl crate::Writable for OSCTUNCLR {}
#[doc = "OSCTUNCLR register"]
pub mod osctunclr;
#[doc = "OSCTUNSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctunset](osctunset) module"]
pub type OSCTUNSET = crate::Reg<u32, _OSCTUNSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCTUNSET;
#[doc = "`read()` method returns [osctunset::R](osctunset::R) reader structure"]
impl crate::Readable for OSCTUNSET {}
#[doc = "`write(|w| ..)` method takes [osctunset::W](osctunset::W) writer structure"]
impl crate::Writable for OSCTUNSET {}
#[doc = "OSCTUNSET register"]
pub mod osctunset;
#[doc = "OSCTUNINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osctuninv](osctuninv) module"]
pub type OSCTUNINV = crate::Reg<u32, _OSCTUNINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSCTUNINV;
#[doc = "`read()` method returns [osctuninv::R](osctuninv::R) reader structure"]
impl crate::Readable for OSCTUNINV {}
#[doc = "`write(|w| ..)` method takes [osctuninv::W](osctuninv::W) writer structure"]
impl crate::Writable for OSCTUNINV {}
#[doc = "OSCTUNINV register"]
pub mod osctuninv;
#[doc = "SPLLCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllcon](spllcon) module"]
pub type SPLLCON = crate::Reg<u32, _SPLLCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPLLCON;
#[doc = "`read()` method returns [spllcon::R](spllcon::R) reader structure"]
impl crate::Readable for SPLLCON {}
#[doc = "`write(|w| ..)` method takes [spllcon::W](spllcon::W) writer structure"]
impl crate::Writable for SPLLCON {}
#[doc = "SPLLCON register"]
pub mod spllcon;
#[doc = "SPLLCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllconclr](spllconclr) module"]
pub type SPLLCONCLR = crate::Reg<u32, _SPLLCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPLLCONCLR;
#[doc = "`read()` method returns [spllconclr::R](spllconclr::R) reader structure"]
impl crate::Readable for SPLLCONCLR {}
#[doc = "`write(|w| ..)` method takes [spllconclr::W](spllconclr::W) writer structure"]
impl crate::Writable for SPLLCONCLR {}
#[doc = "SPLLCONCLR register"]
pub mod spllconclr;
#[doc = "SPLLCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllconset](spllconset) module"]
pub type SPLLCONSET = crate::Reg<u32, _SPLLCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPLLCONSET;
#[doc = "`read()` method returns [spllconset::R](spllconset::R) reader structure"]
impl crate::Readable for SPLLCONSET {}
#[doc = "`write(|w| ..)` method takes [spllconset::W](spllconset::W) writer structure"]
impl crate::Writable for SPLLCONSET {}
#[doc = "SPLLCONSET register"]
pub mod spllconset;
#[doc = "SPLLCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllconinv](spllconinv) module"]
pub type SPLLCONINV = crate::Reg<u32, _SPLLCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPLLCONINV;
#[doc = "`read()` method returns [spllconinv::R](spllconinv::R) reader structure"]
impl crate::Readable for SPLLCONINV {}
#[doc = "`write(|w| ..)` method takes [spllconinv::W](spllconinv::W) writer structure"]
impl crate::Writable for SPLLCONINV {}
#[doc = "SPLLCONINV register"]
pub mod spllconinv;
#[doc = "UPLLCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upllcon](upllcon) module"]
pub type UPLLCON = crate::Reg<u32, _UPLLCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPLLCON;
#[doc = "`read()` method returns [upllcon::R](upllcon::R) reader structure"]
impl crate::Readable for UPLLCON {}
#[doc = "`write(|w| ..)` method takes [upllcon::W](upllcon::W) writer structure"]
impl crate::Writable for UPLLCON {}
#[doc = "UPLLCON register"]
pub mod upllcon;
#[doc = "UPLLCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upllconclr](upllconclr) module"]
pub type UPLLCONCLR = crate::Reg<u32, _UPLLCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPLLCONCLR;
#[doc = "`read()` method returns [upllconclr::R](upllconclr::R) reader structure"]
impl crate::Readable for UPLLCONCLR {}
#[doc = "`write(|w| ..)` method takes [upllconclr::W](upllconclr::W) writer structure"]
impl crate::Writable for UPLLCONCLR {}
#[doc = "UPLLCONCLR register"]
pub mod upllconclr;
#[doc = "UPLLCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upllconset](upllconset) module"]
pub type UPLLCONSET = crate::Reg<u32, _UPLLCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPLLCONSET;
#[doc = "`read()` method returns [upllconset::R](upllconset::R) reader structure"]
impl crate::Readable for UPLLCONSET {}
#[doc = "`write(|w| ..)` method takes [upllconset::W](upllconset::W) writer structure"]
impl crate::Writable for UPLLCONSET {}
#[doc = "UPLLCONSET register"]
pub mod upllconset;
#[doc = "UPLLCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upllconinv](upllconinv) module"]
pub type UPLLCONINV = crate::Reg<u32, _UPLLCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UPLLCONINV;
#[doc = "`read()` method returns [upllconinv::R](upllconinv::R) reader structure"]
impl crate::Readable for UPLLCONINV {}
#[doc = "`write(|w| ..)` method takes [upllconinv::W](upllconinv::W) writer structure"]
impl crate::Writable for UPLLCONINV {}
#[doc = "UPLLCONINV register"]
pub mod upllconinv;
#[doc = "RCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcon](rcon) module"]
pub type RCON = crate::Reg<u32, _RCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCON;
#[doc = "`read()` method returns [rcon::R](rcon::R) reader structure"]
impl crate::Readable for RCON {}
#[doc = "`write(|w| ..)` method takes [rcon::W](rcon::W) writer structure"]
impl crate::Writable for RCON {}
#[doc = "RCON register"]
pub mod rcon;
#[doc = "RCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rconclr](rconclr) module"]
pub type RCONCLR = crate::Reg<u32, _RCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCONCLR;
#[doc = "`read()` method returns [rconclr::R](rconclr::R) reader structure"]
impl crate::Readable for RCONCLR {}
#[doc = "`write(|w| ..)` method takes [rconclr::W](rconclr::W) writer structure"]
impl crate::Writable for RCONCLR {}
#[doc = "RCONCLR register"]
pub mod rconclr;
#[doc = "RCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rconset](rconset) module"]
pub type RCONSET = crate::Reg<u32, _RCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCONSET;
#[doc = "`read()` method returns [rconset::R](rconset::R) reader structure"]
impl crate::Readable for RCONSET {}
#[doc = "`write(|w| ..)` method takes [rconset::W](rconset::W) writer structure"]
impl crate::Writable for RCONSET {}
#[doc = "RCONSET register"]
pub mod rconset;
#[doc = "RCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rconinv](rconinv) module"]
pub type RCONINV = crate::Reg<u32, _RCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCONINV;
#[doc = "`read()` method returns [rconinv::R](rconinv::R) reader structure"]
impl crate::Readable for RCONINV {}
#[doc = "`write(|w| ..)` method takes [rconinv::W](rconinv::W) writer structure"]
impl crate::Writable for RCONINV {}
#[doc = "RCONINV register"]
pub mod rconinv;
#[doc = "RSWRST register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrst](rswrst) module"]
pub type RSWRST = crate::Reg<u32, _RSWRST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWRST;
#[doc = "`read()` method returns [rswrst::R](rswrst::R) reader structure"]
impl crate::Readable for RSWRST {}
#[doc = "`write(|w| ..)` method takes [rswrst::W](rswrst::W) writer structure"]
impl crate::Writable for RSWRST {}
#[doc = "RSWRST register"]
pub mod rswrst;
#[doc = "RSWRSTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrstclr](rswrstclr) module"]
pub type RSWRSTCLR = crate::Reg<u32, _RSWRSTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWRSTCLR;
#[doc = "`read()` method returns [rswrstclr::R](rswrstclr::R) reader structure"]
impl crate::Readable for RSWRSTCLR {}
#[doc = "`write(|w| ..)` method takes [rswrstclr::W](rswrstclr::W) writer structure"]
impl crate::Writable for RSWRSTCLR {}
#[doc = "RSWRSTCLR register"]
pub mod rswrstclr;
#[doc = "RSWRSTSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrstset](rswrstset) module"]
pub type RSWRSTSET = crate::Reg<u32, _RSWRSTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWRSTSET;
#[doc = "`read()` method returns [rswrstset::R](rswrstset::R) reader structure"]
impl crate::Readable for RSWRSTSET {}
#[doc = "`write(|w| ..)` method takes [rswrstset::W](rswrstset::W) writer structure"]
impl crate::Writable for RSWRSTSET {}
#[doc = "RSWRSTSET register"]
pub mod rswrstset;
#[doc = "RSWRSTINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrstinv](rswrstinv) module"]
pub type RSWRSTINV = crate::Reg<u32, _RSWRSTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWRSTINV;
#[doc = "`read()` method returns [rswrstinv::R](rswrstinv::R) reader structure"]
impl crate::Readable for RSWRSTINV {}
#[doc = "`write(|w| ..)` method takes [rswrstinv::W](rswrstinv::W) writer structure"]
impl crate::Writable for RSWRSTINV {}
#[doc = "RSWRSTINV register"]
pub mod rswrstinv;
#[doc = "RNMICON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnmicon](rnmicon) module"]
pub type RNMICON = crate::Reg<u32, _RNMICON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNMICON;
#[doc = "`read()` method returns [rnmicon::R](rnmicon::R) reader structure"]
impl crate::Readable for RNMICON {}
#[doc = "`write(|w| ..)` method takes [rnmicon::W](rnmicon::W) writer structure"]
impl crate::Writable for RNMICON {}
#[doc = "RNMICON register"]
pub mod rnmicon;
#[doc = "RNMICONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnmiconclr](rnmiconclr) module"]
pub type RNMICONCLR = crate::Reg<u32, _RNMICONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNMICONCLR;
#[doc = "`read()` method returns [rnmiconclr::R](rnmiconclr::R) reader structure"]
impl crate::Readable for RNMICONCLR {}
#[doc = "`write(|w| ..)` method takes [rnmiconclr::W](rnmiconclr::W) writer structure"]
impl crate::Writable for RNMICONCLR {}
#[doc = "RNMICONCLR register"]
pub mod rnmiconclr;
#[doc = "RNMICONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnmiconset](rnmiconset) module"]
pub type RNMICONSET = crate::Reg<u32, _RNMICONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNMICONSET;
#[doc = "`read()` method returns [rnmiconset::R](rnmiconset::R) reader structure"]
impl crate::Readable for RNMICONSET {}
#[doc = "`write(|w| ..)` method takes [rnmiconset::W](rnmiconset::W) writer structure"]
impl crate::Writable for RNMICONSET {}
#[doc = "RNMICONSET register"]
pub mod rnmiconset;
#[doc = "RNMICONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnmiconinv](rnmiconinv) module"]
pub type RNMICONINV = crate::Reg<u32, _RNMICONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RNMICONINV;
#[doc = "`read()` method returns [rnmiconinv::R](rnmiconinv::R) reader structure"]
impl crate::Readable for RNMICONINV {}
#[doc = "`write(|w| ..)` method takes [rnmiconinv::W](rnmiconinv::W) writer structure"]
impl crate::Writable for RNMICONINV {}
#[doc = "RNMICONINV register"]
pub mod rnmiconinv;
#[doc = "PWRCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcon](pwrcon) module"]
pub type PWRCON = crate::Reg<u32, _PWRCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCON;
#[doc = "`read()` method returns [pwrcon::R](pwrcon::R) reader structure"]
impl crate::Readable for PWRCON {}
#[doc = "`write(|w| ..)` method takes [pwrcon::W](pwrcon::W) writer structure"]
impl crate::Writable for PWRCON {}
#[doc = "PWRCON register"]
pub mod pwrcon;
#[doc = "PWRCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrconclr](pwrconclr) module"]
pub type PWRCONCLR = crate::Reg<u32, _PWRCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCONCLR;
#[doc = "`read()` method returns [pwrconclr::R](pwrconclr::R) reader structure"]
impl crate::Readable for PWRCONCLR {}
#[doc = "`write(|w| ..)` method takes [pwrconclr::W](pwrconclr::W) writer structure"]
impl crate::Writable for PWRCONCLR {}
#[doc = "PWRCONCLR register"]
pub mod pwrconclr;
#[doc = "PWRCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrconset](pwrconset) module"]
pub type PWRCONSET = crate::Reg<u32, _PWRCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCONSET;
#[doc = "`read()` method returns [pwrconset::R](pwrconset::R) reader structure"]
impl crate::Readable for PWRCONSET {}
#[doc = "`write(|w| ..)` method takes [pwrconset::W](pwrconset::W) writer structure"]
impl crate::Writable for PWRCONSET {}
#[doc = "PWRCONSET register"]
pub mod pwrconset;
#[doc = "PWRCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrconinv](pwrconinv) module"]
pub type PWRCONINV = crate::Reg<u32, _PWRCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWRCONINV;
#[doc = "`read()` method returns [pwrconinv::R](pwrconinv::R) reader structure"]
impl crate::Readable for PWRCONINV {}
#[doc = "`write(|w| ..)` method takes [pwrconinv::W](pwrconinv::W) writer structure"]
impl crate::Writable for PWRCONINV {}
#[doc = "PWRCONINV register"]
pub mod pwrconinv;
#[doc = "REFO1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refo1con](refo1con) module"]
pub type REFO1CON = crate::Reg<u32, _REFO1CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFO1CON;
#[doc = "`read()` method returns [refo1con::R](refo1con::R) reader structure"]
impl crate::Readable for REFO1CON {}
#[doc = "`write(|w| ..)` method takes [refo1con::W](refo1con::W) writer structure"]
impl crate::Writable for REFO1CON {}
#[doc = "REFO1CON register"]
pub mod refo1con;
#[doc = "REFO1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refo1conclr](refo1conclr) module"]
pub type REFO1CONCLR = crate::Reg<u32, _REFO1CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFO1CONCLR;
#[doc = "`read()` method returns [refo1conclr::R](refo1conclr::R) reader structure"]
impl crate::Readable for REFO1CONCLR {}
#[doc = "`write(|w| ..)` method takes [refo1conclr::W](refo1conclr::W) writer structure"]
impl crate::Writable for REFO1CONCLR {}
#[doc = "REFO1CONCLR register"]
pub mod refo1conclr;
#[doc = "REFO1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refo1conset](refo1conset) module"]
pub type REFO1CONSET = crate::Reg<u32, _REFO1CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFO1CONSET;
#[doc = "`read()` method returns [refo1conset::R](refo1conset::R) reader structure"]
impl crate::Readable for REFO1CONSET {}
#[doc = "`write(|w| ..)` method takes [refo1conset::W](refo1conset::W) writer structure"]
impl crate::Writable for REFO1CONSET {}
#[doc = "REFO1CONSET register"]
pub mod refo1conset;
#[doc = "REFO1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refo1coninv](refo1coninv) module"]
pub type REFO1CONINV = crate::Reg<u32, _REFO1CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFO1CONINV;
#[doc = "`read()` method returns [refo1coninv::R](refo1coninv::R) reader structure"]
impl crate::Readable for REFO1CONINV {}
#[doc = "`write(|w| ..)` method takes [refo1coninv::W](refo1coninv::W) writer structure"]
impl crate::Writable for REFO1CONINV {}
#[doc = "REFO1CONINV register"]
pub mod refo1coninv;
#[doc = "REFO1TRIM register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refo1trim](refo1trim) module"]
pub type REFO1TRIM = crate::Reg<u32, _REFO1TRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFO1TRIM;
#[doc = "`read()` method returns [refo1trim::R](refo1trim::R) reader structure"]
impl crate::Readable for REFO1TRIM {}
#[doc = "`write(|w| ..)` method takes [refo1trim::W](refo1trim::W) writer structure"]
impl crate::Writable for REFO1TRIM {}
#[doc = "REFO1TRIM register"]
pub mod refo1trim;
#[doc = "REFO1TRIMCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refo1trimclr](refo1trimclr) module"]
pub type REFO1TRIMCLR = crate::Reg<u32, _REFO1TRIMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFO1TRIMCLR;
#[doc = "`read()` method returns [refo1trimclr::R](refo1trimclr::R) reader structure"]
impl crate::Readable for REFO1TRIMCLR {}
#[doc = "`write(|w| ..)` method takes [refo1trimclr::W](refo1trimclr::W) writer structure"]
impl crate::Writable for REFO1TRIMCLR {}
#[doc = "REFO1TRIMCLR register"]
pub mod refo1trimclr;
#[doc = "REFO1TRIMSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refo1trimset](refo1trimset) module"]
pub type REFO1TRIMSET = crate::Reg<u32, _REFO1TRIMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFO1TRIMSET;
#[doc = "`read()` method returns [refo1trimset::R](refo1trimset::R) reader structure"]
impl crate::Readable for REFO1TRIMSET {}
#[doc = "`write(|w| ..)` method takes [refo1trimset::W](refo1trimset::W) writer structure"]
impl crate::Writable for REFO1TRIMSET {}
#[doc = "REFO1TRIMSET register"]
pub mod refo1trimset;
#[doc = "REFO1TRIMINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refo1triminv](refo1triminv) module"]
pub type REFO1TRIMINV = crate::Reg<u32, _REFO1TRIMINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFO1TRIMINV;
#[doc = "`read()` method returns [refo1triminv::R](refo1triminv::R) reader structure"]
impl crate::Readable for REFO1TRIMINV {}
#[doc = "`write(|w| ..)` method takes [refo1triminv::W](refo1triminv::W) writer structure"]
impl crate::Writable for REFO1TRIMINV {}
#[doc = "REFO1TRIMINV register"]
pub mod refo1triminv;
#[doc = "PB1DIV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb1div](pb1div) module"]
pub type PB1DIV = crate::Reg<u32, _PB1DIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB1DIV;
#[doc = "`read()` method returns [pb1div::R](pb1div::R) reader structure"]
impl crate::Readable for PB1DIV {}
#[doc = "`write(|w| ..)` method takes [pb1div::W](pb1div::W) writer structure"]
impl crate::Writable for PB1DIV {}
#[doc = "PB1DIV register"]
pub mod pb1div;
#[doc = "PB1DIVCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb1divclr](pb1divclr) module"]
pub type PB1DIVCLR = crate::Reg<u32, _PB1DIVCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB1DIVCLR;
#[doc = "`read()` method returns [pb1divclr::R](pb1divclr::R) reader structure"]
impl crate::Readable for PB1DIVCLR {}
#[doc = "`write(|w| ..)` method takes [pb1divclr::W](pb1divclr::W) writer structure"]
impl crate::Writable for PB1DIVCLR {}
#[doc = "PB1DIVCLR register"]
pub mod pb1divclr;
#[doc = "PB1DIVSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb1divset](pb1divset) module"]
pub type PB1DIVSET = crate::Reg<u32, _PB1DIVSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB1DIVSET;
#[doc = "`read()` method returns [pb1divset::R](pb1divset::R) reader structure"]
impl crate::Readable for PB1DIVSET {}
#[doc = "`write(|w| ..)` method takes [pb1divset::W](pb1divset::W) writer structure"]
impl crate::Writable for PB1DIVSET {}
#[doc = "PB1DIVSET register"]
pub mod pb1divset;
#[doc = "PB1DIVINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pb1divinv](pb1divinv) module"]
pub type PB1DIVINV = crate::Reg<u32, _PB1DIVINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PB1DIVINV;
#[doc = "`read()` method returns [pb1divinv::R](pb1divinv::R) reader structure"]
impl crate::Readable for PB1DIVINV {}
#[doc = "`write(|w| ..)` method takes [pb1divinv::W](pb1divinv::W) writer structure"]
impl crate::Writable for PB1DIVINV {}
#[doc = "PB1DIVINV register"]
pub mod pb1divinv;
#[doc = "CLKSTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstat](clkstat) module"]
pub type CLKSTAT = crate::Reg<u32, _CLKSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSTAT;
#[doc = "`read()` method returns [clkstat::R](clkstat::R) reader structure"]
impl crate::Readable for CLKSTAT {}
#[doc = "`write(|w| ..)` method takes [clkstat::W](clkstat::W) writer structure"]
impl crate::Writable for CLKSTAT {}
#[doc = "CLKSTAT register"]
pub mod clkstat;
#[doc = "CLKSTATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstatclr](clkstatclr) module"]
pub type CLKSTATCLR = crate::Reg<u32, _CLKSTATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSTATCLR;
#[doc = "`read()` method returns [clkstatclr::R](clkstatclr::R) reader structure"]
impl crate::Readable for CLKSTATCLR {}
#[doc = "`write(|w| ..)` method takes [clkstatclr::W](clkstatclr::W) writer structure"]
impl crate::Writable for CLKSTATCLR {}
#[doc = "CLKSTATCLR register"]
pub mod clkstatclr;
#[doc = "CLKSTATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstatset](clkstatset) module"]
pub type CLKSTATSET = crate::Reg<u32, _CLKSTATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSTATSET;
#[doc = "`read()` method returns [clkstatset::R](clkstatset::R) reader structure"]
impl crate::Readable for CLKSTATSET {}
#[doc = "`write(|w| ..)` method takes [clkstatset::W](clkstatset::W) writer structure"]
impl crate::Writable for CLKSTATSET {}
#[doc = "CLKSTATSET register"]
pub mod clkstatset;
#[doc = "CLKSTATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstatinv](clkstatinv) module"]
pub type CLKSTATINV = crate::Reg<u32, _CLKSTATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSTATINV;
#[doc = "`read()` method returns [clkstatinv::R](clkstatinv::R) reader structure"]
impl crate::Readable for CLKSTATINV {}
#[doc = "`write(|w| ..)` method takes [clkstatinv::W](clkstatinv::W) writer structure"]
impl crate::Writable for CLKSTATINV {}
#[doc = "CLKSTATINV register"]
pub mod clkstatinv;
