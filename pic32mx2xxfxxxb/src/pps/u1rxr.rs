#[doc = "Register `U1RXR` reader"]
pub struct R(crate::R<U1RXR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1RXR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1RXR_SPEC>> for R {
    fn from(reader: crate::R<U1RXR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1RXR` writer"]
pub struct W(crate::W<U1RXR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1RXR_SPEC>;
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
impl core::convert::From<crate::W<U1RXR_SPEC>> for W {
    fn from(writer: crate::W<U1RXR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U1RXR` reader - "]
pub struct U1RXR_R(crate::FieldReader<u8, u8>);
impl U1RXR_R {
    pub(crate) fn new(bits: u8) -> Self {
        U1RXR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1RXR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1RXR` writer - "]
pub struct U1RXR_W<'a> {
    w: &'a mut W,
}
impl<'a> U1RXR_W<'a> {
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
    pub fn u1rxr(&self) -> U1RXR_R {
        U1RXR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn u1rxr(&mut self) -> U1RXR_W {
        U1RXR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1RXR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1rxr](index.html) module"]
pub struct U1RXR_SPEC;
impl crate::RegisterSpec for U1RXR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1rxr::R](R) reader structure"]
impl crate::Readable for U1RXR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1rxr::W](W) writer structure"]
impl crate::Writable for U1RXR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1RXR to value 0"]
impl crate::Resettable for U1RXR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
