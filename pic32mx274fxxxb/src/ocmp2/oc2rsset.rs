#[doc = "Register `OC2RSSET` reader"]
pub struct R(crate::R<OC2RSSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OC2RSSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OC2RSSET_SPEC>> for R {
    fn from(reader: crate::R<OC2RSSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OC2RSSET` writer"]
pub struct W(crate::W<OC2RSSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OC2RSSET_SPEC>;
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
impl core::convert::From<crate::W<OC2RSSET_SPEC>> for W {
    fn from(writer: crate::W<OC2RSSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC2RS` reader - "]
pub struct OC2RS_R(crate::FieldReader<u32, u32>);
impl OC2RS_R {
    pub(crate) fn new(bits: u32) -> Self {
        OC2RS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2RS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2RS` writer - "]
pub struct OC2RS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2RS_W<'a> {
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
    pub fn oc2rs(&self) -> OC2RS_R {
        OC2RS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc2rs(&mut self) -> OC2RS_W {
        OC2RS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OC2RSSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc2rsset](index.html) module"]
pub struct OC2RSSET_SPEC;
impl crate::RegisterSpec for OC2RSSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oc2rsset::R](R) reader structure"]
impl crate::Readable for OC2RSSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oc2rsset::W](W) writer structure"]
impl crate::Writable for OC2RSSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OC2RSSET to value 0"]
impl crate::Resettable for OC2RSSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
