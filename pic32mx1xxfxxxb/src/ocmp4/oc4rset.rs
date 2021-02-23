#[doc = "Register `OC4RSET` reader"]
pub struct R(crate::R<OC4RSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OC4RSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<OC4RSET_SPEC>> for R {
    fn from(reader: crate::R<OC4RSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OC4RSET` writer"]
pub struct W(crate::W<OC4RSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OC4RSET_SPEC>;
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
impl core::convert::From<crate::W<OC4RSET_SPEC>> for W {
    fn from(writer: crate::W<OC4RSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC4R` reader - "]
pub struct OC4R_R(crate::FieldReader<u32, u32>);
impl OC4R_R {
    pub(crate) fn new(bits: u32) -> Self {
        OC4R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4R_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4R` writer - "]
pub struct OC4R_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4R_W<'a> {
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
    pub fn oc4r(&self) -> OC4R_R {
        OC4R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn oc4r(&mut self) -> OC4R_W {
        OC4R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OC4RSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc4rset](index.html) module"]
pub struct OC4RSET_SPEC;
impl crate::RegisterSpec for OC4RSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oc4rset::R](R) reader structure"]
impl crate::Readable for OC4RSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oc4rset::W](W) writer structure"]
impl crate::Writable for OC4RSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OC4RSET to value 0"]
impl crate::Resettable for OC4RSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
