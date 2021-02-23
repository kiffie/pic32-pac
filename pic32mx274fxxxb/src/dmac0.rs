#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCH0CON register"]
    pub cont: crate::Reg<cont::CONT_SPEC>,
    #[doc = "0x04 - DCH0CONCLR register"]
    pub contclr: crate::Reg<contclr::CONTCLR_SPEC>,
    #[doc = "0x08 - DCH0CONSET register"]
    pub contset: crate::Reg<contset::CONTSET_SPEC>,
    #[doc = "0x0c - DCH0CONINV register"]
    pub continv: crate::Reg<continv::CONTINV_SPEC>,
    #[doc = "0x10 - DCH0ECON register"]
    pub econ: crate::Reg<econ::ECON_SPEC>,
    #[doc = "0x14 - DCH0ECONCLR register"]
    pub econclr: crate::Reg<econclr::ECONCLR_SPEC>,
    #[doc = "0x18 - DCH0ECONSET register"]
    pub econset: crate::Reg<econset::ECONSET_SPEC>,
    #[doc = "0x1c - DCH0ECONINV register"]
    pub econinv: crate::Reg<econinv::ECONINV_SPEC>,
    #[doc = "0x20 - DCH0INT register"]
    pub int: crate::Reg<int::INT_SPEC>,
    #[doc = "0x24 - DCH0INTCLR register"]
    pub intclr: crate::Reg<intclr::INTCLR_SPEC>,
    #[doc = "0x28 - DCH0INTSET register"]
    pub intset: crate::Reg<intset::INTSET_SPEC>,
    #[doc = "0x2c - DCH0INTINV register"]
    pub intinv: crate::Reg<intinv::INTINV_SPEC>,
    #[doc = "0x30 - DCH0SSA register"]
    pub ssa: crate::Reg<ssa::SSA_SPEC>,
    #[doc = "0x34 - DCH0SSACLR register"]
    pub ssaclr: crate::Reg<ssaclr::SSACLR_SPEC>,
    #[doc = "0x38 - DCH0SSASET register"]
    pub ssaset: crate::Reg<ssaset::SSASET_SPEC>,
    #[doc = "0x3c - DCH0SSAINV register"]
    pub ssainv: crate::Reg<ssainv::SSAINV_SPEC>,
    #[doc = "0x40 - DCH0DSA register"]
    pub dsa: crate::Reg<dsa::DSA_SPEC>,
    #[doc = "0x44 - DCH0DSACLR register"]
    pub dsaclr: crate::Reg<dsaclr::DSACLR_SPEC>,
    #[doc = "0x48 - DCH0DSASET register"]
    pub dsaset: crate::Reg<dsaset::DSASET_SPEC>,
    #[doc = "0x4c - DCH0DSAINV register"]
    pub dsainv: crate::Reg<dsainv::DSAINV_SPEC>,
    #[doc = "0x50 - DCH0SSIZ register"]
    pub ssiz: crate::Reg<ssiz::SSIZ_SPEC>,
    #[doc = "0x54 - DCH0SSIZCLR register"]
    pub ssizclr: crate::Reg<ssizclr::SSIZCLR_SPEC>,
    #[doc = "0x58 - DCH0SSIZSET register"]
    pub ssizset: crate::Reg<ssizset::SSIZSET_SPEC>,
    #[doc = "0x5c - DCH0SSIZINV register"]
    pub ssizinv: crate::Reg<ssizinv::SSIZINV_SPEC>,
    #[doc = "0x60 - DCH0DSIZ register"]
    pub dsiz: crate::Reg<dsiz::DSIZ_SPEC>,
    #[doc = "0x64 - DCH0DSIZCLR register"]
    pub dsizclr: crate::Reg<dsizclr::DSIZCLR_SPEC>,
    #[doc = "0x68 - DCH0DSIZSET register"]
    pub dsizset: crate::Reg<dsizset::DSIZSET_SPEC>,
    #[doc = "0x6c - DCH0DSIZINV register"]
    pub dsizinv: crate::Reg<dsizinv::DSIZINV_SPEC>,
    #[doc = "0x70 - DCH0SPTR register"]
    pub sptr: crate::Reg<sptr::SPTR_SPEC>,
    #[doc = "0x74 - DCH0SPTRCLR register"]
    pub sptrclr: crate::Reg<sptrclr::SPTRCLR_SPEC>,
    #[doc = "0x78 - DCH0SPTRSET register"]
    pub sptrset: crate::Reg<sptrset::SPTRSET_SPEC>,
    #[doc = "0x7c - DCH0SPTRINV register"]
    pub sptrinv: crate::Reg<sptrinv::SPTRINV_SPEC>,
    #[doc = "0x80 - DCH0DPTR register"]
    pub dptr: crate::Reg<dptr::DPTR_SPEC>,
    #[doc = "0x84 - DCH0DPTRCLR register"]
    pub dptrclr: crate::Reg<dptrclr::DPTRCLR_SPEC>,
    #[doc = "0x88 - DCH0DPTRSET register"]
    pub dptrset: crate::Reg<dptrset::DPTRSET_SPEC>,
    #[doc = "0x8c - DCH0DPTRINV register"]
    pub dptrinv: crate::Reg<dptrinv::DPTRINV_SPEC>,
    #[doc = "0x90 - DCH0CSIZ register"]
    pub csiz: crate::Reg<csiz::CSIZ_SPEC>,
    #[doc = "0x94 - DCH0CSIZCLR register"]
    pub csizclr: crate::Reg<csizclr::CSIZCLR_SPEC>,
    #[doc = "0x98 - DCH0CSIZSET register"]
    pub csizset: crate::Reg<csizset::CSIZSET_SPEC>,
    #[doc = "0x9c - DCH0CSIZINV register"]
    pub csizinv: crate::Reg<csizinv::CSIZINV_SPEC>,
    #[doc = "0xa0 - DCH0CPTR register"]
    pub cptr: crate::Reg<cptr::CPTR_SPEC>,
    #[doc = "0xa4 - DCH0CPTRCLR register"]
    pub cptrclr: crate::Reg<cptrclr::CPTRCLR_SPEC>,
    #[doc = "0xa8 - DCH0CPTRSET register"]
    pub cptrset: crate::Reg<cptrset::CPTRSET_SPEC>,
    #[doc = "0xac - DCH0CPTRINV register"]
    pub cptrinv: crate::Reg<cptrinv::CPTRINV_SPEC>,
    #[doc = "0xb0 - DCH0DAT register"]
    pub dat: crate::Reg<dat::DAT_SPEC>,
    #[doc = "0xb4 - DCH0DATCLR register"]
    pub datclr: crate::Reg<datclr::DATCLR_SPEC>,
    #[doc = "0xb8 - DCH0DATSET register"]
    pub datset: crate::Reg<datset::DATSET_SPEC>,
    #[doc = "0xbc - DCH0DATINV register"]
    pub datinv: crate::Reg<datinv::DATINV_SPEC>,
}
#[doc = "CONT register accessor: an alias for `Reg<CONT_SPEC>`"]
pub type CONT = crate::Reg<cont::CONT_SPEC>;
#[doc = "DCH0CON register"]
pub mod cont;
#[doc = "CONTCLR register accessor: an alias for `Reg<CONTCLR_SPEC>`"]
pub type CONTCLR = crate::Reg<contclr::CONTCLR_SPEC>;
#[doc = "DCH0CONCLR register"]
pub mod contclr;
#[doc = "CONTSET register accessor: an alias for `Reg<CONTSET_SPEC>`"]
pub type CONTSET = crate::Reg<contset::CONTSET_SPEC>;
#[doc = "DCH0CONSET register"]
pub mod contset;
#[doc = "CONTINV register accessor: an alias for `Reg<CONTINV_SPEC>`"]
pub type CONTINV = crate::Reg<continv::CONTINV_SPEC>;
#[doc = "DCH0CONINV register"]
pub mod continv;
#[doc = "ECON register accessor: an alias for `Reg<ECON_SPEC>`"]
pub type ECON = crate::Reg<econ::ECON_SPEC>;
#[doc = "DCH0ECON register"]
pub mod econ;
#[doc = "ECONCLR register accessor: an alias for `Reg<ECONCLR_SPEC>`"]
pub type ECONCLR = crate::Reg<econclr::ECONCLR_SPEC>;
#[doc = "DCH0ECONCLR register"]
pub mod econclr;
#[doc = "ECONSET register accessor: an alias for `Reg<ECONSET_SPEC>`"]
pub type ECONSET = crate::Reg<econset::ECONSET_SPEC>;
#[doc = "DCH0ECONSET register"]
pub mod econset;
#[doc = "ECONINV register accessor: an alias for `Reg<ECONINV_SPEC>`"]
pub type ECONINV = crate::Reg<econinv::ECONINV_SPEC>;
#[doc = "DCH0ECONINV register"]
pub mod econinv;
#[doc = "INT register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "DCH0INT register"]
pub mod int;
#[doc = "INTCLR register accessor: an alias for `Reg<INTCLR_SPEC>`"]
pub type INTCLR = crate::Reg<intclr::INTCLR_SPEC>;
#[doc = "DCH0INTCLR register"]
pub mod intclr;
#[doc = "INTSET register accessor: an alias for `Reg<INTSET_SPEC>`"]
pub type INTSET = crate::Reg<intset::INTSET_SPEC>;
#[doc = "DCH0INTSET register"]
pub mod intset;
#[doc = "INTINV register accessor: an alias for `Reg<INTINV_SPEC>`"]
pub type INTINV = crate::Reg<intinv::INTINV_SPEC>;
#[doc = "DCH0INTINV register"]
pub mod intinv;
#[doc = "SSA register accessor: an alias for `Reg<SSA_SPEC>`"]
pub type SSA = crate::Reg<ssa::SSA_SPEC>;
#[doc = "DCH0SSA register"]
pub mod ssa;
#[doc = "SSACLR register accessor: an alias for `Reg<SSACLR_SPEC>`"]
pub type SSACLR = crate::Reg<ssaclr::SSACLR_SPEC>;
#[doc = "DCH0SSACLR register"]
pub mod ssaclr;
#[doc = "SSASET register accessor: an alias for `Reg<SSASET_SPEC>`"]
pub type SSASET = crate::Reg<ssaset::SSASET_SPEC>;
#[doc = "DCH0SSASET register"]
pub mod ssaset;
#[doc = "SSAINV register accessor: an alias for `Reg<SSAINV_SPEC>`"]
pub type SSAINV = crate::Reg<ssainv::SSAINV_SPEC>;
#[doc = "DCH0SSAINV register"]
pub mod ssainv;
#[doc = "DSA register accessor: an alias for `Reg<DSA_SPEC>`"]
pub type DSA = crate::Reg<dsa::DSA_SPEC>;
#[doc = "DCH0DSA register"]
pub mod dsa;
#[doc = "DSACLR register accessor: an alias for `Reg<DSACLR_SPEC>`"]
pub type DSACLR = crate::Reg<dsaclr::DSACLR_SPEC>;
#[doc = "DCH0DSACLR register"]
pub mod dsaclr;
#[doc = "DSASET register accessor: an alias for `Reg<DSASET_SPEC>`"]
pub type DSASET = crate::Reg<dsaset::DSASET_SPEC>;
#[doc = "DCH0DSASET register"]
pub mod dsaset;
#[doc = "DSAINV register accessor: an alias for `Reg<DSAINV_SPEC>`"]
pub type DSAINV = crate::Reg<dsainv::DSAINV_SPEC>;
#[doc = "DCH0DSAINV register"]
pub mod dsainv;
#[doc = "SSIZ register accessor: an alias for `Reg<SSIZ_SPEC>`"]
pub type SSIZ = crate::Reg<ssiz::SSIZ_SPEC>;
#[doc = "DCH0SSIZ register"]
pub mod ssiz;
#[doc = "SSIZCLR register accessor: an alias for `Reg<SSIZCLR_SPEC>`"]
pub type SSIZCLR = crate::Reg<ssizclr::SSIZCLR_SPEC>;
#[doc = "DCH0SSIZCLR register"]
pub mod ssizclr;
#[doc = "SSIZSET register accessor: an alias for `Reg<SSIZSET_SPEC>`"]
pub type SSIZSET = crate::Reg<ssizset::SSIZSET_SPEC>;
#[doc = "DCH0SSIZSET register"]
pub mod ssizset;
#[doc = "SSIZINV register accessor: an alias for `Reg<SSIZINV_SPEC>`"]
pub type SSIZINV = crate::Reg<ssizinv::SSIZINV_SPEC>;
#[doc = "DCH0SSIZINV register"]
pub mod ssizinv;
#[doc = "DSIZ register accessor: an alias for `Reg<DSIZ_SPEC>`"]
pub type DSIZ = crate::Reg<dsiz::DSIZ_SPEC>;
#[doc = "DCH0DSIZ register"]
pub mod dsiz;
#[doc = "DSIZCLR register accessor: an alias for `Reg<DSIZCLR_SPEC>`"]
pub type DSIZCLR = crate::Reg<dsizclr::DSIZCLR_SPEC>;
#[doc = "DCH0DSIZCLR register"]
pub mod dsizclr;
#[doc = "DSIZSET register accessor: an alias for `Reg<DSIZSET_SPEC>`"]
pub type DSIZSET = crate::Reg<dsizset::DSIZSET_SPEC>;
#[doc = "DCH0DSIZSET register"]
pub mod dsizset;
#[doc = "DSIZINV register accessor: an alias for `Reg<DSIZINV_SPEC>`"]
pub type DSIZINV = crate::Reg<dsizinv::DSIZINV_SPEC>;
#[doc = "DCH0DSIZINV register"]
pub mod dsizinv;
#[doc = "SPTR register accessor: an alias for `Reg<SPTR_SPEC>`"]
pub type SPTR = crate::Reg<sptr::SPTR_SPEC>;
#[doc = "DCH0SPTR register"]
pub mod sptr;
#[doc = "SPTRCLR register accessor: an alias for `Reg<SPTRCLR_SPEC>`"]
pub type SPTRCLR = crate::Reg<sptrclr::SPTRCLR_SPEC>;
#[doc = "DCH0SPTRCLR register"]
pub mod sptrclr;
#[doc = "SPTRSET register accessor: an alias for `Reg<SPTRSET_SPEC>`"]
pub type SPTRSET = crate::Reg<sptrset::SPTRSET_SPEC>;
#[doc = "DCH0SPTRSET register"]
pub mod sptrset;
#[doc = "SPTRINV register accessor: an alias for `Reg<SPTRINV_SPEC>`"]
pub type SPTRINV = crate::Reg<sptrinv::SPTRINV_SPEC>;
#[doc = "DCH0SPTRINV register"]
pub mod sptrinv;
#[doc = "DPTR register accessor: an alias for `Reg<DPTR_SPEC>`"]
pub type DPTR = crate::Reg<dptr::DPTR_SPEC>;
#[doc = "DCH0DPTR register"]
pub mod dptr;
#[doc = "DPTRCLR register accessor: an alias for `Reg<DPTRCLR_SPEC>`"]
pub type DPTRCLR = crate::Reg<dptrclr::DPTRCLR_SPEC>;
#[doc = "DCH0DPTRCLR register"]
pub mod dptrclr;
#[doc = "DPTRSET register accessor: an alias for `Reg<DPTRSET_SPEC>`"]
pub type DPTRSET = crate::Reg<dptrset::DPTRSET_SPEC>;
#[doc = "DCH0DPTRSET register"]
pub mod dptrset;
#[doc = "DPTRINV register accessor: an alias for `Reg<DPTRINV_SPEC>`"]
pub type DPTRINV = crate::Reg<dptrinv::DPTRINV_SPEC>;
#[doc = "DCH0DPTRINV register"]
pub mod dptrinv;
#[doc = "CSIZ register accessor: an alias for `Reg<CSIZ_SPEC>`"]
pub type CSIZ = crate::Reg<csiz::CSIZ_SPEC>;
#[doc = "DCH0CSIZ register"]
pub mod csiz;
#[doc = "CSIZCLR register accessor: an alias for `Reg<CSIZCLR_SPEC>`"]
pub type CSIZCLR = crate::Reg<csizclr::CSIZCLR_SPEC>;
#[doc = "DCH0CSIZCLR register"]
pub mod csizclr;
#[doc = "CSIZSET register accessor: an alias for `Reg<CSIZSET_SPEC>`"]
pub type CSIZSET = crate::Reg<csizset::CSIZSET_SPEC>;
#[doc = "DCH0CSIZSET register"]
pub mod csizset;
#[doc = "CSIZINV register accessor: an alias for `Reg<CSIZINV_SPEC>`"]
pub type CSIZINV = crate::Reg<csizinv::CSIZINV_SPEC>;
#[doc = "DCH0CSIZINV register"]
pub mod csizinv;
#[doc = "CPTR register accessor: an alias for `Reg<CPTR_SPEC>`"]
pub type CPTR = crate::Reg<cptr::CPTR_SPEC>;
#[doc = "DCH0CPTR register"]
pub mod cptr;
#[doc = "CPTRCLR register accessor: an alias for `Reg<CPTRCLR_SPEC>`"]
pub type CPTRCLR = crate::Reg<cptrclr::CPTRCLR_SPEC>;
#[doc = "DCH0CPTRCLR register"]
pub mod cptrclr;
#[doc = "CPTRSET register accessor: an alias for `Reg<CPTRSET_SPEC>`"]
pub type CPTRSET = crate::Reg<cptrset::CPTRSET_SPEC>;
#[doc = "DCH0CPTRSET register"]
pub mod cptrset;
#[doc = "CPTRINV register accessor: an alias for `Reg<CPTRINV_SPEC>`"]
pub type CPTRINV = crate::Reg<cptrinv::CPTRINV_SPEC>;
#[doc = "DCH0CPTRINV register"]
pub mod cptrinv;
#[doc = "DAT register accessor: an alias for `Reg<DAT_SPEC>`"]
pub type DAT = crate::Reg<dat::DAT_SPEC>;
#[doc = "DCH0DAT register"]
pub mod dat;
#[doc = "DATCLR register accessor: an alias for `Reg<DATCLR_SPEC>`"]
pub type DATCLR = crate::Reg<datclr::DATCLR_SPEC>;
#[doc = "DCH0DATCLR register"]
pub mod datclr;
#[doc = "DATSET register accessor: an alias for `Reg<DATSET_SPEC>`"]
pub type DATSET = crate::Reg<datset::DATSET_SPEC>;
#[doc = "DCH0DATSET register"]
pub mod datset;
#[doc = "DATINV register accessor: an alias for `Reg<DATINV_SPEC>`"]
pub type DATINV = crate::Reg<datinv::DATINV_SPEC>;
#[doc = "DCH0DATINV register"]
pub mod datinv;
