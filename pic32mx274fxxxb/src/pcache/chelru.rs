#[doc = "Register `CHELRU` reader"]
pub struct R(crate::R<CHELRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHELRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHELRU_SPEC>> for R {
    fn from(reader: crate::R<CHELRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHELRU` writer"]
pub struct W(crate::W<CHELRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHELRU_SPEC>;
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
impl core::convert::From<crate::W<CHELRU_SPEC>> for W {
    fn from(writer: crate::W<CHELRU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHELRU` reader - "]
pub struct CHELRU_R(crate::FieldReader<u32, u32>);
impl CHELRU_R {
    pub(crate) fn new(bits: u32) -> Self {
        CHELRU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHELRU_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHELRU` writer - "]
pub struct CHELRU_W<'a> {
    w: &'a mut W,
}
impl<'a> CHELRU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn chelru(&self) -> CHELRU_R {
        CHELRU_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn chelru(&mut self) -> CHELRU_W {
        CHELRU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHELRU register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chelru](index.html) module"]
pub struct CHELRU_SPEC;
impl crate::RegisterSpec for CHELRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chelru::R](R) reader structure"]
impl crate::Readable for CHELRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chelru::W](W) writer structure"]
impl crate::Writable for CHELRU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHELRU to value 0"]
impl crate::Resettable for CHELRU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
