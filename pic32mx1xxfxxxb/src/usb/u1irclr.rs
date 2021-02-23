#[doc = "Register `U1IRCLR` reader"]
pub struct R(crate::R<U1IRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1IRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1IRCLR_SPEC>> for R {
    fn from(reader: crate::R<U1IRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1IRCLR` writer"]
pub struct W(crate::W<U1IRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1IRCLR_SPEC>;
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
impl core::convert::From<crate::W<U1IRCLR_SPEC>> for W {
    fn from(writer: crate::W<U1IRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `URSTIF_DETACHIF` reader - "]
pub struct URSTIF_DETACHIF_R(crate::FieldReader<bool, bool>);
impl URSTIF_DETACHIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        URSTIF_DETACHIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URSTIF_DETACHIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URSTIF_DETACHIF` writer - "]
pub struct URSTIF_DETACHIF_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTIF_DETACHIF_W<'a> {
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
#[doc = "Field `UERRIF` reader - "]
pub struct UERRIF_R(crate::FieldReader<bool, bool>);
impl UERRIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UERRIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UERRIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UERRIF` writer - "]
pub struct UERRIF_W<'a> {
    w: &'a mut W,
}
impl<'a> UERRIF_W<'a> {
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
#[doc = "Field `SOFIF` reader - "]
pub struct SOFIF_R(crate::FieldReader<bool, bool>);
impl SOFIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFIF` writer - "]
pub struct SOFIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIF_W<'a> {
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
#[doc = "Field `TRNIF` reader - "]
pub struct TRNIF_R(crate::FieldReader<bool, bool>);
impl TRNIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRNIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNIF` writer - "]
pub struct TRNIF_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNIF_W<'a> {
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
#[doc = "Field `IDLEIF` reader - "]
pub struct IDLEIF_R(crate::FieldReader<bool, bool>);
impl IDLEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLEIF` writer - "]
pub struct IDLEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEIF_W<'a> {
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
#[doc = "Field `RESUMEIF` reader - "]
pub struct RESUMEIF_R(crate::FieldReader<bool, bool>);
impl RESUMEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUMEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUMEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUMEIF` writer - "]
pub struct RESUMEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIF_W<'a> {
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
#[doc = "Field `ATTACHIF` reader - "]
pub struct ATTACHIF_R(crate::FieldReader<bool, bool>);
impl ATTACHIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATTACHIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTACHIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTACHIF` writer - "]
pub struct ATTACHIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTACHIF_W<'a> {
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
#[doc = "Field `STALLIF` reader - "]
pub struct STALLIF_R(crate::FieldReader<bool, bool>);
impl STALLIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLIF` writer - "]
pub struct STALLIF_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLIF_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urstif_detachif(&self) -> URSTIF_DETACHIF_R {
        URSTIF_DETACHIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uerrif(&self) -> UERRIF_R {
        UERRIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sofif(&self) -> SOFIF_R {
        SOFIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trnif(&self) -> TRNIF_R {
        TRNIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idleif(&self) -> IDLEIF_R {
        IDLEIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn resumeif(&self) -> RESUMEIF_R {
        RESUMEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn attachif(&self) -> ATTACHIF_R {
        ATTACHIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stallif(&self) -> STALLIF_R {
        STALLIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urstif_detachif(&mut self) -> URSTIF_DETACHIF_W {
        URSTIF_DETACHIF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uerrif(&mut self) -> UERRIF_W {
        UERRIF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sofif(&mut self) -> SOFIF_W {
        SOFIF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trnif(&mut self) -> TRNIF_W {
        TRNIF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idleif(&mut self) -> IDLEIF_W {
        IDLEIF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn resumeif(&mut self) -> RESUMEIF_W {
        RESUMEIF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn attachif(&mut self) -> ATTACHIF_W {
        ATTACHIF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stallif(&mut self) -> STALLIF_W {
        STALLIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1IRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1irclr](index.html) module"]
pub struct U1IRCLR_SPEC;
impl crate::RegisterSpec for U1IRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1irclr::R](R) reader structure"]
impl crate::Readable for U1IRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1irclr::W](W) writer structure"]
impl crate::Writable for U1IRCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1IRCLR to value 0"]
impl crate::Resettable for U1IRCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
