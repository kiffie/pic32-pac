#[doc = "Register `DSGPR12` reader"]
pub struct R(crate::R<DSGPR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSGPR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DSGPR12_SPEC>> for R {
    fn from(reader: crate::R<DSGPR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSGPR12` writer"]
pub struct W(crate::W<DSGPR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSGPR12_SPEC>;
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
impl core::convert::From<crate::W<DSGPR12_SPEC>> for W {
    fn from(writer: crate::W<DSGPR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSGPR` reader - "]
pub struct DSGPR_R(crate::FieldReader<u32, u32>);
impl DSGPR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DSGPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSGPR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSGPR` writer - "]
pub struct DSGPR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSGPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dsgpr(&self) -> DSGPR_R {
        DSGPR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dsgpr(&mut self) -> DSGPR_W {
        DSGPR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSGPR12 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsgpr12](index.html) module"]
pub struct DSGPR12_SPEC;
impl crate::RegisterSpec for DSGPR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsgpr12::R](R) reader structure"]
impl crate::Readable for DSGPR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsgpr12::W](W) writer structure"]
impl crate::Writable for DSGPR12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSGPR12 to value 0"]
impl crate::Resettable for DSGPR12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
