#![doc = "Peripheral access API for PIC32MX274F256B microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "DSCTRL peripheral"]
pub struct DSCTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DSCTRL {}
impl DSCTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dsctrl::RegisterBlock {
        0xbf80_0000 as *const _
    }
}
impl Deref for DSCTRL {
    type Target = dsctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DSCTRL::ptr() }
    }
}
#[doc = "DSCTRL peripheral"]
pub mod dsctrl;
#[doc = "RTCC peripheral"]
pub struct RTCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCC {}
impl RTCC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtcc::RegisterBlock {
        0xbf80_0200 as *const _
    }
}
impl Deref for RTCC {
    type Target = rtcc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTCC::ptr() }
    }
}
#[doc = "RTCC peripheral"]
pub mod rtcc;
#[doc = "TMR1 peripheral"]
pub struct TMR1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR1 {}
impl TMR1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr1::RegisterBlock {
        0xbf80_0600 as *const _
    }
}
impl Deref for TMR1 {
    type Target = tmr1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR1::ptr() }
    }
}
#[doc = "TMR1 peripheral"]
pub mod tmr1;
#[doc = "TMR2 peripheral"]
pub struct TMR2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR2 {}
impl TMR2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr2::RegisterBlock {
        0xbf80_0800 as *const _
    }
}
impl Deref for TMR2 {
    type Target = tmr2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR2::ptr() }
    }
}
#[doc = "TMR2 peripheral"]
pub mod tmr2;
#[doc = "TMR3 peripheral"]
pub struct TMR3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR3 {}
impl TMR3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr3::RegisterBlock {
        0xbf80_0a00 as *const _
    }
}
impl Deref for TMR3 {
    type Target = tmr3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR3::ptr() }
    }
}
#[doc = "TMR3 peripheral"]
pub mod tmr3;
#[doc = "TMR4 peripheral"]
pub struct TMR4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR4 {}
impl TMR4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr4::RegisterBlock {
        0xbf80_0c00 as *const _
    }
}
impl Deref for TMR4 {
    type Target = tmr4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR4::ptr() }
    }
}
#[doc = "TMR4 peripheral"]
pub mod tmr4;
#[doc = "TMR5 peripheral"]
pub struct TMR5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMR5 {}
impl TMR5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tmr5::RegisterBlock {
        0xbf80_0e00 as *const _
    }
}
impl Deref for TMR5 {
    type Target = tmr5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TMR5::ptr() }
    }
}
#[doc = "TMR5 peripheral"]
pub mod tmr5;
#[doc = "ICAP1 peripheral"]
pub struct ICAP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICAP1 {}
impl ICAP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icap1::RegisterBlock {
        0xbf80_2000 as *const _
    }
}
impl Deref for ICAP1 {
    type Target = icap1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICAP1::ptr() }
    }
}
#[doc = "ICAP1 peripheral"]
pub mod icap1;
#[doc = "ICAP2 peripheral"]
pub struct ICAP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICAP2 {}
impl ICAP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icap2::RegisterBlock {
        0xbf80_2200 as *const _
    }
}
impl Deref for ICAP2 {
    type Target = icap2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICAP2::ptr() }
    }
}
#[doc = "ICAP2 peripheral"]
pub mod icap2;
#[doc = "ICAP3 peripheral"]
pub struct ICAP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICAP3 {}
impl ICAP3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icap3::RegisterBlock {
        0xbf80_2400 as *const _
    }
}
impl Deref for ICAP3 {
    type Target = icap3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICAP3::ptr() }
    }
}
#[doc = "ICAP3 peripheral"]
pub mod icap3;
#[doc = "ICAP4 peripheral"]
pub struct ICAP4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICAP4 {}
impl ICAP4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icap4::RegisterBlock {
        0xbf80_2600 as *const _
    }
}
impl Deref for ICAP4 {
    type Target = icap4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICAP4::ptr() }
    }
}
#[doc = "ICAP4 peripheral"]
pub mod icap4;
#[doc = "ICAP5 peripheral"]
pub struct ICAP5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICAP5 {}
impl ICAP5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icap5::RegisterBlock {
        0xbf80_2800 as *const _
    }
}
impl Deref for ICAP5 {
    type Target = icap5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICAP5::ptr() }
    }
}
#[doc = "ICAP5 peripheral"]
pub mod icap5;
#[doc = "OCMP1 peripheral"]
pub struct OCMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCMP1 {}
impl OCMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ocmp1::RegisterBlock {
        0xbf80_3000 as *const _
    }
}
impl Deref for OCMP1 {
    type Target = ocmp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCMP1::ptr() }
    }
}
#[doc = "OCMP1 peripheral"]
pub mod ocmp1;
#[doc = "OCMP2 peripheral"]
pub struct OCMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCMP2 {}
impl OCMP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ocmp2::RegisterBlock {
        0xbf80_3200 as *const _
    }
}
impl Deref for OCMP2 {
    type Target = ocmp2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCMP2::ptr() }
    }
}
#[doc = "OCMP2 peripheral"]
pub mod ocmp2;
#[doc = "OCMP3 peripheral"]
pub struct OCMP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCMP3 {}
impl OCMP3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ocmp3::RegisterBlock {
        0xbf80_3400 as *const _
    }
}
impl Deref for OCMP3 {
    type Target = ocmp3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCMP3::ptr() }
    }
}
#[doc = "OCMP3 peripheral"]
pub mod ocmp3;
#[doc = "OCMP4 peripheral"]
pub struct OCMP4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCMP4 {}
impl OCMP4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ocmp4::RegisterBlock {
        0xbf80_3600 as *const _
    }
}
impl Deref for OCMP4 {
    type Target = ocmp4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCMP4::ptr() }
    }
}
#[doc = "OCMP4 peripheral"]
pub mod ocmp4;
#[doc = "OCMP5 peripheral"]
pub struct OCMP5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OCMP5 {}
impl OCMP5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ocmp5::RegisterBlock {
        0xbf80_3800 as *const _
    }
}
impl Deref for OCMP5 {
    type Target = ocmp5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OCMP5::ptr() }
    }
}
#[doc = "OCMP5 peripheral"]
pub mod ocmp5;
#[doc = "I2C1 peripheral"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0xbf80_5000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "I2C1 peripheral"]
pub mod i2c1;
#[doc = "I2C1 peripheral"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0xbf80_5100 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "SPI1 peripheral"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0xbf80_5800 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "SPI1 peripheral"]
pub mod spi1;
#[doc = "SPI1 peripheral"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        0xbf80_5a00 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "UART1 peripheral"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0xbf80_6000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART1 peripheral"]
pub mod uart1;
#[doc = "UART1 peripheral"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0xbf80_6200 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "PMP peripheral"]
pub struct PMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMP {}
impl PMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmp::RegisterBlock {
        0xbf80_7000 as *const _
    }
}
impl Deref for PMP {
    type Target = pmp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMP::ptr() }
    }
}
#[doc = "PMP peripheral"]
pub mod pmp;
#[doc = "ADC10 peripheral"]
pub struct ADC10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC10 {}
impl ADC10 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc10::RegisterBlock {
        0xbf80_9000 as *const _
    }
}
impl Deref for ADC10 {
    type Target = adc10::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC10::ptr() }
    }
}
#[doc = "ADC10 peripheral"]
pub mod adc10;
#[doc = "CVR peripheral"]
pub struct CVR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CVR {}
impl CVR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cvr::RegisterBlock {
        0xbf80_9800 as *const _
    }
}
impl Deref for CVR {
    type Target = cvr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CVR::ptr() }
    }
}
#[doc = "CVR peripheral"]
pub mod cvr;
#[doc = "CMP1 peripheral"]
pub struct CMP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP1 {}
impl CMP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp1::RegisterBlock {
        0xbf80_a000 as *const _
    }
}
impl Deref for CMP1 {
    type Target = cmp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP1::ptr() }
    }
}
#[doc = "CMP1 peripheral"]
pub mod cmp1;
#[doc = "CMP2 peripheral"]
pub struct CMP2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP2 {}
impl CMP2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp2::RegisterBlock {
        0xbf80_a010 as *const _
    }
}
impl Deref for CMP2 {
    type Target = cmp2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP2::ptr() }
    }
}
#[doc = "CMP2 peripheral"]
pub mod cmp2;
#[doc = "CMP3 peripheral"]
pub struct CMP3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP3 {}
impl CMP3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp3::RegisterBlock {
        0xbf80_a020 as *const _
    }
}
impl Deref for CMP3 {
    type Target = cmp3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP3::ptr() }
    }
}
#[doc = "CMP3 peripheral"]
pub mod cmp3;
#[doc = "CMP peripheral"]
pub struct CMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP {}
impl CMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp::RegisterBlock {
        0xbf80_a060 as *const _
    }
}
impl Deref for CMP {
    type Target = cmp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP::ptr() }
    }
}
#[doc = "CMP peripheral"]
pub mod cmp;
#[doc = "CTMU peripheral"]
pub struct CTMU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CTMU {}
impl CTMU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ctmu::RegisterBlock {
        0xbf80_a200 as *const _
    }
}
impl Deref for CTMU {
    type Target = ctmu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CTMU::ptr() }
    }
}
#[doc = "CTMU peripheral"]
pub mod ctmu;
#[doc = "CRU peripheral"]
pub struct CRU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRU {}
impl CRU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cru::RegisterBlock {
        0xbf80_f000 as *const _
    }
}
impl Deref for CRU {
    type Target = cru::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRU::ptr() }
    }
}
#[doc = "CRU peripheral"]
pub mod cru;
#[doc = "CFG peripheral"]
pub struct CFG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CFG {}
impl CFG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cfg::RegisterBlock {
        0xbf80_f200 as *const _
    }
}
impl Deref for CFG {
    type Target = cfg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CFG::ptr() }
    }
}
#[doc = "CFG peripheral"]
pub mod cfg;
#[doc = "NVM peripheral"]
pub struct NVM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVM {}
impl NVM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const nvm::RegisterBlock {
        0xbf80_f400 as *const _
    }
}
impl Deref for NVM {
    type Target = nvm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*NVM::ptr() }
    }
}
#[doc = "NVM peripheral"]
pub mod nvm;
#[doc = "WDT peripheral"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0xbf80_f600 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "WDT peripheral"]
pub mod wdt;
#[doc = "PPS peripheral"]
pub struct PPS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPS {}
impl PPS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pps::RegisterBlock {
        0xbf80_fa04 as *const _
    }
}
impl Deref for PPS {
    type Target = pps::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PPS::ptr() }
    }
}
#[doc = "PPS peripheral"]
pub mod pps;
#[doc = "HLVD peripheral"]
pub struct HLVD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HLVD {}
impl HLVD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hlvd::RegisterBlock {
        0xbf80_fc00 as *const _
    }
}
impl Deref for HLVD {
    type Target = hlvd::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HLVD::ptr() }
    }
}
#[doc = "HLVD peripheral"]
pub mod hlvd;
#[doc = "INT peripheral"]
pub struct INT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for INT {}
impl INT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const int::RegisterBlock {
        0xbf88_1000 as *const _
    }
}
impl Deref for INT {
    type Target = int::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*INT::ptr() }
    }
}
#[doc = "INT peripheral"]
pub mod int;
#[doc = "BMX peripheral"]
pub struct BMX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BMX {}
impl BMX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bmx::RegisterBlock {
        0xbf88_2000 as *const _
    }
}
impl Deref for BMX {
    type Target = bmx::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BMX::ptr() }
    }
}
#[doc = "BMX peripheral"]
pub mod bmx;
#[doc = "DMAC peripheral"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac::RegisterBlock {
        0xbf88_3000 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "DMAC peripheral"]
pub mod dmac;
#[doc = "DMAC0 peripheral"]
pub struct DMAC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC0 {}
impl DMAC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac0::RegisterBlock {
        0xbf88_3060 as *const _
    }
}
impl Deref for DMAC0 {
    type Target = dmac0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC0::ptr() }
    }
}
#[doc = "DMAC0 peripheral"]
pub mod dmac0;
#[doc = "DMAC1 peripheral"]
pub struct DMAC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC1 {}
impl DMAC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac1::RegisterBlock {
        0xbf88_3120 as *const _
    }
}
impl Deref for DMAC1 {
    type Target = dmac1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC1::ptr() }
    }
}
#[doc = "DMAC1 peripheral"]
pub mod dmac1;
#[doc = "DMAC2 peripheral"]
pub struct DMAC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC2 {}
impl DMAC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac2::RegisterBlock {
        0xbf88_31e0 as *const _
    }
}
impl Deref for DMAC2 {
    type Target = dmac2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC2::ptr() }
    }
}
#[doc = "DMAC2 peripheral"]
pub mod dmac2;
#[doc = "DMAC3 peripheral"]
pub struct DMAC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC3 {}
impl DMAC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dmac3::RegisterBlock {
        0xbf88_32a0 as *const _
    }
}
impl Deref for DMAC3 {
    type Target = dmac3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMAC3::ptr() }
    }
}
#[doc = "DMAC3 peripheral"]
pub mod dmac3;
#[doc = "PCACHE peripheral"]
pub struct PCACHE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PCACHE {}
impl PCACHE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pcache::RegisterBlock {
        0xbf88_4000 as *const _
    }
}
impl Deref for PCACHE {
    type Target = pcache::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PCACHE::ptr() }
    }
}
#[doc = "PCACHE peripheral"]
pub mod pcache;
#[doc = "USB peripheral"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0xbf88_5040 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB peripheral"]
pub mod usb;
#[doc = "PORTA peripheral"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porta::RegisterBlock {
        0xbf88_6000 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "PORTA peripheral"]
pub mod porta;
#[doc = "PORTB peripheral"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portb::RegisterBlock {
        0xbf88_6100 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "PORTB peripheral"]
pub mod portb;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "DSCTRL"]
    pub DSCTRL: DSCTRL,
    #[doc = "RTCC"]
    pub RTCC: RTCC,
    #[doc = "TMR1"]
    pub TMR1: TMR1,
    #[doc = "TMR2"]
    pub TMR2: TMR2,
    #[doc = "TMR3"]
    pub TMR3: TMR3,
    #[doc = "TMR4"]
    pub TMR4: TMR4,
    #[doc = "TMR5"]
    pub TMR5: TMR5,
    #[doc = "ICAP1"]
    pub ICAP1: ICAP1,
    #[doc = "ICAP2"]
    pub ICAP2: ICAP2,
    #[doc = "ICAP3"]
    pub ICAP3: ICAP3,
    #[doc = "ICAP4"]
    pub ICAP4: ICAP4,
    #[doc = "ICAP5"]
    pub ICAP5: ICAP5,
    #[doc = "OCMP1"]
    pub OCMP1: OCMP1,
    #[doc = "OCMP2"]
    pub OCMP2: OCMP2,
    #[doc = "OCMP3"]
    pub OCMP3: OCMP3,
    #[doc = "OCMP4"]
    pub OCMP4: OCMP4,
    #[doc = "OCMP5"]
    pub OCMP5: OCMP5,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "PMP"]
    pub PMP: PMP,
    #[doc = "ADC10"]
    pub ADC10: ADC10,
    #[doc = "CVR"]
    pub CVR: CVR,
    #[doc = "CMP1"]
    pub CMP1: CMP1,
    #[doc = "CMP2"]
    pub CMP2: CMP2,
    #[doc = "CMP3"]
    pub CMP3: CMP3,
    #[doc = "CMP"]
    pub CMP: CMP,
    #[doc = "CTMU"]
    pub CTMU: CTMU,
    #[doc = "CRU"]
    pub CRU: CRU,
    #[doc = "CFG"]
    pub CFG: CFG,
    #[doc = "NVM"]
    pub NVM: NVM,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "PPS"]
    pub PPS: PPS,
    #[doc = "HLVD"]
    pub HLVD: HLVD,
    #[doc = "INT"]
    pub INT: INT,
    #[doc = "BMX"]
    pub BMX: BMX,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DMAC0"]
    pub DMAC0: DMAC0,
    #[doc = "DMAC1"]
    pub DMAC1: DMAC1,
    #[doc = "DMAC2"]
    pub DMAC2: DMAC2,
    #[doc = "DMAC3"]
    pub DMAC3: DMAC3,
    #[doc = "PCACHE"]
    pub PCACHE: PCACHE,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
}
impl Peripherals {
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            DSCTRL: DSCTRL {
                _marker: PhantomData,
            },
            RTCC: RTCC {
                _marker: PhantomData,
            },
            TMR1: TMR1 {
                _marker: PhantomData,
            },
            TMR2: TMR2 {
                _marker: PhantomData,
            },
            TMR3: TMR3 {
                _marker: PhantomData,
            },
            TMR4: TMR4 {
                _marker: PhantomData,
            },
            TMR5: TMR5 {
                _marker: PhantomData,
            },
            ICAP1: ICAP1 {
                _marker: PhantomData,
            },
            ICAP2: ICAP2 {
                _marker: PhantomData,
            },
            ICAP3: ICAP3 {
                _marker: PhantomData,
            },
            ICAP4: ICAP4 {
                _marker: PhantomData,
            },
            ICAP5: ICAP5 {
                _marker: PhantomData,
            },
            OCMP1: OCMP1 {
                _marker: PhantomData,
            },
            OCMP2: OCMP2 {
                _marker: PhantomData,
            },
            OCMP3: OCMP3 {
                _marker: PhantomData,
            },
            OCMP4: OCMP4 {
                _marker: PhantomData,
            },
            OCMP5: OCMP5 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            PMP: PMP {
                _marker: PhantomData,
            },
            ADC10: ADC10 {
                _marker: PhantomData,
            },
            CVR: CVR {
                _marker: PhantomData,
            },
            CMP1: CMP1 {
                _marker: PhantomData,
            },
            CMP2: CMP2 {
                _marker: PhantomData,
            },
            CMP3: CMP3 {
                _marker: PhantomData,
            },
            CMP: CMP {
                _marker: PhantomData,
            },
            CTMU: CTMU {
                _marker: PhantomData,
            },
            CRU: CRU {
                _marker: PhantomData,
            },
            CFG: CFG {
                _marker: PhantomData,
            },
            NVM: NVM {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            PPS: PPS {
                _marker: PhantomData,
            },
            HLVD: HLVD {
                _marker: PhantomData,
            },
            INT: INT {
                _marker: PhantomData,
            },
            BMX: BMX {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DMAC0: DMAC0 {
                _marker: PhantomData,
            },
            DMAC1: DMAC1 {
                _marker: PhantomData,
            },
            DMAC2: DMAC2 {
                _marker: PhantomData,
            },
            DMAC3: DMAC3 {
                _marker: PhantomData,
            },
            PCACHE: PCACHE {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTB: PORTB {
                _marker: PhantomData,
            },
        }
    }
}
