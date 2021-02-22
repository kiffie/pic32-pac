#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T1CON register"]
    pub t1con: crate::Reg<t1con::T1CON_SPEC>,
    #[doc = "0x04 - T1CONCLR register"]
    pub t1conclr: crate::Reg<t1conclr::T1CONCLR_SPEC>,
    #[doc = "0x08 - T1CONSET register"]
    pub t1conset: crate::Reg<t1conset::T1CONSET_SPEC>,
    #[doc = "0x0c - T1CONINV register"]
    pub t1coninv: crate::Reg<t1coninv::T1CONINV_SPEC>,
    #[doc = "0x10 - TMR1 register"]
    pub tmr1: crate::Reg<tmr1::TMR1_SPEC>,
    #[doc = "0x14 - TMR1CLR register"]
    pub tmr1clr: crate::Reg<tmr1clr::TMR1CLR_SPEC>,
    #[doc = "0x18 - TMR1SET register"]
    pub tmr1set: crate::Reg<tmr1set::TMR1SET_SPEC>,
    #[doc = "0x1c - TMR1INV register"]
    pub tmr1inv: crate::Reg<tmr1inv::TMR1INV_SPEC>,
    #[doc = "0x20 - PR1 register"]
    pub pr1: crate::Reg<pr1::PR1_SPEC>,
    #[doc = "0x24 - PR1CLR register"]
    pub pr1clr: crate::Reg<pr1clr::PR1CLR_SPEC>,
    #[doc = "0x28 - PR1SET register"]
    pub pr1set: crate::Reg<pr1set::PR1SET_SPEC>,
    #[doc = "0x2c - PR1INV register"]
    pub pr1inv: crate::Reg<pr1inv::PR1INV_SPEC>,
}
#[doc = "T1CON register accessor: an alias for `Reg<T1CON_SPEC>`"]
pub type T1CON = crate::Reg<t1con::T1CON_SPEC>;
#[doc = "T1CON register"]
pub mod t1con;
#[doc = "T1CONCLR register accessor: an alias for `Reg<T1CONCLR_SPEC>`"]
pub type T1CONCLR = crate::Reg<t1conclr::T1CONCLR_SPEC>;
#[doc = "T1CONCLR register"]
pub mod t1conclr;
#[doc = "T1CONSET register accessor: an alias for `Reg<T1CONSET_SPEC>`"]
pub type T1CONSET = crate::Reg<t1conset::T1CONSET_SPEC>;
#[doc = "T1CONSET register"]
pub mod t1conset;
#[doc = "T1CONINV register accessor: an alias for `Reg<T1CONINV_SPEC>`"]
pub type T1CONINV = crate::Reg<t1coninv::T1CONINV_SPEC>;
#[doc = "T1CONINV register"]
pub mod t1coninv;
#[doc = "TMR1 register accessor: an alias for `Reg<TMR1_SPEC>`"]
pub type TMR1 = crate::Reg<tmr1::TMR1_SPEC>;
#[doc = "TMR1 register"]
pub mod tmr1;
#[doc = "TMR1CLR register accessor: an alias for `Reg<TMR1CLR_SPEC>`"]
pub type TMR1CLR = crate::Reg<tmr1clr::TMR1CLR_SPEC>;
#[doc = "TMR1CLR register"]
pub mod tmr1clr;
#[doc = "TMR1SET register accessor: an alias for `Reg<TMR1SET_SPEC>`"]
pub type TMR1SET = crate::Reg<tmr1set::TMR1SET_SPEC>;
#[doc = "TMR1SET register"]
pub mod tmr1set;
#[doc = "TMR1INV register accessor: an alias for `Reg<TMR1INV_SPEC>`"]
pub type TMR1INV = crate::Reg<tmr1inv::TMR1INV_SPEC>;
#[doc = "TMR1INV register"]
pub mod tmr1inv;
#[doc = "PR1 register accessor: an alias for `Reg<PR1_SPEC>`"]
pub type PR1 = crate::Reg<pr1::PR1_SPEC>;
#[doc = "PR1 register"]
pub mod pr1;
#[doc = "PR1CLR register accessor: an alias for `Reg<PR1CLR_SPEC>`"]
pub type PR1CLR = crate::Reg<pr1clr::PR1CLR_SPEC>;
#[doc = "PR1CLR register"]
pub mod pr1clr;
#[doc = "PR1SET register accessor: an alias for `Reg<PR1SET_SPEC>`"]
pub type PR1SET = crate::Reg<pr1set::PR1SET_SPEC>;
#[doc = "PR1SET register"]
pub mod pr1set;
#[doc = "PR1INV register accessor: an alias for `Reg<PR1INV_SPEC>`"]
pub type PR1INV = crate::Reg<pr1inv::PR1INV_SPEC>;
#[doc = "PR1INV register"]
pub mod pr1inv;
