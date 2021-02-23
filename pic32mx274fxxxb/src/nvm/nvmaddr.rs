#[doc = "Register `NVMADDR` reader"]
pub struct R(crate::R<NVMADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVMADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NVMADDR_SPEC>> for R {
    fn from(reader: crate::R<NVMADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVMADDR` writer"]
pub struct W(crate::W<NVMADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVMADDR_SPEC>;
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
impl core::convert::From<crate::W<NVMADDR_SPEC>> for W {
    fn from(writer: crate::W<NVMADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NVMADDR` reader - "]
pub struct NVMADDR_R(crate::FieldReader<u32, u32>);
impl NVMADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        NVMADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMADDR` writer - "]
pub struct NVMADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMADDR_W<'a> {
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
    pub fn nvmaddr(&self) -> NVMADDR_R {
        NVMADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn nvmaddr(&mut self) -> NVMADDR_W {
        NVMADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NVMADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmaddr](index.html) module"]
pub struct NVMADDR_SPEC;
impl crate::RegisterSpec for NVMADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvmaddr::R](R) reader structure"]
impl crate::Readable for NVMADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvmaddr::W](W) writer structure"]
impl crate::Writable for NVMADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVMADDR to value 0"]
impl crate::Resettable for NVMADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
