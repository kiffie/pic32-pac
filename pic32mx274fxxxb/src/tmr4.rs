#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - T4CON register"]
    pub t4con: crate::Reg<t4con::T4CON_SPEC>,
    #[doc = "0x04 - T4CONCLR register"]
    pub t4conclr: crate::Reg<t4conclr::T4CONCLR_SPEC>,
    #[doc = "0x08 - T4CONSET register"]
    pub t4conset: crate::Reg<t4conset::T4CONSET_SPEC>,
    #[doc = "0x0c - T4CONINV register"]
    pub t4coninv: crate::Reg<t4coninv::T4CONINV_SPEC>,
    #[doc = "0x10 - TMR4 register"]
    pub tmr4: crate::Reg<tmr4::TMR4_SPEC>,
    #[doc = "0x14 - TMR4CLR register"]
    pub tmr4clr: crate::Reg<tmr4clr::TMR4CLR_SPEC>,
    #[doc = "0x18 - TMR4SET register"]
    pub tmr4set: crate::Reg<tmr4set::TMR4SET_SPEC>,
    #[doc = "0x1c - TMR4INV register"]
    pub tmr4inv: crate::Reg<tmr4inv::TMR4INV_SPEC>,
    #[doc = "0x20 - PR4 register"]
    pub pr4: crate::Reg<pr4::PR4_SPEC>,
    #[doc = "0x24 - PR4CLR register"]
    pub pr4clr: crate::Reg<pr4clr::PR4CLR_SPEC>,
    #[doc = "0x28 - PR4SET register"]
    pub pr4set: crate::Reg<pr4set::PR4SET_SPEC>,
    #[doc = "0x2c - PR4INV register"]
    pub pr4inv: crate::Reg<pr4inv::PR4INV_SPEC>,
}
#[doc = "T4CON register accessor: an alias for `Reg<T4CON_SPEC>`"]
pub type T4CON = crate::Reg<t4con::T4CON_SPEC>;
#[doc = "T4CON register"]
pub mod t4con;
#[doc = "T4CONCLR register accessor: an alias for `Reg<T4CONCLR_SPEC>`"]
pub type T4CONCLR = crate::Reg<t4conclr::T4CONCLR_SPEC>;
#[doc = "T4CONCLR register"]
pub mod t4conclr;
#[doc = "T4CONSET register accessor: an alias for `Reg<T4CONSET_SPEC>`"]
pub type T4CONSET = crate::Reg<t4conset::T4CONSET_SPEC>;
#[doc = "T4CONSET register"]
pub mod t4conset;
#[doc = "T4CONINV register accessor: an alias for `Reg<T4CONINV_SPEC>`"]
pub type T4CONINV = crate::Reg<t4coninv::T4CONINV_SPEC>;
#[doc = "T4CONINV register"]
pub mod t4coninv;
#[doc = "TMR4 register accessor: an alias for `Reg<TMR4_SPEC>`"]
pub type TMR4 = crate::Reg<tmr4::TMR4_SPEC>;
#[doc = "TMR4 register"]
pub mod tmr4;
#[doc = "TMR4CLR register accessor: an alias for `Reg<TMR4CLR_SPEC>`"]
pub type TMR4CLR = crate::Reg<tmr4clr::TMR4CLR_SPEC>;
#[doc = "TMR4CLR register"]
pub mod tmr4clr;
#[doc = "TMR4SET register accessor: an alias for `Reg<TMR4SET_SPEC>`"]
pub type TMR4SET = crate::Reg<tmr4set::TMR4SET_SPEC>;
#[doc = "TMR4SET register"]
pub mod tmr4set;
#[doc = "TMR4INV register accessor: an alias for `Reg<TMR4INV_SPEC>`"]
pub type TMR4INV = crate::Reg<tmr4inv::TMR4INV_SPEC>;
#[doc = "TMR4INV register"]
pub mod tmr4inv;
#[doc = "PR4 register accessor: an alias for `Reg<PR4_SPEC>`"]
pub type PR4 = crate::Reg<pr4::PR4_SPEC>;
#[doc = "PR4 register"]
pub mod pr4;
#[doc = "PR4CLR register accessor: an alias for `Reg<PR4CLR_SPEC>`"]
pub type PR4CLR = crate::Reg<pr4clr::PR4CLR_SPEC>;
#[doc = "PR4CLR register"]
pub mod pr4clr;
#[doc = "PR4SET register accessor: an alias for `Reg<PR4SET_SPEC>`"]
pub type PR4SET = crate::Reg<pr4set::PR4SET_SPEC>;
#[doc = "PR4SET register"]
pub mod pr4set;
#[doc = "PR4INV register accessor: an alias for `Reg<PR4INV_SPEC>`"]
pub type PR4INV = crate::Reg<pr4inv::PR4INV_SPEC>;
#[doc = "PR4INV register"]
pub mod pr4inv;
