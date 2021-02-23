#[doc = "Register `PMD1` reader"]
pub struct R(crate::R<PMD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMD1_SPEC>> for R {
    fn from(reader: crate::R<PMD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMD1` writer"]
pub struct W(crate::W<PMD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMD1_SPEC>;
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
impl core::convert::From<crate::W<PMD1_SPEC>> for W {
    fn from(writer: crate::W<PMD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1MD` reader - "]
pub struct AD1MD_R(crate::FieldReader<bool, bool>);
impl AD1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        AD1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1MD` writer - "]
pub struct AD1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1MD_W<'a> {
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
#[doc = "Field `CTMUMD` reader - "]
pub struct CTMUMD_R(crate::FieldReader<bool, bool>);
impl CTMUMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTMUMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTMUMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTMUMD` writer - "]
pub struct CTMUMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUMD_W<'a> {
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
#[doc = "Field `CVRMD` reader - "]
pub struct CVRMD_R(crate::FieldReader<bool, bool>);
impl CVRMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CVRMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVRMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVRMD` writer - "]
pub struct CVRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CVRMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ad1md(&self) -> AD1MD_R {
        AD1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ctmumd(&self) -> CTMUMD_R {
        CTMUMD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cvrmd(&self) -> CVRMD_R {
        CVRMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ad1md(&mut self) -> AD1MD_W {
        AD1MD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ctmumd(&mut self) -> CTMUMD_W {
        CTMUMD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cvrmd(&mut self) -> CVRMD_W {
        CVRMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMD1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd1](index.html) module"]
pub struct PMD1_SPEC;
impl crate::RegisterSpec for PMD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmd1::R](R) reader structure"]
impl crate::Readable for PMD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmd1::W](W) writer structure"]
impl crate::Writable for PMD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMD1 to value 0"]
impl crate::Resettable for PMD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
