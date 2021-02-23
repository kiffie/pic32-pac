#[doc = "Register `OSCCONINV` reader"]
pub struct R(crate::R<OSCCONINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCONINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OSCCONINV_SPEC>> for R {
    fn from(reader: crate::R<OSCCONINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCONINV` writer"]
pub struct W(crate::W<OSCCONINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCONINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<OSCCONINV_SPEC>> for W {
    fn from(writer: crate::W<OSCCONINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSWEN` reader - "]
pub struct OSWEN_R(crate::FieldReader<bool, bool>);
impl OSWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSWEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSWEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSWEN` writer - "]
pub struct OSWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OSWEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Field `SOSCEN` reader - "]
pub struct SOSCEN_R(crate::FieldReader<bool, bool>);
impl SOSCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOSCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOSCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCEN` writer - "]
pub struct SOSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UFRCEN` reader - "]
pub struct UFRCEN_R(crate::FieldReader<bool, bool>);
impl UFRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UFRCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UFRCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UFRCEN` writer - "]
pub struct UFRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UFRCEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CF` reader - "]
pub struct CF_R(crate::FieldReader<bool, bool>);
impl CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF` writer - "]
pub struct CF_W<'a> {
    w: &'a mut W,
}
impl<'a> CF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SLPEN` reader - "]
pub struct SLPEN_R(crate::FieldReader<bool, bool>);
impl SLPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLPEN` writer - "]
pub struct SLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLPEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SLOCK` reader - "]
pub struct SLOCK_R(crate::FieldReader<bool, bool>);
impl SLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOCK` writer - "]
pub struct SLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ULOCK` reader - "]
pub struct ULOCK_R(crate::FieldReader<bool, bool>);
impl ULOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ULOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULOCK` writer - "]
pub struct ULOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ULOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CLKLOCK` reader - "]
pub struct CLKLOCK_R(crate::FieldReader<bool, bool>);
impl CLKLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKLOCK` writer - "]
pub struct CLKLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKLOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `NOSC` reader - "]
pub struct NOSC_R(crate::FieldReader<u8, u8>);
impl NOSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        NOSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOSC` writer - "]
pub struct NOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> NOSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `COSC` reader - "]
pub struct COSC_R(crate::FieldReader<u8, u8>);
impl COSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        COSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COSC` writer - "]
pub struct COSC_W<'a> {
    w: &'a mut W,
}
impl<'a> COSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `PLLMULT` reader - "]
pub struct PLLMULT_R(crate::FieldReader<u8, u8>);
impl PLLMULT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLMULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLMULT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLMULT` writer - "]
pub struct PLLMULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `PBDIV` reader - "]
pub struct PBDIV_R(crate::FieldReader<u8, u8>);
impl PBDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBDIV` writer - "]
pub struct PBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Field `PBDIVRDY` reader - "]
pub struct PBDIVRDY_R(crate::FieldReader<bool, bool>);
impl PBDIVRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBDIVRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBDIVRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBDIVRDY` writer - "]
pub struct PBDIVRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> PBDIVRDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `SOSCRDY` reader - "]
pub struct SOSCRDY_R(crate::FieldReader<bool, bool>);
impl SOSCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOSCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOSCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOSCRDY` writer - "]
pub struct SOSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCRDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `FRCDIV` reader - "]
pub struct FRCDIV_R(crate::FieldReader<u8, u8>);
impl FRCDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRCDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRCDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRCDIV` writer - "]
pub struct FRCDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `PLLODIV` reader - "]
pub struct PLLODIV_R(crate::FieldReader<u8, u8>);
impl PLLODIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLODIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLODIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLODIV` writer - "]
pub struct PLLODIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLODIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn oswen(&self) -> OSWEN_R {
        OSWEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn soscen(&self) -> SOSCEN_R {
        SOSCEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ufrcen(&self) -> UFRCEN_R {
        UFRCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slpen(&self) -> SLPEN_R {
        SLPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slock(&self) -> SLOCK_R {
        SLOCK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ulock(&self) -> ULOCK_R {
        ULOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clklock(&self) -> CLKLOCK_R {
        CLKLOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn nosc(&self) -> NOSC_R {
        NOSC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn cosc(&self) -> COSC_R {
        COSC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pllmult(&self) -> PLLMULT_R {
        PLLMULT_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pbdivrdy(&self) -> PBDIVRDY_R {
        PBDIVRDY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn soscrdy(&self) -> SOSCRDY_R {
        SOSCRDY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frcdiv(&self) -> FRCDIV_R {
        FRCDIV_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn pllodiv(&self) -> PLLODIV_R {
        PLLODIV_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn oswen(&mut self) -> OSWEN_W {
        OSWEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn soscen(&mut self) -> SOSCEN_W {
        SOSCEN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ufrcen(&mut self) -> UFRCEN_W {
        UFRCEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slpen(&mut self) -> SLPEN_W {
        SLPEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slock(&mut self) -> SLOCK_W {
        SLOCK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ulock(&mut self) -> ULOCK_W {
        ULOCK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clklock(&mut self) -> CLKLOCK_W {
        CLKLOCK_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn nosc(&mut self) -> NOSC_W {
        NOSC_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn cosc(&mut self) -> COSC_W {
        COSC_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pllmult(&mut self) -> PLLMULT_W {
        PLLMULT_W { w: self }
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn pbdiv(&mut self) -> PBDIV_W {
        PBDIV_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pbdivrdy(&mut self) -> PBDIVRDY_W {
        PBDIVRDY_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn soscrdy(&mut self) -> SOSCRDY_W {
        SOSCRDY_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frcdiv(&mut self) -> FRCDIV_W {
        FRCDIV_W { w: self }
    }
    #[doc = "Bits 27:29"]
    #[inline(always)]
    pub fn pllodiv(&mut self) -> PLLODIV_W {
        PLLODIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSCCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscconinv](index.html) module"]
pub struct OSCCONINV_SPEC;
impl crate::RegisterSpec for OSCCONINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscconinv::R](R) reader structure"]
impl crate::Readable for OSCCONINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscconinv::W](W) writer structure"]
impl crate::Writable for OSCCONINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCCONINV to value 0"]
impl crate::Resettable for OSCCONINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
