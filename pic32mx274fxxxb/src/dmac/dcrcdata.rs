#[doc = "Register `DCRCDATA` reader"]
pub struct R(crate::R<DCRCDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCRCDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DCRCDATA_SPEC>> for R {
    fn from(reader: crate::R<DCRCDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCRCDATA` writer"]
pub struct W(crate::W<DCRCDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRCDATA_SPEC>;
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
impl core::convert::From<crate::W<DCRCDATA_SPEC>> for W {
    fn from(writer: crate::W<DCRCDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCRCDATA` reader - "]
pub struct DCRCDATA_R(crate::FieldReader<u32, u32>);
impl DCRCDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DCRCDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCRCDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCRCDATA` writer - "]
pub struct DCRCDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCDATA_W<'a> {
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
    pub fn dcrcdata(&self) -> DCRCDATA_R {
        DCRCDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dcrcdata(&mut self) -> DCRCDATA_W {
        DCRCDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCRCDATA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcdata](index.html) module"]
pub struct DCRCDATA_SPEC;
impl crate::RegisterSpec for DCRCDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcrcdata::R](R) reader structure"]
impl crate::Readable for DCRCDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcrcdata::W](W) writer structure"]
impl crate::Writable for DCRCDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCRCDATA to value 0"]
impl crate::Resettable for DCRCDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
