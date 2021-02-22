#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T2CON register"]
    pub t2con: crate::Reg<t2con::T2CON_SPEC>,
    #[doc = "0x04 - T2CONCLR register"]
    pub t2conclr: crate::Reg<t2conclr::T2CONCLR_SPEC>,
    #[doc = "0x08 - T2CONSET register"]
    pub t2conset: crate::Reg<t2conset::T2CONSET_SPEC>,
    #[doc = "0x0c - T2CONINV register"]
    pub t2coninv: crate::Reg<t2coninv::T2CONINV_SPEC>,
    #[doc = "0x10 - TMR2 register"]
    pub tmr2: crate::Reg<tmr2::TMR2_SPEC>,
    #[doc = "0x14 - TMR2CLR register"]
    pub tmr2clr: crate::Reg<tmr2clr::TMR2CLR_SPEC>,
    #[doc = "0x18 - TMR2SET register"]
    pub tmr2set: crate::Reg<tmr2set::TMR2SET_SPEC>,
    #[doc = "0x1c - TMR2INV register"]
    pub tmr2inv: crate::Reg<tmr2inv::TMR2INV_SPEC>,
    #[doc = "0x20 - PR2 register"]
    pub pr2: crate::Reg<pr2::PR2_SPEC>,
    #[doc = "0x24 - PR2CLR register"]
    pub pr2clr: crate::Reg<pr2clr::PR2CLR_SPEC>,
    #[doc = "0x28 - PR2SET register"]
    pub pr2set: crate::Reg<pr2set::PR2SET_SPEC>,
    #[doc = "0x2c - PR2INV register"]
    pub pr2inv: crate::Reg<pr2inv::PR2INV_SPEC>,
}
#[doc = "T2CON register accessor: an alias for `Reg<T2CON_SPEC>`"]
pub type T2CON = crate::Reg<t2con::T2CON_SPEC>;
#[doc = "T2CON register"]
pub mod t2con;
#[doc = "T2CONCLR register accessor: an alias for `Reg<T2CONCLR_SPEC>`"]
pub type T2CONCLR = crate::Reg<t2conclr::T2CONCLR_SPEC>;
#[doc = "T2CONCLR register"]
pub mod t2conclr;
#[doc = "T2CONSET register accessor: an alias for `Reg<T2CONSET_SPEC>`"]
pub type T2CONSET = crate::Reg<t2conset::T2CONSET_SPEC>;
#[doc = "T2CONSET register"]
pub mod t2conset;
#[doc = "T2CONINV register accessor: an alias for `Reg<T2CONINV_SPEC>`"]
pub type T2CONINV = crate::Reg<t2coninv::T2CONINV_SPEC>;
#[doc = "T2CONINV register"]
pub mod t2coninv;
#[doc = "TMR2 register accessor: an alias for `Reg<TMR2_SPEC>`"]
pub type TMR2 = crate::Reg<tmr2::TMR2_SPEC>;
#[doc = "TMR2 register"]
pub mod tmr2;
#[doc = "TMR2CLR register accessor: an alias for `Reg<TMR2CLR_SPEC>`"]
pub type TMR2CLR = crate::Reg<tmr2clr::TMR2CLR_SPEC>;
#[doc = "TMR2CLR register"]
pub mod tmr2clr;
#[doc = "TMR2SET register accessor: an alias for `Reg<TMR2SET_SPEC>`"]
pub type TMR2SET = crate::Reg<tmr2set::TMR2SET_SPEC>;
#[doc = "TMR2SET register"]
pub mod tmr2set;
#[doc = "TMR2INV register accessor: an alias for `Reg<TMR2INV_SPEC>`"]
pub type TMR2INV = crate::Reg<tmr2inv::TMR2INV_SPEC>;
#[doc = "TMR2INV register"]
pub mod tmr2inv;
#[doc = "PR2 register accessor: an alias for `Reg<PR2_SPEC>`"]
pub type PR2 = crate::Reg<pr2::PR2_SPEC>;
#[doc = "PR2 register"]
pub mod pr2;
#[doc = "PR2CLR register accessor: an alias for `Reg<PR2CLR_SPEC>`"]
pub type PR2CLR = crate::Reg<pr2clr::PR2CLR_SPEC>;
#[doc = "PR2CLR register"]
pub mod pr2clr;
#[doc = "PR2SET register accessor: an alias for `Reg<PR2SET_SPEC>`"]
pub type PR2SET = crate::Reg<pr2set::PR2SET_SPEC>;
#[doc = "PR2SET register"]
pub mod pr2set;
#[doc = "PR2INV register accessor: an alias for `Reg<PR2INV_SPEC>`"]
pub type PR2INV = crate::Reg<pr2inv::PR2INV_SPEC>;
#[doc = "PR2INV register"]
pub mod pr2inv;
