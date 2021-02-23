#[doc = "Register `DCRCXORCLR` reader"]
pub struct R(crate::R<DCRCXORCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCRCXORCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DCRCXORCLR_SPEC>> for R {
    fn from(reader: crate::R<DCRCXORCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCRCXORCLR` writer"]
pub struct W(crate::W<DCRCXORCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRCXORCLR_SPEC>;
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
impl core::convert::From<crate::W<DCRCXORCLR_SPEC>> for W {
    fn from(writer: crate::W<DCRCXORCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCRCXOR` reader - "]
pub struct DCRCXOR_R(crate::FieldReader<u32, u32>);
impl DCRCXOR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DCRCXOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCXOR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCXOR` writer - "]
pub struct DCRCXOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCXOR_W<'a> {
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
    pub fn dcrcxor(&self) -> DCRCXOR_R {
        DCRCXOR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dcrcxor(&mut self) -> DCRCXOR_W {
        DCRCXOR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCRCXORCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcxorclr](index.html) module"]
pub struct DCRCXORCLR_SPEC;
impl crate::RegisterSpec for DCRCXORCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcrcxorclr::R](R) reader structure"]
impl crate::Readable for DCRCXORCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcrcxorclr::W](W) writer structure"]
impl crate::Writable for DCRCXORCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCRCXORCLR to value 0"]
impl crate::Resettable for DCRCXORCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
