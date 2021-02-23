#[doc = "Register `CHEMSKSET` reader"]
pub struct R(crate::R<CHEMSKSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHEMSKSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CHEMSKSET_SPEC>> for R {
    fn from(reader: crate::R<CHEMSKSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHEMSKSET` writer"]
pub struct W(crate::W<CHEMSKSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHEMSKSET_SPEC>;
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
impl core::convert::From<crate::W<CHEMSKSET_SPEC>> for W {
    fn from(writer: crate::W<CHEMSKSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LMASK` reader - "]
pub struct LMASK_R(crate::FieldReader<u16, u16>);
impl LMASK_R {
    pub(crate) fn new(bits: u16) -> Self {
        LMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMASK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMASK` writer - "]
pub struct LMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> LMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | (((value as u32) & 0x07ff) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn lmask(&self) -> LMASK_R {
        LMASK_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn lmask(&mut self) -> LMASK_W {
        LMASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHEMSKSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chemskset](index.html) module"]
pub struct CHEMSKSET_SPEC;
impl crate::RegisterSpec for CHEMSKSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chemskset::R](R) reader structure"]
impl crate::Readable for CHEMSKSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chemskset::W](W) writer structure"]
impl crate::Writable for CHEMSKSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHEMSKSET to value 0"]
impl crate::Resettable for CHEMSKSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
