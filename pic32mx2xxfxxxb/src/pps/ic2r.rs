#[doc = "Register `IC2R` reader"]
pub struct R(crate::R<IC2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IC2R_SPEC>> for R {
    fn from(reader: crate::R<IC2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC2R` writer"]
pub struct W(crate::W<IC2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC2R_SPEC>;
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
impl core::convert::From<crate::W<IC2R_SPEC>> for W {
    fn from(writer: crate::W<IC2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC2R` reader - "]
pub struct IC2R_R(crate::FieldReader<u8, u8>);
impl IC2R_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2R` writer - "]
pub struct IC2R_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2R_W<'a> {
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
    pub fn ic2r(&self) -> IC2R_R {
        IC2R_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ic2r(&mut self) -> IC2R_W {
        IC2R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IC2R register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic2r](index.html) module"]
pub struct IC2R_SPEC;
impl crate::RegisterSpec for IC2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic2r::R](R) reader structure"]
impl crate::Readable for IC2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic2r::W](W) writer structure"]
impl crate::Writable for IC2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC2R to value 0"]
impl crate::Resettable for IC2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
