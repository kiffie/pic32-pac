#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ANSELB register"]
    pub ansel: ANSEL,
    #[doc = "0x04 - ANSELBCLR register"]
    pub anselclr: ANSELCLR,
    #[doc = "0x08 - ANSELBSET register"]
    pub anselset: ANSELSET,
    #[doc = "0x0c - ANSELBINV register"]
    pub anselinv: ANSELINV,
    #[doc = "0x10 - TRISB register"]
    pub tris: TRIS,
    #[doc = "0x14 - TRISBCLR register"]
    pub trisclr: TRISCLR,
    #[doc = "0x18 - TRISBSET register"]
    pub trisset: TRISSET,
    #[doc = "0x1c - TRISBINV register"]
    pub trisinv: TRISINV,
    #[doc = "0x20 - PORTB register"]
    pub port: PORT,
    #[doc = "0x24 - PORTBCLR register"]
    pub portclr: PORTCLR,
    #[doc = "0x28 - PORTBSET register"]
    pub portset: PORTSET,
    #[doc = "0x2c - PORTBINV register"]
    pub portinv: PORTINV,
    #[doc = "0x30 - LATB register"]
    pub lat: LAT,
    #[doc = "0x34 - LATBCLR register"]
    pub latclr: LATCLR,
    #[doc = "0x38 - LATBSET register"]
    pub latset: LATSET,
    #[doc = "0x3c - LATBINV register"]
    pub latinv: LATINV,
    #[doc = "0x40 - ODCB register"]
    pub odc: ODC,
    #[doc = "0x44 - ODCBCLR register"]
    pub odcclr: ODCCLR,
    #[doc = "0x48 - ODCBSET register"]
    pub odcset: ODCSET,
    #[doc = "0x4c - ODCBINV register"]
    pub odcinv: ODCINV,
    #[doc = "0x50 - CNPUB register"]
    pub cnpu: CNPU,
    #[doc = "0x54 - CNPUBCLR register"]
    pub cnpuclr: CNPUCLR,
    #[doc = "0x58 - CNPUBSET register"]
    pub cnpuset: CNPUSET,
    #[doc = "0x5c - CNPUBINV register"]
    pub cnpuinv: CNPUINV,
    #[doc = "0x60 - CNPDB register"]
    pub cnpd: CNPD,
    #[doc = "0x64 - CNPDBCLR register"]
    pub cnpdclr: CNPDCLR,
    #[doc = "0x68 - CNPDBSET register"]
    pub cnpdset: CNPDSET,
    #[doc = "0x6c - CNPDBINV register"]
    pub cnpdinv: CNPDINV,
    #[doc = "0x70 - CNCONB register"]
    pub cncon: CNCON,
    #[doc = "0x74 - CNCONBCLR register"]
    pub cnconclr: CNCONCLR,
    #[doc = "0x78 - CNCONBSET register"]
    pub cnconset: CNCONSET,
    #[doc = "0x7c - CNCONBINV register"]
    pub cnconinv: CNCONINV,
    #[doc = "0x80 - CNENB register"]
    pub cnen: CNEN,
    #[doc = "0x84 - CNENBCLR register"]
    pub cnenclr: CNENCLR,
    #[doc = "0x88 - CNENBSET register"]
    pub cnenset: CNENSET,
    #[doc = "0x8c - CNENBINV register"]
    pub cneninv: CNENINV,
    #[doc = "0x90 - CNSTATB register"]
    pub cnstat: CNSTAT,
    #[doc = "0x94 - CNSTATBCLR register"]
    pub cnstatclr: CNSTATCLR,
    #[doc = "0x98 - CNSTATBSET register"]
    pub cnstatset: CNSTATSET,
    #[doc = "0x9c - CNSTATBINV register"]
    pub cnstatinv: CNSTATINV,
}
#[doc = "ANSELB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ansel](ansel) module"]
pub type ANSEL = crate::Reg<u32, _ANSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANSEL;
#[doc = "`read()` method returns [ansel::R](ansel::R) reader structure"]
impl crate::Readable for ANSEL {}
#[doc = "`write(|w| ..)` method takes [ansel::W](ansel::W) writer structure"]
impl crate::Writable for ANSEL {}
#[doc = "ANSELB register"]
pub mod ansel;
#[doc = "ANSELBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anselclr](anselclr) module"]
pub type ANSELCLR = crate::Reg<u32, _ANSELCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANSELCLR;
#[doc = "`read()` method returns [anselclr::R](anselclr::R) reader structure"]
impl crate::Readable for ANSELCLR {}
#[doc = "`write(|w| ..)` method takes [anselclr::W](anselclr::W) writer structure"]
impl crate::Writable for ANSELCLR {}
#[doc = "ANSELBCLR register"]
pub mod anselclr;
#[doc = "ANSELBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anselset](anselset) module"]
pub type ANSELSET = crate::Reg<u32, _ANSELSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANSELSET;
#[doc = "`read()` method returns [anselset::R](anselset::R) reader structure"]
impl crate::Readable for ANSELSET {}
#[doc = "`write(|w| ..)` method takes [anselset::W](anselset::W) writer structure"]
impl crate::Writable for ANSELSET {}
#[doc = "ANSELBSET register"]
pub mod anselset;
#[doc = "ANSELBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anselinv](anselinv) module"]
pub type ANSELINV = crate::Reg<u32, _ANSELINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANSELINV;
#[doc = "`read()` method returns [anselinv::R](anselinv::R) reader structure"]
impl crate::Readable for ANSELINV {}
#[doc = "`write(|w| ..)` method takes [anselinv::W](anselinv::W) writer structure"]
impl crate::Writable for ANSELINV {}
#[doc = "ANSELBINV register"]
pub mod anselinv;
#[doc = "TRISB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tris](tris) module"]
pub type TRIS = crate::Reg<u32, _TRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIS;
#[doc = "`read()` method returns [tris::R](tris::R) reader structure"]
impl crate::Readable for TRIS {}
#[doc = "`write(|w| ..)` method takes [tris::W](tris::W) writer structure"]
impl crate::Writable for TRIS {}
#[doc = "TRISB register"]
pub mod tris;
#[doc = "TRISBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trisclr](trisclr) module"]
pub type TRISCLR = crate::Reg<u32, _TRISCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRISCLR;
#[doc = "`read()` method returns [trisclr::R](trisclr::R) reader structure"]
impl crate::Readable for TRISCLR {}
#[doc = "`write(|w| ..)` method takes [trisclr::W](trisclr::W) writer structure"]
impl crate::Writable for TRISCLR {}
#[doc = "TRISBCLR register"]
pub mod trisclr;
#[doc = "TRISBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trisset](trisset) module"]
pub type TRISSET = crate::Reg<u32, _TRISSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRISSET;
#[doc = "`read()` method returns [trisset::R](trisset::R) reader structure"]
impl crate::Readable for TRISSET {}
#[doc = "`write(|w| ..)` method takes [trisset::W](trisset::W) writer structure"]
impl crate::Writable for TRISSET {}
#[doc = "TRISBSET register"]
pub mod trisset;
#[doc = "TRISBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trisinv](trisinv) module"]
pub type TRISINV = crate::Reg<u32, _TRISINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRISINV;
#[doc = "`read()` method returns [trisinv::R](trisinv::R) reader structure"]
impl crate::Readable for TRISINV {}
#[doc = "`write(|w| ..)` method takes [trisinv::W](trisinv::W) writer structure"]
impl crate::Writable for TRISINV {}
#[doc = "TRISBINV register"]
pub mod trisinv;
#[doc = "PORTB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port](port) module"]
pub type PORT = crate::Reg<u32, _PORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORT;
#[doc = "`read()` method returns [port::R](port::R) reader structure"]
impl crate::Readable for PORT {}
#[doc = "`write(|w| ..)` method takes [port::W](port::W) writer structure"]
impl crate::Writable for PORT {}
#[doc = "PORTB register"]
pub mod port;
#[doc = "PORTBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portclr](portclr) module"]
pub type PORTCLR = crate::Reg<u32, _PORTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTCLR;
#[doc = "`read()` method returns [portclr::R](portclr::R) reader structure"]
impl crate::Readable for PORTCLR {}
#[doc = "`write(|w| ..)` method takes [portclr::W](portclr::W) writer structure"]
impl crate::Writable for PORTCLR {}
#[doc = "PORTBCLR register"]
pub mod portclr;
#[doc = "PORTBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portset](portset) module"]
pub type PORTSET = crate::Reg<u32, _PORTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTSET;
#[doc = "`read()` method returns [portset::R](portset::R) reader structure"]
impl crate::Readable for PORTSET {}
#[doc = "`write(|w| ..)` method takes [portset::W](portset::W) writer structure"]
impl crate::Writable for PORTSET {}
#[doc = "PORTBSET register"]
pub mod portset;
#[doc = "PORTBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portinv](portinv) module"]
pub type PORTINV = crate::Reg<u32, _PORTINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORTINV;
#[doc = "`read()` method returns [portinv::R](portinv::R) reader structure"]
impl crate::Readable for PORTINV {}
#[doc = "`write(|w| ..)` method takes [portinv::W](portinv::W) writer structure"]
impl crate::Writable for PORTINV {}
#[doc = "PORTBINV register"]
pub mod portinv;
#[doc = "LATB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lat](lat) module"]
pub type LAT = crate::Reg<u32, _LAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAT;
#[doc = "`read()` method returns [lat::R](lat::R) reader structure"]
impl crate::Readable for LAT {}
#[doc = "`write(|w| ..)` method takes [lat::W](lat::W) writer structure"]
impl crate::Writable for LAT {}
#[doc = "LATB register"]
pub mod lat;
#[doc = "LATBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [latclr](latclr) module"]
pub type LATCLR = crate::Reg<u32, _LATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LATCLR;
#[doc = "`read()` method returns [latclr::R](latclr::R) reader structure"]
impl crate::Readable for LATCLR {}
#[doc = "`write(|w| ..)` method takes [latclr::W](latclr::W) writer structure"]
impl crate::Writable for LATCLR {}
#[doc = "LATBCLR register"]
pub mod latclr;
#[doc = "LATBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [latset](latset) module"]
pub type LATSET = crate::Reg<u32, _LATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LATSET;
#[doc = "`read()` method returns [latset::R](latset::R) reader structure"]
impl crate::Readable for LATSET {}
#[doc = "`write(|w| ..)` method takes [latset::W](latset::W) writer structure"]
impl crate::Writable for LATSET {}
#[doc = "LATBSET register"]
pub mod latset;
#[doc = "LATBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [latinv](latinv) module"]
pub type LATINV = crate::Reg<u32, _LATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LATINV;
#[doc = "`read()` method returns [latinv::R](latinv::R) reader structure"]
impl crate::Readable for LATINV {}
#[doc = "`write(|w| ..)` method takes [latinv::W](latinv::W) writer structure"]
impl crate::Writable for LATINV {}
#[doc = "LATBINV register"]
pub mod latinv;
#[doc = "ODCB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odc](odc) module"]
pub type ODC = crate::Reg<u32, _ODC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODC;
#[doc = "`read()` method returns [odc::R](odc::R) reader structure"]
impl crate::Readable for ODC {}
#[doc = "`write(|w| ..)` method takes [odc::W](odc::W) writer structure"]
impl crate::Writable for ODC {}
#[doc = "ODCB register"]
pub mod odc;
#[doc = "ODCBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcclr](odcclr) module"]
pub type ODCCLR = crate::Reg<u32, _ODCCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCCLR;
#[doc = "`read()` method returns [odcclr::R](odcclr::R) reader structure"]
impl crate::Readable for ODCCLR {}
#[doc = "`write(|w| ..)` method takes [odcclr::W](odcclr::W) writer structure"]
impl crate::Writable for ODCCLR {}
#[doc = "ODCBCLR register"]
pub mod odcclr;
#[doc = "ODCBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcset](odcset) module"]
pub type ODCSET = crate::Reg<u32, _ODCSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCSET;
#[doc = "`read()` method returns [odcset::R](odcset::R) reader structure"]
impl crate::Readable for ODCSET {}
#[doc = "`write(|w| ..)` method takes [odcset::W](odcset::W) writer structure"]
impl crate::Writable for ODCSET {}
#[doc = "ODCBSET register"]
pub mod odcset;
#[doc = "ODCBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcinv](odcinv) module"]
pub type ODCINV = crate::Reg<u32, _ODCINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODCINV;
#[doc = "`read()` method returns [odcinv::R](odcinv::R) reader structure"]
impl crate::Readable for ODCINV {}
#[doc = "`write(|w| ..)` method takes [odcinv::W](odcinv::W) writer structure"]
impl crate::Writable for ODCINV {}
#[doc = "ODCBINV register"]
pub mod odcinv;
#[doc = "CNPUB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpu](cnpu) module"]
pub type CNPU = crate::Reg<u32, _CNPU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNPU;
#[doc = "`read()` method returns [cnpu::R](cnpu::R) reader structure"]
impl crate::Readable for CNPU {}
#[doc = "`write(|w| ..)` method takes [cnpu::W](cnpu::W) writer structure"]
impl crate::Writable for CNPU {}
#[doc = "CNPUB register"]
pub mod cnpu;
#[doc = "CNPUBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpuclr](cnpuclr) module"]
pub type CNPUCLR = crate::Reg<u32, _CNPUCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNPUCLR;
#[doc = "`read()` method returns [cnpuclr::R](cnpuclr::R) reader structure"]
impl crate::Readable for CNPUCLR {}
#[doc = "`write(|w| ..)` method takes [cnpuclr::W](cnpuclr::W) writer structure"]
impl crate::Writable for CNPUCLR {}
#[doc = "CNPUBCLR register"]
pub mod cnpuclr;
#[doc = "CNPUBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpuset](cnpuset) module"]
pub type CNPUSET = crate::Reg<u32, _CNPUSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNPUSET;
#[doc = "`read()` method returns [cnpuset::R](cnpuset::R) reader structure"]
impl crate::Readable for CNPUSET {}
#[doc = "`write(|w| ..)` method takes [cnpuset::W](cnpuset::W) writer structure"]
impl crate::Writable for CNPUSET {}
#[doc = "CNPUBSET register"]
pub mod cnpuset;
#[doc = "CNPUBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpuinv](cnpuinv) module"]
pub type CNPUINV = crate::Reg<u32, _CNPUINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNPUINV;
#[doc = "`read()` method returns [cnpuinv::R](cnpuinv::R) reader structure"]
impl crate::Readable for CNPUINV {}
#[doc = "`write(|w| ..)` method takes [cnpuinv::W](cnpuinv::W) writer structure"]
impl crate::Writable for CNPUINV {}
#[doc = "CNPUBINV register"]
pub mod cnpuinv;
#[doc = "CNPDB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpd](cnpd) module"]
pub type CNPD = crate::Reg<u32, _CNPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNPD;
#[doc = "`read()` method returns [cnpd::R](cnpd::R) reader structure"]
impl crate::Readable for CNPD {}
#[doc = "`write(|w| ..)` method takes [cnpd::W](cnpd::W) writer structure"]
impl crate::Writable for CNPD {}
#[doc = "CNPDB register"]
pub mod cnpd;
#[doc = "CNPDBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpdclr](cnpdclr) module"]
pub type CNPDCLR = crate::Reg<u32, _CNPDCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNPDCLR;
#[doc = "`read()` method returns [cnpdclr::R](cnpdclr::R) reader structure"]
impl crate::Readable for CNPDCLR {}
#[doc = "`write(|w| ..)` method takes [cnpdclr::W](cnpdclr::W) writer structure"]
impl crate::Writable for CNPDCLR {}
#[doc = "CNPDBCLR register"]
pub mod cnpdclr;
#[doc = "CNPDBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpdset](cnpdset) module"]
pub type CNPDSET = crate::Reg<u32, _CNPDSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNPDSET;
#[doc = "`read()` method returns [cnpdset::R](cnpdset::R) reader structure"]
impl crate::Readable for CNPDSET {}
#[doc = "`write(|w| ..)` method takes [cnpdset::W](cnpdset::W) writer structure"]
impl crate::Writable for CNPDSET {}
#[doc = "CNPDBSET register"]
pub mod cnpdset;
#[doc = "CNPDBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpdinv](cnpdinv) module"]
pub type CNPDINV = crate::Reg<u32, _CNPDINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNPDINV;
#[doc = "`read()` method returns [cnpdinv::R](cnpdinv::R) reader structure"]
impl crate::Readable for CNPDINV {}
#[doc = "`write(|w| ..)` method takes [cnpdinv::W](cnpdinv::W) writer structure"]
impl crate::Writable for CNPDINV {}
#[doc = "CNPDBINV register"]
pub mod cnpdinv;
#[doc = "CNCONB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cncon](cncon) module"]
pub type CNCON = crate::Reg<u32, _CNCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNCON;
#[doc = "`read()` method returns [cncon::R](cncon::R) reader structure"]
impl crate::Readable for CNCON {}
#[doc = "`write(|w| ..)` method takes [cncon::W](cncon::W) writer structure"]
impl crate::Writable for CNCON {}
#[doc = "CNCONB register"]
pub mod cncon;
#[doc = "CNCONBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnconclr](cnconclr) module"]
pub type CNCONCLR = crate::Reg<u32, _CNCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNCONCLR;
#[doc = "`read()` method returns [cnconclr::R](cnconclr::R) reader structure"]
impl crate::Readable for CNCONCLR {}
#[doc = "`write(|w| ..)` method takes [cnconclr::W](cnconclr::W) writer structure"]
impl crate::Writable for CNCONCLR {}
#[doc = "CNCONBCLR register"]
pub mod cnconclr;
#[doc = "CNCONBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnconset](cnconset) module"]
pub type CNCONSET = crate::Reg<u32, _CNCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNCONSET;
#[doc = "`read()` method returns [cnconset::R](cnconset::R) reader structure"]
impl crate::Readable for CNCONSET {}
#[doc = "`write(|w| ..)` method takes [cnconset::W](cnconset::W) writer structure"]
impl crate::Writable for CNCONSET {}
#[doc = "CNCONBSET register"]
pub mod cnconset;
#[doc = "CNCONBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnconinv](cnconinv) module"]
pub type CNCONINV = crate::Reg<u32, _CNCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNCONINV;
#[doc = "`read()` method returns [cnconinv::R](cnconinv::R) reader structure"]
impl crate::Readable for CNCONINV {}
#[doc = "`write(|w| ..)` method takes [cnconinv::W](cnconinv::W) writer structure"]
impl crate::Writable for CNCONINV {}
#[doc = "CNCONBINV register"]
pub mod cnconinv;
#[doc = "CNENB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnen](cnen) module"]
pub type CNEN = crate::Reg<u32, _CNEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNEN;
#[doc = "`read()` method returns [cnen::R](cnen::R) reader structure"]
impl crate::Readable for CNEN {}
#[doc = "`write(|w| ..)` method takes [cnen::W](cnen::W) writer structure"]
impl crate::Writable for CNEN {}
#[doc = "CNENB register"]
pub mod cnen;
#[doc = "CNENBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnenclr](cnenclr) module"]
pub type CNENCLR = crate::Reg<u32, _CNENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNENCLR;
#[doc = "`read()` method returns [cnenclr::R](cnenclr::R) reader structure"]
impl crate::Readable for CNENCLR {}
#[doc = "`write(|w| ..)` method takes [cnenclr::W](cnenclr::W) writer structure"]
impl crate::Writable for CNENCLR {}
#[doc = "CNENBCLR register"]
pub mod cnenclr;
#[doc = "CNENBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnenset](cnenset) module"]
pub type CNENSET = crate::Reg<u32, _CNENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNENSET;
#[doc = "`read()` method returns [cnenset::R](cnenset::R) reader structure"]
impl crate::Readable for CNENSET {}
#[doc = "`write(|w| ..)` method takes [cnenset::W](cnenset::W) writer structure"]
impl crate::Writable for CNENSET {}
#[doc = "CNENBSET register"]
pub mod cnenset;
#[doc = "CNENBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cneninv](cneninv) module"]
pub type CNENINV = crate::Reg<u32, _CNENINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNENINV;
#[doc = "`read()` method returns [cneninv::R](cneninv::R) reader structure"]
impl crate::Readable for CNENINV {}
#[doc = "`write(|w| ..)` method takes [cneninv::W](cneninv::W) writer structure"]
impl crate::Writable for CNENINV {}
#[doc = "CNENBINV register"]
pub mod cneninv;
#[doc = "CNSTATB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnstat](cnstat) module"]
pub type CNSTAT = crate::Reg<u32, _CNSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNSTAT;
#[doc = "`read()` method returns [cnstat::R](cnstat::R) reader structure"]
impl crate::Readable for CNSTAT {}
#[doc = "`write(|w| ..)` method takes [cnstat::W](cnstat::W) writer structure"]
impl crate::Writable for CNSTAT {}
#[doc = "CNSTATB register"]
pub mod cnstat;
#[doc = "CNSTATBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnstatclr](cnstatclr) module"]
pub type CNSTATCLR = crate::Reg<u32, _CNSTATCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNSTATCLR;
#[doc = "`read()` method returns [cnstatclr::R](cnstatclr::R) reader structure"]
impl crate::Readable for CNSTATCLR {}
#[doc = "`write(|w| ..)` method takes [cnstatclr::W](cnstatclr::W) writer structure"]
impl crate::Writable for CNSTATCLR {}
#[doc = "CNSTATBCLR register"]
pub mod cnstatclr;
#[doc = "CNSTATBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnstatset](cnstatset) module"]
pub type CNSTATSET = crate::Reg<u32, _CNSTATSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNSTATSET;
#[doc = "`read()` method returns [cnstatset::R](cnstatset::R) reader structure"]
impl crate::Readable for CNSTATSET {}
#[doc = "`write(|w| ..)` method takes [cnstatset::W](cnstatset::W) writer structure"]
impl crate::Writable for CNSTATSET {}
#[doc = "CNSTATBSET register"]
pub mod cnstatset;
#[doc = "CNSTATBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnstatinv](cnstatinv) module"]
pub type CNSTATINV = crate::Reg<u32, _CNSTATINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNSTATINV;
#[doc = "`read()` method returns [cnstatinv::R](cnstatinv::R) reader structure"]
impl crate::Readable for CNSTATINV {}
#[doc = "`write(|w| ..)` method takes [cnstatinv::W](cnstatinv::W) writer structure"]
impl crate::Writable for CNSTATINV {}
#[doc = "CNSTATBINV register"]
pub mod cnstatinv;
