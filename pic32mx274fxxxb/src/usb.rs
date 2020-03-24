#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U1OTGIR register"]
    pub u1otgir: U1OTGIR,
    #[doc = "0x04 - U1OTGIRCLR register"]
    pub u1otgirclr: U1OTGIRCLR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - U1OTGIE register"]
    pub u1otgie: U1OTGIE,
    #[doc = "0x14 - U1OTGIECLR register"]
    pub u1otgieclr: U1OTGIECLR,
    #[doc = "0x18 - U1OTGIESET register"]
    pub u1otgieset: U1OTGIESET,
    #[doc = "0x1c - U1OTGIEINV register"]
    pub u1otgieinv: U1OTGIEINV,
    #[doc = "0x20 - U1OTGSTAT register"]
    pub u1otgstat: U1OTGSTAT,
    _reserved7: [u8; 12usize],
    #[doc = "0x30 - U1OTGCON register"]
    pub u1otgcon: U1OTGCON,
    #[doc = "0x34 - U1OTGCONCLR register"]
    pub u1otgconclr: U1OTGCONCLR,
    #[doc = "0x38 - U1OTGCONSET register"]
    pub u1otgconset: U1OTGCONSET,
    #[doc = "0x3c - U1OTGCONINV register"]
    pub u1otgconinv: U1OTGCONINV,
    #[doc = "0x40 - U1PWRC register"]
    pub u1pwrc: U1PWRC,
    #[doc = "0x44 - U1PWRCCLR register"]
    pub u1pwrcclr: U1PWRCCLR,
    #[doc = "0x48 - U1PWRCSET register"]
    pub u1pwrcset: U1PWRCSET,
    #[doc = "0x4c - U1PWRCINV register"]
    pub u1pwrcinv: U1PWRCINV,
    _reserved15: [u8; 368usize],
    #[doc = "0x1c0 - U1IR register"]
    pub u1ir: U1IR,
    #[doc = "0x1c4 - U1IRCLR register"]
    pub u1irclr: U1IRCLR,
    _reserved17: [u8; 8usize],
    #[doc = "0x1d0 - U1IE register"]
    pub u1ie: U1IE,
    #[doc = "0x1d4 - U1IECLR register"]
    pub u1ieclr: U1IECLR,
    #[doc = "0x1d8 - U1IESET register"]
    pub u1ieset: U1IESET,
    #[doc = "0x1dc - U1IEINV register"]
    pub u1ieinv: U1IEINV,
    #[doc = "0x1e0 - U1EIR register"]
    pub u1eir: U1EIR,
    #[doc = "0x1e4 - U1EIRCLR register"]
    pub u1eirclr: U1EIRCLR,
    _reserved23: [u8; 8usize],
    #[doc = "0x1f0 - U1EIE register"]
    pub u1eie: U1EIE,
    #[doc = "0x1f4 - U1EIECLR register"]
    pub u1eieclr: U1EIECLR,
    #[doc = "0x1f8 - U1EIESET register"]
    pub u1eieset: U1EIESET,
    #[doc = "0x1fc - U1EIEINV register"]
    pub u1eieinv: U1EIEINV,
    #[doc = "0x200 - U1STAT register"]
    pub u1stat: U1STAT,
    _reserved28: [u8; 12usize],
    #[doc = "0x210 - U1CON register"]
    pub u1con: U1CON,
    #[doc = "0x214 - U1CONCLR register"]
    pub u1conclr: U1CONCLR,
    #[doc = "0x218 - U1CONSET register"]
    pub u1conset: U1CONSET,
    #[doc = "0x21c - U1CONINV register"]
    pub u1coninv: U1CONINV,
    #[doc = "0x220 - U1ADDR register"]
    pub u1addr: U1ADDR,
    #[doc = "0x224 - U1ADDRCLR register"]
    pub u1addrclr: U1ADDRCLR,
    #[doc = "0x228 - U1ADDRSET register"]
    pub u1addrset: U1ADDRSET,
    #[doc = "0x22c - U1ADDRINV register"]
    pub u1addrinv: U1ADDRINV,
    #[doc = "0x230 - U1BDTP1 register"]
    pub u1bdtp1: U1BDTP1,
    #[doc = "0x234 - U1BDTP1CLR register"]
    pub u1bdtp1clr: U1BDTP1CLR,
    #[doc = "0x238 - U1BDTP1SET register"]
    pub u1bdtp1set: U1BDTP1SET,
    #[doc = "0x23c - U1BDTP1INV register"]
    pub u1bdtp1inv: U1BDTP1INV,
    #[doc = "0x240 - U1FRML register"]
    pub u1frml: U1FRML,
    _reserved41: [u8; 12usize],
    #[doc = "0x250 - U1FRMH register"]
    pub u1frmh: U1FRMH,
    _reserved42: [u8; 12usize],
    #[doc = "0x260 - U1TOK register"]
    pub u1tok: U1TOK,
    #[doc = "0x264 - U1TOKCLR register"]
    pub u1tokclr: U1TOKCLR,
    #[doc = "0x268 - U1TOKSET register"]
    pub u1tokset: U1TOKSET,
    #[doc = "0x26c - U1TOKINV register"]
    pub u1tokinv: U1TOKINV,
    #[doc = "0x270 - U1SOF register"]
    pub u1sof: U1SOF,
    #[doc = "0x274 - U1SOFCLR register"]
    pub u1sofclr: U1SOFCLR,
    #[doc = "0x278 - U1SOFSET register"]
    pub u1sofset: U1SOFSET,
    #[doc = "0x27c - U1SOFINV register"]
    pub u1sofinv: U1SOFINV,
    #[doc = "0x280 - U1BDTP2 register"]
    pub u1bdtp2: U1BDTP2,
    #[doc = "0x284 - U1BDTP2CLR register"]
    pub u1bdtp2clr: U1BDTP2CLR,
    #[doc = "0x288 - U1BDTP2SET register"]
    pub u1bdtp2set: U1BDTP2SET,
    #[doc = "0x28c - U1BDTP2INV register"]
    pub u1bdtp2inv: U1BDTP2INV,
    #[doc = "0x290 - U1BDTP3 register"]
    pub u1bdtp3: U1BDTP3,
    #[doc = "0x294 - U1BDTP3CLR register"]
    pub u1bdtp3clr: U1BDTP3CLR,
    #[doc = "0x298 - U1BDTP3SET register"]
    pub u1bdtp3set: U1BDTP3SET,
    #[doc = "0x29c - U1BDTP3INV register"]
    pub u1bdtp3inv: U1BDTP3INV,
    #[doc = "0x2a0 - U1CNFG1 register"]
    pub u1cnfg1: U1CNFG1,
    #[doc = "0x2a4 - U1CNFG1CLR register"]
    pub u1cnfg1clr: U1CNFG1CLR,
    #[doc = "0x2a8 - U1CNFG1SET register"]
    pub u1cnfg1set: U1CNFG1SET,
    #[doc = "0x2ac - U1CNFG1INV register"]
    pub u1cnfg1inv: U1CNFG1INV,
    _reserved62: [u8; 16usize],
    #[doc = "0x2c0 - U1EP0 register"]
    pub u1ep0: U1EP0,
    #[doc = "0x2c4 - U1EP0CLR register"]
    pub u1ep0clr: U1EP0CLR,
    #[doc = "0x2c8 - U1EP0SET register"]
    pub u1ep0set: U1EP0SET,
    #[doc = "0x2cc - U1EP0INV register"]
    pub u1ep0inv: U1EP0INV,
    #[doc = "0x2d0 - U1EP1 register"]
    pub u1ep1: U1EP1,
    #[doc = "0x2d4 - U1EP1CLR register"]
    pub u1ep1clr: U1EP1CLR,
    #[doc = "0x2d8 - U1EP1SET register"]
    pub u1ep1set: U1EP1SET,
    #[doc = "0x2dc - U1EP1INV register"]
    pub u1ep1inv: U1EP1INV,
    #[doc = "0x2e0 - U1EP2 register"]
    pub u1ep2: U1EP2,
    #[doc = "0x2e4 - U1EP2CLR register"]
    pub u1ep2clr: U1EP2CLR,
    #[doc = "0x2e8 - U1EP2SET register"]
    pub u1ep2set: U1EP2SET,
    #[doc = "0x2ec - U1EP2INV register"]
    pub u1ep2inv: U1EP2INV,
    #[doc = "0x2f0 - U1EP3 register"]
    pub u1ep3: U1EP3,
    #[doc = "0x2f4 - U1EP3CLR register"]
    pub u1ep3clr: U1EP3CLR,
    #[doc = "0x2f8 - U1EP3SET register"]
    pub u1ep3set: U1EP3SET,
    #[doc = "0x2fc - U1EP3INV register"]
    pub u1ep3inv: U1EP3INV,
    #[doc = "0x300 - U1EP4 register"]
    pub u1ep4: U1EP4,
    #[doc = "0x304 - U1EP4CLR register"]
    pub u1ep4clr: U1EP4CLR,
    #[doc = "0x308 - U1EP4SET register"]
    pub u1ep4set: U1EP4SET,
    #[doc = "0x30c - U1EP4INV register"]
    pub u1ep4inv: U1EP4INV,
    #[doc = "0x310 - U1EP5 register"]
    pub u1ep5: U1EP5,
    #[doc = "0x314 - U1EP5CLR register"]
    pub u1ep5clr: U1EP5CLR,
    #[doc = "0x318 - U1EP5SET register"]
    pub u1ep5set: U1EP5SET,
    #[doc = "0x31c - U1EP5INV register"]
    pub u1ep5inv: U1EP5INV,
    #[doc = "0x320 - U1EP6 register"]
    pub u1ep6: U1EP6,
    #[doc = "0x324 - U1EP6CLR register"]
    pub u1ep6clr: U1EP6CLR,
    #[doc = "0x328 - U1EP6SET register"]
    pub u1ep6set: U1EP6SET,
    #[doc = "0x32c - U1EP6INV register"]
    pub u1ep6inv: U1EP6INV,
    #[doc = "0x330 - U1EP7 register"]
    pub u1ep7: U1EP7,
    #[doc = "0x334 - U1EP7CLR register"]
    pub u1ep7clr: U1EP7CLR,
    #[doc = "0x338 - U1EP7SET register"]
    pub u1ep7set: U1EP7SET,
    #[doc = "0x33c - U1EP7INV register"]
    pub u1ep7inv: U1EP7INV,
    #[doc = "0x340 - U1EP8 register"]
    pub u1ep8: U1EP8,
    #[doc = "0x344 - U1EP8CLR register"]
    pub u1ep8clr: U1EP8CLR,
    #[doc = "0x348 - U1EP8SET register"]
    pub u1ep8set: U1EP8SET,
    #[doc = "0x34c - U1EP8INV register"]
    pub u1ep8inv: U1EP8INV,
    #[doc = "0x350 - U1EP9 register"]
    pub u1ep9: U1EP9,
    #[doc = "0x354 - U1EP9CLR register"]
    pub u1ep9clr: U1EP9CLR,
    #[doc = "0x358 - U1EP9SET register"]
    pub u1ep9set: U1EP9SET,
    #[doc = "0x35c - U1EP9INV register"]
    pub u1ep9inv: U1EP9INV,
    #[doc = "0x360 - U1EP10 register"]
    pub u1ep10: U1EP10,
    #[doc = "0x364 - U1EP10CLR register"]
    pub u1ep10clr: U1EP10CLR,
    #[doc = "0x368 - U1EP10SET register"]
    pub u1ep10set: U1EP10SET,
    #[doc = "0x36c - U1EP10INV register"]
    pub u1ep10inv: U1EP10INV,
    #[doc = "0x370 - U1EP11 register"]
    pub u1ep11: U1EP11,
    #[doc = "0x374 - U1EP11CLR register"]
    pub u1ep11clr: U1EP11CLR,
    #[doc = "0x378 - U1EP11SET register"]
    pub u1ep11set: U1EP11SET,
    #[doc = "0x37c - U1EP11INV register"]
    pub u1ep11inv: U1EP11INV,
    #[doc = "0x380 - U1EP12 register"]
    pub u1ep12: U1EP12,
    #[doc = "0x384 - U1EP12CLR register"]
    pub u1ep12clr: U1EP12CLR,
    #[doc = "0x388 - U1EP12SET register"]
    pub u1ep12set: U1EP12SET,
    #[doc = "0x38c - U1EP12INV register"]
    pub u1ep12inv: U1EP12INV,
    #[doc = "0x390 - U1EP13 register"]
    pub u1ep13: U1EP13,
    #[doc = "0x394 - U1EP13CLR register"]
    pub u1ep13clr: U1EP13CLR,
    #[doc = "0x398 - U1EP13SET register"]
    pub u1ep13set: U1EP13SET,
    #[doc = "0x39c - U1EP13INV register"]
    pub u1ep13inv: U1EP13INV,
    #[doc = "0x3a0 - U1EP14 register"]
    pub u1ep14: U1EP14,
    #[doc = "0x3a4 - U1EP14CLR register"]
    pub u1ep14clr: U1EP14CLR,
    #[doc = "0x3a8 - U1EP14SET register"]
    pub u1ep14set: U1EP14SET,
    #[doc = "0x3ac - U1EP14INV register"]
    pub u1ep14inv: U1EP14INV,
    #[doc = "0x3b0 - U1EP15 register"]
    pub u1ep15: U1EP15,
    #[doc = "0x3b4 - U1EP15CLR register"]
    pub u1ep15clr: U1EP15CLR,
    #[doc = "0x3b8 - U1EP15SET register"]
    pub u1ep15set: U1EP15SET,
    #[doc = "0x3bc - U1EP15INV register"]
    pub u1ep15inv: U1EP15INV,
}
#[doc = "U1OTGIR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgir](u1otgir) module"]
pub type U1OTGIR = crate::Reg<u32, _U1OTGIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGIR;
#[doc = "`read()` method returns [u1otgir::R](u1otgir::R) reader structure"]
impl crate::Readable for U1OTGIR {}
#[doc = "`write(|w| ..)` method takes [u1otgir::W](u1otgir::W) writer structure"]
impl crate::Writable for U1OTGIR {}
#[doc = "U1OTGIR register"]
pub mod u1otgir;
#[doc = "U1OTGIRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgirclr](u1otgirclr) module"]
pub type U1OTGIRCLR = crate::Reg<u32, _U1OTGIRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGIRCLR;
#[doc = "`read()` method returns [u1otgirclr::R](u1otgirclr::R) reader structure"]
impl crate::Readable for U1OTGIRCLR {}
#[doc = "`write(|w| ..)` method takes [u1otgirclr::W](u1otgirclr::W) writer structure"]
impl crate::Writable for U1OTGIRCLR {}
#[doc = "U1OTGIRCLR register"]
pub mod u1otgirclr;
#[doc = "U1OTGIE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgie](u1otgie) module"]
pub type U1OTGIE = crate::Reg<u32, _U1OTGIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGIE;
#[doc = "`read()` method returns [u1otgie::R](u1otgie::R) reader structure"]
impl crate::Readable for U1OTGIE {}
#[doc = "`write(|w| ..)` method takes [u1otgie::W](u1otgie::W) writer structure"]
impl crate::Writable for U1OTGIE {}
#[doc = "U1OTGIE register"]
pub mod u1otgie;
#[doc = "U1OTGIECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgieclr](u1otgieclr) module"]
pub type U1OTGIECLR = crate::Reg<u32, _U1OTGIECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGIECLR;
#[doc = "`read()` method returns [u1otgieclr::R](u1otgieclr::R) reader structure"]
impl crate::Readable for U1OTGIECLR {}
#[doc = "`write(|w| ..)` method takes [u1otgieclr::W](u1otgieclr::W) writer structure"]
impl crate::Writable for U1OTGIECLR {}
#[doc = "U1OTGIECLR register"]
pub mod u1otgieclr;
#[doc = "U1OTGIESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgieset](u1otgieset) module"]
pub type U1OTGIESET = crate::Reg<u32, _U1OTGIESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGIESET;
#[doc = "`read()` method returns [u1otgieset::R](u1otgieset::R) reader structure"]
impl crate::Readable for U1OTGIESET {}
#[doc = "`write(|w| ..)` method takes [u1otgieset::W](u1otgieset::W) writer structure"]
impl crate::Writable for U1OTGIESET {}
#[doc = "U1OTGIESET register"]
pub mod u1otgieset;
#[doc = "U1OTGIEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgieinv](u1otgieinv) module"]
pub type U1OTGIEINV = crate::Reg<u32, _U1OTGIEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGIEINV;
#[doc = "`read()` method returns [u1otgieinv::R](u1otgieinv::R) reader structure"]
impl crate::Readable for U1OTGIEINV {}
#[doc = "`write(|w| ..)` method takes [u1otgieinv::W](u1otgieinv::W) writer structure"]
impl crate::Writable for U1OTGIEINV {}
#[doc = "U1OTGIEINV register"]
pub mod u1otgieinv;
#[doc = "U1OTGSTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgstat](u1otgstat) module"]
pub type U1OTGSTAT = crate::Reg<u32, _U1OTGSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGSTAT;
#[doc = "`read()` method returns [u1otgstat::R](u1otgstat::R) reader structure"]
impl crate::Readable for U1OTGSTAT {}
#[doc = "`write(|w| ..)` method takes [u1otgstat::W](u1otgstat::W) writer structure"]
impl crate::Writable for U1OTGSTAT {}
#[doc = "U1OTGSTAT register"]
pub mod u1otgstat;
#[doc = "U1OTGCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgcon](u1otgcon) module"]
pub type U1OTGCON = crate::Reg<u32, _U1OTGCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGCON;
#[doc = "`read()` method returns [u1otgcon::R](u1otgcon::R) reader structure"]
impl crate::Readable for U1OTGCON {}
#[doc = "`write(|w| ..)` method takes [u1otgcon::W](u1otgcon::W) writer structure"]
impl crate::Writable for U1OTGCON {}
#[doc = "U1OTGCON register"]
pub mod u1otgcon;
#[doc = "U1OTGCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgconclr](u1otgconclr) module"]
pub type U1OTGCONCLR = crate::Reg<u32, _U1OTGCONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGCONCLR;
#[doc = "`read()` method returns [u1otgconclr::R](u1otgconclr::R) reader structure"]
impl crate::Readable for U1OTGCONCLR {}
#[doc = "`write(|w| ..)` method takes [u1otgconclr::W](u1otgconclr::W) writer structure"]
impl crate::Writable for U1OTGCONCLR {}
#[doc = "U1OTGCONCLR register"]
pub mod u1otgconclr;
#[doc = "U1OTGCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgconset](u1otgconset) module"]
pub type U1OTGCONSET = crate::Reg<u32, _U1OTGCONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGCONSET;
#[doc = "`read()` method returns [u1otgconset::R](u1otgconset::R) reader structure"]
impl crate::Readable for U1OTGCONSET {}
#[doc = "`write(|w| ..)` method takes [u1otgconset::W](u1otgconset::W) writer structure"]
impl crate::Writable for U1OTGCONSET {}
#[doc = "U1OTGCONSET register"]
pub mod u1otgconset;
#[doc = "U1OTGCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgconinv](u1otgconinv) module"]
pub type U1OTGCONINV = crate::Reg<u32, _U1OTGCONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1OTGCONINV;
#[doc = "`read()` method returns [u1otgconinv::R](u1otgconinv::R) reader structure"]
impl crate::Readable for U1OTGCONINV {}
#[doc = "`write(|w| ..)` method takes [u1otgconinv::W](u1otgconinv::W) writer structure"]
impl crate::Writable for U1OTGCONINV {}
#[doc = "U1OTGCONINV register"]
pub mod u1otgconinv;
#[doc = "U1PWRC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1pwrc](u1pwrc) module"]
pub type U1PWRC = crate::Reg<u32, _U1PWRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1PWRC;
#[doc = "`read()` method returns [u1pwrc::R](u1pwrc::R) reader structure"]
impl crate::Readable for U1PWRC {}
#[doc = "`write(|w| ..)` method takes [u1pwrc::W](u1pwrc::W) writer structure"]
impl crate::Writable for U1PWRC {}
#[doc = "U1PWRC register"]
pub mod u1pwrc;
#[doc = "U1PWRCCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1pwrcclr](u1pwrcclr) module"]
pub type U1PWRCCLR = crate::Reg<u32, _U1PWRCCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1PWRCCLR;
#[doc = "`read()` method returns [u1pwrcclr::R](u1pwrcclr::R) reader structure"]
impl crate::Readable for U1PWRCCLR {}
#[doc = "`write(|w| ..)` method takes [u1pwrcclr::W](u1pwrcclr::W) writer structure"]
impl crate::Writable for U1PWRCCLR {}
#[doc = "U1PWRCCLR register"]
pub mod u1pwrcclr;
#[doc = "U1PWRCSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1pwrcset](u1pwrcset) module"]
pub type U1PWRCSET = crate::Reg<u32, _U1PWRCSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1PWRCSET;
#[doc = "`read()` method returns [u1pwrcset::R](u1pwrcset::R) reader structure"]
impl crate::Readable for U1PWRCSET {}
#[doc = "`write(|w| ..)` method takes [u1pwrcset::W](u1pwrcset::W) writer structure"]
impl crate::Writable for U1PWRCSET {}
#[doc = "U1PWRCSET register"]
pub mod u1pwrcset;
#[doc = "U1PWRCINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1pwrcinv](u1pwrcinv) module"]
pub type U1PWRCINV = crate::Reg<u32, _U1PWRCINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1PWRCINV;
#[doc = "`read()` method returns [u1pwrcinv::R](u1pwrcinv::R) reader structure"]
impl crate::Readable for U1PWRCINV {}
#[doc = "`write(|w| ..)` method takes [u1pwrcinv::W](u1pwrcinv::W) writer structure"]
impl crate::Writable for U1PWRCINV {}
#[doc = "U1PWRCINV register"]
pub mod u1pwrcinv;
#[doc = "U1IR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ir](u1ir) module"]
pub type U1IR = crate::Reg<u32, _U1IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1IR;
#[doc = "`read()` method returns [u1ir::R](u1ir::R) reader structure"]
impl crate::Readable for U1IR {}
#[doc = "`write(|w| ..)` method takes [u1ir::W](u1ir::W) writer structure"]
impl crate::Writable for U1IR {}
#[doc = "U1IR register"]
pub mod u1ir;
#[doc = "U1IRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1irclr](u1irclr) module"]
pub type U1IRCLR = crate::Reg<u32, _U1IRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1IRCLR;
#[doc = "`read()` method returns [u1irclr::R](u1irclr::R) reader structure"]
impl crate::Readable for U1IRCLR {}
#[doc = "`write(|w| ..)` method takes [u1irclr::W](u1irclr::W) writer structure"]
impl crate::Writable for U1IRCLR {}
#[doc = "U1IRCLR register"]
pub mod u1irclr;
#[doc = "U1IE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ie](u1ie) module"]
pub type U1IE = crate::Reg<u32, _U1IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1IE;
#[doc = "`read()` method returns [u1ie::R](u1ie::R) reader structure"]
impl crate::Readable for U1IE {}
#[doc = "`write(|w| ..)` method takes [u1ie::W](u1ie::W) writer structure"]
impl crate::Writable for U1IE {}
#[doc = "U1IE register"]
pub mod u1ie;
#[doc = "U1IECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ieclr](u1ieclr) module"]
pub type U1IECLR = crate::Reg<u32, _U1IECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1IECLR;
#[doc = "`read()` method returns [u1ieclr::R](u1ieclr::R) reader structure"]
impl crate::Readable for U1IECLR {}
#[doc = "`write(|w| ..)` method takes [u1ieclr::W](u1ieclr::W) writer structure"]
impl crate::Writable for U1IECLR {}
#[doc = "U1IECLR register"]
pub mod u1ieclr;
#[doc = "U1IESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ieset](u1ieset) module"]
pub type U1IESET = crate::Reg<u32, _U1IESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1IESET;
#[doc = "`read()` method returns [u1ieset::R](u1ieset::R) reader structure"]
impl crate::Readable for U1IESET {}
#[doc = "`write(|w| ..)` method takes [u1ieset::W](u1ieset::W) writer structure"]
impl crate::Writable for U1IESET {}
#[doc = "U1IESET register"]
pub mod u1ieset;
#[doc = "U1IEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ieinv](u1ieinv) module"]
pub type U1IEINV = crate::Reg<u32, _U1IEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1IEINV;
#[doc = "`read()` method returns [u1ieinv::R](u1ieinv::R) reader structure"]
impl crate::Readable for U1IEINV {}
#[doc = "`write(|w| ..)` method takes [u1ieinv::W](u1ieinv::W) writer structure"]
impl crate::Writable for U1IEINV {}
#[doc = "U1IEINV register"]
pub mod u1ieinv;
#[doc = "U1EIR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1eir](u1eir) module"]
pub type U1EIR = crate::Reg<u32, _U1EIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EIR;
#[doc = "`read()` method returns [u1eir::R](u1eir::R) reader structure"]
impl crate::Readable for U1EIR {}
#[doc = "`write(|w| ..)` method takes [u1eir::W](u1eir::W) writer structure"]
impl crate::Writable for U1EIR {}
#[doc = "U1EIR register"]
pub mod u1eir;
#[doc = "U1EIRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1eirclr](u1eirclr) module"]
pub type U1EIRCLR = crate::Reg<u32, _U1EIRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EIRCLR;
#[doc = "`read()` method returns [u1eirclr::R](u1eirclr::R) reader structure"]
impl crate::Readable for U1EIRCLR {}
#[doc = "`write(|w| ..)` method takes [u1eirclr::W](u1eirclr::W) writer structure"]
impl crate::Writable for U1EIRCLR {}
#[doc = "U1EIRCLR register"]
pub mod u1eirclr;
#[doc = "U1EIE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1eie](u1eie) module"]
pub type U1EIE = crate::Reg<u32, _U1EIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EIE;
#[doc = "`read()` method returns [u1eie::R](u1eie::R) reader structure"]
impl crate::Readable for U1EIE {}
#[doc = "`write(|w| ..)` method takes [u1eie::W](u1eie::W) writer structure"]
impl crate::Writable for U1EIE {}
#[doc = "U1EIE register"]
pub mod u1eie;
#[doc = "U1EIECLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1eieclr](u1eieclr) module"]
pub type U1EIECLR = crate::Reg<u32, _U1EIECLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EIECLR;
#[doc = "`read()` method returns [u1eieclr::R](u1eieclr::R) reader structure"]
impl crate::Readable for U1EIECLR {}
#[doc = "`write(|w| ..)` method takes [u1eieclr::W](u1eieclr::W) writer structure"]
impl crate::Writable for U1EIECLR {}
#[doc = "U1EIECLR register"]
pub mod u1eieclr;
#[doc = "U1EIESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1eieset](u1eieset) module"]
pub type U1EIESET = crate::Reg<u32, _U1EIESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EIESET;
#[doc = "`read()` method returns [u1eieset::R](u1eieset::R) reader structure"]
impl crate::Readable for U1EIESET {}
#[doc = "`write(|w| ..)` method takes [u1eieset::W](u1eieset::W) writer structure"]
impl crate::Writable for U1EIESET {}
#[doc = "U1EIESET register"]
pub mod u1eieset;
#[doc = "U1EIEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1eieinv](u1eieinv) module"]
pub type U1EIEINV = crate::Reg<u32, _U1EIEINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EIEINV;
#[doc = "`read()` method returns [u1eieinv::R](u1eieinv::R) reader structure"]
impl crate::Readable for U1EIEINV {}
#[doc = "`write(|w| ..)` method takes [u1eieinv::W](u1eieinv::W) writer structure"]
impl crate::Writable for U1EIEINV {}
#[doc = "U1EIEINV register"]
pub mod u1eieinv;
#[doc = "U1STAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1stat](u1stat) module"]
pub type U1STAT = crate::Reg<u32, _U1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1STAT;
#[doc = "`read()` method returns [u1stat::R](u1stat::R) reader structure"]
impl crate::Readable for U1STAT {}
#[doc = "`write(|w| ..)` method takes [u1stat::W](u1stat::W) writer structure"]
impl crate::Writable for U1STAT {}
#[doc = "U1STAT register"]
pub mod u1stat;
#[doc = "U1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1con](u1con) module"]
pub type U1CON = crate::Reg<u32, _U1CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CON;
#[doc = "`read()` method returns [u1con::R](u1con::R) reader structure"]
impl crate::Readable for U1CON {}
#[doc = "`write(|w| ..)` method takes [u1con::W](u1con::W) writer structure"]
impl crate::Writable for U1CON {}
#[doc = "U1CON register"]
pub mod u1con;
#[doc = "U1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1conclr](u1conclr) module"]
pub type U1CONCLR = crate::Reg<u32, _U1CONCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CONCLR;
#[doc = "`read()` method returns [u1conclr::R](u1conclr::R) reader structure"]
impl crate::Readable for U1CONCLR {}
#[doc = "`write(|w| ..)` method takes [u1conclr::W](u1conclr::W) writer structure"]
impl crate::Writable for U1CONCLR {}
#[doc = "U1CONCLR register"]
pub mod u1conclr;
#[doc = "U1CONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1conset](u1conset) module"]
pub type U1CONSET = crate::Reg<u32, _U1CONSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CONSET;
#[doc = "`read()` method returns [u1conset::R](u1conset::R) reader structure"]
impl crate::Readable for U1CONSET {}
#[doc = "`write(|w| ..)` method takes [u1conset::W](u1conset::W) writer structure"]
impl crate::Writable for U1CONSET {}
#[doc = "U1CONSET register"]
pub mod u1conset;
#[doc = "U1CONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1coninv](u1coninv) module"]
pub type U1CONINV = crate::Reg<u32, _U1CONINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CONINV;
#[doc = "`read()` method returns [u1coninv::R](u1coninv::R) reader structure"]
impl crate::Readable for U1CONINV {}
#[doc = "`write(|w| ..)` method takes [u1coninv::W](u1coninv::W) writer structure"]
impl crate::Writable for U1CONINV {}
#[doc = "U1CONINV register"]
pub mod u1coninv;
#[doc = "U1ADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1addr](u1addr) module"]
pub type U1ADDR = crate::Reg<u32, _U1ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1ADDR;
#[doc = "`read()` method returns [u1addr::R](u1addr::R) reader structure"]
impl crate::Readable for U1ADDR {}
#[doc = "`write(|w| ..)` method takes [u1addr::W](u1addr::W) writer structure"]
impl crate::Writable for U1ADDR {}
#[doc = "U1ADDR register"]
pub mod u1addr;
#[doc = "U1ADDRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1addrclr](u1addrclr) module"]
pub type U1ADDRCLR = crate::Reg<u32, _U1ADDRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1ADDRCLR;
#[doc = "`read()` method returns [u1addrclr::R](u1addrclr::R) reader structure"]
impl crate::Readable for U1ADDRCLR {}
#[doc = "`write(|w| ..)` method takes [u1addrclr::W](u1addrclr::W) writer structure"]
impl crate::Writable for U1ADDRCLR {}
#[doc = "U1ADDRCLR register"]
pub mod u1addrclr;
#[doc = "U1ADDRSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1addrset](u1addrset) module"]
pub type U1ADDRSET = crate::Reg<u32, _U1ADDRSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1ADDRSET;
#[doc = "`read()` method returns [u1addrset::R](u1addrset::R) reader structure"]
impl crate::Readable for U1ADDRSET {}
#[doc = "`write(|w| ..)` method takes [u1addrset::W](u1addrset::W) writer structure"]
impl crate::Writable for U1ADDRSET {}
#[doc = "U1ADDRSET register"]
pub mod u1addrset;
#[doc = "U1ADDRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1addrinv](u1addrinv) module"]
pub type U1ADDRINV = crate::Reg<u32, _U1ADDRINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1ADDRINV;
#[doc = "`read()` method returns [u1addrinv::R](u1addrinv::R) reader structure"]
impl crate::Readable for U1ADDRINV {}
#[doc = "`write(|w| ..)` method takes [u1addrinv::W](u1addrinv::W) writer structure"]
impl crate::Writable for U1ADDRINV {}
#[doc = "U1ADDRINV register"]
pub mod u1addrinv;
#[doc = "U1BDTP1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp1](u1bdtp1) module"]
pub type U1BDTP1 = crate::Reg<u32, _U1BDTP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP1;
#[doc = "`read()` method returns [u1bdtp1::R](u1bdtp1::R) reader structure"]
impl crate::Readable for U1BDTP1 {}
#[doc = "`write(|w| ..)` method takes [u1bdtp1::W](u1bdtp1::W) writer structure"]
impl crate::Writable for U1BDTP1 {}
#[doc = "U1BDTP1 register"]
pub mod u1bdtp1;
#[doc = "U1BDTP1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp1clr](u1bdtp1clr) module"]
pub type U1BDTP1CLR = crate::Reg<u32, _U1BDTP1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP1CLR;
#[doc = "`read()` method returns [u1bdtp1clr::R](u1bdtp1clr::R) reader structure"]
impl crate::Readable for U1BDTP1CLR {}
#[doc = "`write(|w| ..)` method takes [u1bdtp1clr::W](u1bdtp1clr::W) writer structure"]
impl crate::Writable for U1BDTP1CLR {}
#[doc = "U1BDTP1CLR register"]
pub mod u1bdtp1clr;
#[doc = "U1BDTP1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp1set](u1bdtp1set) module"]
pub type U1BDTP1SET = crate::Reg<u32, _U1BDTP1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP1SET;
#[doc = "`read()` method returns [u1bdtp1set::R](u1bdtp1set::R) reader structure"]
impl crate::Readable for U1BDTP1SET {}
#[doc = "`write(|w| ..)` method takes [u1bdtp1set::W](u1bdtp1set::W) writer structure"]
impl crate::Writable for U1BDTP1SET {}
#[doc = "U1BDTP1SET register"]
pub mod u1bdtp1set;
#[doc = "U1BDTP1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp1inv](u1bdtp1inv) module"]
pub type U1BDTP1INV = crate::Reg<u32, _U1BDTP1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP1INV;
#[doc = "`read()` method returns [u1bdtp1inv::R](u1bdtp1inv::R) reader structure"]
impl crate::Readable for U1BDTP1INV {}
#[doc = "`write(|w| ..)` method takes [u1bdtp1inv::W](u1bdtp1inv::W) writer structure"]
impl crate::Writable for U1BDTP1INV {}
#[doc = "U1BDTP1INV register"]
pub mod u1bdtp1inv;
#[doc = "U1FRML register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1frml](u1frml) module"]
pub type U1FRML = crate::Reg<u32, _U1FRML>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1FRML;
#[doc = "`read()` method returns [u1frml::R](u1frml::R) reader structure"]
impl crate::Readable for U1FRML {}
#[doc = "`write(|w| ..)` method takes [u1frml::W](u1frml::W) writer structure"]
impl crate::Writable for U1FRML {}
#[doc = "U1FRML register"]
pub mod u1frml;
#[doc = "U1FRMH register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1frmh](u1frmh) module"]
pub type U1FRMH = crate::Reg<u32, _U1FRMH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1FRMH;
#[doc = "`read()` method returns [u1frmh::R](u1frmh::R) reader structure"]
impl crate::Readable for U1FRMH {}
#[doc = "`write(|w| ..)` method takes [u1frmh::W](u1frmh::W) writer structure"]
impl crate::Writable for U1FRMH {}
#[doc = "U1FRMH register"]
pub mod u1frmh;
#[doc = "U1TOK register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1tok](u1tok) module"]
pub type U1TOK = crate::Reg<u32, _U1TOK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1TOK;
#[doc = "`read()` method returns [u1tok::R](u1tok::R) reader structure"]
impl crate::Readable for U1TOK {}
#[doc = "`write(|w| ..)` method takes [u1tok::W](u1tok::W) writer structure"]
impl crate::Writable for U1TOK {}
#[doc = "U1TOK register"]
pub mod u1tok;
#[doc = "U1TOKCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1tokclr](u1tokclr) module"]
pub type U1TOKCLR = crate::Reg<u32, _U1TOKCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1TOKCLR;
#[doc = "`read()` method returns [u1tokclr::R](u1tokclr::R) reader structure"]
impl crate::Readable for U1TOKCLR {}
#[doc = "`write(|w| ..)` method takes [u1tokclr::W](u1tokclr::W) writer structure"]
impl crate::Writable for U1TOKCLR {}
#[doc = "U1TOKCLR register"]
pub mod u1tokclr;
#[doc = "U1TOKSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1tokset](u1tokset) module"]
pub type U1TOKSET = crate::Reg<u32, _U1TOKSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1TOKSET;
#[doc = "`read()` method returns [u1tokset::R](u1tokset::R) reader structure"]
impl crate::Readable for U1TOKSET {}
#[doc = "`write(|w| ..)` method takes [u1tokset::W](u1tokset::W) writer structure"]
impl crate::Writable for U1TOKSET {}
#[doc = "U1TOKSET register"]
pub mod u1tokset;
#[doc = "U1TOKINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1tokinv](u1tokinv) module"]
pub type U1TOKINV = crate::Reg<u32, _U1TOKINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1TOKINV;
#[doc = "`read()` method returns [u1tokinv::R](u1tokinv::R) reader structure"]
impl crate::Readable for U1TOKINV {}
#[doc = "`write(|w| ..)` method takes [u1tokinv::W](u1tokinv::W) writer structure"]
impl crate::Writable for U1TOKINV {}
#[doc = "U1TOKINV register"]
pub mod u1tokinv;
#[doc = "U1SOF register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1sof](u1sof) module"]
pub type U1SOF = crate::Reg<u32, _U1SOF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1SOF;
#[doc = "`read()` method returns [u1sof::R](u1sof::R) reader structure"]
impl crate::Readable for U1SOF {}
#[doc = "`write(|w| ..)` method takes [u1sof::W](u1sof::W) writer structure"]
impl crate::Writable for U1SOF {}
#[doc = "U1SOF register"]
pub mod u1sof;
#[doc = "U1SOFCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1sofclr](u1sofclr) module"]
pub type U1SOFCLR = crate::Reg<u32, _U1SOFCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1SOFCLR;
#[doc = "`read()` method returns [u1sofclr::R](u1sofclr::R) reader structure"]
impl crate::Readable for U1SOFCLR {}
#[doc = "`write(|w| ..)` method takes [u1sofclr::W](u1sofclr::W) writer structure"]
impl crate::Writable for U1SOFCLR {}
#[doc = "U1SOFCLR register"]
pub mod u1sofclr;
#[doc = "U1SOFSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1sofset](u1sofset) module"]
pub type U1SOFSET = crate::Reg<u32, _U1SOFSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1SOFSET;
#[doc = "`read()` method returns [u1sofset::R](u1sofset::R) reader structure"]
impl crate::Readable for U1SOFSET {}
#[doc = "`write(|w| ..)` method takes [u1sofset::W](u1sofset::W) writer structure"]
impl crate::Writable for U1SOFSET {}
#[doc = "U1SOFSET register"]
pub mod u1sofset;
#[doc = "U1SOFINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1sofinv](u1sofinv) module"]
pub type U1SOFINV = crate::Reg<u32, _U1SOFINV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1SOFINV;
#[doc = "`read()` method returns [u1sofinv::R](u1sofinv::R) reader structure"]
impl crate::Readable for U1SOFINV {}
#[doc = "`write(|w| ..)` method takes [u1sofinv::W](u1sofinv::W) writer structure"]
impl crate::Writable for U1SOFINV {}
#[doc = "U1SOFINV register"]
pub mod u1sofinv;
#[doc = "U1BDTP2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp2](u1bdtp2) module"]
pub type U1BDTP2 = crate::Reg<u32, _U1BDTP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP2;
#[doc = "`read()` method returns [u1bdtp2::R](u1bdtp2::R) reader structure"]
impl crate::Readable for U1BDTP2 {}
#[doc = "`write(|w| ..)` method takes [u1bdtp2::W](u1bdtp2::W) writer structure"]
impl crate::Writable for U1BDTP2 {}
#[doc = "U1BDTP2 register"]
pub mod u1bdtp2;
#[doc = "U1BDTP2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp2clr](u1bdtp2clr) module"]
pub type U1BDTP2CLR = crate::Reg<u32, _U1BDTP2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP2CLR;
#[doc = "`read()` method returns [u1bdtp2clr::R](u1bdtp2clr::R) reader structure"]
impl crate::Readable for U1BDTP2CLR {}
#[doc = "`write(|w| ..)` method takes [u1bdtp2clr::W](u1bdtp2clr::W) writer structure"]
impl crate::Writable for U1BDTP2CLR {}
#[doc = "U1BDTP2CLR register"]
pub mod u1bdtp2clr;
#[doc = "U1BDTP2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp2set](u1bdtp2set) module"]
pub type U1BDTP2SET = crate::Reg<u32, _U1BDTP2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP2SET;
#[doc = "`read()` method returns [u1bdtp2set::R](u1bdtp2set::R) reader structure"]
impl crate::Readable for U1BDTP2SET {}
#[doc = "`write(|w| ..)` method takes [u1bdtp2set::W](u1bdtp2set::W) writer structure"]
impl crate::Writable for U1BDTP2SET {}
#[doc = "U1BDTP2SET register"]
pub mod u1bdtp2set;
#[doc = "U1BDTP2INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp2inv](u1bdtp2inv) module"]
pub type U1BDTP2INV = crate::Reg<u32, _U1BDTP2INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP2INV;
#[doc = "`read()` method returns [u1bdtp2inv::R](u1bdtp2inv::R) reader structure"]
impl crate::Readable for U1BDTP2INV {}
#[doc = "`write(|w| ..)` method takes [u1bdtp2inv::W](u1bdtp2inv::W) writer structure"]
impl crate::Writable for U1BDTP2INV {}
#[doc = "U1BDTP2INV register"]
pub mod u1bdtp2inv;
#[doc = "U1BDTP3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp3](u1bdtp3) module"]
pub type U1BDTP3 = crate::Reg<u32, _U1BDTP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP3;
#[doc = "`read()` method returns [u1bdtp3::R](u1bdtp3::R) reader structure"]
impl crate::Readable for U1BDTP3 {}
#[doc = "`write(|w| ..)` method takes [u1bdtp3::W](u1bdtp3::W) writer structure"]
impl crate::Writable for U1BDTP3 {}
#[doc = "U1BDTP3 register"]
pub mod u1bdtp3;
#[doc = "U1BDTP3CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp3clr](u1bdtp3clr) module"]
pub type U1BDTP3CLR = crate::Reg<u32, _U1BDTP3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP3CLR;
#[doc = "`read()` method returns [u1bdtp3clr::R](u1bdtp3clr::R) reader structure"]
impl crate::Readable for U1BDTP3CLR {}
#[doc = "`write(|w| ..)` method takes [u1bdtp3clr::W](u1bdtp3clr::W) writer structure"]
impl crate::Writable for U1BDTP3CLR {}
#[doc = "U1BDTP3CLR register"]
pub mod u1bdtp3clr;
#[doc = "U1BDTP3SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp3set](u1bdtp3set) module"]
pub type U1BDTP3SET = crate::Reg<u32, _U1BDTP3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP3SET;
#[doc = "`read()` method returns [u1bdtp3set::R](u1bdtp3set::R) reader structure"]
impl crate::Readable for U1BDTP3SET {}
#[doc = "`write(|w| ..)` method takes [u1bdtp3set::W](u1bdtp3set::W) writer structure"]
impl crate::Writable for U1BDTP3SET {}
#[doc = "U1BDTP3SET register"]
pub mod u1bdtp3set;
#[doc = "U1BDTP3INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1bdtp3inv](u1bdtp3inv) module"]
pub type U1BDTP3INV = crate::Reg<u32, _U1BDTP3INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1BDTP3INV;
#[doc = "`read()` method returns [u1bdtp3inv::R](u1bdtp3inv::R) reader structure"]
impl crate::Readable for U1BDTP3INV {}
#[doc = "`write(|w| ..)` method takes [u1bdtp3inv::W](u1bdtp3inv::W) writer structure"]
impl crate::Writable for U1BDTP3INV {}
#[doc = "U1BDTP3INV register"]
pub mod u1bdtp3inv;
#[doc = "U1CNFG1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1cnfg1](u1cnfg1) module"]
pub type U1CNFG1 = crate::Reg<u32, _U1CNFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CNFG1;
#[doc = "`read()` method returns [u1cnfg1::R](u1cnfg1::R) reader structure"]
impl crate::Readable for U1CNFG1 {}
#[doc = "`write(|w| ..)` method takes [u1cnfg1::W](u1cnfg1::W) writer structure"]
impl crate::Writable for U1CNFG1 {}
#[doc = "U1CNFG1 register"]
pub mod u1cnfg1;
#[doc = "U1CNFG1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1cnfg1clr](u1cnfg1clr) module"]
pub type U1CNFG1CLR = crate::Reg<u32, _U1CNFG1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CNFG1CLR;
#[doc = "`read()` method returns [u1cnfg1clr::R](u1cnfg1clr::R) reader structure"]
impl crate::Readable for U1CNFG1CLR {}
#[doc = "`write(|w| ..)` method takes [u1cnfg1clr::W](u1cnfg1clr::W) writer structure"]
impl crate::Writable for U1CNFG1CLR {}
#[doc = "U1CNFG1CLR register"]
pub mod u1cnfg1clr;
#[doc = "U1CNFG1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1cnfg1set](u1cnfg1set) module"]
pub type U1CNFG1SET = crate::Reg<u32, _U1CNFG1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CNFG1SET;
#[doc = "`read()` method returns [u1cnfg1set::R](u1cnfg1set::R) reader structure"]
impl crate::Readable for U1CNFG1SET {}
#[doc = "`write(|w| ..)` method takes [u1cnfg1set::W](u1cnfg1set::W) writer structure"]
impl crate::Writable for U1CNFG1SET {}
#[doc = "U1CNFG1SET register"]
pub mod u1cnfg1set;
#[doc = "U1CNFG1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1cnfg1inv](u1cnfg1inv) module"]
pub type U1CNFG1INV = crate::Reg<u32, _U1CNFG1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1CNFG1INV;
#[doc = "`read()` method returns [u1cnfg1inv::R](u1cnfg1inv::R) reader structure"]
impl crate::Readable for U1CNFG1INV {}
#[doc = "`write(|w| ..)` method takes [u1cnfg1inv::W](u1cnfg1inv::W) writer structure"]
impl crate::Writable for U1CNFG1INV {}
#[doc = "U1CNFG1INV register"]
pub mod u1cnfg1inv;
#[doc = "U1EP0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep0](u1ep0) module"]
pub type U1EP0 = crate::Reg<u32, _U1EP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP0;
#[doc = "`read()` method returns [u1ep0::R](u1ep0::R) reader structure"]
impl crate::Readable for U1EP0 {}
#[doc = "`write(|w| ..)` method takes [u1ep0::W](u1ep0::W) writer structure"]
impl crate::Writable for U1EP0 {}
#[doc = "U1EP0 register"]
pub mod u1ep0;
#[doc = "U1EP0CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep0clr](u1ep0clr) module"]
pub type U1EP0CLR = crate::Reg<u32, _U1EP0CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP0CLR;
#[doc = "`read()` method returns [u1ep0clr::R](u1ep0clr::R) reader structure"]
impl crate::Readable for U1EP0CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep0clr::W](u1ep0clr::W) writer structure"]
impl crate::Writable for U1EP0CLR {}
#[doc = "U1EP0CLR register"]
pub mod u1ep0clr;
#[doc = "U1EP0SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep0set](u1ep0set) module"]
pub type U1EP0SET = crate::Reg<u32, _U1EP0SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP0SET;
#[doc = "`read()` method returns [u1ep0set::R](u1ep0set::R) reader structure"]
impl crate::Readable for U1EP0SET {}
#[doc = "`write(|w| ..)` method takes [u1ep0set::W](u1ep0set::W) writer structure"]
impl crate::Writable for U1EP0SET {}
#[doc = "U1EP0SET register"]
pub mod u1ep0set;
#[doc = "U1EP0INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep0inv](u1ep0inv) module"]
pub type U1EP0INV = crate::Reg<u32, _U1EP0INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP0INV;
#[doc = "`read()` method returns [u1ep0inv::R](u1ep0inv::R) reader structure"]
impl crate::Readable for U1EP0INV {}
#[doc = "`write(|w| ..)` method takes [u1ep0inv::W](u1ep0inv::W) writer structure"]
impl crate::Writable for U1EP0INV {}
#[doc = "U1EP0INV register"]
pub mod u1ep0inv;
#[doc = "U1EP1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep1](u1ep1) module"]
pub type U1EP1 = crate::Reg<u32, _U1EP1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP1;
#[doc = "`read()` method returns [u1ep1::R](u1ep1::R) reader structure"]
impl crate::Readable for U1EP1 {}
#[doc = "`write(|w| ..)` method takes [u1ep1::W](u1ep1::W) writer structure"]
impl crate::Writable for U1EP1 {}
#[doc = "U1EP1 register"]
pub mod u1ep1;
#[doc = "U1EP1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep1clr](u1ep1clr) module"]
pub type U1EP1CLR = crate::Reg<u32, _U1EP1CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP1CLR;
#[doc = "`read()` method returns [u1ep1clr::R](u1ep1clr::R) reader structure"]
impl crate::Readable for U1EP1CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep1clr::W](u1ep1clr::W) writer structure"]
impl crate::Writable for U1EP1CLR {}
#[doc = "U1EP1CLR register"]
pub mod u1ep1clr;
#[doc = "U1EP1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep1set](u1ep1set) module"]
pub type U1EP1SET = crate::Reg<u32, _U1EP1SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP1SET;
#[doc = "`read()` method returns [u1ep1set::R](u1ep1set::R) reader structure"]
impl crate::Readable for U1EP1SET {}
#[doc = "`write(|w| ..)` method takes [u1ep1set::W](u1ep1set::W) writer structure"]
impl crate::Writable for U1EP1SET {}
#[doc = "U1EP1SET register"]
pub mod u1ep1set;
#[doc = "U1EP1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep1inv](u1ep1inv) module"]
pub type U1EP1INV = crate::Reg<u32, _U1EP1INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP1INV;
#[doc = "`read()` method returns [u1ep1inv::R](u1ep1inv::R) reader structure"]
impl crate::Readable for U1EP1INV {}
#[doc = "`write(|w| ..)` method takes [u1ep1inv::W](u1ep1inv::W) writer structure"]
impl crate::Writable for U1EP1INV {}
#[doc = "U1EP1INV register"]
pub mod u1ep1inv;
#[doc = "U1EP2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep2](u1ep2) module"]
pub type U1EP2 = crate::Reg<u32, _U1EP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP2;
#[doc = "`read()` method returns [u1ep2::R](u1ep2::R) reader structure"]
impl crate::Readable for U1EP2 {}
#[doc = "`write(|w| ..)` method takes [u1ep2::W](u1ep2::W) writer structure"]
impl crate::Writable for U1EP2 {}
#[doc = "U1EP2 register"]
pub mod u1ep2;
#[doc = "U1EP2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep2clr](u1ep2clr) module"]
pub type U1EP2CLR = crate::Reg<u32, _U1EP2CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP2CLR;
#[doc = "`read()` method returns [u1ep2clr::R](u1ep2clr::R) reader structure"]
impl crate::Readable for U1EP2CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep2clr::W](u1ep2clr::W) writer structure"]
impl crate::Writable for U1EP2CLR {}
#[doc = "U1EP2CLR register"]
pub mod u1ep2clr;
#[doc = "U1EP2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep2set](u1ep2set) module"]
pub type U1EP2SET = crate::Reg<u32, _U1EP2SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP2SET;
#[doc = "`read()` method returns [u1ep2set::R](u1ep2set::R) reader structure"]
impl crate::Readable for U1EP2SET {}
#[doc = "`write(|w| ..)` method takes [u1ep2set::W](u1ep2set::W) writer structure"]
impl crate::Writable for U1EP2SET {}
#[doc = "U1EP2SET register"]
pub mod u1ep2set;
#[doc = "U1EP2INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep2inv](u1ep2inv) module"]
pub type U1EP2INV = crate::Reg<u32, _U1EP2INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP2INV;
#[doc = "`read()` method returns [u1ep2inv::R](u1ep2inv::R) reader structure"]
impl crate::Readable for U1EP2INV {}
#[doc = "`write(|w| ..)` method takes [u1ep2inv::W](u1ep2inv::W) writer structure"]
impl crate::Writable for U1EP2INV {}
#[doc = "U1EP2INV register"]
pub mod u1ep2inv;
#[doc = "U1EP3 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep3](u1ep3) module"]
pub type U1EP3 = crate::Reg<u32, _U1EP3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP3;
#[doc = "`read()` method returns [u1ep3::R](u1ep3::R) reader structure"]
impl crate::Readable for U1EP3 {}
#[doc = "`write(|w| ..)` method takes [u1ep3::W](u1ep3::W) writer structure"]
impl crate::Writable for U1EP3 {}
#[doc = "U1EP3 register"]
pub mod u1ep3;
#[doc = "U1EP3CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep3clr](u1ep3clr) module"]
pub type U1EP3CLR = crate::Reg<u32, _U1EP3CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP3CLR;
#[doc = "`read()` method returns [u1ep3clr::R](u1ep3clr::R) reader structure"]
impl crate::Readable for U1EP3CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep3clr::W](u1ep3clr::W) writer structure"]
impl crate::Writable for U1EP3CLR {}
#[doc = "U1EP3CLR register"]
pub mod u1ep3clr;
#[doc = "U1EP3SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep3set](u1ep3set) module"]
pub type U1EP3SET = crate::Reg<u32, _U1EP3SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP3SET;
#[doc = "`read()` method returns [u1ep3set::R](u1ep3set::R) reader structure"]
impl crate::Readable for U1EP3SET {}
#[doc = "`write(|w| ..)` method takes [u1ep3set::W](u1ep3set::W) writer structure"]
impl crate::Writable for U1EP3SET {}
#[doc = "U1EP3SET register"]
pub mod u1ep3set;
#[doc = "U1EP3INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep3inv](u1ep3inv) module"]
pub type U1EP3INV = crate::Reg<u32, _U1EP3INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP3INV;
#[doc = "`read()` method returns [u1ep3inv::R](u1ep3inv::R) reader structure"]
impl crate::Readable for U1EP3INV {}
#[doc = "`write(|w| ..)` method takes [u1ep3inv::W](u1ep3inv::W) writer structure"]
impl crate::Writable for U1EP3INV {}
#[doc = "U1EP3INV register"]
pub mod u1ep3inv;
#[doc = "U1EP4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep4](u1ep4) module"]
pub type U1EP4 = crate::Reg<u32, _U1EP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP4;
#[doc = "`read()` method returns [u1ep4::R](u1ep4::R) reader structure"]
impl crate::Readable for U1EP4 {}
#[doc = "`write(|w| ..)` method takes [u1ep4::W](u1ep4::W) writer structure"]
impl crate::Writable for U1EP4 {}
#[doc = "U1EP4 register"]
pub mod u1ep4;
#[doc = "U1EP4CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep4clr](u1ep4clr) module"]
pub type U1EP4CLR = crate::Reg<u32, _U1EP4CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP4CLR;
#[doc = "`read()` method returns [u1ep4clr::R](u1ep4clr::R) reader structure"]
impl crate::Readable for U1EP4CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep4clr::W](u1ep4clr::W) writer structure"]
impl crate::Writable for U1EP4CLR {}
#[doc = "U1EP4CLR register"]
pub mod u1ep4clr;
#[doc = "U1EP4SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep4set](u1ep4set) module"]
pub type U1EP4SET = crate::Reg<u32, _U1EP4SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP4SET;
#[doc = "`read()` method returns [u1ep4set::R](u1ep4set::R) reader structure"]
impl crate::Readable for U1EP4SET {}
#[doc = "`write(|w| ..)` method takes [u1ep4set::W](u1ep4set::W) writer structure"]
impl crate::Writable for U1EP4SET {}
#[doc = "U1EP4SET register"]
pub mod u1ep4set;
#[doc = "U1EP4INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep4inv](u1ep4inv) module"]
pub type U1EP4INV = crate::Reg<u32, _U1EP4INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP4INV;
#[doc = "`read()` method returns [u1ep4inv::R](u1ep4inv::R) reader structure"]
impl crate::Readable for U1EP4INV {}
#[doc = "`write(|w| ..)` method takes [u1ep4inv::W](u1ep4inv::W) writer structure"]
impl crate::Writable for U1EP4INV {}
#[doc = "U1EP4INV register"]
pub mod u1ep4inv;
#[doc = "U1EP5 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep5](u1ep5) module"]
pub type U1EP5 = crate::Reg<u32, _U1EP5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP5;
#[doc = "`read()` method returns [u1ep5::R](u1ep5::R) reader structure"]
impl crate::Readable for U1EP5 {}
#[doc = "`write(|w| ..)` method takes [u1ep5::W](u1ep5::W) writer structure"]
impl crate::Writable for U1EP5 {}
#[doc = "U1EP5 register"]
pub mod u1ep5;
#[doc = "U1EP5CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep5clr](u1ep5clr) module"]
pub type U1EP5CLR = crate::Reg<u32, _U1EP5CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP5CLR;
#[doc = "`read()` method returns [u1ep5clr::R](u1ep5clr::R) reader structure"]
impl crate::Readable for U1EP5CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep5clr::W](u1ep5clr::W) writer structure"]
impl crate::Writable for U1EP5CLR {}
#[doc = "U1EP5CLR register"]
pub mod u1ep5clr;
#[doc = "U1EP5SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep5set](u1ep5set) module"]
pub type U1EP5SET = crate::Reg<u32, _U1EP5SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP5SET;
#[doc = "`read()` method returns [u1ep5set::R](u1ep5set::R) reader structure"]
impl crate::Readable for U1EP5SET {}
#[doc = "`write(|w| ..)` method takes [u1ep5set::W](u1ep5set::W) writer structure"]
impl crate::Writable for U1EP5SET {}
#[doc = "U1EP5SET register"]
pub mod u1ep5set;
#[doc = "U1EP5INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep5inv](u1ep5inv) module"]
pub type U1EP5INV = crate::Reg<u32, _U1EP5INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP5INV;
#[doc = "`read()` method returns [u1ep5inv::R](u1ep5inv::R) reader structure"]
impl crate::Readable for U1EP5INV {}
#[doc = "`write(|w| ..)` method takes [u1ep5inv::W](u1ep5inv::W) writer structure"]
impl crate::Writable for U1EP5INV {}
#[doc = "U1EP5INV register"]
pub mod u1ep5inv;
#[doc = "U1EP6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep6](u1ep6) module"]
pub type U1EP6 = crate::Reg<u32, _U1EP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP6;
#[doc = "`read()` method returns [u1ep6::R](u1ep6::R) reader structure"]
impl crate::Readable for U1EP6 {}
#[doc = "`write(|w| ..)` method takes [u1ep6::W](u1ep6::W) writer structure"]
impl crate::Writable for U1EP6 {}
#[doc = "U1EP6 register"]
pub mod u1ep6;
#[doc = "U1EP6CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep6clr](u1ep6clr) module"]
pub type U1EP6CLR = crate::Reg<u32, _U1EP6CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP6CLR;
#[doc = "`read()` method returns [u1ep6clr::R](u1ep6clr::R) reader structure"]
impl crate::Readable for U1EP6CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep6clr::W](u1ep6clr::W) writer structure"]
impl crate::Writable for U1EP6CLR {}
#[doc = "U1EP6CLR register"]
pub mod u1ep6clr;
#[doc = "U1EP6SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep6set](u1ep6set) module"]
pub type U1EP6SET = crate::Reg<u32, _U1EP6SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP6SET;
#[doc = "`read()` method returns [u1ep6set::R](u1ep6set::R) reader structure"]
impl crate::Readable for U1EP6SET {}
#[doc = "`write(|w| ..)` method takes [u1ep6set::W](u1ep6set::W) writer structure"]
impl crate::Writable for U1EP6SET {}
#[doc = "U1EP6SET register"]
pub mod u1ep6set;
#[doc = "U1EP6INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep6inv](u1ep6inv) module"]
pub type U1EP6INV = crate::Reg<u32, _U1EP6INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP6INV;
#[doc = "`read()` method returns [u1ep6inv::R](u1ep6inv::R) reader structure"]
impl crate::Readable for U1EP6INV {}
#[doc = "`write(|w| ..)` method takes [u1ep6inv::W](u1ep6inv::W) writer structure"]
impl crate::Writable for U1EP6INV {}
#[doc = "U1EP6INV register"]
pub mod u1ep6inv;
#[doc = "U1EP7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep7](u1ep7) module"]
pub type U1EP7 = crate::Reg<u32, _U1EP7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP7;
#[doc = "`read()` method returns [u1ep7::R](u1ep7::R) reader structure"]
impl crate::Readable for U1EP7 {}
#[doc = "`write(|w| ..)` method takes [u1ep7::W](u1ep7::W) writer structure"]
impl crate::Writable for U1EP7 {}
#[doc = "U1EP7 register"]
pub mod u1ep7;
#[doc = "U1EP7CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep7clr](u1ep7clr) module"]
pub type U1EP7CLR = crate::Reg<u32, _U1EP7CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP7CLR;
#[doc = "`read()` method returns [u1ep7clr::R](u1ep7clr::R) reader structure"]
impl crate::Readable for U1EP7CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep7clr::W](u1ep7clr::W) writer structure"]
impl crate::Writable for U1EP7CLR {}
#[doc = "U1EP7CLR register"]
pub mod u1ep7clr;
#[doc = "U1EP7SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep7set](u1ep7set) module"]
pub type U1EP7SET = crate::Reg<u32, _U1EP7SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP7SET;
#[doc = "`read()` method returns [u1ep7set::R](u1ep7set::R) reader structure"]
impl crate::Readable for U1EP7SET {}
#[doc = "`write(|w| ..)` method takes [u1ep7set::W](u1ep7set::W) writer structure"]
impl crate::Writable for U1EP7SET {}
#[doc = "U1EP7SET register"]
pub mod u1ep7set;
#[doc = "U1EP7INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep7inv](u1ep7inv) module"]
pub type U1EP7INV = crate::Reg<u32, _U1EP7INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP7INV;
#[doc = "`read()` method returns [u1ep7inv::R](u1ep7inv::R) reader structure"]
impl crate::Readable for U1EP7INV {}
#[doc = "`write(|w| ..)` method takes [u1ep7inv::W](u1ep7inv::W) writer structure"]
impl crate::Writable for U1EP7INV {}
#[doc = "U1EP7INV register"]
pub mod u1ep7inv;
#[doc = "U1EP8 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep8](u1ep8) module"]
pub type U1EP8 = crate::Reg<u32, _U1EP8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP8;
#[doc = "`read()` method returns [u1ep8::R](u1ep8::R) reader structure"]
impl crate::Readable for U1EP8 {}
#[doc = "`write(|w| ..)` method takes [u1ep8::W](u1ep8::W) writer structure"]
impl crate::Writable for U1EP8 {}
#[doc = "U1EP8 register"]
pub mod u1ep8;
#[doc = "U1EP8CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep8clr](u1ep8clr) module"]
pub type U1EP8CLR = crate::Reg<u32, _U1EP8CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP8CLR;
#[doc = "`read()` method returns [u1ep8clr::R](u1ep8clr::R) reader structure"]
impl crate::Readable for U1EP8CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep8clr::W](u1ep8clr::W) writer structure"]
impl crate::Writable for U1EP8CLR {}
#[doc = "U1EP8CLR register"]
pub mod u1ep8clr;
#[doc = "U1EP8SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep8set](u1ep8set) module"]
pub type U1EP8SET = crate::Reg<u32, _U1EP8SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP8SET;
#[doc = "`read()` method returns [u1ep8set::R](u1ep8set::R) reader structure"]
impl crate::Readable for U1EP8SET {}
#[doc = "`write(|w| ..)` method takes [u1ep8set::W](u1ep8set::W) writer structure"]
impl crate::Writable for U1EP8SET {}
#[doc = "U1EP8SET register"]
pub mod u1ep8set;
#[doc = "U1EP8INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep8inv](u1ep8inv) module"]
pub type U1EP8INV = crate::Reg<u32, _U1EP8INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP8INV;
#[doc = "`read()` method returns [u1ep8inv::R](u1ep8inv::R) reader structure"]
impl crate::Readable for U1EP8INV {}
#[doc = "`write(|w| ..)` method takes [u1ep8inv::W](u1ep8inv::W) writer structure"]
impl crate::Writable for U1EP8INV {}
#[doc = "U1EP8INV register"]
pub mod u1ep8inv;
#[doc = "U1EP9 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep9](u1ep9) module"]
pub type U1EP9 = crate::Reg<u32, _U1EP9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP9;
#[doc = "`read()` method returns [u1ep9::R](u1ep9::R) reader structure"]
impl crate::Readable for U1EP9 {}
#[doc = "`write(|w| ..)` method takes [u1ep9::W](u1ep9::W) writer structure"]
impl crate::Writable for U1EP9 {}
#[doc = "U1EP9 register"]
pub mod u1ep9;
#[doc = "U1EP9CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep9clr](u1ep9clr) module"]
pub type U1EP9CLR = crate::Reg<u32, _U1EP9CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP9CLR;
#[doc = "`read()` method returns [u1ep9clr::R](u1ep9clr::R) reader structure"]
impl crate::Readable for U1EP9CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep9clr::W](u1ep9clr::W) writer structure"]
impl crate::Writable for U1EP9CLR {}
#[doc = "U1EP9CLR register"]
pub mod u1ep9clr;
#[doc = "U1EP9SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep9set](u1ep9set) module"]
pub type U1EP9SET = crate::Reg<u32, _U1EP9SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP9SET;
#[doc = "`read()` method returns [u1ep9set::R](u1ep9set::R) reader structure"]
impl crate::Readable for U1EP9SET {}
#[doc = "`write(|w| ..)` method takes [u1ep9set::W](u1ep9set::W) writer structure"]
impl crate::Writable for U1EP9SET {}
#[doc = "U1EP9SET register"]
pub mod u1ep9set;
#[doc = "U1EP9INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep9inv](u1ep9inv) module"]
pub type U1EP9INV = crate::Reg<u32, _U1EP9INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP9INV;
#[doc = "`read()` method returns [u1ep9inv::R](u1ep9inv::R) reader structure"]
impl crate::Readable for U1EP9INV {}
#[doc = "`write(|w| ..)` method takes [u1ep9inv::W](u1ep9inv::W) writer structure"]
impl crate::Writable for U1EP9INV {}
#[doc = "U1EP9INV register"]
pub mod u1ep9inv;
#[doc = "U1EP10 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep10](u1ep10) module"]
pub type U1EP10 = crate::Reg<u32, _U1EP10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP10;
#[doc = "`read()` method returns [u1ep10::R](u1ep10::R) reader structure"]
impl crate::Readable for U1EP10 {}
#[doc = "`write(|w| ..)` method takes [u1ep10::W](u1ep10::W) writer structure"]
impl crate::Writable for U1EP10 {}
#[doc = "U1EP10 register"]
pub mod u1ep10;
#[doc = "U1EP10CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep10clr](u1ep10clr) module"]
pub type U1EP10CLR = crate::Reg<u32, _U1EP10CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP10CLR;
#[doc = "`read()` method returns [u1ep10clr::R](u1ep10clr::R) reader structure"]
impl crate::Readable for U1EP10CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep10clr::W](u1ep10clr::W) writer structure"]
impl crate::Writable for U1EP10CLR {}
#[doc = "U1EP10CLR register"]
pub mod u1ep10clr;
#[doc = "U1EP10SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep10set](u1ep10set) module"]
pub type U1EP10SET = crate::Reg<u32, _U1EP10SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP10SET;
#[doc = "`read()` method returns [u1ep10set::R](u1ep10set::R) reader structure"]
impl crate::Readable for U1EP10SET {}
#[doc = "`write(|w| ..)` method takes [u1ep10set::W](u1ep10set::W) writer structure"]
impl crate::Writable for U1EP10SET {}
#[doc = "U1EP10SET register"]
pub mod u1ep10set;
#[doc = "U1EP10INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep10inv](u1ep10inv) module"]
pub type U1EP10INV = crate::Reg<u32, _U1EP10INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP10INV;
#[doc = "`read()` method returns [u1ep10inv::R](u1ep10inv::R) reader structure"]
impl crate::Readable for U1EP10INV {}
#[doc = "`write(|w| ..)` method takes [u1ep10inv::W](u1ep10inv::W) writer structure"]
impl crate::Writable for U1EP10INV {}
#[doc = "U1EP10INV register"]
pub mod u1ep10inv;
#[doc = "U1EP11 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep11](u1ep11) module"]
pub type U1EP11 = crate::Reg<u32, _U1EP11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP11;
#[doc = "`read()` method returns [u1ep11::R](u1ep11::R) reader structure"]
impl crate::Readable for U1EP11 {}
#[doc = "`write(|w| ..)` method takes [u1ep11::W](u1ep11::W) writer structure"]
impl crate::Writable for U1EP11 {}
#[doc = "U1EP11 register"]
pub mod u1ep11;
#[doc = "U1EP11CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep11clr](u1ep11clr) module"]
pub type U1EP11CLR = crate::Reg<u32, _U1EP11CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP11CLR;
#[doc = "`read()` method returns [u1ep11clr::R](u1ep11clr::R) reader structure"]
impl crate::Readable for U1EP11CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep11clr::W](u1ep11clr::W) writer structure"]
impl crate::Writable for U1EP11CLR {}
#[doc = "U1EP11CLR register"]
pub mod u1ep11clr;
#[doc = "U1EP11SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep11set](u1ep11set) module"]
pub type U1EP11SET = crate::Reg<u32, _U1EP11SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP11SET;
#[doc = "`read()` method returns [u1ep11set::R](u1ep11set::R) reader structure"]
impl crate::Readable for U1EP11SET {}
#[doc = "`write(|w| ..)` method takes [u1ep11set::W](u1ep11set::W) writer structure"]
impl crate::Writable for U1EP11SET {}
#[doc = "U1EP11SET register"]
pub mod u1ep11set;
#[doc = "U1EP11INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep11inv](u1ep11inv) module"]
pub type U1EP11INV = crate::Reg<u32, _U1EP11INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP11INV;
#[doc = "`read()` method returns [u1ep11inv::R](u1ep11inv::R) reader structure"]
impl crate::Readable for U1EP11INV {}
#[doc = "`write(|w| ..)` method takes [u1ep11inv::W](u1ep11inv::W) writer structure"]
impl crate::Writable for U1EP11INV {}
#[doc = "U1EP11INV register"]
pub mod u1ep11inv;
#[doc = "U1EP12 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep12](u1ep12) module"]
pub type U1EP12 = crate::Reg<u32, _U1EP12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP12;
#[doc = "`read()` method returns [u1ep12::R](u1ep12::R) reader structure"]
impl crate::Readable for U1EP12 {}
#[doc = "`write(|w| ..)` method takes [u1ep12::W](u1ep12::W) writer structure"]
impl crate::Writable for U1EP12 {}
#[doc = "U1EP12 register"]
pub mod u1ep12;
#[doc = "U1EP12CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep12clr](u1ep12clr) module"]
pub type U1EP12CLR = crate::Reg<u32, _U1EP12CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP12CLR;
#[doc = "`read()` method returns [u1ep12clr::R](u1ep12clr::R) reader structure"]
impl crate::Readable for U1EP12CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep12clr::W](u1ep12clr::W) writer structure"]
impl crate::Writable for U1EP12CLR {}
#[doc = "U1EP12CLR register"]
pub mod u1ep12clr;
#[doc = "U1EP12SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep12set](u1ep12set) module"]
pub type U1EP12SET = crate::Reg<u32, _U1EP12SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP12SET;
#[doc = "`read()` method returns [u1ep12set::R](u1ep12set::R) reader structure"]
impl crate::Readable for U1EP12SET {}
#[doc = "`write(|w| ..)` method takes [u1ep12set::W](u1ep12set::W) writer structure"]
impl crate::Writable for U1EP12SET {}
#[doc = "U1EP12SET register"]
pub mod u1ep12set;
#[doc = "U1EP12INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep12inv](u1ep12inv) module"]
pub type U1EP12INV = crate::Reg<u32, _U1EP12INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP12INV;
#[doc = "`read()` method returns [u1ep12inv::R](u1ep12inv::R) reader structure"]
impl crate::Readable for U1EP12INV {}
#[doc = "`write(|w| ..)` method takes [u1ep12inv::W](u1ep12inv::W) writer structure"]
impl crate::Writable for U1EP12INV {}
#[doc = "U1EP12INV register"]
pub mod u1ep12inv;
#[doc = "U1EP13 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep13](u1ep13) module"]
pub type U1EP13 = crate::Reg<u32, _U1EP13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP13;
#[doc = "`read()` method returns [u1ep13::R](u1ep13::R) reader structure"]
impl crate::Readable for U1EP13 {}
#[doc = "`write(|w| ..)` method takes [u1ep13::W](u1ep13::W) writer structure"]
impl crate::Writable for U1EP13 {}
#[doc = "U1EP13 register"]
pub mod u1ep13;
#[doc = "U1EP13CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep13clr](u1ep13clr) module"]
pub type U1EP13CLR = crate::Reg<u32, _U1EP13CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP13CLR;
#[doc = "`read()` method returns [u1ep13clr::R](u1ep13clr::R) reader structure"]
impl crate::Readable for U1EP13CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep13clr::W](u1ep13clr::W) writer structure"]
impl crate::Writable for U1EP13CLR {}
#[doc = "U1EP13CLR register"]
pub mod u1ep13clr;
#[doc = "U1EP13SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep13set](u1ep13set) module"]
pub type U1EP13SET = crate::Reg<u32, _U1EP13SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP13SET;
#[doc = "`read()` method returns [u1ep13set::R](u1ep13set::R) reader structure"]
impl crate::Readable for U1EP13SET {}
#[doc = "`write(|w| ..)` method takes [u1ep13set::W](u1ep13set::W) writer structure"]
impl crate::Writable for U1EP13SET {}
#[doc = "U1EP13SET register"]
pub mod u1ep13set;
#[doc = "U1EP13INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep13inv](u1ep13inv) module"]
pub type U1EP13INV = crate::Reg<u32, _U1EP13INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP13INV;
#[doc = "`read()` method returns [u1ep13inv::R](u1ep13inv::R) reader structure"]
impl crate::Readable for U1EP13INV {}
#[doc = "`write(|w| ..)` method takes [u1ep13inv::W](u1ep13inv::W) writer structure"]
impl crate::Writable for U1EP13INV {}
#[doc = "U1EP13INV register"]
pub mod u1ep13inv;
#[doc = "U1EP14 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep14](u1ep14) module"]
pub type U1EP14 = crate::Reg<u32, _U1EP14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP14;
#[doc = "`read()` method returns [u1ep14::R](u1ep14::R) reader structure"]
impl crate::Readable for U1EP14 {}
#[doc = "`write(|w| ..)` method takes [u1ep14::W](u1ep14::W) writer structure"]
impl crate::Writable for U1EP14 {}
#[doc = "U1EP14 register"]
pub mod u1ep14;
#[doc = "U1EP14CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep14clr](u1ep14clr) module"]
pub type U1EP14CLR = crate::Reg<u32, _U1EP14CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP14CLR;
#[doc = "`read()` method returns [u1ep14clr::R](u1ep14clr::R) reader structure"]
impl crate::Readable for U1EP14CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep14clr::W](u1ep14clr::W) writer structure"]
impl crate::Writable for U1EP14CLR {}
#[doc = "U1EP14CLR register"]
pub mod u1ep14clr;
#[doc = "U1EP14SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep14set](u1ep14set) module"]
pub type U1EP14SET = crate::Reg<u32, _U1EP14SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP14SET;
#[doc = "`read()` method returns [u1ep14set::R](u1ep14set::R) reader structure"]
impl crate::Readable for U1EP14SET {}
#[doc = "`write(|w| ..)` method takes [u1ep14set::W](u1ep14set::W) writer structure"]
impl crate::Writable for U1EP14SET {}
#[doc = "U1EP14SET register"]
pub mod u1ep14set;
#[doc = "U1EP14INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep14inv](u1ep14inv) module"]
pub type U1EP14INV = crate::Reg<u32, _U1EP14INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP14INV;
#[doc = "`read()` method returns [u1ep14inv::R](u1ep14inv::R) reader structure"]
impl crate::Readable for U1EP14INV {}
#[doc = "`write(|w| ..)` method takes [u1ep14inv::W](u1ep14inv::W) writer structure"]
impl crate::Writable for U1EP14INV {}
#[doc = "U1EP14INV register"]
pub mod u1ep14inv;
#[doc = "U1EP15 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep15](u1ep15) module"]
pub type U1EP15 = crate::Reg<u32, _U1EP15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP15;
#[doc = "`read()` method returns [u1ep15::R](u1ep15::R) reader structure"]
impl crate::Readable for U1EP15 {}
#[doc = "`write(|w| ..)` method takes [u1ep15::W](u1ep15::W) writer structure"]
impl crate::Writable for U1EP15 {}
#[doc = "U1EP15 register"]
pub mod u1ep15;
#[doc = "U1EP15CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep15clr](u1ep15clr) module"]
pub type U1EP15CLR = crate::Reg<u32, _U1EP15CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP15CLR;
#[doc = "`read()` method returns [u1ep15clr::R](u1ep15clr::R) reader structure"]
impl crate::Readable for U1EP15CLR {}
#[doc = "`write(|w| ..)` method takes [u1ep15clr::W](u1ep15clr::W) writer structure"]
impl crate::Writable for U1EP15CLR {}
#[doc = "U1EP15CLR register"]
pub mod u1ep15clr;
#[doc = "U1EP15SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep15set](u1ep15set) module"]
pub type U1EP15SET = crate::Reg<u32, _U1EP15SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP15SET;
#[doc = "`read()` method returns [u1ep15set::R](u1ep15set::R) reader structure"]
impl crate::Readable for U1EP15SET {}
#[doc = "`write(|w| ..)` method takes [u1ep15set::W](u1ep15set::W) writer structure"]
impl crate::Writable for U1EP15SET {}
#[doc = "U1EP15SET register"]
pub mod u1ep15set;
#[doc = "U1EP15INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep15inv](u1ep15inv) module"]
pub type U1EP15INV = crate::Reg<u32, _U1EP15INV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _U1EP15INV;
#[doc = "`read()` method returns [u1ep15inv::R](u1ep15inv::R) reader structure"]
impl crate::Readable for U1EP15INV {}
#[doc = "`write(|w| ..)` method takes [u1ep15inv::W](u1ep15inv::W) writer structure"]
impl crate::Writable for U1EP15INV {}
#[doc = "U1EP15INV register"]
pub mod u1ep15inv;
