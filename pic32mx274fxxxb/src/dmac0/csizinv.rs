#[doc = "Register `CSIZINV` reader"]
pub struct R(crate::R<CSIZINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIZINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSIZINV_SPEC>> for R {
    fn from(reader: crate::R<CSIZINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIZINV` writer"]
pub struct W(crate::W<CSIZINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIZINV_SPEC>;
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
impl core::convert::From<crate::W<CSIZINV_SPEC>> for W {
    fn from(writer: crate::W<CSIZINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHCSIZ` reader - "]
pub struct CHCSIZ_R(crate::FieldReader<u16, u16>);
impl CHCSIZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        CHCSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHCSIZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHCSIZ` writer - "]
pub struct CHCSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCSIZ_W<'a> {
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
    pub fn chcsiz(&self) -> CHCSIZ_R {
        CHCSIZ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chcsiz(&mut self) -> CHCSIZ_W {
        CHCSIZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0CSIZINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csizinv](index.html) module"]
pub struct CSIZINV_SPEC;
impl crate::RegisterSpec for CSIZINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csizinv::R](R) reader structure"]
impl crate::Readable for CSIZINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csizinv::W](W) writer structure"]
impl crate::Writable for CSIZINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSIZINV to value 0"]
impl crate::Resettable for CSIZINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
