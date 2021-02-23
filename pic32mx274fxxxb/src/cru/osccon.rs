#[doc = "Register `OSCCON` reader"]
pub struct R(crate::R<OSCCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OSCCON_SPEC>> for R {
    fn from(reader: crate::R<OSCCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCON` writer"]
pub struct W(crate::W<OSCCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCON_SPEC>;
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
impl core::convert::From<crate::W<OSCCON_SPEC>> for W {
    fn from(writer: crate::W<OSCCON_SPEC>) -> Self {
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
#[doc = "Field `SLP2SPD` reader - "]
pub struct SLP2SPD_R(crate::FieldReader<bool, bool>);
impl SLP2SPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLP2SPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLP2SPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLP2SPD` writer - "]
pub struct SLP2SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLP2SPD_W<'a> {
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
#[doc = "Field `DRMEN` reader - "]
pub struct DRMEN_R(crate::FieldReader<bool, bool>);
impl DRMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRMEN` writer - "]
pub struct DRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DRMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
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
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slp2spd(&self) -> SLP2SPD_R {
        SLP2SPD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn drmen(&self) -> DRMEN_R {
        DRMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frcdiv(&self) -> FRCDIV_R {
        FRCDIV_R::new(((self.bits >> 24) & 0x07) as u8)
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
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slp2spd(&mut self) -> SLP2SPD_W {
        SLP2SPD_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn drmen(&mut self) -> DRMEN_W {
        DRMEN_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frcdiv(&mut self) -> FRCDIV_W {
        FRCDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSCCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osccon](index.html) module"]
pub struct OSCCON_SPEC;
impl crate::RegisterSpec for OSCCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osccon::R](R) reader structure"]
impl crate::Readable for OSCCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osccon::W](W) writer structure"]
impl crate::Writable for OSCCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCCON to value 0"]
impl crate::Resettable for OSCCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
