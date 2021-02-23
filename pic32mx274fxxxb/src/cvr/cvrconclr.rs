#[doc = "Register `CVRCONCLR` reader"]
pub struct R(crate::R<CVRCONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVRCONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CVRCONCLR_SPEC>> for R {
    fn from(reader: crate::R<CVRCONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CVRCONCLR` writer"]
pub struct W(crate::W<CVRCONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CVRCONCLR_SPEC>;
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
impl core::convert::From<crate::W<CVRCONCLR_SPEC>> for W {
    fn from(writer: crate::W<CVRCONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVR` reader - "]
pub struct CVR_R(crate::FieldReader<u8, u8>);
impl CVR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVR` writer - "]
pub struct CVR_W<'a> {
    w: &'a mut W,
}
impl<'a> CVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Field `CVRSS` reader - "]
pub struct CVRSS_R(crate::FieldReader<bool, bool>);
impl CVRSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CVRSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVRSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVRSS` writer - "]
pub struct CVRSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CVRSS_W<'a> {
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
#[doc = "Field `CVRR` reader - "]
pub struct CVRR_R(crate::FieldReader<bool, bool>);
impl CVRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CVRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVRR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVRR` writer - "]
pub struct CVRR_W<'a> {
    w: &'a mut W,
}
impl<'a> CVRR_W<'a> {
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
#[doc = "Field `CVROE` reader - "]
pub struct CVROE_R(crate::FieldReader<bool, bool>);
impl CVROE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CVROE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVROE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVROE` writer - "]
pub struct CVROE_W<'a> {
    w: &'a mut W,
}
impl<'a> CVROE_W<'a> {
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
#[doc = "Field `ON` reader - "]
pub struct ON_R(crate::FieldReader<bool, bool>);
impl ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ON` writer - "]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cvr(&self) -> CVR_R {
        CVR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cvrss(&self) -> CVRSS_R {
        CVRSS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cvrr(&self) -> CVRR_R {
        CVRR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cvroe(&self) -> CVROE_R {
        CVROE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cvr(&mut self) -> CVR_W {
        CVR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cvrss(&mut self) -> CVRSS_W {
        CVRSS_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cvrr(&mut self) -> CVRR_W {
        CVRR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cvroe(&mut self) -> CVROE_W {
        CVROE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CVRCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvrconclr](index.html) module"]
pub struct CVRCONCLR_SPEC;
impl crate::RegisterSpec for CVRCONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cvrconclr::R](R) reader structure"]
impl crate::Readable for CVRCONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cvrconclr::W](W) writer structure"]
impl crate::Writable for CVRCONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CVRCONCLR to value 0"]
impl crate::Resettable for CVRCONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
