#[doc = "Register `RPA9R` reader"]
pub struct R(crate::R<RPA9R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPA9R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RPA9R_SPEC>> for R {
    fn from(reader: crate::R<RPA9R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPA9R` writer"]
pub struct W(crate::W<RPA9R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPA9R_SPEC>;
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
impl core::convert::From<crate::W<RPA9R_SPEC>> for W {
    fn from(writer: crate::W<RPA9R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPA9R` reader - "]
pub struct RPA9R_R(crate::FieldReader<u8, u8>);
impl RPA9R_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPA9R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPA9R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPA9R` writer - "]
pub struct RPA9R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPA9R_W<'a> {
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
    pub fn rpa9r(&self) -> RPA9R_R {
        RPA9R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpa9r(&mut self) -> RPA9R_W {
        RPA9R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RPA9R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa9r](index.html) module"]
pub struct RPA9R_SPEC;
impl crate::RegisterSpec for RPA9R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpa9r::R](R) reader structure"]
impl crate::Readable for RPA9R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpa9r::W](W) writer structure"]
impl crate::Writable for RPA9R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPA9R to value 0"]
impl crate::Resettable for RPA9R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
