#[doc = "Register `PMD2` reader"]
pub struct R(crate::R<PMD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMD2_SPEC>> for R {
    fn from(reader: crate::R<PMD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMD2` writer"]
pub struct W(crate::W<PMD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMD2_SPEC>;
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
impl core::convert::From<crate::W<PMD2_SPEC>> for W {
    fn from(writer: crate::W<PMD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1MD` reader - "]
pub struct CMP1MD_R(crate::FieldReader<bool, bool>);
impl CMP1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1MD` writer - "]
pub struct CMP1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1MD_W<'a> {
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
#[doc = "Field `CMP2MD` reader - "]
pub struct CMP2MD_R(crate::FieldReader<bool, bool>);
impl CMP2MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2MD` writer - "]
pub struct CMP2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2MD_W<'a> {
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
#[doc = "Field `CMP3MD` reader - "]
pub struct CMP3MD_R(crate::FieldReader<bool, bool>);
impl CMP3MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3MD` writer - "]
pub struct CMP3MD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3MD_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmp1md(&self) -> CMP1MD_R {
        CMP1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmp2md(&self) -> CMP2MD_R {
        CMP2MD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmp3md(&self) -> CMP3MD_R {
        CMP3MD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmp1md(&mut self) -> CMP1MD_W {
        CMP1MD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmp2md(&mut self) -> CMP2MD_W {
        CMP2MD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmp3md(&mut self) -> CMP3MD_W {
        CMP3MD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMD2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd2](index.html) module"]
pub struct PMD2_SPEC;
impl crate::RegisterSpec for PMD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmd2::R](R) reader structure"]
impl crate::Readable for PMD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmd2::W](W) writer structure"]
impl crate::Writable for PMD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMD2 to value 0"]
impl crate::Resettable for PMD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
