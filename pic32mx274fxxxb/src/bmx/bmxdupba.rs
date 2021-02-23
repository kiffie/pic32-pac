#[doc = "Register `BMXDUPBA` reader"]
pub struct R(crate::R<BMXDUPBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMXDUPBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMXDUPBA_SPEC>> for R {
    fn from(reader: crate::R<BMXDUPBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMXDUPBA` writer"]
pub struct W(crate::W<BMXDUPBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMXDUPBA_SPEC>;
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
impl core::convert::From<crate::W<BMXDUPBA_SPEC>> for W {
    fn from(writer: crate::W<BMXDUPBA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMXDUPBA` reader - "]
pub struct BMXDUPBA_R(crate::FieldReader<u32, u32>);
impl BMXDUPBA_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMXDUPBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXDUPBA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXDUPBA` writer - "]
pub struct BMXDUPBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXDUPBA_W<'a> {
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
    pub fn bmxdupba(&self) -> BMXDUPBA_R {
        BMXDUPBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxdupba(&mut self) -> BMXDUPBA_W {
        BMXDUPBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMXDUPBA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdupba](index.html) module"]
pub struct BMXDUPBA_SPEC;
impl crate::RegisterSpec for BMXDUPBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmxdupba::R](R) reader structure"]
impl crate::Readable for BMXDUPBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmxdupba::W](W) writer structure"]
impl crate::Writable for BMXDUPBA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMXDUPBA to value 0"]
impl crate::Resettable for BMXDUPBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
