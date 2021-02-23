#[doc = "Register `CHECONCLR` reader"]
pub struct R(crate::R<CHECONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHECONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHECONCLR_SPEC>> for R {
    fn from(reader: crate::R<CHECONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHECONCLR` writer"]
pub struct W(crate::W<CHECONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHECONCLR_SPEC>;
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
impl core::convert::From<crate::W<CHECONCLR_SPEC>> for W {
    fn from(writer: crate::W<CHECONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFMWS` reader - "]
pub struct PFMWS_R(crate::FieldReader<u8, u8>);
impl PFMWS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PFMWS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFMWS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFMWS` writer - "]
pub struct PFMWS_W<'a> {
    w: &'a mut W,
}
impl<'a> PFMWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Field `PREFEN` reader - "]
pub struct PREFEN_R(crate::FieldReader<u8, u8>);
impl PREFEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PREFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREFEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREFEN` writer - "]
pub struct PREFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DCSZ` reader - "]
pub struct DCSZ_R(crate::FieldReader<u8, u8>);
impl DCSZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCSZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCSZ` writer - "]
pub struct DCSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DCSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CHECOH` reader - "]
pub struct CHECOH_R(crate::FieldReader<bool, bool>);
impl CHECOH_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHECOH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHECOH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHECOH` writer - "]
pub struct CHECOH_W<'a> {
    w: &'a mut W,
}
impl<'a> CHECOH_W<'a> {
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
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pfmws(&self) -> PFMWS_R {
        PFMWS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn prefen(&self) -> PREFEN_R {
        PREFEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dcsz(&self) -> DCSZ_R {
        DCSZ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn checoh(&self) -> CHECOH_R {
        CHECOH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pfmws(&mut self) -> PFMWS_W {
        PFMWS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn prefen(&mut self) -> PREFEN_W {
        PREFEN_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dcsz(&mut self) -> DCSZ_W {
        DCSZ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn checoh(&mut self) -> CHECOH_W {
        CHECOH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHECONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [checonclr](index.html) module"]
pub struct CHECONCLR_SPEC;
impl crate::RegisterSpec for CHECONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [checonclr::R](R) reader structure"]
impl crate::Readable for CHECONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [checonclr::W](W) writer structure"]
impl crate::Writable for CHECONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHECONCLR to value 0"]
impl crate::Resettable for CHECONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
