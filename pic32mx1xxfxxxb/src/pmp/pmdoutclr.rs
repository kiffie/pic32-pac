#[doc = "Register `PMDOUTCLR` reader"]
pub struct R(crate::R<PMDOUTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMDOUTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMDOUTCLR_SPEC>> for R {
    fn from(reader: crate::R<PMDOUTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMDOUTCLR` writer"]
pub struct W(crate::W<PMDOUTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMDOUTCLR_SPEC>;
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
impl core::convert::From<crate::W<PMDOUTCLR_SPEC>> for W {
    fn from(writer: crate::W<PMDOUTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAOUT` reader - "]
pub struct DATAOUT_R(crate::FieldReader<u32, u32>);
impl DATAOUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATAOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAOUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAOUT` writer - "]
pub struct DATAOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dataout(&mut self) -> DATAOUT_W {
        DATAOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMDOUTCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmdoutclr](index.html) module"]
pub struct PMDOUTCLR_SPEC;
impl crate::RegisterSpec for PMDOUTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmdoutclr::R](R) reader structure"]
impl crate::Readable for PMDOUTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmdoutclr::W](W) writer structure"]
impl crate::Writable for PMDOUTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMDOUTCLR to value 0"]
impl crate::Resettable for PMDOUTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
