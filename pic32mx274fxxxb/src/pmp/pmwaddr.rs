#[doc = "Register `PMWADDR` reader"]
pub struct R(crate::R<PMWADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMWADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMWADDR_SPEC>> for R {
    fn from(reader: crate::R<PMWADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMWADDR` writer"]
pub struct W(crate::W<PMWADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMWADDR_SPEC>;
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
impl core::convert::From<crate::W<PMWADDR_SPEC>> for W {
    fn from(writer: crate::W<PMWADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WADDR` reader - "]
pub struct WADDR_R(crate::FieldReader<u32, u32>);
impl WADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        WADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WADDR` writer - "]
pub struct WADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn waddr(&self) -> WADDR_R {
        WADDR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn waddr(&mut self) -> WADDR_W {
        WADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMWADDR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmwaddr](index.html) module"]
pub struct PMWADDR_SPEC;
impl crate::RegisterSpec for PMWADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmwaddr::R](R) reader structure"]
impl crate::Readable for PMWADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmwaddr::W](W) writer structure"]
impl crate::Writable for PMWADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMWADDR to value 0"]
impl crate::Resettable for PMWADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
