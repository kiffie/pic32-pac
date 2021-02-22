#[doc = "Register `SSIZSET` reader"]
pub struct R(crate::R<SSIZSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSIZSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SSIZSET_SPEC>> for R {
    fn from(reader: crate::R<SSIZSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSIZSET` writer"]
pub struct W(crate::W<SSIZSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSIZSET_SPEC>;
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
impl core::convert::From<crate::W<SSIZSET_SPEC>> for W {
    fn from(writer: crate::W<SSIZSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSSIZ` reader - "]
pub struct CHSSIZ_R(crate::FieldReader<u16, u16>);
impl CHSSIZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        CHSSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSSIZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSSIZ` writer - "]
pub struct CHSSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSSIZ_W<'a> {
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
    pub fn chssiz(&self) -> CHSSIZ_R {
        CHSSIZ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chssiz(&mut self) -> CHSSIZ_W {
        CHSSIZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0SSIZSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssizset](index.html) module"]
pub struct SSIZSET_SPEC;
impl crate::RegisterSpec for SSIZSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssizset::R](R) reader structure"]
impl crate::Readable for SSIZSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssizset::W](W) writer structure"]
impl crate::Writable for SSIZSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSIZSET to value 0"]
impl crate::Resettable for SSIZSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
