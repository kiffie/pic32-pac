#[doc = "Register `PMAENCLR` reader"]
pub struct R(crate::R<PMAENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMAENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMAENCLR_SPEC>> for R {
    fn from(reader: crate::R<PMAENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMAENCLR` writer"]
pub struct W(crate::W<PMAENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMAENCLR_SPEC>;
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
impl core::convert::From<crate::W<PMAENCLR_SPEC>> for W {
    fn from(writer: crate::W<PMAENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTEN` reader - "]
pub struct PTEN_R(crate::FieldReader<u16, u16>);
impl PTEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEN` writer - "]
pub struct PTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEN_W<'a> {
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
    pub fn pten(&self) -> PTEN_R {
        PTEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pten(&mut self) -> PTEN_W {
        PTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMAENCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaenclr](index.html) module"]
pub struct PMAENCLR_SPEC;
impl crate::RegisterSpec for PMAENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmaenclr::R](R) reader structure"]
impl crate::Readable for PMAENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmaenclr::W](W) writer structure"]
impl crate::Writable for PMAENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMAENCLR to value 0"]
impl crate::Resettable for PMAENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
