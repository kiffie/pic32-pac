#[doc = "Register `NVMKEY` reader"]
pub struct R(crate::R<NVMKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVMKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NVMKEY_SPEC>> for R {
    fn from(reader: crate::R<NVMKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVMKEY` writer"]
pub struct W(crate::W<NVMKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVMKEY_SPEC>;
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
impl core::convert::From<crate::W<NVMKEY_SPEC>> for W {
    fn from(writer: crate::W<NVMKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NVMKEY` reader - "]
pub struct NVMKEY_R(crate::FieldReader<u32, u32>);
impl NVMKEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        NVMKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMKEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMKEY` writer - "]
pub struct NVMKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMKEY_W<'a> {
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
    pub fn nvmkey(&self) -> NVMKEY_R {
        NVMKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn nvmkey(&mut self) -> NVMKEY_W {
        NVMKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NVMKEY register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmkey](index.html) module"]
pub struct NVMKEY_SPEC;
impl crate::RegisterSpec for NVMKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvmkey::R](R) reader structure"]
impl crate::Readable for NVMKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvmkey::W](W) writer structure"]
impl crate::Writable for NVMKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVMKEY to value 0"]
impl crate::Resettable for NVMKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
