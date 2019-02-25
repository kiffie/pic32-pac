#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AD1CON1 register"]
    pub ad1con1: AD1CON1,
    #[doc = "0x04 - AD1CON1CLR register"]
    pub ad1con1clr: AD1CON1CLR,
    #[doc = "0x08 - AD1CON1SET register"]
    pub ad1con1set: AD1CON1SET,
    #[doc = "0x0c - AD1CON1INV register"]
    pub ad1con1inv: AD1CON1INV,
    #[doc = "0x10 - AD1CON2 register"]
    pub ad1con2: AD1CON2,
    #[doc = "0x14 - AD1CON2CLR register"]
    pub ad1con2clr: AD1CON2CLR,
    #[doc = "0x18 - AD1CON2SET register"]
    pub ad1con2set: AD1CON2SET,
    #[doc = "0x1c - AD1CON2INV register"]
    pub ad1con2inv: AD1CON2INV,
    #[doc = "0x20 - AD1CON3 register"]
    pub ad1con3: AD1CON3,
    #[doc = "0x24 - AD1CON3CLR register"]
    pub ad1con3clr: AD1CON3CLR,
    #[doc = "0x28 - AD1CON3SET register"]
    pub ad1con3set: AD1CON3SET,
    #[doc = "0x2c - AD1CON3INV register"]
    pub ad1con3inv: AD1CON3INV,
    _reserved0: [u8; 16usize],
    #[doc = "0x40 - AD1CHS register"]
    pub ad1chs: AD1CHS,
    #[doc = "0x44 - AD1CHSCLR register"]
    pub ad1chsclr: AD1CHSCLR,
    #[doc = "0x48 - AD1CHSSET register"]
    pub ad1chsset: AD1CHSSET,
    #[doc = "0x4c - AD1CHSINV register"]
    pub ad1chsinv: AD1CHSINV,
    #[doc = "0x50 - AD1CSSL register"]
    pub ad1cssl: AD1CSSL,
    #[doc = "0x54 - AD1CSSLCLR register"]
    pub ad1csslclr: AD1CSSLCLR,
    #[doc = "0x58 - AD1CSSLSET register"]
    pub ad1csslset: AD1CSSLSET,
    #[doc = "0x5c - AD1CSSLINV register"]
    pub ad1csslinv: AD1CSSLINV,
    _reserved1: [u8; 16usize],
    #[doc = "0x70 - ADC1BUF0 register"]
    pub adc1buf0: ADC1BUF0,
    _reserved2: [u8; 12usize],
    #[doc = "0x80 - ADC1BUF1 register"]
    pub adc1buf1: ADC1BUF1,
    _reserved3: [u8; 12usize],
    #[doc = "0x90 - ADC1BUF2 register"]
    pub adc1buf2: ADC1BUF2,
    _reserved4: [u8; 12usize],
    #[doc = "0xa0 - ADC1BUF3 register"]
    pub adc1buf3: ADC1BUF3,
    _reserved5: [u8; 12usize],
    #[doc = "0xb0 - ADC1BUF4 register"]
    pub adc1buf4: ADC1BUF4,
    _reserved6: [u8; 12usize],
    #[doc = "0xc0 - ADC1BUF5 register"]
    pub adc1buf5: ADC1BUF5,
    _reserved7: [u8; 12usize],
    #[doc = "0xd0 - ADC1BUF6 register"]
    pub adc1buf6: ADC1BUF6,
    _reserved8: [u8; 12usize],
    #[doc = "0xe0 - ADC1BUF7 register"]
    pub adc1buf7: ADC1BUF7,
    _reserved9: [u8; 12usize],
    #[doc = "0xf0 - ADC1BUF8 register"]
    pub adc1buf8: ADC1BUF8,
    _reserved10: [u8; 12usize],
    #[doc = "0x100 - ADC1BUF9 register"]
    pub adc1buf9: ADC1BUF9,
    _reserved11: [u8; 12usize],
    #[doc = "0x110 - ADC1BUFA register"]
    pub adc1bufa: ADC1BUFA,
    _reserved12: [u8; 12usize],
    #[doc = "0x120 - ADC1BUFB register"]
    pub adc1bufb: ADC1BUFB,
    _reserved13: [u8; 12usize],
    #[doc = "0x130 - ADC1BUFC register"]
    pub adc1bufc: ADC1BUFC,
    _reserved14: [u8; 12usize],
    #[doc = "0x140 - ADC1BUFD register"]
    pub adc1bufd: ADC1BUFD,
    _reserved15: [u8; 12usize],
    #[doc = "0x150 - ADC1BUFE register"]
    pub adc1bufe: ADC1BUFE,
    _reserved16: [u8; 12usize],
    #[doc = "0x160 - ADC1BUFF register"]
    pub adc1buff: ADC1BUFF,
}
#[doc = "AD1CON1 register"]
pub struct AD1CON1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON1 register"]
pub mod ad1con1;
#[doc = "AD1CON1CLR register"]
pub struct AD1CON1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON1CLR register"]
pub mod ad1con1clr;
#[doc = "AD1CON1SET register"]
pub struct AD1CON1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON1SET register"]
pub mod ad1con1set;
#[doc = "AD1CON1INV register"]
pub struct AD1CON1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON1INV register"]
pub mod ad1con1inv;
#[doc = "AD1CON2 register"]
pub struct AD1CON2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON2 register"]
pub mod ad1con2;
#[doc = "AD1CON2CLR register"]
pub struct AD1CON2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON2CLR register"]
pub mod ad1con2clr;
#[doc = "AD1CON2SET register"]
pub struct AD1CON2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON2SET register"]
pub mod ad1con2set;
#[doc = "AD1CON2INV register"]
pub struct AD1CON2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON2INV register"]
pub mod ad1con2inv;
#[doc = "AD1CON3 register"]
pub struct AD1CON3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON3 register"]
pub mod ad1con3;
#[doc = "AD1CON3CLR register"]
pub struct AD1CON3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON3CLR register"]
pub mod ad1con3clr;
#[doc = "AD1CON3SET register"]
pub struct AD1CON3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON3SET register"]
pub mod ad1con3set;
#[doc = "AD1CON3INV register"]
pub struct AD1CON3INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CON3INV register"]
pub mod ad1con3inv;
#[doc = "AD1CHS register"]
pub struct AD1CHS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CHS register"]
pub mod ad1chs;
#[doc = "AD1CHSCLR register"]
pub struct AD1CHSCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CHSCLR register"]
pub mod ad1chsclr;
#[doc = "AD1CHSSET register"]
pub struct AD1CHSSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CHSSET register"]
pub mod ad1chsset;
#[doc = "AD1CHSINV register"]
pub struct AD1CHSINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CHSINV register"]
pub mod ad1chsinv;
#[doc = "AD1CSSL register"]
pub struct AD1CSSL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CSSL register"]
pub mod ad1cssl;
#[doc = "AD1CSSLCLR register"]
pub struct AD1CSSLCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CSSLCLR register"]
pub mod ad1csslclr;
#[doc = "AD1CSSLSET register"]
pub struct AD1CSSLSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CSSLSET register"]
pub mod ad1csslset;
#[doc = "AD1CSSLINV register"]
pub struct AD1CSSLINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "AD1CSSLINV register"]
pub mod ad1csslinv;
#[doc = "ADC1BUF0 register"]
pub struct ADC1BUF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF0 register"]
pub mod adc1buf0;
#[doc = "ADC1BUF1 register"]
pub struct ADC1BUF1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF1 register"]
pub mod adc1buf1;
#[doc = "ADC1BUF2 register"]
pub struct ADC1BUF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF2 register"]
pub mod adc1buf2;
#[doc = "ADC1BUF3 register"]
pub struct ADC1BUF3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF3 register"]
pub mod adc1buf3;
#[doc = "ADC1BUF4 register"]
pub struct ADC1BUF4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF4 register"]
pub mod adc1buf4;
#[doc = "ADC1BUF5 register"]
pub struct ADC1BUF5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF5 register"]
pub mod adc1buf5;
#[doc = "ADC1BUF6 register"]
pub struct ADC1BUF6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF6 register"]
pub mod adc1buf6;
#[doc = "ADC1BUF7 register"]
pub struct ADC1BUF7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF7 register"]
pub mod adc1buf7;
#[doc = "ADC1BUF8 register"]
pub struct ADC1BUF8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF8 register"]
pub mod adc1buf8;
#[doc = "ADC1BUF9 register"]
pub struct ADC1BUF9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUF9 register"]
pub mod adc1buf9;
#[doc = "ADC1BUFA register"]
pub struct ADC1BUFA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUFA register"]
pub mod adc1bufa;
#[doc = "ADC1BUFB register"]
pub struct ADC1BUFB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUFB register"]
pub mod adc1bufb;
#[doc = "ADC1BUFC register"]
pub struct ADC1BUFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUFC register"]
pub mod adc1bufc;
#[doc = "ADC1BUFD register"]
pub struct ADC1BUFD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUFD register"]
pub mod adc1bufd;
#[doc = "ADC1BUFE register"]
pub struct ADC1BUFE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUFE register"]
pub mod adc1bufe;
#[doc = "ADC1BUFF register"]
pub struct ADC1BUFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ADC1BUFF register"]
pub mod adc1buff;
