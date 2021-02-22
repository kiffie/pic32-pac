#[doc = "Register `TRNCLR` reader"]
pub struct R(crate::R<TRNCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRNCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TRNCLR_SPEC>> for R {
    fn from(reader: crate::R<TRNCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRNCLR` writer"]
pub struct W(crate::W<TRNCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRNCLR_SPEC>;
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
impl core::convert::From<crate::W<TRNCLR_SPEC>> for W {
    fn from(writer: crate::W<TRNCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRN` reader - "]
pub struct TRN_R(crate::FieldReader<u8, u8>);
impl TRN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRN` writer - "]
pub struct TRN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn trn(&self) -> TRN_R {
        TRN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn trn(&mut self) -> TRN_W {
        TRN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C1TRNCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trnclr](index.html) module"]
pub struct TRNCLR_SPEC;
impl crate::RegisterSpec for TRNCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trnclr::R](R) reader structure"]
impl crate::Readable for TRNCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trnclr::W](W) writer structure"]
impl crate::Writable for TRNCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRNCLR to value 0"]
impl crate::Resettable for TRNCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
