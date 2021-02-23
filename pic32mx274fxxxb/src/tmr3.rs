#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T3CON register"]
    pub t3con: crate::Reg<t3con::T3CON_SPEC>,
    #[doc = "0x04 - T3CONCLR register"]
    pub t3conclr: crate::Reg<t3conclr::T3CONCLR_SPEC>,
    #[doc = "0x08 - T3CONSET register"]
    pub t3conset: crate::Reg<t3conset::T3CONSET_SPEC>,
    #[doc = "0x0c - T3CONINV register"]
    pub t3coninv: crate::Reg<t3coninv::T3CONINV_SPEC>,
    #[doc = "0x10 - TMR3 register"]
    pub tmr3: crate::Reg<tmr3::TMR3_SPEC>,
    #[doc = "0x14 - TMR3CLR register"]
    pub tmr3clr: crate::Reg<tmr3clr::TMR3CLR_SPEC>,
    #[doc = "0x18 - TMR3SET register"]
    pub tmr3set: crate::Reg<tmr3set::TMR3SET_SPEC>,
    #[doc = "0x1c - TMR3INV register"]
    pub tmr3inv: crate::Reg<tmr3inv::TMR3INV_SPEC>,
    #[doc = "0x20 - PR3 register"]
    pub pr3: crate::Reg<pr3::PR3_SPEC>,
    #[doc = "0x24 - PR3CLR register"]
    pub pr3clr: crate::Reg<pr3clr::PR3CLR_SPEC>,
    #[doc = "0x28 - PR3SET register"]
    pub pr3set: crate::Reg<pr3set::PR3SET_SPEC>,
    #[doc = "0x2c - PR3INV register"]
    pub pr3inv: crate::Reg<pr3inv::PR3INV_SPEC>,
}
#[doc = "T3CON register accessor: an alias for `Reg<T3CON_SPEC>`"]
pub type T3CON = crate::Reg<t3con::T3CON_SPEC>;
#[doc = "T3CON register"]
pub mod t3con;
#[doc = "T3CONCLR register accessor: an alias for `Reg<T3CONCLR_SPEC>`"]
pub type T3CONCLR = crate::Reg<t3conclr::T3CONCLR_SPEC>;
#[doc = "T3CONCLR register"]
pub mod t3conclr;
#[doc = "T3CONSET register accessor: an alias for `Reg<T3CONSET_SPEC>`"]
pub type T3CONSET = crate::Reg<t3conset::T3CONSET_SPEC>;
#[doc = "T3CONSET register"]
pub mod t3conset;
#[doc = "T3CONINV register accessor: an alias for `Reg<T3CONINV_SPEC>`"]
pub type T3CONINV = crate::Reg<t3coninv::T3CONINV_SPEC>;
#[doc = "T3CONINV register"]
pub mod t3coninv;
#[doc = "TMR3 register accessor: an alias for `Reg<TMR3_SPEC>`"]
pub type TMR3 = crate::Reg<tmr3::TMR3_SPEC>;
#[doc = "TMR3 register"]
pub mod tmr3;
#[doc = "TMR3CLR register accessor: an alias for `Reg<TMR3CLR_SPEC>`"]
pub type TMR3CLR = crate::Reg<tmr3clr::TMR3CLR_SPEC>;
#[doc = "TMR3CLR register"]
pub mod tmr3clr;
#[doc = "TMR3SET register accessor: an alias for `Reg<TMR3SET_SPEC>`"]
pub type TMR3SET = crate::Reg<tmr3set::TMR3SET_SPEC>;
#[doc = "TMR3SET register"]
pub mod tmr3set;
#[doc = "TMR3INV register accessor: an alias for `Reg<TMR3INV_SPEC>`"]
pub type TMR3INV = crate::Reg<tmr3inv::TMR3INV_SPEC>;
#[doc = "TMR3INV register"]
pub mod tmr3inv;
#[doc = "PR3 register accessor: an alias for `Reg<PR3_SPEC>`"]
pub type PR3 = crate::Reg<pr3::PR3_SPEC>;
#[doc = "PR3 register"]
pub mod pr3;
#[doc = "PR3CLR register accessor: an alias for `Reg<PR3CLR_SPEC>`"]
pub type PR3CLR = crate::Reg<pr3clr::PR3CLR_SPEC>;
#[doc = "PR3CLR register"]
pub mod pr3clr;
#[doc = "PR3SET register accessor: an alias for `Reg<PR3SET_SPEC>`"]
pub type PR3SET = crate::Reg<pr3set::PR3SET_SPEC>;
#[doc = "PR3SET register"]
pub mod pr3set;
#[doc = "PR3INV register accessor: an alias for `Reg<PR3INV_SPEC>`"]
pub type PR3INV = crate::Reg<pr3inv::PR3INV_SPEC>;
#[doc = "PR3INV register"]
pub mod pr3inv;
