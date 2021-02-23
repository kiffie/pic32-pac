#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - INTCON register"]
    pub intcon: crate::Reg<intcon::INTCON_SPEC>,
    #[doc = "0x04 - INTCONCLR register"]
    pub intconclr: crate::Reg<intconclr::INTCONCLR_SPEC>,
    #[doc = "0x08 - INTCONSET register"]
    pub intconset: crate::Reg<intconset::INTCONSET_SPEC>,
    #[doc = "0x0c - INTCONINV register"]
    pub intconinv: crate::Reg<intconinv::INTCONINV_SPEC>,
    #[doc = "0x10 - INTSTAT register"]
    pub intstat: crate::Reg<intstat::INTSTAT_SPEC>,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - IPTMR register"]
    pub iptmr: crate::Reg<iptmr::IPTMR_SPEC>,
    #[doc = "0x24 - IPTMRCLR register"]
    pub iptmrclr: crate::Reg<iptmrclr::IPTMRCLR_SPEC>,
    #[doc = "0x28 - IPTMRSET register"]
    pub iptmrset: crate::Reg<iptmrset::IPTMRSET_SPEC>,
    #[doc = "0x2c - IPTMRINV register"]
    pub iptmrinv: crate::Reg<iptmrinv::IPTMRINV_SPEC>,
    #[doc = "0x30 - IFS0 register"]
    pub ifs0: crate::Reg<ifs0::IFS0_SPEC>,
    #[doc = "0x34 - IFS0CLR register"]
    pub ifs0clr: crate::Reg<ifs0clr::IFS0CLR_SPEC>,
    #[doc = "0x38 - IFS0SET register"]
    pub ifs0set: crate::Reg<ifs0set::IFS0SET_SPEC>,
    #[doc = "0x3c - IFS0INV register"]
    pub ifs0inv: crate::Reg<ifs0inv::IFS0INV_SPEC>,
    #[doc = "0x40 - IFS1 register"]
    pub ifs1: crate::Reg<ifs1::IFS1_SPEC>,
    #[doc = "0x44 - IFS1CLR register"]
    pub ifs1clr: crate::Reg<ifs1clr::IFS1CLR_SPEC>,
    #[doc = "0x48 - IFS1SET register"]
    pub ifs1set: crate::Reg<ifs1set::IFS1SET_SPEC>,
    #[doc = "0x4c - IFS1INV register"]
    pub ifs1inv: crate::Reg<ifs1inv::IFS1INV_SPEC>,
    _reserved17: [u8; 16usize],
    #[doc = "0x60 - IEC0 register"]
    pub iec0: crate::Reg<iec0::IEC0_SPEC>,
    #[doc = "0x64 - IEC0CLR register"]
    pub iec0clr: crate::Reg<iec0clr::IEC0CLR_SPEC>,
    #[doc = "0x68 - IEC0SET register"]
    pub iec0set: crate::Reg<iec0set::IEC0SET_SPEC>,
    #[doc = "0x6c - IEC0INV register"]
    pub iec0inv: crate::Reg<iec0inv::IEC0INV_SPEC>,
    #[doc = "0x70 - IEC1 register"]
    pub iec1: crate::Reg<iec1::IEC1_SPEC>,
    #[doc = "0x74 - IEC1CLR register"]
    pub iec1clr: crate::Reg<iec1clr::IEC1CLR_SPEC>,
    #[doc = "0x78 - IEC1SET register"]
    pub iec1set: crate::Reg<iec1set::IEC1SET_SPEC>,
    #[doc = "0x7c - IEC1INV register"]
    pub iec1inv: crate::Reg<iec1inv::IEC1INV_SPEC>,
    _reserved25: [u8; 16usize],
    #[doc = "0x90 - IPC0 register"]
    pub ipc0: crate::Reg<ipc0::IPC0_SPEC>,
    #[doc = "0x94 - IPC0CLR register"]
    pub ipc0clr: crate::Reg<ipc0clr::IPC0CLR_SPEC>,
    #[doc = "0x98 - IPC0SET register"]
    pub ipc0set: crate::Reg<ipc0set::IPC0SET_SPEC>,
    #[doc = "0x9c - IPC0INV register"]
    pub ipc0inv: crate::Reg<ipc0inv::IPC0INV_SPEC>,
    #[doc = "0xa0 - IPC1 register"]
    pub ipc1: crate::Reg<ipc1::IPC1_SPEC>,
    #[doc = "0xa4 - IPC1CLR register"]
    pub ipc1clr: crate::Reg<ipc1clr::IPC1CLR_SPEC>,
    #[doc = "0xa8 - IPC1SET register"]
    pub ipc1set: crate::Reg<ipc1set::IPC1SET_SPEC>,
    #[doc = "0xac - IPC1INV register"]
    pub ipc1inv: crate::Reg<ipc1inv::IPC1INV_SPEC>,
    #[doc = "0xb0 - IPC2 register"]
    pub ipc2: crate::Reg<ipc2::IPC2_SPEC>,
    #[doc = "0xb4 - IPC2CLR register"]
    pub ipc2clr: crate::Reg<ipc2clr::IPC2CLR_SPEC>,
    #[doc = "0xb8 - IPC2SET register"]
    pub ipc2set: crate::Reg<ipc2set::IPC2SET_SPEC>,
    #[doc = "0xbc - IPC2INV register"]
    pub ipc2inv: crate::Reg<ipc2inv::IPC2INV_SPEC>,
    #[doc = "0xc0 - IPC3 register"]
    pub ipc3: crate::Reg<ipc3::IPC3_SPEC>,
    #[doc = "0xc4 - IPC3CLR register"]
    pub ipc3clr: crate::Reg<ipc3clr::IPC3CLR_SPEC>,
    #[doc = "0xc8 - IPC3SET register"]
    pub ipc3set: crate::Reg<ipc3set::IPC3SET_SPEC>,
    #[doc = "0xcc - IPC3INV register"]
    pub ipc3inv: crate::Reg<ipc3inv::IPC3INV_SPEC>,
    #[doc = "0xd0 - IPC4 register"]
    pub ipc4: crate::Reg<ipc4::IPC4_SPEC>,
    #[doc = "0xd4 - IPC4CLR register"]
    pub ipc4clr: crate::Reg<ipc4clr::IPC4CLR_SPEC>,
    #[doc = "0xd8 - IPC4SET register"]
    pub ipc4set: crate::Reg<ipc4set::IPC4SET_SPEC>,
    #[doc = "0xdc - IPC4INV register"]
    pub ipc4inv: crate::Reg<ipc4inv::IPC4INV_SPEC>,
    #[doc = "0xe0 - IPC5 register"]
    pub ipc5: crate::Reg<ipc5::IPC5_SPEC>,
    #[doc = "0xe4 - IPC5CLR register"]
    pub ipc5clr: crate::Reg<ipc5clr::IPC5CLR_SPEC>,
    #[doc = "0xe8 - IPC5SET register"]
    pub ipc5set: crate::Reg<ipc5set::IPC5SET_SPEC>,
    #[doc = "0xec - IPC5INV register"]
    pub ipc5inv: crate::Reg<ipc5inv::IPC5INV_SPEC>,
    #[doc = "0xf0 - IPC6 register"]
    pub ipc6: crate::Reg<ipc6::IPC6_SPEC>,
    #[doc = "0xf4 - IPC6CLR register"]
    pub ipc6clr: crate::Reg<ipc6clr::IPC6CLR_SPEC>,
    #[doc = "0xf8 - IPC6SET register"]
    pub ipc6set: crate::Reg<ipc6set::IPC6SET_SPEC>,
    #[doc = "0xfc - IPC6INV register"]
    pub ipc6inv: crate::Reg<ipc6inv::IPC6INV_SPEC>,
    #[doc = "0x100 - IPC7 register"]
    pub ipc7: crate::Reg<ipc7::IPC7_SPEC>,
    #[doc = "0x104 - IPC7CLR register"]
    pub ipc7clr: crate::Reg<ipc7clr::IPC7CLR_SPEC>,
    #[doc = "0x108 - IPC7SET register"]
    pub ipc7set: crate::Reg<ipc7set::IPC7SET_SPEC>,
    #[doc = "0x10c - IPC7INV register"]
    pub ipc7inv: crate::Reg<ipc7inv::IPC7INV_SPEC>,
    #[doc = "0x110 - IPC8 register"]
    pub ipc8: crate::Reg<ipc8::IPC8_SPEC>,
    #[doc = "0x114 - IPC8CLR register"]
    pub ipc8clr: crate::Reg<ipc8clr::IPC8CLR_SPEC>,
    #[doc = "0x118 - IPC8SET register"]
    pub ipc8set: crate::Reg<ipc8set::IPC8SET_SPEC>,
    #[doc = "0x11c - IPC8INV register"]
    pub ipc8inv: crate::Reg<ipc8inv::IPC8INV_SPEC>,
    #[doc = "0x120 - IPC9 register"]
    pub ipc9: crate::Reg<ipc9::IPC9_SPEC>,
    #[doc = "0x124 - IPC9CLR register"]
    pub ipc9clr: crate::Reg<ipc9clr::IPC9CLR_SPEC>,
    #[doc = "0x128 - IPC9SET register"]
    pub ipc9set: crate::Reg<ipc9set::IPC9SET_SPEC>,
    #[doc = "0x12c - IPC9INV register"]
    pub ipc9inv: crate::Reg<ipc9inv::IPC9INV_SPEC>,
    #[doc = "0x130 - IPC10 register"]
    pub ipc10: crate::Reg<ipc10::IPC10_SPEC>,
    #[doc = "0x134 - IPC10CLR register"]
    pub ipc10clr: crate::Reg<ipc10clr::IPC10CLR_SPEC>,
    #[doc = "0x138 - IPC10SET register"]
    pub ipc10set: crate::Reg<ipc10set::IPC10SET_SPEC>,
    #[doc = "0x13c - IPC10INV register"]
    pub ipc10inv: crate::Reg<ipc10inv::IPC10INV_SPEC>,
}
#[doc = "INTCON register accessor: an alias for `Reg<INTCON_SPEC>`"]
pub type INTCON = crate::Reg<intcon::INTCON_SPEC>;
#[doc = "INTCON register"]
pub mod intcon;
#[doc = "INTCONCLR register accessor: an alias for `Reg<INTCONCLR_SPEC>`"]
pub type INTCONCLR = crate::Reg<intconclr::INTCONCLR_SPEC>;
#[doc = "INTCONCLR register"]
pub mod intconclr;
#[doc = "INTCONSET register accessor: an alias for `Reg<INTCONSET_SPEC>`"]
pub type INTCONSET = crate::Reg<intconset::INTCONSET_SPEC>;
#[doc = "INTCONSET register"]
pub mod intconset;
#[doc = "INTCONINV register accessor: an alias for `Reg<INTCONINV_SPEC>`"]
pub type INTCONINV = crate::Reg<intconinv::INTCONINV_SPEC>;
#[doc = "INTCONINV register"]
pub mod intconinv;
#[doc = "INTSTAT register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "INTSTAT register"]
pub mod intstat;
#[doc = "IPTMR register accessor: an alias for `Reg<IPTMR_SPEC>`"]
pub type IPTMR = crate::Reg<iptmr::IPTMR_SPEC>;
#[doc = "IPTMR register"]
pub mod iptmr;
#[doc = "IPTMRCLR register accessor: an alias for `Reg<IPTMRCLR_SPEC>`"]
pub type IPTMRCLR = crate::Reg<iptmrclr::IPTMRCLR_SPEC>;
#[doc = "IPTMRCLR register"]
pub mod iptmrclr;
#[doc = "IPTMRSET register accessor: an alias for `Reg<IPTMRSET_SPEC>`"]
pub type IPTMRSET = crate::Reg<iptmrset::IPTMRSET_SPEC>;
#[doc = "IPTMRSET register"]
pub mod iptmrset;
#[doc = "IPTMRINV register accessor: an alias for `Reg<IPTMRINV_SPEC>`"]
pub type IPTMRINV = crate::Reg<iptmrinv::IPTMRINV_SPEC>;
#[doc = "IPTMRINV register"]
pub mod iptmrinv;
#[doc = "IFS0 register accessor: an alias for `Reg<IFS0_SPEC>`"]
pub type IFS0 = crate::Reg<ifs0::IFS0_SPEC>;
#[doc = "IFS0 register"]
pub mod ifs0;
#[doc = "IFS0CLR register accessor: an alias for `Reg<IFS0CLR_SPEC>`"]
pub type IFS0CLR = crate::Reg<ifs0clr::IFS0CLR_SPEC>;
#[doc = "IFS0CLR register"]
pub mod ifs0clr;
#[doc = "IFS0SET register accessor: an alias for `Reg<IFS0SET_SPEC>`"]
pub type IFS0SET = crate::Reg<ifs0set::IFS0SET_SPEC>;
#[doc = "IFS0SET register"]
pub mod ifs0set;
#[doc = "IFS0INV register accessor: an alias for `Reg<IFS0INV_SPEC>`"]
pub type IFS0INV = crate::Reg<ifs0inv::IFS0INV_SPEC>;
#[doc = "IFS0INV register"]
pub mod ifs0inv;
#[doc = "IFS1 register accessor: an alias for `Reg<IFS1_SPEC>`"]
pub type IFS1 = crate::Reg<ifs1::IFS1_SPEC>;
#[doc = "IFS1 register"]
pub mod ifs1;
#[doc = "IFS1CLR register accessor: an alias for `Reg<IFS1CLR_SPEC>`"]
pub type IFS1CLR = crate::Reg<ifs1clr::IFS1CLR_SPEC>;
#[doc = "IFS1CLR register"]
pub mod ifs1clr;
#[doc = "IFS1SET register accessor: an alias for `Reg<IFS1SET_SPEC>`"]
pub type IFS1SET = crate::Reg<ifs1set::IFS1SET_SPEC>;
#[doc = "IFS1SET register"]
pub mod ifs1set;
#[doc = "IFS1INV register accessor: an alias for `Reg<IFS1INV_SPEC>`"]
pub type IFS1INV = crate::Reg<ifs1inv::IFS1INV_SPEC>;
#[doc = "IFS1INV register"]
pub mod ifs1inv;
#[doc = "IEC0 register accessor: an alias for `Reg<IEC0_SPEC>`"]
pub type IEC0 = crate::Reg<iec0::IEC0_SPEC>;
#[doc = "IEC0 register"]
pub mod iec0;
#[doc = "IEC0CLR register accessor: an alias for `Reg<IEC0CLR_SPEC>`"]
pub type IEC0CLR = crate::Reg<iec0clr::IEC0CLR_SPEC>;
#[doc = "IEC0CLR register"]
pub mod iec0clr;
#[doc = "IEC0SET register accessor: an alias for `Reg<IEC0SET_SPEC>`"]
pub type IEC0SET = crate::Reg<iec0set::IEC0SET_SPEC>;
#[doc = "IEC0SET register"]
pub mod iec0set;
#[doc = "IEC0INV register accessor: an alias for `Reg<IEC0INV_SPEC>`"]
pub type IEC0INV = crate::Reg<iec0inv::IEC0INV_SPEC>;
#[doc = "IEC0INV register"]
pub mod iec0inv;
#[doc = "IEC1 register accessor: an alias for `Reg<IEC1_SPEC>`"]
pub type IEC1 = crate::Reg<iec1::IEC1_SPEC>;
#[doc = "IEC1 register"]
pub mod iec1;
#[doc = "IEC1CLR register accessor: an alias for `Reg<IEC1CLR_SPEC>`"]
pub type IEC1CLR = crate::Reg<iec1clr::IEC1CLR_SPEC>;
#[doc = "IEC1CLR register"]
pub mod iec1clr;
#[doc = "IEC1SET register accessor: an alias for `Reg<IEC1SET_SPEC>`"]
pub type IEC1SET = crate::Reg<iec1set::IEC1SET_SPEC>;
#[doc = "IEC1SET register"]
pub mod iec1set;
#[doc = "IEC1INV register accessor: an alias for `Reg<IEC1INV_SPEC>`"]
pub type IEC1INV = crate::Reg<iec1inv::IEC1INV_SPEC>;
#[doc = "IEC1INV register"]
pub mod iec1inv;
#[doc = "IPC0 register accessor: an alias for `Reg<IPC0_SPEC>`"]
pub type IPC0 = crate::Reg<ipc0::IPC0_SPEC>;
#[doc = "IPC0 register"]
pub mod ipc0;
#[doc = "IPC0CLR register accessor: an alias for `Reg<IPC0CLR_SPEC>`"]
pub type IPC0CLR = crate::Reg<ipc0clr::IPC0CLR_SPEC>;
#[doc = "IPC0CLR register"]
pub mod ipc0clr;
#[doc = "IPC0SET register accessor: an alias for `Reg<IPC0SET_SPEC>`"]
pub type IPC0SET = crate::Reg<ipc0set::IPC0SET_SPEC>;
#[doc = "IPC0SET register"]
pub mod ipc0set;
#[doc = "IPC0INV register accessor: an alias for `Reg<IPC0INV_SPEC>`"]
pub type IPC0INV = crate::Reg<ipc0inv::IPC0INV_SPEC>;
#[doc = "IPC0INV register"]
pub mod ipc0inv;
#[doc = "IPC1 register accessor: an alias for `Reg<IPC1_SPEC>`"]
pub type IPC1 = crate::Reg<ipc1::IPC1_SPEC>;
#[doc = "IPC1 register"]
pub mod ipc1;
#[doc = "IPC1CLR register accessor: an alias for `Reg<IPC1CLR_SPEC>`"]
pub type IPC1CLR = crate::Reg<ipc1clr::IPC1CLR_SPEC>;
#[doc = "IPC1CLR register"]
pub mod ipc1clr;
#[doc = "IPC1SET register accessor: an alias for `Reg<IPC1SET_SPEC>`"]
pub type IPC1SET = crate::Reg<ipc1set::IPC1SET_SPEC>;
#[doc = "IPC1SET register"]
pub mod ipc1set;
#[doc = "IPC1INV register accessor: an alias for `Reg<IPC1INV_SPEC>`"]
pub type IPC1INV = crate::Reg<ipc1inv::IPC1INV_SPEC>;
#[doc = "IPC1INV register"]
pub mod ipc1inv;
#[doc = "IPC2 register accessor: an alias for `Reg<IPC2_SPEC>`"]
pub type IPC2 = crate::Reg<ipc2::IPC2_SPEC>;
#[doc = "IPC2 register"]
pub mod ipc2;
#[doc = "IPC2CLR register accessor: an alias for `Reg<IPC2CLR_SPEC>`"]
pub type IPC2CLR = crate::Reg<ipc2clr::IPC2CLR_SPEC>;
#[doc = "IPC2CLR register"]
pub mod ipc2clr;
#[doc = "IPC2SET register accessor: an alias for `Reg<IPC2SET_SPEC>`"]
pub type IPC2SET = crate::Reg<ipc2set::IPC2SET_SPEC>;
#[doc = "IPC2SET register"]
pub mod ipc2set;
#[doc = "IPC2INV register accessor: an alias for `Reg<IPC2INV_SPEC>`"]
pub type IPC2INV = crate::Reg<ipc2inv::IPC2INV_SPEC>;
#[doc = "IPC2INV register"]
pub mod ipc2inv;
#[doc = "IPC3 register accessor: an alias for `Reg<IPC3_SPEC>`"]
pub type IPC3 = crate::Reg<ipc3::IPC3_SPEC>;
#[doc = "IPC3 register"]
pub mod ipc3;
#[doc = "IPC3CLR register accessor: an alias for `Reg<IPC3CLR_SPEC>`"]
pub type IPC3CLR = crate::Reg<ipc3clr::IPC3CLR_SPEC>;
#[doc = "IPC3CLR register"]
pub mod ipc3clr;
#[doc = "IPC3SET register accessor: an alias for `Reg<IPC3SET_SPEC>`"]
pub type IPC3SET = crate::Reg<ipc3set::IPC3SET_SPEC>;
#[doc = "IPC3SET register"]
pub mod ipc3set;
#[doc = "IPC3INV register accessor: an alias for `Reg<IPC3INV_SPEC>`"]
pub type IPC3INV = crate::Reg<ipc3inv::IPC3INV_SPEC>;
#[doc = "IPC3INV register"]
pub mod ipc3inv;
#[doc = "IPC4 register accessor: an alias for `Reg<IPC4_SPEC>`"]
pub type IPC4 = crate::Reg<ipc4::IPC4_SPEC>;
#[doc = "IPC4 register"]
pub mod ipc4;
#[doc = "IPC4CLR register accessor: an alias for `Reg<IPC4CLR_SPEC>`"]
pub type IPC4CLR = crate::Reg<ipc4clr::IPC4CLR_SPEC>;
#[doc = "IPC4CLR register"]
pub mod ipc4clr;
#[doc = "IPC4SET register accessor: an alias for `Reg<IPC4SET_SPEC>`"]
pub type IPC4SET = crate::Reg<ipc4set::IPC4SET_SPEC>;
#[doc = "IPC4SET register"]
pub mod ipc4set;
#[doc = "IPC4INV register accessor: an alias for `Reg<IPC4INV_SPEC>`"]
pub type IPC4INV = crate::Reg<ipc4inv::IPC4INV_SPEC>;
#[doc = "IPC4INV register"]
pub mod ipc4inv;
#[doc = "IPC5 register accessor: an alias for `Reg<IPC5_SPEC>`"]
pub type IPC5 = crate::Reg<ipc5::IPC5_SPEC>;
#[doc = "IPC5 register"]
pub mod ipc5;
#[doc = "IPC5CLR register accessor: an alias for `Reg<IPC5CLR_SPEC>`"]
pub type IPC5CLR = crate::Reg<ipc5clr::IPC5CLR_SPEC>;
#[doc = "IPC5CLR register"]
pub mod ipc5clr;
#[doc = "IPC5SET register accessor: an alias for `Reg<IPC5SET_SPEC>`"]
pub type IPC5SET = crate::Reg<ipc5set::IPC5SET_SPEC>;
#[doc = "IPC5SET register"]
pub mod ipc5set;
#[doc = "IPC5INV register accessor: an alias for `Reg<IPC5INV_SPEC>`"]
pub type IPC5INV = crate::Reg<ipc5inv::IPC5INV_SPEC>;
#[doc = "IPC5INV register"]
pub mod ipc5inv;
#[doc = "IPC6 register accessor: an alias for `Reg<IPC6_SPEC>`"]
pub type IPC6 = crate::Reg<ipc6::IPC6_SPEC>;
#[doc = "IPC6 register"]
pub mod ipc6;
#[doc = "IPC6CLR register accessor: an alias for `Reg<IPC6CLR_SPEC>`"]
pub type IPC6CLR = crate::Reg<ipc6clr::IPC6CLR_SPEC>;
#[doc = "IPC6CLR register"]
pub mod ipc6clr;
#[doc = "IPC6SET register accessor: an alias for `Reg<IPC6SET_SPEC>`"]
pub type IPC6SET = crate::Reg<ipc6set::IPC6SET_SPEC>;
#[doc = "IPC6SET register"]
pub mod ipc6set;
#[doc = "IPC6INV register accessor: an alias for `Reg<IPC6INV_SPEC>`"]
pub type IPC6INV = crate::Reg<ipc6inv::IPC6INV_SPEC>;
#[doc = "IPC6INV register"]
pub mod ipc6inv;
#[doc = "IPC7 register accessor: an alias for `Reg<IPC7_SPEC>`"]
pub type IPC7 = crate::Reg<ipc7::IPC7_SPEC>;
#[doc = "IPC7 register"]
pub mod ipc7;
#[doc = "IPC7CLR register accessor: an alias for `Reg<IPC7CLR_SPEC>`"]
pub type IPC7CLR = crate::Reg<ipc7clr::IPC7CLR_SPEC>;
#[doc = "IPC7CLR register"]
pub mod ipc7clr;
#[doc = "IPC7SET register accessor: an alias for `Reg<IPC7SET_SPEC>`"]
pub type IPC7SET = crate::Reg<ipc7set::IPC7SET_SPEC>;
#[doc = "IPC7SET register"]
pub mod ipc7set;
#[doc = "IPC7INV register accessor: an alias for `Reg<IPC7INV_SPEC>`"]
pub type IPC7INV = crate::Reg<ipc7inv::IPC7INV_SPEC>;
#[doc = "IPC7INV register"]
pub mod ipc7inv;
#[doc = "IPC8 register accessor: an alias for `Reg<IPC8_SPEC>`"]
pub type IPC8 = crate::Reg<ipc8::IPC8_SPEC>;
#[doc = "IPC8 register"]
pub mod ipc8;
#[doc = "IPC8CLR register accessor: an alias for `Reg<IPC8CLR_SPEC>`"]
pub type IPC8CLR = crate::Reg<ipc8clr::IPC8CLR_SPEC>;
#[doc = "IPC8CLR register"]
pub mod ipc8clr;
#[doc = "IPC8SET register accessor: an alias for `Reg<IPC8SET_SPEC>`"]
pub type IPC8SET = crate::Reg<ipc8set::IPC8SET_SPEC>;
#[doc = "IPC8SET register"]
pub mod ipc8set;
#[doc = "IPC8INV register accessor: an alias for `Reg<IPC8INV_SPEC>`"]
pub type IPC8INV = crate::Reg<ipc8inv::IPC8INV_SPEC>;
#[doc = "IPC8INV register"]
pub mod ipc8inv;
#[doc = "IPC9 register accessor: an alias for `Reg<IPC9_SPEC>`"]
pub type IPC9 = crate::Reg<ipc9::IPC9_SPEC>;
#[doc = "IPC9 register"]
pub mod ipc9;
#[doc = "IPC9CLR register accessor: an alias for `Reg<IPC9CLR_SPEC>`"]
pub type IPC9CLR = crate::Reg<ipc9clr::IPC9CLR_SPEC>;
#[doc = "IPC9CLR register"]
pub mod ipc9clr;
#[doc = "IPC9SET register accessor: an alias for `Reg<IPC9SET_SPEC>`"]
pub type IPC9SET = crate::Reg<ipc9set::IPC9SET_SPEC>;
#[doc = "IPC9SET register"]
pub mod ipc9set;
#[doc = "IPC9INV register accessor: an alias for `Reg<IPC9INV_SPEC>`"]
pub type IPC9INV = crate::Reg<ipc9inv::IPC9INV_SPEC>;
#[doc = "IPC9INV register"]
pub mod ipc9inv;
#[doc = "IPC10 register accessor: an alias for `Reg<IPC10_SPEC>`"]
pub type IPC10 = crate::Reg<ipc10::IPC10_SPEC>;
#[doc = "IPC10 register"]
pub mod ipc10;
#[doc = "IPC10CLR register accessor: an alias for `Reg<IPC10CLR_SPEC>`"]
pub type IPC10CLR = crate::Reg<ipc10clr::IPC10CLR_SPEC>;
#[doc = "IPC10CLR register"]
pub mod ipc10clr;
#[doc = "IPC10SET register accessor: an alias for `Reg<IPC10SET_SPEC>`"]
pub type IPC10SET = crate::Reg<ipc10set::IPC10SET_SPEC>;
#[doc = "IPC10SET register"]
pub mod ipc10set;
#[doc = "IPC10INV register accessor: an alias for `Reg<IPC10INV_SPEC>`"]
pub type IPC10INV = crate::Reg<ipc10inv::IPC10INV_SPEC>;
#[doc = "IPC10INV register"]
pub mod ipc10inv;
