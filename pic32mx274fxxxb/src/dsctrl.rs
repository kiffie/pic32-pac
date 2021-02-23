#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSCON register"]
    pub dscon: crate::Reg<dscon::DSCON_SPEC>,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - DSWAKE register"]
    pub dswake: crate::Reg<dswake::DSWAKE_SPEC>,
    _reserved2: [u8; 12usize],
    #[doc = "0x20 - DSGPR0 register"]
    pub dsgpr0: crate::Reg<dsgpr0::DSGPR0_SPEC>,
    _reserved3: [u8; 28usize],
    #[doc = "0x40 - DSGPR1 register"]
    pub dsgpr1: crate::Reg<dsgpr1::DSGPR1_SPEC>,
    #[doc = "0x44 - DSGPR2 register"]
    pub dsgpr2: crate::Reg<dsgpr2::DSGPR2_SPEC>,
    #[doc = "0x48 - DSGPR3 register"]
    pub dsgpr3: crate::Reg<dsgpr3::DSGPR3_SPEC>,
    #[doc = "0x4c - DSGPR4 register"]
    pub dsgpr4: crate::Reg<dsgpr4::DSGPR4_SPEC>,
    #[doc = "0x50 - DSGPR5 register"]
    pub dsgpr5: crate::Reg<dsgpr5::DSGPR5_SPEC>,
    #[doc = "0x54 - DSGPR6 register"]
    pub dsgpr6: crate::Reg<dsgpr6::DSGPR6_SPEC>,
    #[doc = "0x58 - DSGPR7 register"]
    pub dsgpr7: crate::Reg<dsgpr7::DSGPR7_SPEC>,
    #[doc = "0x5c - DSGPR8 register"]
    pub dsgpr8: crate::Reg<dsgpr8::DSGPR8_SPEC>,
    #[doc = "0x60 - DSGPR9 register"]
    pub dsgpr9: crate::Reg<dsgpr9::DSGPR9_SPEC>,
    #[doc = "0x64 - DSGPR10 register"]
    pub dsgpr10: crate::Reg<dsgpr10::DSGPR10_SPEC>,
    #[doc = "0x68 - DSGPR11 register"]
    pub dsgpr11: crate::Reg<dsgpr11::DSGPR11_SPEC>,
    #[doc = "0x6c - DSGPR12 register"]
    pub dsgpr12: crate::Reg<dsgpr12::DSGPR12_SPEC>,
    #[doc = "0x70 - DSGPR13 register"]
    pub dsgpr13: crate::Reg<dsgpr13::DSGPR13_SPEC>,
    #[doc = "0x74 - DSGPR14 register"]
    pub dsgpr14: crate::Reg<dsgpr14::DSGPR14_SPEC>,
    #[doc = "0x78 - DSGPR15 register"]
    pub dsgpr15: crate::Reg<dsgpr15::DSGPR15_SPEC>,
    #[doc = "0x7c - DSGPR16 register"]
    pub dsgpr16: crate::Reg<dsgpr16::DSGPR16_SPEC>,
    #[doc = "0x80 - DSGPR17 register"]
    pub dsgpr17: crate::Reg<dsgpr17::DSGPR17_SPEC>,
    #[doc = "0x84 - DSGPR18 register"]
    pub dsgpr18: crate::Reg<dsgpr18::DSGPR18_SPEC>,
    #[doc = "0x88 - DSGPR19 register"]
    pub dsgpr19: crate::Reg<dsgpr19::DSGPR19_SPEC>,
    #[doc = "0x8c - DSGPR20 register"]
    pub dsgpr20: crate::Reg<dsgpr20::DSGPR20_SPEC>,
    #[doc = "0x90 - DSGPR21 register"]
    pub dsgpr21: crate::Reg<dsgpr21::DSGPR21_SPEC>,
    #[doc = "0x94 - DSGPR22 register"]
    pub dsgpr22: crate::Reg<dsgpr22::DSGPR22_SPEC>,
    #[doc = "0x98 - DSGPR23 register"]
    pub dsgpr23: crate::Reg<dsgpr23::DSGPR23_SPEC>,
    #[doc = "0x9c - DSGPR24 register"]
    pub dsgpr24: crate::Reg<dsgpr24::DSGPR24_SPEC>,
    #[doc = "0xa0 - DSGPR25 register"]
    pub dsgpr25: crate::Reg<dsgpr25::DSGPR25_SPEC>,
    #[doc = "0xa4 - DSGPR26 register"]
    pub dsgpr26: crate::Reg<dsgpr26::DSGPR26_SPEC>,
    #[doc = "0xa8 - DSGPR27 register"]
    pub dsgpr27: crate::Reg<dsgpr27::DSGPR27_SPEC>,
    #[doc = "0xac - DSGPR28 register"]
    pub dsgpr28: crate::Reg<dsgpr28::DSGPR28_SPEC>,
    #[doc = "0xb0 - DSGPR29 register"]
    pub dsgpr29: crate::Reg<dsgpr29::DSGPR29_SPEC>,
    #[doc = "0xb4 - DSGPR30 register"]
    pub dsgpr30: crate::Reg<dsgpr30::DSGPR30_SPEC>,
    #[doc = "0xb8 - DSGPR31 register"]
    pub dsgpr31: crate::Reg<dsgpr31::DSGPR31_SPEC>,
    #[doc = "0xbc - DSGPR32 register"]
    pub dsgpr32: crate::Reg<dsgpr32::DSGPR32_SPEC>,
}
#[doc = "DSCON register accessor: an alias for `Reg<DSCON_SPEC>`"]
pub type DSCON = crate::Reg<dscon::DSCON_SPEC>;
#[doc = "DSCON register"]
pub mod dscon;
#[doc = "DSWAKE register accessor: an alias for `Reg<DSWAKE_SPEC>`"]
pub type DSWAKE = crate::Reg<dswake::DSWAKE_SPEC>;
#[doc = "DSWAKE register"]
pub mod dswake;
#[doc = "DSGPR0 register accessor: an alias for `Reg<DSGPR0_SPEC>`"]
pub type DSGPR0 = crate::Reg<dsgpr0::DSGPR0_SPEC>;
#[doc = "DSGPR0 register"]
pub mod dsgpr0;
#[doc = "DSGPR1 register accessor: an alias for `Reg<DSGPR1_SPEC>`"]
pub type DSGPR1 = crate::Reg<dsgpr1::DSGPR1_SPEC>;
#[doc = "DSGPR1 register"]
pub mod dsgpr1;
#[doc = "DSGPR2 register accessor: an alias for `Reg<DSGPR2_SPEC>`"]
pub type DSGPR2 = crate::Reg<dsgpr2::DSGPR2_SPEC>;
#[doc = "DSGPR2 register"]
pub mod dsgpr2;
#[doc = "DSGPR3 register accessor: an alias for `Reg<DSGPR3_SPEC>`"]
pub type DSGPR3 = crate::Reg<dsgpr3::DSGPR3_SPEC>;
#[doc = "DSGPR3 register"]
pub mod dsgpr3;
#[doc = "DSGPR4 register accessor: an alias for `Reg<DSGPR4_SPEC>`"]
pub type DSGPR4 = crate::Reg<dsgpr4::DSGPR4_SPEC>;
#[doc = "DSGPR4 register"]
pub mod dsgpr4;
#[doc = "DSGPR5 register accessor: an alias for `Reg<DSGPR5_SPEC>`"]
pub type DSGPR5 = crate::Reg<dsgpr5::DSGPR5_SPEC>;
#[doc = "DSGPR5 register"]
pub mod dsgpr5;
#[doc = "DSGPR6 register accessor: an alias for `Reg<DSGPR6_SPEC>`"]
pub type DSGPR6 = crate::Reg<dsgpr6::DSGPR6_SPEC>;
#[doc = "DSGPR6 register"]
pub mod dsgpr6;
#[doc = "DSGPR7 register accessor: an alias for `Reg<DSGPR7_SPEC>`"]
pub type DSGPR7 = crate::Reg<dsgpr7::DSGPR7_SPEC>;
#[doc = "DSGPR7 register"]
pub mod dsgpr7;
#[doc = "DSGPR8 register accessor: an alias for `Reg<DSGPR8_SPEC>`"]
pub type DSGPR8 = crate::Reg<dsgpr8::DSGPR8_SPEC>;
#[doc = "DSGPR8 register"]
pub mod dsgpr8;
#[doc = "DSGPR9 register accessor: an alias for `Reg<DSGPR9_SPEC>`"]
pub type DSGPR9 = crate::Reg<dsgpr9::DSGPR9_SPEC>;
#[doc = "DSGPR9 register"]
pub mod dsgpr9;
#[doc = "DSGPR10 register accessor: an alias for `Reg<DSGPR10_SPEC>`"]
pub type DSGPR10 = crate::Reg<dsgpr10::DSGPR10_SPEC>;
#[doc = "DSGPR10 register"]
pub mod dsgpr10;
#[doc = "DSGPR11 register accessor: an alias for `Reg<DSGPR11_SPEC>`"]
pub type DSGPR11 = crate::Reg<dsgpr11::DSGPR11_SPEC>;
#[doc = "DSGPR11 register"]
pub mod dsgpr11;
#[doc = "DSGPR12 register accessor: an alias for `Reg<DSGPR12_SPEC>`"]
pub type DSGPR12 = crate::Reg<dsgpr12::DSGPR12_SPEC>;
#[doc = "DSGPR12 register"]
pub mod dsgpr12;
#[doc = "DSGPR13 register accessor: an alias for `Reg<DSGPR13_SPEC>`"]
pub type DSGPR13 = crate::Reg<dsgpr13::DSGPR13_SPEC>;
#[doc = "DSGPR13 register"]
pub mod dsgpr13;
#[doc = "DSGPR14 register accessor: an alias for `Reg<DSGPR14_SPEC>`"]
pub type DSGPR14 = crate::Reg<dsgpr14::DSGPR14_SPEC>;
#[doc = "DSGPR14 register"]
pub mod dsgpr14;
#[doc = "DSGPR15 register accessor: an alias for `Reg<DSGPR15_SPEC>`"]
pub type DSGPR15 = crate::Reg<dsgpr15::DSGPR15_SPEC>;
#[doc = "DSGPR15 register"]
pub mod dsgpr15;
#[doc = "DSGPR16 register accessor: an alias for `Reg<DSGPR16_SPEC>`"]
pub type DSGPR16 = crate::Reg<dsgpr16::DSGPR16_SPEC>;
#[doc = "DSGPR16 register"]
pub mod dsgpr16;
#[doc = "DSGPR17 register accessor: an alias for `Reg<DSGPR17_SPEC>`"]
pub type DSGPR17 = crate::Reg<dsgpr17::DSGPR17_SPEC>;
#[doc = "DSGPR17 register"]
pub mod dsgpr17;
#[doc = "DSGPR18 register accessor: an alias for `Reg<DSGPR18_SPEC>`"]
pub type DSGPR18 = crate::Reg<dsgpr18::DSGPR18_SPEC>;
#[doc = "DSGPR18 register"]
pub mod dsgpr18;
#[doc = "DSGPR19 register accessor: an alias for `Reg<DSGPR19_SPEC>`"]
pub type DSGPR19 = crate::Reg<dsgpr19::DSGPR19_SPEC>;
#[doc = "DSGPR19 register"]
pub mod dsgpr19;
#[doc = "DSGPR20 register accessor: an alias for `Reg<DSGPR20_SPEC>`"]
pub type DSGPR20 = crate::Reg<dsgpr20::DSGPR20_SPEC>;
#[doc = "DSGPR20 register"]
pub mod dsgpr20;
#[doc = "DSGPR21 register accessor: an alias for `Reg<DSGPR21_SPEC>`"]
pub type DSGPR21 = crate::Reg<dsgpr21::DSGPR21_SPEC>;
#[doc = "DSGPR21 register"]
pub mod dsgpr21;
#[doc = "DSGPR22 register accessor: an alias for `Reg<DSGPR22_SPEC>`"]
pub type DSGPR22 = crate::Reg<dsgpr22::DSGPR22_SPEC>;
#[doc = "DSGPR22 register"]
pub mod dsgpr22;
#[doc = "DSGPR23 register accessor: an alias for `Reg<DSGPR23_SPEC>`"]
pub type DSGPR23 = crate::Reg<dsgpr23::DSGPR23_SPEC>;
#[doc = "DSGPR23 register"]
pub mod dsgpr23;
#[doc = "DSGPR24 register accessor: an alias for `Reg<DSGPR24_SPEC>`"]
pub type DSGPR24 = crate::Reg<dsgpr24::DSGPR24_SPEC>;
#[doc = "DSGPR24 register"]
pub mod dsgpr24;
#[doc = "DSGPR25 register accessor: an alias for `Reg<DSGPR25_SPEC>`"]
pub type DSGPR25 = crate::Reg<dsgpr25::DSGPR25_SPEC>;
#[doc = "DSGPR25 register"]
pub mod dsgpr25;
#[doc = "DSGPR26 register accessor: an alias for `Reg<DSGPR26_SPEC>`"]
pub type DSGPR26 = crate::Reg<dsgpr26::DSGPR26_SPEC>;
#[doc = "DSGPR26 register"]
pub mod dsgpr26;
#[doc = "DSGPR27 register accessor: an alias for `Reg<DSGPR27_SPEC>`"]
pub type DSGPR27 = crate::Reg<dsgpr27::DSGPR27_SPEC>;
#[doc = "DSGPR27 register"]
pub mod dsgpr27;
#[doc = "DSGPR28 register accessor: an alias for `Reg<DSGPR28_SPEC>`"]
pub type DSGPR28 = crate::Reg<dsgpr28::DSGPR28_SPEC>;
#[doc = "DSGPR28 register"]
pub mod dsgpr28;
#[doc = "DSGPR29 register accessor: an alias for `Reg<DSGPR29_SPEC>`"]
pub type DSGPR29 = crate::Reg<dsgpr29::DSGPR29_SPEC>;
#[doc = "DSGPR29 register"]
pub mod dsgpr29;
#[doc = "DSGPR30 register accessor: an alias for `Reg<DSGPR30_SPEC>`"]
pub type DSGPR30 = crate::Reg<dsgpr30::DSGPR30_SPEC>;
#[doc = "DSGPR30 register"]
pub mod dsgpr30;
#[doc = "DSGPR31 register accessor: an alias for `Reg<DSGPR31_SPEC>`"]
pub type DSGPR31 = crate::Reg<dsgpr31::DSGPR31_SPEC>;
#[doc = "DSGPR31 register"]
pub mod dsgpr31;
#[doc = "DSGPR32 register accessor: an alias for `Reg<DSGPR32_SPEC>`"]
pub type DSGPR32 = crate::Reg<dsgpr32::DSGPR32_SPEC>;
#[doc = "DSGPR32 register"]
pub mod dsgpr32;
