#[doc = "Register `SYSKEY` reader"]
pub struct R(crate::R<SYSKEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSKEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYSKEY_SPEC>> for R {
    fn from(reader: crate::R<SYSKEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSKEY` writer"]
pub struct W(crate::W<SYSKEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSKEY_SPEC>;
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
impl core::convert::From<crate::W<SYSKEY_SPEC>> for W {
    fn from(writer: crate::W<SYSKEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSKEY` reader - "]
pub struct SYSKEY_R(crate::FieldReader<u32, u32>);
impl SYSKEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        SYSKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSKEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSKEY` writer - "]
pub struct SYSKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSKEY_W<'a> {
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
    pub fn syskey(&self) -> SYSKEY_R {
        SYSKEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn syskey(&mut self) -> SYSKEY_W {
        SYSKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSKEY register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syskey](index.html) module"]
pub struct SYSKEY_SPEC;
impl crate::RegisterSpec for SYSKEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syskey::R](R) reader structure"]
impl crate::Readable for SYSKEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syskey::W](W) writer structure"]
impl crate::Writable for SYSKEY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSKEY to value 0"]
impl crate::Resettable for SYSKEY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
