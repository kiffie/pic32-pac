#[doc = "Register `PMD6SET` reader"]
pub struct R(crate::R<PMD6SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMD6SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMD6SET_SPEC>> for R {
    fn from(reader: crate::R<PMD6SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMD6SET` writer"]
pub struct W(crate::W<PMD6SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMD6SET_SPEC>;
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
impl core::convert::From<crate::W<PMD6SET_SPEC>> for W {
    fn from(writer: crate::W<PMD6SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCCMD` reader - "]
pub struct RTCCMD_R(crate::FieldReader<bool, bool>);
impl RTCCMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCMD` writer - "]
pub struct RTCCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCMD_W<'a> {
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
#[doc = "Field `REFOMD` reader - "]
pub struct REFOMD_R(crate::FieldReader<bool, bool>);
impl REFOMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFOMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFOMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFOMD` writer - "]
pub struct REFOMD_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOMD_W<'a> {
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
#[doc = "Field `PMPMD` reader - "]
pub struct PMPMD_R(crate::FieldReader<bool, bool>);
impl PMPMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMPMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMPMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMPMD` writer - "]
pub struct PMPMD_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtccmd(&self) -> RTCCMD_R {
        RTCCMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn refomd(&self) -> REFOMD_R {
        REFOMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pmpmd(&self) -> PMPMD_R {
        PMPMD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtccmd(&mut self) -> RTCCMD_W {
        RTCCMD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn refomd(&mut self) -> REFOMD_W {
        REFOMD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pmpmd(&mut self) -> PMPMD_W {
        PMPMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMD6SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd6set](index.html) module"]
pub struct PMD6SET_SPEC;
impl crate::RegisterSpec for PMD6SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmd6set::R](R) reader structure"]
impl crate::Readable for PMD6SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmd6set::W](W) writer structure"]
impl crate::Writable for PMD6SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMD6SET to value 0"]
impl crate::Resettable for PMD6SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
