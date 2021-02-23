#[doc = "Register `PMAENSET` reader"]
pub struct R(crate::R<PMAENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMAENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMAENSET_SPEC>> for R {
    fn from(reader: crate::R<PMAENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMAENSET` writer"]
pub struct W(crate::W<PMAENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMAENSET_SPEC>;
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
impl core::convert::From<crate::W<PMAENSET_SPEC>> for W {
    fn from(writer: crate::W<PMAENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTEN` reader - "]
pub struct PTEN_R(crate::FieldReader<u32, u32>);
impl PTEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        PTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PTEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PTEN` writer - "]
pub struct PTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PTEN_W<'a> {
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
    pub fn pten(&self) -> PTEN_R {
        PTEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn pten(&mut self) -> PTEN_W {
        PTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMAENSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmaenset](index.html) module"]
pub struct PMAENSET_SPEC;
impl crate::RegisterSpec for PMAENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmaenset::R](R) reader structure"]
impl crate::Readable for PMAENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmaenset::W](W) writer structure"]
impl crate::Writable for PMAENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMAENSET to value 0"]
impl crate::Resettable for PMAENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
