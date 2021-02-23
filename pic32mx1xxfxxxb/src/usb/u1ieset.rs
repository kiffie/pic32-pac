#[doc = "Register `U1IESET` reader"]
pub struct R(crate::R<U1IESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1IESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1IESET_SPEC>> for R {
    fn from(reader: crate::R<U1IESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1IESET` writer"]
pub struct W(crate::W<U1IESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1IESET_SPEC>;
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
impl core::convert::From<crate::W<U1IESET_SPEC>> for W {
    fn from(writer: crate::W<U1IESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `URSTIE_DETACHIE` reader - "]
pub struct URSTIE_DETACHIE_R(crate::FieldReader<bool, bool>);
impl URSTIE_DETACHIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        URSTIE_DETACHIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URSTIE_DETACHIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URSTIE_DETACHIE` writer - "]
pub struct URSTIE_DETACHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTIE_DETACHIE_W<'a> {
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
#[doc = "Field `UERRIE` reader - "]
pub struct UERRIE_R(crate::FieldReader<bool, bool>);
impl UERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UERRIE` writer - "]
pub struct UERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UERRIE_W<'a> {
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
#[doc = "Field `SOFIE` reader - "]
pub struct SOFIE_R(crate::FieldReader<bool, bool>);
impl SOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFIE` writer - "]
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
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
#[doc = "Field `TRNIE` reader - "]
pub struct TRNIE_R(crate::FieldReader<bool, bool>);
impl TRNIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRNIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNIE` writer - "]
pub struct TRNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRNIE_W<'a> {
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
#[doc = "Field `IDLEIE` reader - "]
pub struct IDLEIE_R(crate::FieldReader<bool, bool>);
impl IDLEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLEIE` writer - "]
pub struct IDLEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLEIE_W<'a> {
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
#[doc = "Field `RESUMEIE` reader - "]
pub struct RESUMEIE_R(crate::FieldReader<bool, bool>);
impl RESUMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUMEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUMEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUMEIE` writer - "]
pub struct RESUMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIE_W<'a> {
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
#[doc = "Field `ATTACHIE` reader - "]
pub struct ATTACHIE_R(crate::FieldReader<bool, bool>);
impl ATTACHIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATTACHIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATTACHIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATTACHIE` writer - "]
pub struct ATTACHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATTACHIE_W<'a> {
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
#[doc = "Field `STALLIE` reader - "]
pub struct STALLIE_R(crate::FieldReader<bool, bool>);
impl STALLIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLIE` writer - "]
pub struct STALLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLIE_W<'a> {
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
    pub fn urstie_detachie(&self) -> URSTIE_DETACHIE_R {
        URSTIE_DETACHIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uerrie(&self) -> UERRIE_R {
        UERRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trnie(&self) -> TRNIE_R {
        TRNIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn resumeie(&self) -> RESUMEIE_R {
        RESUMEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn attachie(&self) -> ATTACHIE_R {
        ATTACHIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stallie(&self) -> STALLIE_R {
        STALLIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urstie_detachie(&mut self) -> URSTIE_DETACHIE_W {
        URSTIE_DETACHIE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uerrie(&mut self) -> UERRIE_W {
        UERRIE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trnie(&mut self) -> TRNIE_W {
        TRNIE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idleie(&mut self) -> IDLEIE_W {
        IDLEIE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn resumeie(&mut self) -> RESUMEIE_W {
        RESUMEIE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn attachie(&mut self) -> ATTACHIE_W {
        ATTACHIE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn stallie(&mut self) -> STALLIE_W {
        STALLIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1IESET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ieset](index.html) module"]
pub struct U1IESET_SPEC;
impl crate::RegisterSpec for U1IESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1ieset::R](R) reader structure"]
impl crate::Readable for U1IESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1ieset::W](W) writer structure"]
impl crate::Writable for U1IESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1IESET to value 0"]
impl crate::Resettable for U1IESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
