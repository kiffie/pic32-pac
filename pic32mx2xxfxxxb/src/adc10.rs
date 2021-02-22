#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AD1CON1 register"]
    pub ad1con1: crate::Reg<ad1con1::AD1CON1_SPEC>,
    #[doc = "0x04 - AD1CON1CLR register"]
    pub ad1con1clr: crate::Reg<ad1con1clr::AD1CON1CLR_SPEC>,
    #[doc = "0x08 - AD1CON1SET register"]
    pub ad1con1set: crate::Reg<ad1con1set::AD1CON1SET_SPEC>,
    #[doc = "0x0c - AD1CON1INV register"]
    pub ad1con1inv: crate::Reg<ad1con1inv::AD1CON1INV_SPEC>,
    #[doc = "0x10 - AD1CON2 register"]
    pub ad1con2: crate::Reg<ad1con2::AD1CON2_SPEC>,
    #[doc = "0x14 - AD1CON2CLR register"]
    pub ad1con2clr: crate::Reg<ad1con2clr::AD1CON2CLR_SPEC>,
    #[doc = "0x18 - AD1CON2SET register"]
    pub ad1con2set: crate::Reg<ad1con2set::AD1CON2SET_SPEC>,
    #[doc = "0x1c - AD1CON2INV register"]
    pub ad1con2inv: crate::Reg<ad1con2inv::AD1CON2INV_SPEC>,
    #[doc = "0x20 - AD1CON3 register"]
    pub ad1con3: crate::Reg<ad1con3::AD1CON3_SPEC>,
    #[doc = "0x24 - AD1CON3CLR register"]
    pub ad1con3clr: crate::Reg<ad1con3clr::AD1CON3CLR_SPEC>,
    #[doc = "0x28 - AD1CON3SET register"]
    pub ad1con3set: crate::Reg<ad1con3set::AD1CON3SET_SPEC>,
    #[doc = "0x2c - AD1CON3INV register"]
    pub ad1con3inv: crate::Reg<ad1con3inv::AD1CON3INV_SPEC>,
    _reserved12: [u8; 16usize],
    #[doc = "0x40 - AD1CHS register"]
    pub ad1chs: crate::Reg<ad1chs::AD1CHS_SPEC>,
    #[doc = "0x44 - AD1CHSCLR register"]
    pub ad1chsclr: crate::Reg<ad1chsclr::AD1CHSCLR_SPEC>,
    #[doc = "0x48 - AD1CHSSET register"]
    pub ad1chsset: crate::Reg<ad1chsset::AD1CHSSET_SPEC>,
    #[doc = "0x4c - AD1CHSINV register"]
    pub ad1chsinv: crate::Reg<ad1chsinv::AD1CHSINV_SPEC>,
    #[doc = "0x50 - AD1CSSL register"]
    pub ad1cssl: crate::Reg<ad1cssl::AD1CSSL_SPEC>,
    #[doc = "0x54 - AD1CSSLCLR register"]
    pub ad1csslclr: crate::Reg<ad1csslclr::AD1CSSLCLR_SPEC>,
    #[doc = "0x58 - AD1CSSLSET register"]
    pub ad1csslset: crate::Reg<ad1csslset::AD1CSSLSET_SPEC>,
    #[doc = "0x5c - AD1CSSLINV register"]
    pub ad1csslinv: crate::Reg<ad1csslinv::AD1CSSLINV_SPEC>,
    _reserved20: [u8; 16usize],
    #[doc = "0x70 - ADC1BUF0 register"]
    pub adc1buf0: crate::Reg<adc1buf0::ADC1BUF0_SPEC>,
    _reserved21: [u8; 12usize],
    #[doc = "0x80 - ADC1BUF1 register"]
    pub adc1buf1: crate::Reg<adc1buf1::ADC1BUF1_SPEC>,
    _reserved22: [u8; 12usize],
    #[doc = "0x90 - ADC1BUF2 register"]
    pub adc1buf2: crate::Reg<adc1buf2::ADC1BUF2_SPEC>,
    _reserved23: [u8; 12usize],
    #[doc = "0xa0 - ADC1BUF3 register"]
    pub adc1buf3: crate::Reg<adc1buf3::ADC1BUF3_SPEC>,
    _reserved24: [u8; 12usize],
    #[doc = "0xb0 - ADC1BUF4 register"]
    pub adc1buf4: crate::Reg<adc1buf4::ADC1BUF4_SPEC>,
    _reserved25: [u8; 12usize],
    #[doc = "0xc0 - ADC1BUF5 register"]
    pub adc1buf5: crate::Reg<adc1buf5::ADC1BUF5_SPEC>,
    _reserved26: [u8; 12usize],
    #[doc = "0xd0 - ADC1BUF6 register"]
    pub adc1buf6: crate::Reg<adc1buf6::ADC1BUF6_SPEC>,
    _reserved27: [u8; 12usize],
    #[doc = "0xe0 - ADC1BUF7 register"]
    pub adc1buf7: crate::Reg<adc1buf7::ADC1BUF7_SPEC>,
    _reserved28: [u8; 12usize],
    #[doc = "0xf0 - ADC1BUF8 register"]
    pub adc1buf8: crate::Reg<adc1buf8::ADC1BUF8_SPEC>,
    _reserved29: [u8; 12usize],
    #[doc = "0x100 - ADC1BUF9 register"]
    pub adc1buf9: crate::Reg<adc1buf9::ADC1BUF9_SPEC>,
    _reserved30: [u8; 12usize],
    #[doc = "0x110 - ADC1BUFA register"]
    pub adc1bufa: crate::Reg<adc1bufa::ADC1BUFA_SPEC>,
    _reserved31: [u8; 12usize],
    #[doc = "0x120 - ADC1BUFB register"]
    pub adc1bufb: crate::Reg<adc1bufb::ADC1BUFB_SPEC>,
    _reserved32: [u8; 12usize],
    #[doc = "0x130 - ADC1BUFC register"]
    pub adc1bufc: crate::Reg<adc1bufc::ADC1BUFC_SPEC>,
    _reserved33: [u8; 12usize],
    #[doc = "0x140 - ADC1BUFD register"]
    pub adc1bufd: crate::Reg<adc1bufd::ADC1BUFD_SPEC>,
    _reserved34: [u8; 12usize],
    #[doc = "0x150 - ADC1BUFE register"]
    pub adc1bufe: crate::Reg<adc1bufe::ADC1BUFE_SPEC>,
    _reserved35: [u8; 12usize],
    #[doc = "0x160 - ADC1BUFF register"]
    pub adc1buff: crate::Reg<adc1buff::ADC1BUFF_SPEC>,
}
#[doc = "AD1CON1 register accessor: an alias for `Reg<AD1CON1_SPEC>`"]
pub type AD1CON1 = crate::Reg<ad1con1::AD1CON1_SPEC>;
#[doc = "AD1CON1 register"]
pub mod ad1con1;
#[doc = "AD1CON1CLR register accessor: an alias for `Reg<AD1CON1CLR_SPEC>`"]
pub type AD1CON1CLR = crate::Reg<ad1con1clr::AD1CON1CLR_SPEC>;
#[doc = "AD1CON1CLR register"]
pub mod ad1con1clr;
#[doc = "AD1CON1SET register accessor: an alias for `Reg<AD1CON1SET_SPEC>`"]
pub type AD1CON1SET = crate::Reg<ad1con1set::AD1CON1SET_SPEC>;
#[doc = "AD1CON1SET register"]
pub mod ad1con1set;
#[doc = "AD1CON1INV register accessor: an alias for `Reg<AD1CON1INV_SPEC>`"]
pub type AD1CON1INV = crate::Reg<ad1con1inv::AD1CON1INV_SPEC>;
#[doc = "AD1CON1INV register"]
pub mod ad1con1inv;
#[doc = "AD1CON2 register accessor: an alias for `Reg<AD1CON2_SPEC>`"]
pub type AD1CON2 = crate::Reg<ad1con2::AD1CON2_SPEC>;
#[doc = "AD1CON2 register"]
pub mod ad1con2;
#[doc = "AD1CON2CLR register accessor: an alias for `Reg<AD1CON2CLR_SPEC>`"]
pub type AD1CON2CLR = crate::Reg<ad1con2clr::AD1CON2CLR_SPEC>;
#[doc = "AD1CON2CLR register"]
pub mod ad1con2clr;
#[doc = "AD1CON2SET register accessor: an alias for `Reg<AD1CON2SET_SPEC>`"]
pub type AD1CON2SET = crate::Reg<ad1con2set::AD1CON2SET_SPEC>;
#[doc = "AD1CON2SET register"]
pub mod ad1con2set;
#[doc = "AD1CON2INV register accessor: an alias for `Reg<AD1CON2INV_SPEC>`"]
pub type AD1CON2INV = crate::Reg<ad1con2inv::AD1CON2INV_SPEC>;
#[doc = "AD1CON2INV register"]
pub mod ad1con2inv;
#[doc = "AD1CON3 register accessor: an alias for `Reg<AD1CON3_SPEC>`"]
pub type AD1CON3 = crate::Reg<ad1con3::AD1CON3_SPEC>;
#[doc = "AD1CON3 register"]
pub mod ad1con3;
#[doc = "AD1CON3CLR register accessor: an alias for `Reg<AD1CON3CLR_SPEC>`"]
pub type AD1CON3CLR = crate::Reg<ad1con3clr::AD1CON3CLR_SPEC>;
#[doc = "AD1CON3CLR register"]
pub mod ad1con3clr;
#[doc = "AD1CON3SET register accessor: an alias for `Reg<AD1CON3SET_SPEC>`"]
pub type AD1CON3SET = crate::Reg<ad1con3set::AD1CON3SET_SPEC>;
#[doc = "AD1CON3SET register"]
pub mod ad1con3set;
#[doc = "AD1CON3INV register accessor: an alias for `Reg<AD1CON3INV_SPEC>`"]
pub type AD1CON3INV = crate::Reg<ad1con3inv::AD1CON3INV_SPEC>;
#[doc = "AD1CON3INV register"]
pub mod ad1con3inv;
#[doc = "AD1CHS register accessor: an alias for `Reg<AD1CHS_SPEC>`"]
pub type AD1CHS = crate::Reg<ad1chs::AD1CHS_SPEC>;
#[doc = "AD1CHS register"]
pub mod ad1chs;
#[doc = "AD1CHSCLR register accessor: an alias for `Reg<AD1CHSCLR_SPEC>`"]
pub type AD1CHSCLR = crate::Reg<ad1chsclr::AD1CHSCLR_SPEC>;
#[doc = "AD1CHSCLR register"]
pub mod ad1chsclr;
#[doc = "AD1CHSSET register accessor: an alias for `Reg<AD1CHSSET_SPEC>`"]
pub type AD1CHSSET = crate::Reg<ad1chsset::AD1CHSSET_SPEC>;
#[doc = "AD1CHSSET register"]
pub mod ad1chsset;
#[doc = "AD1CHSINV register accessor: an alias for `Reg<AD1CHSINV_SPEC>`"]
pub type AD1CHSINV = crate::Reg<ad1chsinv::AD1CHSINV_SPEC>;
#[doc = "AD1CHSINV register"]
pub mod ad1chsinv;
#[doc = "AD1CSSL register accessor: an alias for `Reg<AD1CSSL_SPEC>`"]
pub type AD1CSSL = crate::Reg<ad1cssl::AD1CSSL_SPEC>;
#[doc = "AD1CSSL register"]
pub mod ad1cssl;
#[doc = "AD1CSSLCLR register accessor: an alias for `Reg<AD1CSSLCLR_SPEC>`"]
pub type AD1CSSLCLR = crate::Reg<ad1csslclr::AD1CSSLCLR_SPEC>;
#[doc = "AD1CSSLCLR register"]
pub mod ad1csslclr;
#[doc = "AD1CSSLSET register accessor: an alias for `Reg<AD1CSSLSET_SPEC>`"]
pub type AD1CSSLSET = crate::Reg<ad1csslset::AD1CSSLSET_SPEC>;
#[doc = "AD1CSSLSET register"]
pub mod ad1csslset;
#[doc = "AD1CSSLINV register accessor: an alias for `Reg<AD1CSSLINV_SPEC>`"]
pub type AD1CSSLINV = crate::Reg<ad1csslinv::AD1CSSLINV_SPEC>;
#[doc = "AD1CSSLINV register"]
pub mod ad1csslinv;
#[doc = "ADC1BUF0 register accessor: an alias for `Reg<ADC1BUF0_SPEC>`"]
pub type ADC1BUF0 = crate::Reg<adc1buf0::ADC1BUF0_SPEC>;
#[doc = "ADC1BUF0 register"]
pub mod adc1buf0;
#[doc = "ADC1BUF1 register accessor: an alias for `Reg<ADC1BUF1_SPEC>`"]
pub type ADC1BUF1 = crate::Reg<adc1buf1::ADC1BUF1_SPEC>;
#[doc = "ADC1BUF1 register"]
pub mod adc1buf1;
#[doc = "ADC1BUF2 register accessor: an alias for `Reg<ADC1BUF2_SPEC>`"]
pub type ADC1BUF2 = crate::Reg<adc1buf2::ADC1BUF2_SPEC>;
#[doc = "ADC1BUF2 register"]
pub mod adc1buf2;
#[doc = "ADC1BUF3 register accessor: an alias for `Reg<ADC1BUF3_SPEC>`"]
pub type ADC1BUF3 = crate::Reg<adc1buf3::ADC1BUF3_SPEC>;
#[doc = "ADC1BUF3 register"]
pub mod adc1buf3;
#[doc = "ADC1BUF4 register accessor: an alias for `Reg<ADC1BUF4_SPEC>`"]
pub type ADC1BUF4 = crate::Reg<adc1buf4::ADC1BUF4_SPEC>;
#[doc = "ADC1BUF4 register"]
pub mod adc1buf4;
#[doc = "ADC1BUF5 register accessor: an alias for `Reg<ADC1BUF5_SPEC>`"]
pub type ADC1BUF5 = crate::Reg<adc1buf5::ADC1BUF5_SPEC>;
#[doc = "ADC1BUF5 register"]
pub mod adc1buf5;
#[doc = "ADC1BUF6 register accessor: an alias for `Reg<ADC1BUF6_SPEC>`"]
pub type ADC1BUF6 = crate::Reg<adc1buf6::ADC1BUF6_SPEC>;
#[doc = "ADC1BUF6 register"]
pub mod adc1buf6;
#[doc = "ADC1BUF7 register accessor: an alias for `Reg<ADC1BUF7_SPEC>`"]
pub type ADC1BUF7 = crate::Reg<adc1buf7::ADC1BUF7_SPEC>;
#[doc = "ADC1BUF7 register"]
pub mod adc1buf7;
#[doc = "ADC1BUF8 register accessor: an alias for `Reg<ADC1BUF8_SPEC>`"]
pub type ADC1BUF8 = crate::Reg<adc1buf8::ADC1BUF8_SPEC>;
#[doc = "ADC1BUF8 register"]
pub mod adc1buf8;
#[doc = "ADC1BUF9 register accessor: an alias for `Reg<ADC1BUF9_SPEC>`"]
pub type ADC1BUF9 = crate::Reg<adc1buf9::ADC1BUF9_SPEC>;
#[doc = "ADC1BUF9 register"]
pub mod adc1buf9;
#[doc = "ADC1BUFA register accessor: an alias for `Reg<ADC1BUFA_SPEC>`"]
pub type ADC1BUFA = crate::Reg<adc1bufa::ADC1BUFA_SPEC>;
#[doc = "ADC1BUFA register"]
pub mod adc1bufa;
#[doc = "ADC1BUFB register accessor: an alias for `Reg<ADC1BUFB_SPEC>`"]
pub type ADC1BUFB = crate::Reg<adc1bufb::ADC1BUFB_SPEC>;
#[doc = "ADC1BUFB register"]
pub mod adc1bufb;
#[doc = "ADC1BUFC register accessor: an alias for `Reg<ADC1BUFC_SPEC>`"]
pub type ADC1BUFC = crate::Reg<adc1bufc::ADC1BUFC_SPEC>;
#[doc = "ADC1BUFC register"]
pub mod adc1bufc;
#[doc = "ADC1BUFD register accessor: an alias for `Reg<ADC1BUFD_SPEC>`"]
pub type ADC1BUFD = crate::Reg<adc1bufd::ADC1BUFD_SPEC>;
#[doc = "ADC1BUFD register"]
pub mod adc1bufd;
#[doc = "ADC1BUFE register accessor: an alias for `Reg<ADC1BUFE_SPEC>`"]
pub type ADC1BUFE = crate::Reg<adc1bufe::ADC1BUFE_SPEC>;
#[doc = "ADC1BUFE register"]
pub mod adc1bufe;
#[doc = "ADC1BUFF register accessor: an alias for `Reg<ADC1BUFF_SPEC>`"]
pub type ADC1BUFF = crate::Reg<adc1buff::ADC1BUFF_SPEC>;
#[doc = "ADC1BUFF register"]
pub mod adc1buff;
