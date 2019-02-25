#[doc = r" Register block"]
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
    _reserved0: [u8; 12usize],
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
    #[doc = "0x50 - IFS2 register"]
    pub ifs2: IFS2,
    #[doc = "0x54 - IFS2CLR register"]
    pub ifs2clr: IFS2CLR,
    #[doc = "0x58 - IFS2SET register"]
    pub ifs2set: IFS2SET,
    #[doc = "0x5c - IFS2INV register"]
    pub ifs2inv: IFS2INV,
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
    #[doc = "0x80 - IEC2 register"]
    pub iec2: IEC2,
    #[doc = "0x84 - IEC2CLR register"]
    pub iec2clr: IEC2CLR,
    #[doc = "0x88 - IEC2SET register"]
    pub iec2set: IEC2SET,
    #[doc = "0x8c - IEC2INV register"]
    pub iec2inv: IEC2INV,
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
    #[doc = "0x140 - IPC11 register"]
    pub ipc11: IPC11,
    #[doc = "0x144 - IPC11CLR register"]
    pub ipc11clr: IPC11CLR,
    #[doc = "0x148 - IPC11SET register"]
    pub ipc11set: IPC11SET,
    #[doc = "0x14c - IPC11INV register"]
    pub ipc11inv: IPC11INV,
}
#[doc = "INTCON register"]
pub struct INTCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTCON register"]
pub mod intcon;
#[doc = "INTCONCLR register"]
pub struct INTCONCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTCONCLR register"]
pub mod intconclr;
#[doc = "INTCONSET register"]
pub struct INTCONSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTCONSET register"]
pub mod intconset;
#[doc = "INTCONINV register"]
pub struct INTCONINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTCONINV register"]
pub mod intconinv;
#[doc = "INTSTAT register"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "INTSTAT register"]
pub mod intstat;
#[doc = "IPTMR register"]
pub struct IPTMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPTMR register"]
pub mod iptmr;
#[doc = "IPTMRCLR register"]
pub struct IPTMRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPTMRCLR register"]
pub mod iptmrclr;
#[doc = "IPTMRSET register"]
pub struct IPTMRSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPTMRSET register"]
pub mod iptmrset;
#[doc = "IPTMRINV register"]
pub struct IPTMRINV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPTMRINV register"]
pub mod iptmrinv;
#[doc = "IFS0 register"]
pub struct IFS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS0 register"]
pub mod ifs0;
#[doc = "IFS0CLR register"]
pub struct IFS0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS0CLR register"]
pub mod ifs0clr;
#[doc = "IFS0SET register"]
pub struct IFS0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS0SET register"]
pub mod ifs0set;
#[doc = "IFS0INV register"]
pub struct IFS0INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS0INV register"]
pub mod ifs0inv;
#[doc = "IFS1 register"]
pub struct IFS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS1 register"]
pub mod ifs1;
#[doc = "IFS1CLR register"]
pub struct IFS1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS1CLR register"]
pub mod ifs1clr;
#[doc = "IFS1SET register"]
pub struct IFS1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS1SET register"]
pub mod ifs1set;
#[doc = "IFS1INV register"]
pub struct IFS1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS1INV register"]
pub mod ifs1inv;
#[doc = "IFS2 register"]
pub struct IFS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS2 register"]
pub mod ifs2;
#[doc = "IFS2CLR register"]
pub struct IFS2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS2CLR register"]
pub mod ifs2clr;
#[doc = "IFS2SET register"]
pub struct IFS2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS2SET register"]
pub mod ifs2set;
#[doc = "IFS2INV register"]
pub struct IFS2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IFS2INV register"]
pub mod ifs2inv;
#[doc = "IEC0 register"]
pub struct IEC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC0 register"]
pub mod iec0;
#[doc = "IEC0CLR register"]
pub struct IEC0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC0CLR register"]
pub mod iec0clr;
#[doc = "IEC0SET register"]
pub struct IEC0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC0SET register"]
pub mod iec0set;
#[doc = "IEC0INV register"]
pub struct IEC0INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC0INV register"]
pub mod iec0inv;
#[doc = "IEC1 register"]
pub struct IEC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC1 register"]
pub mod iec1;
#[doc = "IEC1CLR register"]
pub struct IEC1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC1CLR register"]
pub mod iec1clr;
#[doc = "IEC1SET register"]
pub struct IEC1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC1SET register"]
pub mod iec1set;
#[doc = "IEC1INV register"]
pub struct IEC1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC1INV register"]
pub mod iec1inv;
#[doc = "IEC2 register"]
pub struct IEC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC2 register"]
pub mod iec2;
#[doc = "IEC2CLR register"]
pub struct IEC2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC2CLR register"]
pub mod iec2clr;
#[doc = "IEC2SET register"]
pub struct IEC2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC2SET register"]
pub mod iec2set;
#[doc = "IEC2INV register"]
pub struct IEC2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IEC2INV register"]
pub mod iec2inv;
#[doc = "IPC0 register"]
pub struct IPC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC0 register"]
pub mod ipc0;
#[doc = "IPC0CLR register"]
pub struct IPC0CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC0CLR register"]
pub mod ipc0clr;
#[doc = "IPC0SET register"]
pub struct IPC0SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC0SET register"]
pub mod ipc0set;
#[doc = "IPC0INV register"]
pub struct IPC0INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC0INV register"]
pub mod ipc0inv;
#[doc = "IPC1 register"]
pub struct IPC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC1 register"]
pub mod ipc1;
#[doc = "IPC1CLR register"]
pub struct IPC1CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC1CLR register"]
pub mod ipc1clr;
#[doc = "IPC1SET register"]
pub struct IPC1SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC1SET register"]
pub mod ipc1set;
#[doc = "IPC1INV register"]
pub struct IPC1INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC1INV register"]
pub mod ipc1inv;
#[doc = "IPC2 register"]
pub struct IPC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC2 register"]
pub mod ipc2;
#[doc = "IPC2CLR register"]
pub struct IPC2CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC2CLR register"]
pub mod ipc2clr;
#[doc = "IPC2SET register"]
pub struct IPC2SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC2SET register"]
pub mod ipc2set;
#[doc = "IPC2INV register"]
pub struct IPC2INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC2INV register"]
pub mod ipc2inv;
#[doc = "IPC3 register"]
pub struct IPC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC3 register"]
pub mod ipc3;
#[doc = "IPC3CLR register"]
pub struct IPC3CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC3CLR register"]
pub mod ipc3clr;
#[doc = "IPC3SET register"]
pub struct IPC3SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC3SET register"]
pub mod ipc3set;
#[doc = "IPC3INV register"]
pub struct IPC3INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC3INV register"]
pub mod ipc3inv;
#[doc = "IPC4 register"]
pub struct IPC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC4 register"]
pub mod ipc4;
#[doc = "IPC4CLR register"]
pub struct IPC4CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC4CLR register"]
pub mod ipc4clr;
#[doc = "IPC4SET register"]
pub struct IPC4SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC4SET register"]
pub mod ipc4set;
#[doc = "IPC4INV register"]
pub struct IPC4INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC4INV register"]
pub mod ipc4inv;
#[doc = "IPC5 register"]
pub struct IPC5 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC5 register"]
pub mod ipc5;
#[doc = "IPC5CLR register"]
pub struct IPC5CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC5CLR register"]
pub mod ipc5clr;
#[doc = "IPC5SET register"]
pub struct IPC5SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC5SET register"]
pub mod ipc5set;
#[doc = "IPC5INV register"]
pub struct IPC5INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC5INV register"]
pub mod ipc5inv;
#[doc = "IPC6 register"]
pub struct IPC6 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC6 register"]
pub mod ipc6;
#[doc = "IPC6CLR register"]
pub struct IPC6CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC6CLR register"]
pub mod ipc6clr;
#[doc = "IPC6SET register"]
pub struct IPC6SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC6SET register"]
pub mod ipc6set;
#[doc = "IPC6INV register"]
pub struct IPC6INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC6INV register"]
pub mod ipc6inv;
#[doc = "IPC7 register"]
pub struct IPC7 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC7 register"]
pub mod ipc7;
#[doc = "IPC7CLR register"]
pub struct IPC7CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC7CLR register"]
pub mod ipc7clr;
#[doc = "IPC7SET register"]
pub struct IPC7SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC7SET register"]
pub mod ipc7set;
#[doc = "IPC7INV register"]
pub struct IPC7INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC7INV register"]
pub mod ipc7inv;
#[doc = "IPC8 register"]
pub struct IPC8 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC8 register"]
pub mod ipc8;
#[doc = "IPC8CLR register"]
pub struct IPC8CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC8CLR register"]
pub mod ipc8clr;
#[doc = "IPC8SET register"]
pub struct IPC8SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC8SET register"]
pub mod ipc8set;
#[doc = "IPC8INV register"]
pub struct IPC8INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC8INV register"]
pub mod ipc8inv;
#[doc = "IPC9 register"]
pub struct IPC9 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC9 register"]
pub mod ipc9;
#[doc = "IPC9CLR register"]
pub struct IPC9CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC9CLR register"]
pub mod ipc9clr;
#[doc = "IPC9SET register"]
pub struct IPC9SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC9SET register"]
pub mod ipc9set;
#[doc = "IPC9INV register"]
pub struct IPC9INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC9INV register"]
pub mod ipc9inv;
#[doc = "IPC10 register"]
pub struct IPC10 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC10 register"]
pub mod ipc10;
#[doc = "IPC10CLR register"]
pub struct IPC10CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC10CLR register"]
pub mod ipc10clr;
#[doc = "IPC10SET register"]
pub struct IPC10SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC10SET register"]
pub mod ipc10set;
#[doc = "IPC10INV register"]
pub struct IPC10INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC10INV register"]
pub mod ipc10inv;
#[doc = "IPC11 register"]
pub struct IPC11 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC11 register"]
pub mod ipc11;
#[doc = "IPC11CLR register"]
pub struct IPC11CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC11CLR register"]
pub mod ipc11clr;
#[doc = "IPC11SET register"]
pub struct IPC11SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC11SET register"]
pub mod ipc11set;
#[doc = "IPC11INV register"]
pub struct IPC11INV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPC11INV register"]
pub mod ipc11inv;
