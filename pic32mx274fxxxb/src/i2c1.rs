#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C1CON register"]
    pub con: CON,
    #[doc = "0x04 - I2C1CONCLR register"]
    pub conclr: CONCLR,
    #[doc = "0x08 - I2C1CONSET register"]
    pub conset: CONSET,
    #[doc = "0x0c - I2C1CONINV register"]
    pub coninv: CONINV,
    #[doc = "0x10 - I2C1STAT register"]
    pub stat: STAT,
    #[doc = "0x14 - I2C1STATCLR register"]
    pub statclr: STATCLR,
    #[doc = "0x18 - I2C1STATSET register"]
    pub statset: STATSET,
    #[doc = "0x1c - I2C1STATINV register"]
    pub statinv: STATINV,
    #[doc = "0x20 - I2C1ADD register"]
    pub add: ADD,
    #[doc = "0x24 - I2C1ADDCLR register"]
    pub addclr: ADDCLR,
    #[doc = "0x28 - I2C1ADDSET register"]
    pub addset: ADDSET,
    #[doc = "0x2c - I2C1ADDINV register"]
    pub addinv: ADDINV,
    #[doc = "0x30 - I2C1MSK register"]
    pub msk: MSK,
    #[doc = "0x34 - I2C1MSKCLR register"]
    pub mskclr: MSKCLR,
    #[doc = "0x38 - I2C1MSKSET register"]
    pub mskset: MSKSET,
    #[doc = "0x3c - I2C1MSKINV register"]
    pub mskinv: MSKINV,
    #[doc = "0x40 - I2C1BRG register"]
    pub brg: BRG,
    #[doc = "0x44 - I2C1BRGCLR register"]
    pub brgclr: BRGCLR,
    #[doc = "0x48 - I2C1BRGSET register"]
    pub brgset: BRGSET,
    #[doc = "0x4c - I2C1BRGINV register"]
    pub brginv: BRGINV,
    #[doc = "0x50 - I2C1TRN register"]
    pub trn: TRN,
    #[doc = "0x54 - I2C1TRNCLR register"]
    pub trnclr: TRNCLR,
    #[doc = "0x58 - I2C1TRNSET register"]
    pub trnset: TRNSET,
    #[doc = "0x5c - I2C1TRNINV register"]
    pub trninv: TRNINV,
    #[doc = "0x60 - I2C1RCV register"]
    pub rcv: RCV,
}
#[doc = "I2C1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con](con) module"]
pub type CON = crate::Reg<u32, _CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CON;
#[doc = "`read()` method returns [con::R](con::R) reader structure"]
impl crate::Readable for CON {}
#[doc = "`write(|w| ..)` method takes [con::W](con::W) writer structure"]
impl crate::Writable for CON {}
#[doc = "I2C1CON register"]
pub mod con;
#[doc = "I2C1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conclr](conclr) module"]
pub type CONCLR = crate::Reg<u32, _CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONCLR;
#[doc = "`read()` method returns [conclr::R](conclr::R) reader structure"]
impl crate::Readable for CONCLR {}
#[doc = "`write(|w| ..)` method takes [conclr::W](conclr::W) writer structure"]
impl crate::Writable for CONCLR {}
#[doc = "I2C1CONCLR register"]
pub mod conclr;
#[doc = "I2C1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conset](conset) module"]
pub type CONSET = crate::Reg<u32, _CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONSET;
#[doc = "`read()` method returns [conset::R](conset::R) reader structure"]
impl crate::Readable for CONSET {}
#[doc = "`write(|w| ..)` method takes [conset::W](conset::W) writer structure"]
impl crate::Writable for CONSET {}
#[doc = "I2C1CONSET register"]
pub mod conset;
#[doc = "I2C1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coninv](coninv) module"]
pub type CONINV = crate::Reg<u32, _CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONINV;
#[doc = "`read()` method returns [coninv::R](coninv::R) reader structure"]
impl crate::Readable for CONINV {}
#[doc = "`write(|w| ..)` method takes [coninv::W](coninv::W) writer structure"]
impl crate::Writable for CONINV {}
#[doc = "I2C1CONINV register"]
pub mod coninv;
#[doc = "I2C1STAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "I2C1STAT register"]
pub mod stat;
#[doc = "I2C1STATCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statclr](statclr) module"]
pub type STATCLR = crate::Reg<u32, _STATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATCLR;
#[doc = "`read()` method returns [statclr::R](statclr::R) reader structure"]
impl crate::Readable for STATCLR {}
#[doc = "`write(|w| ..)` method takes [statclr::W](statclr::W) writer structure"]
impl crate::Writable for STATCLR {}
#[doc = "I2C1STATCLR register"]
pub mod statclr;
#[doc = "I2C1STATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statset](statset) module"]
pub type STATSET = crate::Reg<u32, _STATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATSET;
#[doc = "`read()` method returns [statset::R](statset::R) reader structure"]
impl crate::Readable for STATSET {}
#[doc = "`write(|w| ..)` method takes [statset::W](statset::W) writer structure"]
impl crate::Writable for STATSET {}
#[doc = "I2C1STATSET register"]
pub mod statset;
#[doc = "I2C1STATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statinv](statinv) module"]
pub type STATINV = crate::Reg<u32, _STATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATINV;
#[doc = "`read()` method returns [statinv::R](statinv::R) reader structure"]
impl crate::Readable for STATINV {}
#[doc = "`write(|w| ..)` method takes [statinv::W](statinv::W) writer structure"]
impl crate::Writable for STATINV {}
#[doc = "I2C1STATINV register"]
pub mod statinv;
#[doc = "I2C1ADD register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [add](add) module"]
pub type ADD = crate::Reg<u32, _ADD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADD;
#[doc = "`read()` method returns [add::R](add::R) reader structure"]
impl crate::Readable for ADD {}
#[doc = "`write(|w| ..)` method takes [add::W](add::W) writer structure"]
impl crate::Writable for ADD {}
#[doc = "I2C1ADD register"]
pub mod add;
#[doc = "I2C1ADDCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addclr](addclr) module"]
pub type ADDCLR = crate::Reg<u32, _ADDCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDCLR;
#[doc = "`read()` method returns [addclr::R](addclr::R) reader structure"]
impl crate::Readable for ADDCLR {}
#[doc = "`write(|w| ..)` method takes [addclr::W](addclr::W) writer structure"]
impl crate::Writable for ADDCLR {}
#[doc = "I2C1ADDCLR register"]
pub mod addclr;
#[doc = "I2C1ADDSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addset](addset) module"]
pub type ADDSET = crate::Reg<u32, _ADDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDSET;
#[doc = "`read()` method returns [addset::R](addset::R) reader structure"]
impl crate::Readable for ADDSET {}
#[doc = "`write(|w| ..)` method takes [addset::W](addset::W) writer structure"]
impl crate::Writable for ADDSET {}
#[doc = "I2C1ADDSET register"]
pub mod addset;
#[doc = "I2C1ADDINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addinv](addinv) module"]
pub type ADDINV = crate::Reg<u32, _ADDINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDINV;
#[doc = "`read()` method returns [addinv::R](addinv::R) reader structure"]
impl crate::Readable for ADDINV {}
#[doc = "`write(|w| ..)` method takes [addinv::W](addinv::W) writer structure"]
impl crate::Writable for ADDINV {}
#[doc = "I2C1ADDINV register"]
pub mod addinv;
#[doc = "I2C1MSK register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msk](msk) module"]
pub type MSK = crate::Reg<u32, _MSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSK;
#[doc = "`read()` method returns [msk::R](msk::R) reader structure"]
impl crate::Readable for MSK {}
#[doc = "`write(|w| ..)` method takes [msk::W](msk::W) writer structure"]
impl crate::Writable for MSK {}
#[doc = "I2C1MSK register"]
pub mod msk;
#[doc = "I2C1MSKCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mskclr](mskclr) module"]
pub type MSKCLR = crate::Reg<u32, _MSKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSKCLR;
#[doc = "`read()` method returns [mskclr::R](mskclr::R) reader structure"]
impl crate::Readable for MSKCLR {}
#[doc = "`write(|w| ..)` method takes [mskclr::W](mskclr::W) writer structure"]
impl crate::Writable for MSKCLR {}
#[doc = "I2C1MSKCLR register"]
pub mod mskclr;
#[doc = "I2C1MSKSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mskset](mskset) module"]
pub type MSKSET = crate::Reg<u32, _MSKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSKSET;
#[doc = "`read()` method returns [mskset::R](mskset::R) reader structure"]
impl crate::Readable for MSKSET {}
#[doc = "`write(|w| ..)` method takes [mskset::W](mskset::W) writer structure"]
impl crate::Writable for MSKSET {}
#[doc = "I2C1MSKSET register"]
pub mod mskset;
#[doc = "I2C1MSKINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mskinv](mskinv) module"]
pub type MSKINV = crate::Reg<u32, _MSKINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSKINV;
#[doc = "`read()` method returns [mskinv::R](mskinv::R) reader structure"]
impl crate::Readable for MSKINV {}
#[doc = "`write(|w| ..)` method takes [mskinv::W](mskinv::W) writer structure"]
impl crate::Writable for MSKINV {}
#[doc = "I2C1MSKINV register"]
pub mod mskinv;
#[doc = "I2C1BRG register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brg](brg) module"]
pub type BRG = crate::Reg<u32, _BRG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRG;
#[doc = "`read()` method returns [brg::R](brg::R) reader structure"]
impl crate::Readable for BRG {}
#[doc = "`write(|w| ..)` method takes [brg::W](brg::W) writer structure"]
impl crate::Writable for BRG {}
#[doc = "I2C1BRG register"]
pub mod brg;
#[doc = "I2C1BRGCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgclr](brgclr) module"]
pub type BRGCLR = crate::Reg<u32, _BRGCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGCLR;
#[doc = "`read()` method returns [brgclr::R](brgclr::R) reader structure"]
impl crate::Readable for BRGCLR {}
#[doc = "`write(|w| ..)` method takes [brgclr::W](brgclr::W) writer structure"]
impl crate::Writable for BRGCLR {}
#[doc = "I2C1BRGCLR register"]
pub mod brgclr;
#[doc = "I2C1BRGSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgset](brgset) module"]
pub type BRGSET = crate::Reg<u32, _BRGSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGSET;
#[doc = "`read()` method returns [brgset::R](brgset::R) reader structure"]
impl crate::Readable for BRGSET {}
#[doc = "`write(|w| ..)` method takes [brgset::W](brgset::W) writer structure"]
impl crate::Writable for BRGSET {}
#[doc = "I2C1BRGSET register"]
pub mod brgset;
#[doc = "I2C1BRGINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brginv](brginv) module"]
pub type BRGINV = crate::Reg<u32, _BRGINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRGINV;
#[doc = "`read()` method returns [brginv::R](brginv::R) reader structure"]
impl crate::Readable for BRGINV {}
#[doc = "`write(|w| ..)` method takes [brginv::W](brginv::W) writer structure"]
impl crate::Writable for BRGINV {}
#[doc = "I2C1BRGINV register"]
pub mod brginv;
#[doc = "I2C1TRN register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trn](trn) module"]
pub type TRN = crate::Reg<u32, _TRN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRN;
#[doc = "`read()` method returns [trn::R](trn::R) reader structure"]
impl crate::Readable for TRN {}
#[doc = "`write(|w| ..)` method takes [trn::W](trn::W) writer structure"]
impl crate::Writable for TRN {}
#[doc = "I2C1TRN register"]
pub mod trn;
#[doc = "I2C1TRNCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trnclr](trnclr) module"]
pub type TRNCLR = crate::Reg<u32, _TRNCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNCLR;
#[doc = "`read()` method returns [trnclr::R](trnclr::R) reader structure"]
impl crate::Readable for TRNCLR {}
#[doc = "`write(|w| ..)` method takes [trnclr::W](trnclr::W) writer structure"]
impl crate::Writable for TRNCLR {}
#[doc = "I2C1TRNCLR register"]
pub mod trnclr;
#[doc = "I2C1TRNSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trnset](trnset) module"]
pub type TRNSET = crate::Reg<u32, _TRNSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNSET;
#[doc = "`read()` method returns [trnset::R](trnset::R) reader structure"]
impl crate::Readable for TRNSET {}
#[doc = "`write(|w| ..)` method takes [trnset::W](trnset::W) writer structure"]
impl crate::Writable for TRNSET {}
#[doc = "I2C1TRNSET register"]
pub mod trnset;
#[doc = "I2C1TRNINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trninv](trninv) module"]
pub type TRNINV = crate::Reg<u32, _TRNINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNINV;
#[doc = "`read()` method returns [trninv::R](trninv::R) reader structure"]
impl crate::Readable for TRNINV {}
#[doc = "`write(|w| ..)` method takes [trninv::W](trninv::W) writer structure"]
impl crate::Writable for TRNINV {}
#[doc = "I2C1TRNINV register"]
pub mod trninv;
#[doc = "I2C1RCV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcv](rcv) module"]
pub type RCV = crate::Reg<u32, _RCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCV;
#[doc = "`read()` method returns [rcv::R](rcv::R) reader structure"]
impl crate::Readable for RCV {}
#[doc = "`write(|w| ..)` method takes [rcv::W](rcv::W) writer structure"]
impl crate::Writable for RCV {}
#[doc = "I2C1RCV register"]
pub mod rcv;
