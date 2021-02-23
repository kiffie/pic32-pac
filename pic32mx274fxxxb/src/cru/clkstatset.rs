#[doc = "Register `CLKSTATSET` reader"]
pub struct R(crate::R<CLKSTATSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKSTATSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLKSTATSET_SPEC>> for R {
    fn from(reader: crate::R<CLKSTATSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKSTATSET` writer"]
pub struct W(crate::W<CLKSTATSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKSTATSET_SPEC>;
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
impl core::convert::From<crate::W<CLKSTATSET_SPEC>> for W {
    fn from(writer: crate::W<CLKSTATSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRCRDY` reader - "]
pub struct FRCRDY_R(crate::FieldReader<bool, bool>);
impl FRCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRCRDY` writer - "]
pub struct FRCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCRDY_W<'a> {
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
#[doc = "Field `SPDIVRDY` reader - "]
pub struct SPDIVRDY_R(crate::FieldReader<bool, bool>);
impl SPDIVRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPDIVRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPDIVRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPDIVRDY` writer - "]
pub struct SPDIVRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIVRDY_W<'a> {
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
#[doc = "Field `POSCRDY` reader - "]
pub struct POSCRDY_R(crate::FieldReader<bool, bool>);
impl POSCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        POSCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POSCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSCRDY` writer - "]
pub struct POSCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> POSCRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `LPRCRDY` reader - "]
pub struct LPRCRDY_R(crate::FieldReader<bool, bool>);
impl LPRCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPRCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPRCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPRCRDY` writer - "]
pub struct LPRCRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRCRDY_W<'a> {
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
#[doc = "Field `SPLLRDY` reader - "]
pub struct SPLLRDY_R(crate::FieldReader<bool, bool>);
impl SPLLRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPLLRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPLLRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPLLRDY` writer - "]
pub struct SPLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLRDY_W<'a> {
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
#[doc = "Field `UPLLRDY` reader - "]
pub struct UPLLRDY_R(crate::FieldReader<bool, bool>);
impl UPLLRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPLLRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPLLRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPLLRDY` writer - "]
pub struct UPLLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLLRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frcrdy(&self) -> FRCRDY_R {
        FRCRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spdivrdy(&self) -> SPDIVRDY_R {
        SPDIVRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn poscrdy(&self) -> POSCRDY_R {
        POSCRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soscrdy(&self) -> SOSCRDY_R {
        SOSCRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lprcrdy(&self) -> LPRCRDY_R {
        LPRCRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spllrdy(&self) -> SPLLRDY_R {
        SPLLRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn upllrdy(&self) -> UPLLRDY_R {
        UPLLRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn frcrdy(&mut self) -> FRCRDY_W {
        FRCRDY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spdivrdy(&mut self) -> SPDIVRDY_W {
        SPDIVRDY_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn poscrdy(&mut self) -> POSCRDY_W {
        POSCRDY_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn soscrdy(&mut self) -> SOSCRDY_W {
        SOSCRDY_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lprcrdy(&mut self) -> LPRCRDY_W {
        LPRCRDY_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn spllrdy(&mut self) -> SPLLRDY_W {
        SPLLRDY_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn upllrdy(&mut self) -> UPLLRDY_W {
        UPLLRDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKSTATSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkstatset](index.html) module"]
pub struct CLKSTATSET_SPEC;
impl crate::RegisterSpec for CLKSTATSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkstatset::R](R) reader structure"]
impl crate::Readable for CLKSTATSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkstatset::W](W) writer structure"]
impl crate::Writable for CLKSTATSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKSTATSET to value 0"]
impl crate::Resettable for CLKSTATSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
