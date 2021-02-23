#[doc = "Register `TRIS` reader"]
pub struct R(crate::R<TRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TRIS_SPEC>> for R {
    fn from(reader: crate::R<TRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIS` writer"]
pub struct W(crate::W<TRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIS_SPEC>;
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
impl core::convert::From<crate::W<TRIS_SPEC>> for W {
    fn from(writer: crate::W<TRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRISA0` reader - "]
pub struct TRISA0_R(crate::FieldReader<bool, bool>);
impl TRISA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISA0` writer - "]
pub struct TRISA0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA0_W<'a> {
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
#[doc = "Field `TRISA1` reader - "]
pub struct TRISA1_R(crate::FieldReader<bool, bool>);
impl TRISA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISA1` writer - "]
pub struct TRISA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA1_W<'a> {
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
#[doc = "Field `TRISA2` reader - "]
pub struct TRISA2_R(crate::FieldReader<bool, bool>);
impl TRISA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISA2` writer - "]
pub struct TRISA2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA2_W<'a> {
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
#[doc = "Field `TRISA3` reader - "]
pub struct TRISA3_R(crate::FieldReader<bool, bool>);
impl TRISA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISA3` writer - "]
pub struct TRISA3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA3_W<'a> {
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
#[doc = "Field `TRISA4` reader - "]
pub struct TRISA4_R(crate::FieldReader<bool, bool>);
impl TRISA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISA4` writer - "]
pub struct TRISA4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISA4_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trisa0(&self) -> TRISA0_R {
        TRISA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trisa1(&self) -> TRISA1_R {
        TRISA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trisa2(&self) -> TRISA2_R {
        TRISA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trisa3(&self) -> TRISA3_R {
        TRISA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trisa4(&self) -> TRISA4_R {
        TRISA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trisa0(&mut self) -> TRISA0_W {
        TRISA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trisa1(&mut self) -> TRISA1_W {
        TRISA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trisa2(&mut self) -> TRISA2_W {
        TRISA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trisa3(&mut self) -> TRISA3_W {
        TRISA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trisa4(&mut self) -> TRISA4_W {
        TRISA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRISA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tris](index.html) module"]
pub struct TRIS_SPEC;
impl crate::RegisterSpec for TRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tris::R](R) reader structure"]
impl crate::Readable for TRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tris::W](W) writer structure"]
impl crate::Writable for TRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIS to value 0x1f"]
impl crate::Resettable for TRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
