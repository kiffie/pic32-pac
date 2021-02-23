#[doc = "Register `RPA2R` reader"]
pub struct R(crate::R<RPA2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPA2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RPA2R_SPEC>> for R {
    fn from(reader: crate::R<RPA2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPA2R` writer"]
pub struct W(crate::W<RPA2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPA2R_SPEC>;
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
impl core::convert::From<crate::W<RPA2R_SPEC>> for W {
    fn from(writer: crate::W<RPA2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPA2R` reader - "]
pub struct RPA2R_R(crate::FieldReader<u8, u8>);
impl RPA2R_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPA2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPA2R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPA2R` writer - "]
pub struct RPA2R_W<'a> {
    w: &'a mut W,
}
impl<'a> RPA2R_W<'a> {
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
    pub fn rpa2r(&self) -> RPA2R_R {
        RPA2R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpa2r(&mut self) -> RPA2R_W {
        RPA2R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RPA2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpa2r](index.html) module"]
pub struct RPA2R_SPEC;
impl crate::RegisterSpec for RPA2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpa2r::R](R) reader structure"]
impl crate::Readable for RPA2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpa2r::W](W) writer structure"]
impl crate::Writable for RPA2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPA2R to value 0"]
impl crate::Resettable for RPA2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
