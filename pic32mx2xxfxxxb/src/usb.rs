#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - U1OTGIR register"]
    pub u1otgir: crate::Reg<u1otgir::U1OTGIR_SPEC>,
    #[doc = "0x04 - U1OTGIRCLR register"]
    pub u1otgirclr: crate::Reg<u1otgirclr::U1OTGIRCLR_SPEC>,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - U1OTGIE register"]
    pub u1otgie: crate::Reg<u1otgie::U1OTGIE_SPEC>,
    #[doc = "0x14 - U1OTGIECLR register"]
    pub u1otgieclr: crate::Reg<u1otgieclr::U1OTGIECLR_SPEC>,
    #[doc = "0x18 - U1OTGIESET register"]
    pub u1otgieset: crate::Reg<u1otgieset::U1OTGIESET_SPEC>,
    #[doc = "0x1c - U1OTGIEINV register"]
    pub u1otgieinv: crate::Reg<u1otgieinv::U1OTGIEINV_SPEC>,
    #[doc = "0x20 - U1OTGSTAT register"]
    pub u1otgstat: crate::Reg<u1otgstat::U1OTGSTAT_SPEC>,
    _reserved7: [u8; 12usize],
    #[doc = "0x30 - U1OTGCON register"]
    pub u1otgcon: crate::Reg<u1otgcon::U1OTGCON_SPEC>,
    #[doc = "0x34 - U1OTGCONCLR register"]
    pub u1otgconclr: crate::Reg<u1otgconclr::U1OTGCONCLR_SPEC>,
    #[doc = "0x38 - U1OTGCONSET register"]
    pub u1otgconset: crate::Reg<u1otgconset::U1OTGCONSET_SPEC>,
    #[doc = "0x3c - U1OTGCONINV register"]
    pub u1otgconinv: crate::Reg<u1otgconinv::U1OTGCONINV_SPEC>,
    #[doc = "0x40 - U1PWRC register"]
    pub u1pwrc: crate::Reg<u1pwrc::U1PWRC_SPEC>,
    #[doc = "0x44 - U1PWRCCLR register"]
    pub u1pwrcclr: crate::Reg<u1pwrcclr::U1PWRCCLR_SPEC>,
    #[doc = "0x48 - U1PWRCSET register"]
    pub u1pwrcset: crate::Reg<u1pwrcset::U1PWRCSET_SPEC>,
    #[doc = "0x4c - U1PWRCINV register"]
    pub u1pwrcinv: crate::Reg<u1pwrcinv::U1PWRCINV_SPEC>,
    _reserved15: [u8; 368usize],
    #[doc = "0x1c0 - U1IR register"]
    pub u1ir: crate::Reg<u1ir::U1IR_SPEC>,
    #[doc = "0x1c4 - U1IRCLR register"]
    pub u1irclr: crate::Reg<u1irclr::U1IRCLR_SPEC>,
    _reserved17: [u8; 8usize],
    #[doc = "0x1d0 - U1IE register"]
    pub u1ie: crate::Reg<u1ie::U1IE_SPEC>,
    #[doc = "0x1d4 - U1IECLR register"]
    pub u1ieclr: crate::Reg<u1ieclr::U1IECLR_SPEC>,
    #[doc = "0x1d8 - U1IESET register"]
    pub u1ieset: crate::Reg<u1ieset::U1IESET_SPEC>,
    #[doc = "0x1dc - U1IEINV register"]
    pub u1ieinv: crate::Reg<u1ieinv::U1IEINV_SPEC>,
    #[doc = "0x1e0 - U1EIR register"]
    pub u1eir: crate::Reg<u1eir::U1EIR_SPEC>,
    #[doc = "0x1e4 - U1EIRCLR register"]
    pub u1eirclr: crate::Reg<u1eirclr::U1EIRCLR_SPEC>,
    _reserved23: [u8; 8usize],
    #[doc = "0x1f0 - U1EIE register"]
    pub u1eie: crate::Reg<u1eie::U1EIE_SPEC>,
    #[doc = "0x1f4 - U1EIECLR register"]
    pub u1eieclr: crate::Reg<u1eieclr::U1EIECLR_SPEC>,
    #[doc = "0x1f8 - U1EIESET register"]
    pub u1eieset: crate::Reg<u1eieset::U1EIESET_SPEC>,
    #[doc = "0x1fc - U1EIEINV register"]
    pub u1eieinv: crate::Reg<u1eieinv::U1EIEINV_SPEC>,
    #[doc = "0x200 - U1STAT register"]
    pub u1stat: crate::Reg<u1stat::U1STAT_SPEC>,
    _reserved28: [u8; 12usize],
    #[doc = "0x210 - U1CON register"]
    pub u1con: crate::Reg<u1con::U1CON_SPEC>,
    #[doc = "0x214 - U1CONCLR register"]
    pub u1conclr: crate::Reg<u1conclr::U1CONCLR_SPEC>,
    #[doc = "0x218 - U1CONSET register"]
    pub u1conset: crate::Reg<u1conset::U1CONSET_SPEC>,
    #[doc = "0x21c - U1CONINV register"]
    pub u1coninv: crate::Reg<u1coninv::U1CONINV_SPEC>,
    #[doc = "0x220 - U1ADDR register"]
    pub u1addr: crate::Reg<u1addr::U1ADDR_SPEC>,
    #[doc = "0x224 - U1ADDRCLR register"]
    pub u1addrclr: crate::Reg<u1addrclr::U1ADDRCLR_SPEC>,
    #[doc = "0x228 - U1ADDRSET register"]
    pub u1addrset: crate::Reg<u1addrset::U1ADDRSET_SPEC>,
    #[doc = "0x22c - U1ADDRINV register"]
    pub u1addrinv: crate::Reg<u1addrinv::U1ADDRINV_SPEC>,
    #[doc = "0x230 - U1BDTP1 register"]
    pub u1bdtp1: crate::Reg<u1bdtp1::U1BDTP1_SPEC>,
    #[doc = "0x234 - U1BDTP1CLR register"]
    pub u1bdtp1clr: crate::Reg<u1bdtp1clr::U1BDTP1CLR_SPEC>,
    #[doc = "0x238 - U1BDTP1SET register"]
    pub u1bdtp1set: crate::Reg<u1bdtp1set::U1BDTP1SET_SPEC>,
    #[doc = "0x23c - U1BDTP1INV register"]
    pub u1bdtp1inv: crate::Reg<u1bdtp1inv::U1BDTP1INV_SPEC>,
    #[doc = "0x240 - U1FRML register"]
    pub u1frml: crate::Reg<u1frml::U1FRML_SPEC>,
    _reserved41: [u8; 12usize],
    #[doc = "0x250 - U1FRMH register"]
    pub u1frmh: crate::Reg<u1frmh::U1FRMH_SPEC>,
    _reserved42: [u8; 12usize],
    #[doc = "0x260 - U1TOK register"]
    pub u1tok: crate::Reg<u1tok::U1TOK_SPEC>,
    #[doc = "0x264 - U1TOKCLR register"]
    pub u1tokclr: crate::Reg<u1tokclr::U1TOKCLR_SPEC>,
    #[doc = "0x268 - U1TOKSET register"]
    pub u1tokset: crate::Reg<u1tokset::U1TOKSET_SPEC>,
    #[doc = "0x26c - U1TOKINV register"]
    pub u1tokinv: crate::Reg<u1tokinv::U1TOKINV_SPEC>,
    #[doc = "0x270 - U1SOF register"]
    pub u1sof: crate::Reg<u1sof::U1SOF_SPEC>,
    #[doc = "0x274 - U1SOFCLR register"]
    pub u1sofclr: crate::Reg<u1sofclr::U1SOFCLR_SPEC>,
    #[doc = "0x278 - U1SOFSET register"]
    pub u1sofset: crate::Reg<u1sofset::U1SOFSET_SPEC>,
    #[doc = "0x27c - U1SOFINV register"]
    pub u1sofinv: crate::Reg<u1sofinv::U1SOFINV_SPEC>,
    #[doc = "0x280 - U1BDTP2 register"]
    pub u1bdtp2: crate::Reg<u1bdtp2::U1BDTP2_SPEC>,
    #[doc = "0x284 - U1BDTP2CLR register"]
    pub u1bdtp2clr: crate::Reg<u1bdtp2clr::U1BDTP2CLR_SPEC>,
    #[doc = "0x288 - U1BDTP2SET register"]
    pub u1bdtp2set: crate::Reg<u1bdtp2set::U1BDTP2SET_SPEC>,
    #[doc = "0x28c - U1BDTP2INV register"]
    pub u1bdtp2inv: crate::Reg<u1bdtp2inv::U1BDTP2INV_SPEC>,
    #[doc = "0x290 - U1BDTP3 register"]
    pub u1bdtp3: crate::Reg<u1bdtp3::U1BDTP3_SPEC>,
    #[doc = "0x294 - U1BDTP3CLR register"]
    pub u1bdtp3clr: crate::Reg<u1bdtp3clr::U1BDTP3CLR_SPEC>,
    #[doc = "0x298 - U1BDTP3SET register"]
    pub u1bdtp3set: crate::Reg<u1bdtp3set::U1BDTP3SET_SPEC>,
    #[doc = "0x29c - U1BDTP3INV register"]
    pub u1bdtp3inv: crate::Reg<u1bdtp3inv::U1BDTP3INV_SPEC>,
    #[doc = "0x2a0 - U1CNFG1 register"]
    pub u1cnfg1: crate::Reg<u1cnfg1::U1CNFG1_SPEC>,
    #[doc = "0x2a4 - U1CNFG1CLR register"]
    pub u1cnfg1clr: crate::Reg<u1cnfg1clr::U1CNFG1CLR_SPEC>,
    #[doc = "0x2a8 - U1CNFG1SET register"]
    pub u1cnfg1set: crate::Reg<u1cnfg1set::U1CNFG1SET_SPEC>,
    #[doc = "0x2ac - U1CNFG1INV register"]
    pub u1cnfg1inv: crate::Reg<u1cnfg1inv::U1CNFG1INV_SPEC>,
    _reserved62: [u8; 16usize],
    #[doc = "0x2c0 - U1EP0 register"]
    pub u1ep0: crate::Reg<u1ep0::U1EP0_SPEC>,
    #[doc = "0x2c4 - U1EP0CLR register"]
    pub u1ep0clr: crate::Reg<u1ep0clr::U1EP0CLR_SPEC>,
    #[doc = "0x2c8 - U1EP0SET register"]
    pub u1ep0set: crate::Reg<u1ep0set::U1EP0SET_SPEC>,
    #[doc = "0x2cc - U1EP0INV register"]
    pub u1ep0inv: crate::Reg<u1ep0inv::U1EP0INV_SPEC>,
    #[doc = "0x2d0 - U1EP1 register"]
    pub u1ep1: crate::Reg<u1ep1::U1EP1_SPEC>,
    #[doc = "0x2d4 - U1EP1CLR register"]
    pub u1ep1clr: crate::Reg<u1ep1clr::U1EP1CLR_SPEC>,
    #[doc = "0x2d8 - U1EP1SET register"]
    pub u1ep1set: crate::Reg<u1ep1set::U1EP1SET_SPEC>,
    #[doc = "0x2dc - U1EP1INV register"]
    pub u1ep1inv: crate::Reg<u1ep1inv::U1EP1INV_SPEC>,
    #[doc = "0x2e0 - U1EP2 register"]
    pub u1ep2: crate::Reg<u1ep2::U1EP2_SPEC>,
    #[doc = "0x2e4 - U1EP2CLR register"]
    pub u1ep2clr: crate::Reg<u1ep2clr::U1EP2CLR_SPEC>,
    #[doc = "0x2e8 - U1EP2SET register"]
    pub u1ep2set: crate::Reg<u1ep2set::U1EP2SET_SPEC>,
    #[doc = "0x2ec - U1EP2INV register"]
    pub u1ep2inv: crate::Reg<u1ep2inv::U1EP2INV_SPEC>,
    #[doc = "0x2f0 - U1EP3 register"]
    pub u1ep3: crate::Reg<u1ep3::U1EP3_SPEC>,
    #[doc = "0x2f4 - U1EP3CLR register"]
    pub u1ep3clr: crate::Reg<u1ep3clr::U1EP3CLR_SPEC>,
    #[doc = "0x2f8 - U1EP3SET register"]
    pub u1ep3set: crate::Reg<u1ep3set::U1EP3SET_SPEC>,
    #[doc = "0x2fc - U1EP3INV register"]
    pub u1ep3inv: crate::Reg<u1ep3inv::U1EP3INV_SPEC>,
    #[doc = "0x300 - U1EP4 register"]
    pub u1ep4: crate::Reg<u1ep4::U1EP4_SPEC>,
    #[doc = "0x304 - U1EP4CLR register"]
    pub u1ep4clr: crate::Reg<u1ep4clr::U1EP4CLR_SPEC>,
    #[doc = "0x308 - U1EP4SET register"]
    pub u1ep4set: crate::Reg<u1ep4set::U1EP4SET_SPEC>,
    #[doc = "0x30c - U1EP4INV register"]
    pub u1ep4inv: crate::Reg<u1ep4inv::U1EP4INV_SPEC>,
    #[doc = "0x310 - U1EP5 register"]
    pub u1ep5: crate::Reg<u1ep5::U1EP5_SPEC>,
    #[doc = "0x314 - U1EP5CLR register"]
    pub u1ep5clr: crate::Reg<u1ep5clr::U1EP5CLR_SPEC>,
    #[doc = "0x318 - U1EP5SET register"]
    pub u1ep5set: crate::Reg<u1ep5set::U1EP5SET_SPEC>,
    #[doc = "0x31c - U1EP5INV register"]
    pub u1ep5inv: crate::Reg<u1ep5inv::U1EP5INV_SPEC>,
    #[doc = "0x320 - U1EP6 register"]
    pub u1ep6: crate::Reg<u1ep6::U1EP6_SPEC>,
    #[doc = "0x324 - U1EP6CLR register"]
    pub u1ep6clr: crate::Reg<u1ep6clr::U1EP6CLR_SPEC>,
    #[doc = "0x328 - U1EP6SET register"]
    pub u1ep6set: crate::Reg<u1ep6set::U1EP6SET_SPEC>,
    #[doc = "0x32c - U1EP6INV register"]
    pub u1ep6inv: crate::Reg<u1ep6inv::U1EP6INV_SPEC>,
    #[doc = "0x330 - U1EP7 register"]
    pub u1ep7: crate::Reg<u1ep7::U1EP7_SPEC>,
    #[doc = "0x334 - U1EP7CLR register"]
    pub u1ep7clr: crate::Reg<u1ep7clr::U1EP7CLR_SPEC>,
    #[doc = "0x338 - U1EP7SET register"]
    pub u1ep7set: crate::Reg<u1ep7set::U1EP7SET_SPEC>,
    #[doc = "0x33c - U1EP7INV register"]
    pub u1ep7inv: crate::Reg<u1ep7inv::U1EP7INV_SPEC>,
    #[doc = "0x340 - U1EP8 register"]
    pub u1ep8: crate::Reg<u1ep8::U1EP8_SPEC>,
    #[doc = "0x344 - U1EP8CLR register"]
    pub u1ep8clr: crate::Reg<u1ep8clr::U1EP8CLR_SPEC>,
    #[doc = "0x348 - U1EP8SET register"]
    pub u1ep8set: crate::Reg<u1ep8set::U1EP8SET_SPEC>,
    #[doc = "0x34c - U1EP8INV register"]
    pub u1ep8inv: crate::Reg<u1ep8inv::U1EP8INV_SPEC>,
    #[doc = "0x350 - U1EP9 register"]
    pub u1ep9: crate::Reg<u1ep9::U1EP9_SPEC>,
    #[doc = "0x354 - U1EP9CLR register"]
    pub u1ep9clr: crate::Reg<u1ep9clr::U1EP9CLR_SPEC>,
    #[doc = "0x358 - U1EP9SET register"]
    pub u1ep9set: crate::Reg<u1ep9set::U1EP9SET_SPEC>,
    #[doc = "0x35c - U1EP9INV register"]
    pub u1ep9inv: crate::Reg<u1ep9inv::U1EP9INV_SPEC>,
    #[doc = "0x360 - U1EP10 register"]
    pub u1ep10: crate::Reg<u1ep10::U1EP10_SPEC>,
    #[doc = "0x364 - U1EP10CLR register"]
    pub u1ep10clr: crate::Reg<u1ep10clr::U1EP10CLR_SPEC>,
    #[doc = "0x368 - U1EP10SET register"]
    pub u1ep10set: crate::Reg<u1ep10set::U1EP10SET_SPEC>,
    #[doc = "0x36c - U1EP10INV register"]
    pub u1ep10inv: crate::Reg<u1ep10inv::U1EP10INV_SPEC>,
    #[doc = "0x370 - U1EP11 register"]
    pub u1ep11: crate::Reg<u1ep11::U1EP11_SPEC>,
    #[doc = "0x374 - U1EP11CLR register"]
    pub u1ep11clr: crate::Reg<u1ep11clr::U1EP11CLR_SPEC>,
    #[doc = "0x378 - U1EP11SET register"]
    pub u1ep11set: crate::Reg<u1ep11set::U1EP11SET_SPEC>,
    #[doc = "0x37c - U1EP11INV register"]
    pub u1ep11inv: crate::Reg<u1ep11inv::U1EP11INV_SPEC>,
    #[doc = "0x380 - U1EP12 register"]
    pub u1ep12: crate::Reg<u1ep12::U1EP12_SPEC>,
    #[doc = "0x384 - U1EP12CLR register"]
    pub u1ep12clr: crate::Reg<u1ep12clr::U1EP12CLR_SPEC>,
    #[doc = "0x388 - U1EP12SET register"]
    pub u1ep12set: crate::Reg<u1ep12set::U1EP12SET_SPEC>,
    #[doc = "0x38c - U1EP12INV register"]
    pub u1ep12inv: crate::Reg<u1ep12inv::U1EP12INV_SPEC>,
    #[doc = "0x390 - U1EP13 register"]
    pub u1ep13: crate::Reg<u1ep13::U1EP13_SPEC>,
    #[doc = "0x394 - U1EP13CLR register"]
    pub u1ep13clr: crate::Reg<u1ep13clr::U1EP13CLR_SPEC>,
    #[doc = "0x398 - U1EP13SET register"]
    pub u1ep13set: crate::Reg<u1ep13set::U1EP13SET_SPEC>,
    #[doc = "0x39c - U1EP13INV register"]
    pub u1ep13inv: crate::Reg<u1ep13inv::U1EP13INV_SPEC>,
    #[doc = "0x3a0 - U1EP14 register"]
    pub u1ep14: crate::Reg<u1ep14::U1EP14_SPEC>,
    #[doc = "0x3a4 - U1EP14CLR register"]
    pub u1ep14clr: crate::Reg<u1ep14clr::U1EP14CLR_SPEC>,
    #[doc = "0x3a8 - U1EP14SET register"]
    pub u1ep14set: crate::Reg<u1ep14set::U1EP14SET_SPEC>,
    #[doc = "0x3ac - U1EP14INV register"]
    pub u1ep14inv: crate::Reg<u1ep14inv::U1EP14INV_SPEC>,
    #[doc = "0x3b0 - U1EP15 register"]
    pub u1ep15: crate::Reg<u1ep15::U1EP15_SPEC>,
    #[doc = "0x3b4 - U1EP15CLR register"]
    pub u1ep15clr: crate::Reg<u1ep15clr::U1EP15CLR_SPEC>,
    #[doc = "0x3b8 - U1EP15SET register"]
    pub u1ep15set: crate::Reg<u1ep15set::U1EP15SET_SPEC>,
    #[doc = "0x3bc - U1EP15INV register"]
    pub u1ep15inv: crate::Reg<u1ep15inv::U1EP15INV_SPEC>,
}
#[doc = "U1OTGIR register accessor: an alias for `Reg<U1OTGIR_SPEC>`"]
pub type U1OTGIR = crate::Reg<u1otgir::U1OTGIR_SPEC>;
#[doc = "U1OTGIR register"]
pub mod u1otgir;
#[doc = "U1OTGIRCLR register accessor: an alias for `Reg<U1OTGIRCLR_SPEC>`"]
pub type U1OTGIRCLR = crate::Reg<u1otgirclr::U1OTGIRCLR_SPEC>;
#[doc = "U1OTGIRCLR register"]
pub mod u1otgirclr;
#[doc = "U1OTGIE register accessor: an alias for `Reg<U1OTGIE_SPEC>`"]
pub type U1OTGIE = crate::Reg<u1otgie::U1OTGIE_SPEC>;
#[doc = "U1OTGIE register"]
pub mod u1otgie;
#[doc = "U1OTGIECLR register accessor: an alias for `Reg<U1OTGIECLR_SPEC>`"]
pub type U1OTGIECLR = crate::Reg<u1otgieclr::U1OTGIECLR_SPEC>;
#[doc = "U1OTGIECLR register"]
pub mod u1otgieclr;
#[doc = "U1OTGIESET register accessor: an alias for `Reg<U1OTGIESET_SPEC>`"]
pub type U1OTGIESET = crate::Reg<u1otgieset::U1OTGIESET_SPEC>;
#[doc = "U1OTGIESET register"]
pub mod u1otgieset;
#[doc = "U1OTGIEINV register accessor: an alias for `Reg<U1OTGIEINV_SPEC>`"]
pub type U1OTGIEINV = crate::Reg<u1otgieinv::U1OTGIEINV_SPEC>;
#[doc = "U1OTGIEINV register"]
pub mod u1otgieinv;
#[doc = "U1OTGSTAT register accessor: an alias for `Reg<U1OTGSTAT_SPEC>`"]
pub type U1OTGSTAT = crate::Reg<u1otgstat::U1OTGSTAT_SPEC>;
#[doc = "U1OTGSTAT register"]
pub mod u1otgstat;
#[doc = "U1OTGCON register accessor: an alias for `Reg<U1OTGCON_SPEC>`"]
pub type U1OTGCON = crate::Reg<u1otgcon::U1OTGCON_SPEC>;
#[doc = "U1OTGCON register"]
pub mod u1otgcon;
#[doc = "U1OTGCONCLR register accessor: an alias for `Reg<U1OTGCONCLR_SPEC>`"]
pub type U1OTGCONCLR = crate::Reg<u1otgconclr::U1OTGCONCLR_SPEC>;
#[doc = "U1OTGCONCLR register"]
pub mod u1otgconclr;
#[doc = "U1OTGCONSET register accessor: an alias for `Reg<U1OTGCONSET_SPEC>`"]
pub type U1OTGCONSET = crate::Reg<u1otgconset::U1OTGCONSET_SPEC>;
#[doc = "U1OTGCONSET register"]
pub mod u1otgconset;
#[doc = "U1OTGCONINV register accessor: an alias for `Reg<U1OTGCONINV_SPEC>`"]
pub type U1OTGCONINV = crate::Reg<u1otgconinv::U1OTGCONINV_SPEC>;
#[doc = "U1OTGCONINV register"]
pub mod u1otgconinv;
#[doc = "U1PWRC register accessor: an alias for `Reg<U1PWRC_SPEC>`"]
pub type U1PWRC = crate::Reg<u1pwrc::U1PWRC_SPEC>;
#[doc = "U1PWRC register"]
pub mod u1pwrc;
#[doc = "U1PWRCCLR register accessor: an alias for `Reg<U1PWRCCLR_SPEC>`"]
pub type U1PWRCCLR = crate::Reg<u1pwrcclr::U1PWRCCLR_SPEC>;
#[doc = "U1PWRCCLR register"]
pub mod u1pwrcclr;
#[doc = "U1PWRCSET register accessor: an alias for `Reg<U1PWRCSET_SPEC>`"]
pub type U1PWRCSET = crate::Reg<u1pwrcset::U1PWRCSET_SPEC>;
#[doc = "U1PWRCSET register"]
pub mod u1pwrcset;
#[doc = "U1PWRCINV register accessor: an alias for `Reg<U1PWRCINV_SPEC>`"]
pub type U1PWRCINV = crate::Reg<u1pwrcinv::U1PWRCINV_SPEC>;
#[doc = "U1PWRCINV register"]
pub mod u1pwrcinv;
#[doc = "U1IR register accessor: an alias for `Reg<U1IR_SPEC>`"]
pub type U1IR = crate::Reg<u1ir::U1IR_SPEC>;
#[doc = "U1IR register"]
pub mod u1ir;
#[doc = "U1IRCLR register accessor: an alias for `Reg<U1IRCLR_SPEC>`"]
pub type U1IRCLR = crate::Reg<u1irclr::U1IRCLR_SPEC>;
#[doc = "U1IRCLR register"]
pub mod u1irclr;
#[doc = "U1IE register accessor: an alias for `Reg<U1IE_SPEC>`"]
pub type U1IE = crate::Reg<u1ie::U1IE_SPEC>;
#[doc = "U1IE register"]
pub mod u1ie;
#[doc = "U1IECLR register accessor: an alias for `Reg<U1IECLR_SPEC>`"]
pub type U1IECLR = crate::Reg<u1ieclr::U1IECLR_SPEC>;
#[doc = "U1IECLR register"]
pub mod u1ieclr;
#[doc = "U1IESET register accessor: an alias for `Reg<U1IESET_SPEC>`"]
pub type U1IESET = crate::Reg<u1ieset::U1IESET_SPEC>;
#[doc = "U1IESET register"]
pub mod u1ieset;
#[doc = "U1IEINV register accessor: an alias for `Reg<U1IEINV_SPEC>`"]
pub type U1IEINV = crate::Reg<u1ieinv::U1IEINV_SPEC>;
#[doc = "U1IEINV register"]
pub mod u1ieinv;
#[doc = "U1EIR register accessor: an alias for `Reg<U1EIR_SPEC>`"]
pub type U1EIR = crate::Reg<u1eir::U1EIR_SPEC>;
#[doc = "U1EIR register"]
pub mod u1eir;
#[doc = "U1EIRCLR register accessor: an alias for `Reg<U1EIRCLR_SPEC>`"]
pub type U1EIRCLR = crate::Reg<u1eirclr::U1EIRCLR_SPEC>;
#[doc = "U1EIRCLR register"]
pub mod u1eirclr;
#[doc = "U1EIE register accessor: an alias for `Reg<U1EIE_SPEC>`"]
pub type U1EIE = crate::Reg<u1eie::U1EIE_SPEC>;
#[doc = "U1EIE register"]
pub mod u1eie;
#[doc = "U1EIECLR register accessor: an alias for `Reg<U1EIECLR_SPEC>`"]
pub type U1EIECLR = crate::Reg<u1eieclr::U1EIECLR_SPEC>;
#[doc = "U1EIECLR register"]
pub mod u1eieclr;
#[doc = "U1EIESET register accessor: an alias for `Reg<U1EIESET_SPEC>`"]
pub type U1EIESET = crate::Reg<u1eieset::U1EIESET_SPEC>;
#[doc = "U1EIESET register"]
pub mod u1eieset;
#[doc = "U1EIEINV register accessor: an alias for `Reg<U1EIEINV_SPEC>`"]
pub type U1EIEINV = crate::Reg<u1eieinv::U1EIEINV_SPEC>;
#[doc = "U1EIEINV register"]
pub mod u1eieinv;
#[doc = "U1STAT register accessor: an alias for `Reg<U1STAT_SPEC>`"]
pub type U1STAT = crate::Reg<u1stat::U1STAT_SPEC>;
#[doc = "U1STAT register"]
pub mod u1stat;
#[doc = "U1CON register accessor: an alias for `Reg<U1CON_SPEC>`"]
pub type U1CON = crate::Reg<u1con::U1CON_SPEC>;
#[doc = "U1CON register"]
pub mod u1con;
#[doc = "U1CONCLR register accessor: an alias for `Reg<U1CONCLR_SPEC>`"]
pub type U1CONCLR = crate::Reg<u1conclr::U1CONCLR_SPEC>;
#[doc = "U1CONCLR register"]
pub mod u1conclr;
#[doc = "U1CONSET register accessor: an alias for `Reg<U1CONSET_SPEC>`"]
pub type U1CONSET = crate::Reg<u1conset::U1CONSET_SPEC>;
#[doc = "U1CONSET register"]
pub mod u1conset;
#[doc = "U1CONINV register accessor: an alias for `Reg<U1CONINV_SPEC>`"]
pub type U1CONINV = crate::Reg<u1coninv::U1CONINV_SPEC>;
#[doc = "U1CONINV register"]
pub mod u1coninv;
#[doc = "U1ADDR register accessor: an alias for `Reg<U1ADDR_SPEC>`"]
pub type U1ADDR = crate::Reg<u1addr::U1ADDR_SPEC>;
#[doc = "U1ADDR register"]
pub mod u1addr;
#[doc = "U1ADDRCLR register accessor: an alias for `Reg<U1ADDRCLR_SPEC>`"]
pub type U1ADDRCLR = crate::Reg<u1addrclr::U1ADDRCLR_SPEC>;
#[doc = "U1ADDRCLR register"]
pub mod u1addrclr;
#[doc = "U1ADDRSET register accessor: an alias for `Reg<U1ADDRSET_SPEC>`"]
pub type U1ADDRSET = crate::Reg<u1addrset::U1ADDRSET_SPEC>;
#[doc = "U1ADDRSET register"]
pub mod u1addrset;
#[doc = "U1ADDRINV register accessor: an alias for `Reg<U1ADDRINV_SPEC>`"]
pub type U1ADDRINV = crate::Reg<u1addrinv::U1ADDRINV_SPEC>;
#[doc = "U1ADDRINV register"]
pub mod u1addrinv;
#[doc = "U1BDTP1 register accessor: an alias for `Reg<U1BDTP1_SPEC>`"]
pub type U1BDTP1 = crate::Reg<u1bdtp1::U1BDTP1_SPEC>;
#[doc = "U1BDTP1 register"]
pub mod u1bdtp1;
#[doc = "U1BDTP1CLR register accessor: an alias for `Reg<U1BDTP1CLR_SPEC>`"]
pub type U1BDTP1CLR = crate::Reg<u1bdtp1clr::U1BDTP1CLR_SPEC>;
#[doc = "U1BDTP1CLR register"]
pub mod u1bdtp1clr;
#[doc = "U1BDTP1SET register accessor: an alias for `Reg<U1BDTP1SET_SPEC>`"]
pub type U1BDTP1SET = crate::Reg<u1bdtp1set::U1BDTP1SET_SPEC>;
#[doc = "U1BDTP1SET register"]
pub mod u1bdtp1set;
#[doc = "U1BDTP1INV register accessor: an alias for `Reg<U1BDTP1INV_SPEC>`"]
pub type U1BDTP1INV = crate::Reg<u1bdtp1inv::U1BDTP1INV_SPEC>;
#[doc = "U1BDTP1INV register"]
pub mod u1bdtp1inv;
#[doc = "U1FRML register accessor: an alias for `Reg<U1FRML_SPEC>`"]
pub type U1FRML = crate::Reg<u1frml::U1FRML_SPEC>;
#[doc = "U1FRML register"]
pub mod u1frml;
#[doc = "U1FRMH register accessor: an alias for `Reg<U1FRMH_SPEC>`"]
pub type U1FRMH = crate::Reg<u1frmh::U1FRMH_SPEC>;
#[doc = "U1FRMH register"]
pub mod u1frmh;
#[doc = "U1TOK register accessor: an alias for `Reg<U1TOK_SPEC>`"]
pub type U1TOK = crate::Reg<u1tok::U1TOK_SPEC>;
#[doc = "U1TOK register"]
pub mod u1tok;
#[doc = "U1TOKCLR register accessor: an alias for `Reg<U1TOKCLR_SPEC>`"]
pub type U1TOKCLR = crate::Reg<u1tokclr::U1TOKCLR_SPEC>;
#[doc = "U1TOKCLR register"]
pub mod u1tokclr;
#[doc = "U1TOKSET register accessor: an alias for `Reg<U1TOKSET_SPEC>`"]
pub type U1TOKSET = crate::Reg<u1tokset::U1TOKSET_SPEC>;
#[doc = "U1TOKSET register"]
pub mod u1tokset;
#[doc = "U1TOKINV register accessor: an alias for `Reg<U1TOKINV_SPEC>`"]
pub type U1TOKINV = crate::Reg<u1tokinv::U1TOKINV_SPEC>;
#[doc = "U1TOKINV register"]
pub mod u1tokinv;
#[doc = "U1SOF register accessor: an alias for `Reg<U1SOF_SPEC>`"]
pub type U1SOF = crate::Reg<u1sof::U1SOF_SPEC>;
#[doc = "U1SOF register"]
pub mod u1sof;
#[doc = "U1SOFCLR register accessor: an alias for `Reg<U1SOFCLR_SPEC>`"]
pub type U1SOFCLR = crate::Reg<u1sofclr::U1SOFCLR_SPEC>;
#[doc = "U1SOFCLR register"]
pub mod u1sofclr;
#[doc = "U1SOFSET register accessor: an alias for `Reg<U1SOFSET_SPEC>`"]
pub type U1SOFSET = crate::Reg<u1sofset::U1SOFSET_SPEC>;
#[doc = "U1SOFSET register"]
pub mod u1sofset;
#[doc = "U1SOFINV register accessor: an alias for `Reg<U1SOFINV_SPEC>`"]
pub type U1SOFINV = crate::Reg<u1sofinv::U1SOFINV_SPEC>;
#[doc = "U1SOFINV register"]
pub mod u1sofinv;
#[doc = "U1BDTP2 register accessor: an alias for `Reg<U1BDTP2_SPEC>`"]
pub type U1BDTP2 = crate::Reg<u1bdtp2::U1BDTP2_SPEC>;
#[doc = "U1BDTP2 register"]
pub mod u1bdtp2;
#[doc = "U1BDTP2CLR register accessor: an alias for `Reg<U1BDTP2CLR_SPEC>`"]
pub type U1BDTP2CLR = crate::Reg<u1bdtp2clr::U1BDTP2CLR_SPEC>;
#[doc = "U1BDTP2CLR register"]
pub mod u1bdtp2clr;
#[doc = "U1BDTP2SET register accessor: an alias for `Reg<U1BDTP2SET_SPEC>`"]
pub type U1BDTP2SET = crate::Reg<u1bdtp2set::U1BDTP2SET_SPEC>;
#[doc = "U1BDTP2SET register"]
pub mod u1bdtp2set;
#[doc = "U1BDTP2INV register accessor: an alias for `Reg<U1BDTP2INV_SPEC>`"]
pub type U1BDTP2INV = crate::Reg<u1bdtp2inv::U1BDTP2INV_SPEC>;
#[doc = "U1BDTP2INV register"]
pub mod u1bdtp2inv;
#[doc = "U1BDTP3 register accessor: an alias for `Reg<U1BDTP3_SPEC>`"]
pub type U1BDTP3 = crate::Reg<u1bdtp3::U1BDTP3_SPEC>;
#[doc = "U1BDTP3 register"]
pub mod u1bdtp3;
#[doc = "U1BDTP3CLR register accessor: an alias for `Reg<U1BDTP3CLR_SPEC>`"]
pub type U1BDTP3CLR = crate::Reg<u1bdtp3clr::U1BDTP3CLR_SPEC>;
#[doc = "U1BDTP3CLR register"]
pub mod u1bdtp3clr;
#[doc = "U1BDTP3SET register accessor: an alias for `Reg<U1BDTP3SET_SPEC>`"]
pub type U1BDTP3SET = crate::Reg<u1bdtp3set::U1BDTP3SET_SPEC>;
#[doc = "U1BDTP3SET register"]
pub mod u1bdtp3set;
#[doc = "U1BDTP3INV register accessor: an alias for `Reg<U1BDTP3INV_SPEC>`"]
pub type U1BDTP3INV = crate::Reg<u1bdtp3inv::U1BDTP3INV_SPEC>;
#[doc = "U1BDTP3INV register"]
pub mod u1bdtp3inv;
#[doc = "U1CNFG1 register accessor: an alias for `Reg<U1CNFG1_SPEC>`"]
pub type U1CNFG1 = crate::Reg<u1cnfg1::U1CNFG1_SPEC>;
#[doc = "U1CNFG1 register"]
pub mod u1cnfg1;
#[doc = "U1CNFG1CLR register accessor: an alias for `Reg<U1CNFG1CLR_SPEC>`"]
pub type U1CNFG1CLR = crate::Reg<u1cnfg1clr::U1CNFG1CLR_SPEC>;
#[doc = "U1CNFG1CLR register"]
pub mod u1cnfg1clr;
#[doc = "U1CNFG1SET register accessor: an alias for `Reg<U1CNFG1SET_SPEC>`"]
pub type U1CNFG1SET = crate::Reg<u1cnfg1set::U1CNFG1SET_SPEC>;
#[doc = "U1CNFG1SET register"]
pub mod u1cnfg1set;
#[doc = "U1CNFG1INV register accessor: an alias for `Reg<U1CNFG1INV_SPEC>`"]
pub type U1CNFG1INV = crate::Reg<u1cnfg1inv::U1CNFG1INV_SPEC>;
#[doc = "U1CNFG1INV register"]
pub mod u1cnfg1inv;
#[doc = "U1EP0 register accessor: an alias for `Reg<U1EP0_SPEC>`"]
pub type U1EP0 = crate::Reg<u1ep0::U1EP0_SPEC>;
#[doc = "U1EP0 register"]
pub mod u1ep0;
#[doc = "U1EP0CLR register accessor: an alias for `Reg<U1EP0CLR_SPEC>`"]
pub type U1EP0CLR = crate::Reg<u1ep0clr::U1EP0CLR_SPEC>;
#[doc = "U1EP0CLR register"]
pub mod u1ep0clr;
#[doc = "U1EP0SET register accessor: an alias for `Reg<U1EP0SET_SPEC>`"]
pub type U1EP0SET = crate::Reg<u1ep0set::U1EP0SET_SPEC>;
#[doc = "U1EP0SET register"]
pub mod u1ep0set;
#[doc = "U1EP0INV register accessor: an alias for `Reg<U1EP0INV_SPEC>`"]
pub type U1EP0INV = crate::Reg<u1ep0inv::U1EP0INV_SPEC>;
#[doc = "U1EP0INV register"]
pub mod u1ep0inv;
#[doc = "U1EP1 register accessor: an alias for `Reg<U1EP1_SPEC>`"]
pub type U1EP1 = crate::Reg<u1ep1::U1EP1_SPEC>;
#[doc = "U1EP1 register"]
pub mod u1ep1;
#[doc = "U1EP1CLR register accessor: an alias for `Reg<U1EP1CLR_SPEC>`"]
pub type U1EP1CLR = crate::Reg<u1ep1clr::U1EP1CLR_SPEC>;
#[doc = "U1EP1CLR register"]
pub mod u1ep1clr;
#[doc = "U1EP1SET register accessor: an alias for `Reg<U1EP1SET_SPEC>`"]
pub type U1EP1SET = crate::Reg<u1ep1set::U1EP1SET_SPEC>;
#[doc = "U1EP1SET register"]
pub mod u1ep1set;
#[doc = "U1EP1INV register accessor: an alias for `Reg<U1EP1INV_SPEC>`"]
pub type U1EP1INV = crate::Reg<u1ep1inv::U1EP1INV_SPEC>;
#[doc = "U1EP1INV register"]
pub mod u1ep1inv;
#[doc = "U1EP2 register accessor: an alias for `Reg<U1EP2_SPEC>`"]
pub type U1EP2 = crate::Reg<u1ep2::U1EP2_SPEC>;
#[doc = "U1EP2 register"]
pub mod u1ep2;
#[doc = "U1EP2CLR register accessor: an alias for `Reg<U1EP2CLR_SPEC>`"]
pub type U1EP2CLR = crate::Reg<u1ep2clr::U1EP2CLR_SPEC>;
#[doc = "U1EP2CLR register"]
pub mod u1ep2clr;
#[doc = "U1EP2SET register accessor: an alias for `Reg<U1EP2SET_SPEC>`"]
pub type U1EP2SET = crate::Reg<u1ep2set::U1EP2SET_SPEC>;
#[doc = "U1EP2SET register"]
pub mod u1ep2set;
#[doc = "U1EP2INV register accessor: an alias for `Reg<U1EP2INV_SPEC>`"]
pub type U1EP2INV = crate::Reg<u1ep2inv::U1EP2INV_SPEC>;
#[doc = "U1EP2INV register"]
pub mod u1ep2inv;
#[doc = "U1EP3 register accessor: an alias for `Reg<U1EP3_SPEC>`"]
pub type U1EP3 = crate::Reg<u1ep3::U1EP3_SPEC>;
#[doc = "U1EP3 register"]
pub mod u1ep3;
#[doc = "U1EP3CLR register accessor: an alias for `Reg<U1EP3CLR_SPEC>`"]
pub type U1EP3CLR = crate::Reg<u1ep3clr::U1EP3CLR_SPEC>;
#[doc = "U1EP3CLR register"]
pub mod u1ep3clr;
#[doc = "U1EP3SET register accessor: an alias for `Reg<U1EP3SET_SPEC>`"]
pub type U1EP3SET = crate::Reg<u1ep3set::U1EP3SET_SPEC>;
#[doc = "U1EP3SET register"]
pub mod u1ep3set;
#[doc = "U1EP3INV register accessor: an alias for `Reg<U1EP3INV_SPEC>`"]
pub type U1EP3INV = crate::Reg<u1ep3inv::U1EP3INV_SPEC>;
#[doc = "U1EP3INV register"]
pub mod u1ep3inv;
#[doc = "U1EP4 register accessor: an alias for `Reg<U1EP4_SPEC>`"]
pub type U1EP4 = crate::Reg<u1ep4::U1EP4_SPEC>;
#[doc = "U1EP4 register"]
pub mod u1ep4;
#[doc = "U1EP4CLR register accessor: an alias for `Reg<U1EP4CLR_SPEC>`"]
pub type U1EP4CLR = crate::Reg<u1ep4clr::U1EP4CLR_SPEC>;
#[doc = "U1EP4CLR register"]
pub mod u1ep4clr;
#[doc = "U1EP4SET register accessor: an alias for `Reg<U1EP4SET_SPEC>`"]
pub type U1EP4SET = crate::Reg<u1ep4set::U1EP4SET_SPEC>;
#[doc = "U1EP4SET register"]
pub mod u1ep4set;
#[doc = "U1EP4INV register accessor: an alias for `Reg<U1EP4INV_SPEC>`"]
pub type U1EP4INV = crate::Reg<u1ep4inv::U1EP4INV_SPEC>;
#[doc = "U1EP4INV register"]
pub mod u1ep4inv;
#[doc = "U1EP5 register accessor: an alias for `Reg<U1EP5_SPEC>`"]
pub type U1EP5 = crate::Reg<u1ep5::U1EP5_SPEC>;
#[doc = "U1EP5 register"]
pub mod u1ep5;
#[doc = "U1EP5CLR register accessor: an alias for `Reg<U1EP5CLR_SPEC>`"]
pub type U1EP5CLR = crate::Reg<u1ep5clr::U1EP5CLR_SPEC>;
#[doc = "U1EP5CLR register"]
pub mod u1ep5clr;
#[doc = "U1EP5SET register accessor: an alias for `Reg<U1EP5SET_SPEC>`"]
pub type U1EP5SET = crate::Reg<u1ep5set::U1EP5SET_SPEC>;
#[doc = "U1EP5SET register"]
pub mod u1ep5set;
#[doc = "U1EP5INV register accessor: an alias for `Reg<U1EP5INV_SPEC>`"]
pub type U1EP5INV = crate::Reg<u1ep5inv::U1EP5INV_SPEC>;
#[doc = "U1EP5INV register"]
pub mod u1ep5inv;
#[doc = "U1EP6 register accessor: an alias for `Reg<U1EP6_SPEC>`"]
pub type U1EP6 = crate::Reg<u1ep6::U1EP6_SPEC>;
#[doc = "U1EP6 register"]
pub mod u1ep6;
#[doc = "U1EP6CLR register accessor: an alias for `Reg<U1EP6CLR_SPEC>`"]
pub type U1EP6CLR = crate::Reg<u1ep6clr::U1EP6CLR_SPEC>;
#[doc = "U1EP6CLR register"]
pub mod u1ep6clr;
#[doc = "U1EP6SET register accessor: an alias for `Reg<U1EP6SET_SPEC>`"]
pub type U1EP6SET = crate::Reg<u1ep6set::U1EP6SET_SPEC>;
#[doc = "U1EP6SET register"]
pub mod u1ep6set;
#[doc = "U1EP6INV register accessor: an alias for `Reg<U1EP6INV_SPEC>`"]
pub type U1EP6INV = crate::Reg<u1ep6inv::U1EP6INV_SPEC>;
#[doc = "U1EP6INV register"]
pub mod u1ep6inv;
#[doc = "U1EP7 register accessor: an alias for `Reg<U1EP7_SPEC>`"]
pub type U1EP7 = crate::Reg<u1ep7::U1EP7_SPEC>;
#[doc = "U1EP7 register"]
pub mod u1ep7;
#[doc = "U1EP7CLR register accessor: an alias for `Reg<U1EP7CLR_SPEC>`"]
pub type U1EP7CLR = crate::Reg<u1ep7clr::U1EP7CLR_SPEC>;
#[doc = "U1EP7CLR register"]
pub mod u1ep7clr;
#[doc = "U1EP7SET register accessor: an alias for `Reg<U1EP7SET_SPEC>`"]
pub type U1EP7SET = crate::Reg<u1ep7set::U1EP7SET_SPEC>;
#[doc = "U1EP7SET register"]
pub mod u1ep7set;
#[doc = "U1EP7INV register accessor: an alias for `Reg<U1EP7INV_SPEC>`"]
pub type U1EP7INV = crate::Reg<u1ep7inv::U1EP7INV_SPEC>;
#[doc = "U1EP7INV register"]
pub mod u1ep7inv;
#[doc = "U1EP8 register accessor: an alias for `Reg<U1EP8_SPEC>`"]
pub type U1EP8 = crate::Reg<u1ep8::U1EP8_SPEC>;
#[doc = "U1EP8 register"]
pub mod u1ep8;
#[doc = "U1EP8CLR register accessor: an alias for `Reg<U1EP8CLR_SPEC>`"]
pub type U1EP8CLR = crate::Reg<u1ep8clr::U1EP8CLR_SPEC>;
#[doc = "U1EP8CLR register"]
pub mod u1ep8clr;
#[doc = "U1EP8SET register accessor: an alias for `Reg<U1EP8SET_SPEC>`"]
pub type U1EP8SET = crate::Reg<u1ep8set::U1EP8SET_SPEC>;
#[doc = "U1EP8SET register"]
pub mod u1ep8set;
#[doc = "U1EP8INV register accessor: an alias for `Reg<U1EP8INV_SPEC>`"]
pub type U1EP8INV = crate::Reg<u1ep8inv::U1EP8INV_SPEC>;
#[doc = "U1EP8INV register"]
pub mod u1ep8inv;
#[doc = "U1EP9 register accessor: an alias for `Reg<U1EP9_SPEC>`"]
pub type U1EP9 = crate::Reg<u1ep9::U1EP9_SPEC>;
#[doc = "U1EP9 register"]
pub mod u1ep9;
#[doc = "U1EP9CLR register accessor: an alias for `Reg<U1EP9CLR_SPEC>`"]
pub type U1EP9CLR = crate::Reg<u1ep9clr::U1EP9CLR_SPEC>;
#[doc = "U1EP9CLR register"]
pub mod u1ep9clr;
#[doc = "U1EP9SET register accessor: an alias for `Reg<U1EP9SET_SPEC>`"]
pub type U1EP9SET = crate::Reg<u1ep9set::U1EP9SET_SPEC>;
#[doc = "U1EP9SET register"]
pub mod u1ep9set;
#[doc = "U1EP9INV register accessor: an alias for `Reg<U1EP9INV_SPEC>`"]
pub type U1EP9INV = crate::Reg<u1ep9inv::U1EP9INV_SPEC>;
#[doc = "U1EP9INV register"]
pub mod u1ep9inv;
#[doc = "U1EP10 register accessor: an alias for `Reg<U1EP10_SPEC>`"]
pub type U1EP10 = crate::Reg<u1ep10::U1EP10_SPEC>;
#[doc = "U1EP10 register"]
pub mod u1ep10;
#[doc = "U1EP10CLR register accessor: an alias for `Reg<U1EP10CLR_SPEC>`"]
pub type U1EP10CLR = crate::Reg<u1ep10clr::U1EP10CLR_SPEC>;
#[doc = "U1EP10CLR register"]
pub mod u1ep10clr;
#[doc = "U1EP10SET register accessor: an alias for `Reg<U1EP10SET_SPEC>`"]
pub type U1EP10SET = crate::Reg<u1ep10set::U1EP10SET_SPEC>;
#[doc = "U1EP10SET register"]
pub mod u1ep10set;
#[doc = "U1EP10INV register accessor: an alias for `Reg<U1EP10INV_SPEC>`"]
pub type U1EP10INV = crate::Reg<u1ep10inv::U1EP10INV_SPEC>;
#[doc = "U1EP10INV register"]
pub mod u1ep10inv;
#[doc = "U1EP11 register accessor: an alias for `Reg<U1EP11_SPEC>`"]
pub type U1EP11 = crate::Reg<u1ep11::U1EP11_SPEC>;
#[doc = "U1EP11 register"]
pub mod u1ep11;
#[doc = "U1EP11CLR register accessor: an alias for `Reg<U1EP11CLR_SPEC>`"]
pub type U1EP11CLR = crate::Reg<u1ep11clr::U1EP11CLR_SPEC>;
#[doc = "U1EP11CLR register"]
pub mod u1ep11clr;
#[doc = "U1EP11SET register accessor: an alias for `Reg<U1EP11SET_SPEC>`"]
pub type U1EP11SET = crate::Reg<u1ep11set::U1EP11SET_SPEC>;
#[doc = "U1EP11SET register"]
pub mod u1ep11set;
#[doc = "U1EP11INV register accessor: an alias for `Reg<U1EP11INV_SPEC>`"]
pub type U1EP11INV = crate::Reg<u1ep11inv::U1EP11INV_SPEC>;
#[doc = "U1EP11INV register"]
pub mod u1ep11inv;
#[doc = "U1EP12 register accessor: an alias for `Reg<U1EP12_SPEC>`"]
pub type U1EP12 = crate::Reg<u1ep12::U1EP12_SPEC>;
#[doc = "U1EP12 register"]
pub mod u1ep12;
#[doc = "U1EP12CLR register accessor: an alias for `Reg<U1EP12CLR_SPEC>`"]
pub type U1EP12CLR = crate::Reg<u1ep12clr::U1EP12CLR_SPEC>;
#[doc = "U1EP12CLR register"]
pub mod u1ep12clr;
#[doc = "U1EP12SET register accessor: an alias for `Reg<U1EP12SET_SPEC>`"]
pub type U1EP12SET = crate::Reg<u1ep12set::U1EP12SET_SPEC>;
#[doc = "U1EP12SET register"]
pub mod u1ep12set;
#[doc = "U1EP12INV register accessor: an alias for `Reg<U1EP12INV_SPEC>`"]
pub type U1EP12INV = crate::Reg<u1ep12inv::U1EP12INV_SPEC>;
#[doc = "U1EP12INV register"]
pub mod u1ep12inv;
#[doc = "U1EP13 register accessor: an alias for `Reg<U1EP13_SPEC>`"]
pub type U1EP13 = crate::Reg<u1ep13::U1EP13_SPEC>;
#[doc = "U1EP13 register"]
pub mod u1ep13;
#[doc = "U1EP13CLR register accessor: an alias for `Reg<U1EP13CLR_SPEC>`"]
pub type U1EP13CLR = crate::Reg<u1ep13clr::U1EP13CLR_SPEC>;
#[doc = "U1EP13CLR register"]
pub mod u1ep13clr;
#[doc = "U1EP13SET register accessor: an alias for `Reg<U1EP13SET_SPEC>`"]
pub type U1EP13SET = crate::Reg<u1ep13set::U1EP13SET_SPEC>;
#[doc = "U1EP13SET register"]
pub mod u1ep13set;
#[doc = "U1EP13INV register accessor: an alias for `Reg<U1EP13INV_SPEC>`"]
pub type U1EP13INV = crate::Reg<u1ep13inv::U1EP13INV_SPEC>;
#[doc = "U1EP13INV register"]
pub mod u1ep13inv;
#[doc = "U1EP14 register accessor: an alias for `Reg<U1EP14_SPEC>`"]
pub type U1EP14 = crate::Reg<u1ep14::U1EP14_SPEC>;
#[doc = "U1EP14 register"]
pub mod u1ep14;
#[doc = "U1EP14CLR register accessor: an alias for `Reg<U1EP14CLR_SPEC>`"]
pub type U1EP14CLR = crate::Reg<u1ep14clr::U1EP14CLR_SPEC>;
#[doc = "U1EP14CLR register"]
pub mod u1ep14clr;
#[doc = "U1EP14SET register accessor: an alias for `Reg<U1EP14SET_SPEC>`"]
pub type U1EP14SET = crate::Reg<u1ep14set::U1EP14SET_SPEC>;
#[doc = "U1EP14SET register"]
pub mod u1ep14set;
#[doc = "U1EP14INV register accessor: an alias for `Reg<U1EP14INV_SPEC>`"]
pub type U1EP14INV = crate::Reg<u1ep14inv::U1EP14INV_SPEC>;
#[doc = "U1EP14INV register"]
pub mod u1ep14inv;
#[doc = "U1EP15 register accessor: an alias for `Reg<U1EP15_SPEC>`"]
pub type U1EP15 = crate::Reg<u1ep15::U1EP15_SPEC>;
#[doc = "U1EP15 register"]
pub mod u1ep15;
#[doc = "U1EP15CLR register accessor: an alias for `Reg<U1EP15CLR_SPEC>`"]
pub type U1EP15CLR = crate::Reg<u1ep15clr::U1EP15CLR_SPEC>;
#[doc = "U1EP15CLR register"]
pub mod u1ep15clr;
#[doc = "U1EP15SET register accessor: an alias for `Reg<U1EP15SET_SPEC>`"]
pub type U1EP15SET = crate::Reg<u1ep15set::U1EP15SET_SPEC>;
#[doc = "U1EP15SET register"]
pub mod u1ep15set;
#[doc = "U1EP15INV register accessor: an alias for `Reg<U1EP15INV_SPEC>`"]
pub type U1EP15INV = crate::Reg<u1ep15inv::U1EP15INV_SPEC>;
#[doc = "U1EP15INV register"]
pub mod u1ep15inv;
