#[doc = "Register `BMXPUPBA` reader"]
pub struct R(crate::R<BMXPUPBA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMXPUPBA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMXPUPBA_SPEC>> for R {
    fn from(reader: crate::R<BMXPUPBA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMXPUPBA` writer"]
pub struct W(crate::W<BMXPUPBA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMXPUPBA_SPEC>;
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
impl core::convert::From<crate::W<BMXPUPBA_SPEC>> for W {
    fn from(writer: crate::W<BMXPUPBA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMXPUPBA` reader - "]
pub struct BMXPUPBA_R(crate::FieldReader<u32, u32>);
impl BMXPUPBA_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMXPUPBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXPUPBA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXPUPBA` writer - "]
pub struct BMXPUPBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXPUPBA_W<'a> {
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
    pub fn bmxpupba(&self) -> BMXPUPBA_R {
        BMXPUPBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxpupba(&mut self) -> BMXPUPBA_W {
        BMXPUPBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMXPUPBA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxpupba](index.html) module"]
pub struct BMXPUPBA_SPEC;
impl crate::RegisterSpec for BMXPUPBA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmxpupba::R](R) reader structure"]
impl crate::Readable for BMXPUPBA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmxpupba::W](W) writer structure"]
impl crate::Writable for BMXPUPBA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMXPUPBA to value 0"]
impl crate::Resettable for BMXPUPBA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
