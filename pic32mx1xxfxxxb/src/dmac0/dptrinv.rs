#[doc = "Register `DPTRINV` reader"]
pub struct R(crate::R<DPTRINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPTRINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DPTRINV_SPEC>> for R {
    fn from(reader: crate::R<DPTRINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPTRINV` writer"]
pub struct W(crate::W<DPTRINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPTRINV_SPEC>;
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
impl core::convert::From<crate::W<DPTRINV_SPEC>> for W {
    fn from(writer: crate::W<DPTRINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHDPTR` reader - "]
pub struct CHDPTR_R(crate::FieldReader<u16, u16>);
impl CHDPTR_R {
    pub(crate) fn new(bits: u16) -> Self {
        CHDPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDPTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDPTR` writer - "]
pub struct CHDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chdptr(&self) -> CHDPTR_R {
        CHDPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chdptr(&mut self) -> CHDPTR_W {
        CHDPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0DPTRINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dptrinv](index.html) module"]
pub struct DPTRINV_SPEC;
impl crate::RegisterSpec for DPTRINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dptrinv::R](R) reader structure"]
impl crate::Readable for DPTRINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dptrinv::W](W) writer structure"]
impl crate::Writable for DPTRINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPTRINV to value 0"]
impl crate::Resettable for DPTRINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
