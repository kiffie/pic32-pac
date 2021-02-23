#[doc = "Register `U1SOFINV` reader"]
pub struct R(crate::R<U1SOFINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1SOFINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1SOFINV_SPEC>> for R {
    fn from(reader: crate::R<U1SOFINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1SOFINV` writer"]
pub struct W(crate::W<U1SOFINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1SOFINV_SPEC>;
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
impl core::convert::From<crate::W<U1SOFINV_SPEC>> for W {
    fn from(writer: crate::W<U1SOFINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - "]
pub struct CNT_R(crate::FieldReader<u8, u8>);
impl CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT` writer - "]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
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
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1SOFINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1sofinv](index.html) module"]
pub struct U1SOFINV_SPEC;
impl crate::RegisterSpec for U1SOFINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1sofinv::R](R) reader structure"]
impl crate::Readable for U1SOFINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1sofinv::W](W) writer structure"]
impl crate::Writable for U1SOFINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1SOFINV to value 0"]
impl crate::Resettable for U1SOFINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
