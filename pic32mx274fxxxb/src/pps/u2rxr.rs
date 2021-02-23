#[doc = "Register `U2RXR` reader"]
pub struct R(crate::R<U2RXR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U2RXR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U2RXR_SPEC>> for R {
    fn from(reader: crate::R<U2RXR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U2RXR` writer"]
pub struct W(crate::W<U2RXR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U2RXR_SPEC>;
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
impl core::convert::From<crate::W<U2RXR_SPEC>> for W {
    fn from(writer: crate::W<U2RXR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U2RXR` reader - "]
pub struct U2RXR_R(crate::FieldReader<u8, u8>);
impl U2RXR_R {
    pub(crate) fn new(bits: u8) -> Self {
        U2RXR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2RXR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2RXR` writer - "]
pub struct U2RXR_W<'a> {
    w: &'a mut W,
}
impl<'a> U2RXR_W<'a> {
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
    pub fn u2rxr(&self) -> U2RXR_R {
        U2RXR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn u2rxr(&mut self) -> U2RXR_W {
        U2RXR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U2RXR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u2rxr](index.html) module"]
pub struct U2RXR_SPEC;
impl crate::RegisterSpec for U2RXR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u2rxr::R](R) reader structure"]
impl crate::Readable for U2RXR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u2rxr::W](W) writer structure"]
impl crate::Writable for U2RXR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U2RXR to value 0"]
impl crate::Resettable for U2RXR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
