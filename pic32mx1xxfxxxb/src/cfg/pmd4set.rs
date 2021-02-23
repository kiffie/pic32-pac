#[doc = "Register `PMD4SET` reader"]
pub struct R(crate::R<PMD4SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMD4SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMD4SET_SPEC>> for R {
    fn from(reader: crate::R<PMD4SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMD4SET` writer"]
pub struct W(crate::W<PMD4SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMD4SET_SPEC>;
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
impl core::convert::From<crate::W<PMD4SET_SPEC>> for W {
    fn from(writer: crate::W<PMD4SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T1MD` reader - "]
pub struct T1MD_R(crate::FieldReader<bool, bool>);
impl T1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        T1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1MD` writer - "]
pub struct T1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T1MD_W<'a> {
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
#[doc = "Field `T2MD` reader - "]
pub struct T2MD_R(crate::FieldReader<bool, bool>);
impl T2MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        T2MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T2MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T2MD` writer - "]
pub struct T2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T2MD_W<'a> {
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
#[doc = "Field `T3MD` reader - "]
pub struct T3MD_R(crate::FieldReader<bool, bool>);
impl T3MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        T3MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T3MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T3MD` writer - "]
pub struct T3MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T3MD_W<'a> {
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
#[doc = "Field `T4MD` reader - "]
pub struct T4MD_R(crate::FieldReader<bool, bool>);
impl T4MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        T4MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T4MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T4MD` writer - "]
pub struct T4MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T4MD_W<'a> {
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
#[doc = "Field `T5MD` reader - "]
pub struct T5MD_R(crate::FieldReader<bool, bool>);
impl T5MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        T5MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T5MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T5MD` writer - "]
pub struct T5MD_W<'a> {
    w: &'a mut W,
}
impl<'a> T5MD_W<'a> {
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
    pub fn t1md(&self) -> T1MD_R {
        T1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn t2md(&self) -> T2MD_R {
        T2MD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn t3md(&self) -> T3MD_R {
        T3MD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn t4md(&self) -> T4MD_R {
        T4MD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t5md(&self) -> T5MD_R {
        T5MD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn t1md(&mut self) -> T1MD_W {
        T1MD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn t2md(&mut self) -> T2MD_W {
        T2MD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn t3md(&mut self) -> T3MD_W {
        T3MD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn t4md(&mut self) -> T4MD_W {
        T4MD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn t5md(&mut self) -> T5MD_W {
        T5MD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMD4SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd4set](index.html) module"]
pub struct PMD4SET_SPEC;
impl crate::RegisterSpec for PMD4SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmd4set::R](R) reader structure"]
impl crate::Readable for PMD4SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmd4set::W](W) writer structure"]
impl crate::Writable for PMD4SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMD4SET to value 0"]
impl crate::Resettable for PMD4SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
