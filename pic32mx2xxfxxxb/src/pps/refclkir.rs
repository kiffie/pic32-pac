#[doc = "Register `REFCLKIR` reader"]
pub struct R(crate::R<REFCLKIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFCLKIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<REFCLKIR_SPEC>> for R {
    fn from(reader: crate::R<REFCLKIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFCLKIR` writer"]
pub struct W(crate::W<REFCLKIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFCLKIR_SPEC>;
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
impl core::convert::From<crate::W<REFCLKIR_SPEC>> for W {
    fn from(writer: crate::W<REFCLKIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFCLKIR` reader - "]
pub struct REFCLKIR_R(crate::FieldReader<u8, u8>);
impl REFCLKIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFCLKIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFCLKIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFCLKIR` writer - "]
pub struct REFCLKIR_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCLKIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn refclkir(&self) -> REFCLKIR_R {
        REFCLKIR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn refclkir(&mut self) -> REFCLKIR_W {
        REFCLKIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "REFCLKIR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refclkir](index.html) module"]
pub struct REFCLKIR_SPEC;
impl crate::RegisterSpec for REFCLKIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refclkir::R](R) reader structure"]
impl crate::Readable for REFCLKIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refclkir::W](W) writer structure"]
impl crate::Writable for REFCLKIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFCLKIR to value 0"]
impl crate::Resettable for REFCLKIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
