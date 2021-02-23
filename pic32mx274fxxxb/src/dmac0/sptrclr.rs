#[doc = "Register `SPTRCLR` reader"]
pub struct R(crate::R<SPTRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPTRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPTRCLR_SPEC>> for R {
    fn from(reader: crate::R<SPTRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPTRCLR` writer"]
pub struct W(crate::W<SPTRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPTRCLR_SPEC>;
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
impl core::convert::From<crate::W<SPTRCLR_SPEC>> for W {
    fn from(writer: crate::W<SPTRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSPTR` reader - "]
pub struct CHSPTR_R(crate::FieldReader<u16, u16>);
impl CHSPTR_R {
    pub(crate) fn new(bits: u16) -> Self {
        CHSPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSPTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSPTR` writer - "]
pub struct CHSPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chsptr(&self) -> CHSPTR_R {
        CHSPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chsptr(&mut self) -> CHSPTR_W {
        CHSPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0SPTRCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sptrclr](index.html) module"]
pub struct SPTRCLR_SPEC;
impl crate::RegisterSpec for SPTRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sptrclr::R](R) reader structure"]
impl crate::Readable for SPTRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sptrclr::W](W) writer structure"]
impl crate::Writable for SPTRCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPTRCLR to value 0"]
impl crate::Resettable for SPTRCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
