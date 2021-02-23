#[doc = "Register `AD1CHSINV` reader"]
pub struct R(crate::R<AD1CHSINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AD1CHSINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AD1CHSINV_SPEC>> for R {
    fn from(reader: crate::R<AD1CHSINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AD1CHSINV` writer"]
pub struct W(crate::W<AD1CHSINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AD1CHSINV_SPEC>;
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
impl core::convert::From<crate::W<AD1CHSINV_SPEC>> for W {
    fn from(writer: crate::W<AD1CHSINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0SA` reader - "]
pub struct CH0SA_R(crate::FieldReader<u8, u8>);
impl CH0SA_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH0SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0SA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0SA` writer - "]
pub struct CH0SA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `CH0NA` reader - "]
pub struct CH0NA_R(crate::FieldReader<bool, bool>);
impl CH0NA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0NA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0NA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0NA` writer - "]
pub struct CH0NA_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0NA_W<'a> {
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
#[doc = "Field `CH0SB` reader - "]
pub struct CH0SB_R(crate::FieldReader<u8, u8>);
impl CH0SB_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH0SB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0SB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0SB` writer - "]
pub struct CH0SB_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `CH0NB` reader - "]
pub struct CH0NB_R(crate::FieldReader<bool, bool>);
impl CH0NB_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0NB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0NB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0NB` writer - "]
pub struct CH0NB_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0NB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ch0sa(&self) -> CH0SA_R {
        CH0SA_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ch0na(&self) -> CH0NA_R {
        CH0NA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn ch0sb(&self) -> CH0SB_R {
        CH0SB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0nb(&self) -> CH0NB_R {
        CH0NB_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ch0sa(&mut self) -> CH0SA_W {
        CH0SA_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ch0na(&mut self) -> CH0NA_W {
        CH0NA_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn ch0sb(&mut self) -> CH0SB_W {
        CH0SB_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ch0nb(&mut self) -> CH0NB_W {
        CH0NB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AD1CHSINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1chsinv](index.html) module"]
pub struct AD1CHSINV_SPEC;
impl crate::RegisterSpec for AD1CHSINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ad1chsinv::R](R) reader structure"]
impl crate::Readable for AD1CHSINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ad1chsinv::W](W) writer structure"]
impl crate::Writable for AD1CHSINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AD1CHSINV to value 0"]
impl crate::Resettable for AD1CHSINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
