#[doc = "Register `IPTMR` reader"]
pub struct R(crate::R<IPTMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPTMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPTMR_SPEC>> for R {
    fn from(reader: crate::R<IPTMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPTMR` writer"]
pub struct W(crate::W<IPTMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPTMR_SPEC>;
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
impl core::convert::From<crate::W<IPTMR_SPEC>> for W {
    fn from(writer: crate::W<IPTMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPTMR` reader - "]
pub struct IPTMR_R(crate::FieldReader<u32, u32>);
impl IPTMR_R {
    pub(crate) fn new(bits: u32) -> Self {
        IPTMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPTMR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPTMR` writer - "]
pub struct IPTMR_W<'a> {
    w: &'a mut W,
}
impl<'a> IPTMR_W<'a> {
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
    pub fn iptmr(&self) -> IPTMR_R {
        IPTMR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn iptmr(&mut self) -> IPTMR_W {
        IPTMR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPTMR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptmr](index.html) module"]
pub struct IPTMR_SPEC;
impl crate::RegisterSpec for IPTMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iptmr::R](R) reader structure"]
impl crate::Readable for IPTMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iptmr::W](W) writer structure"]
impl crate::Writable for IPTMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPTMR to value 0"]
impl crate::Resettable for IPTMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
