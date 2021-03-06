#[doc = "Register `PR1INV` reader"]
pub struct R(crate::R<PR1INV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR1INV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PR1INV_SPEC>> for R {
    fn from(reader: crate::R<PR1INV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR1INV` writer"]
pub struct W(crate::W<PR1INV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR1INV_SPEC>;
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
impl core::convert::From<crate::W<PR1INV_SPEC>> for W {
    fn from(writer: crate::W<PR1INV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR1` reader - "]
pub struct PR1_R(crate::FieldReader<u32, u32>);
impl PR1_R {
    pub(crate) fn new(bits: u32) -> Self {
        PR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR1` writer - "]
pub struct PR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PR1_W<'a> {
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
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W {
        PR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PR1INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr1inv](index.html) module"]
pub struct PR1INV_SPEC;
impl crate::RegisterSpec for PR1INV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr1inv::R](R) reader structure"]
impl crate::Readable for PR1INV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr1inv::W](W) writer structure"]
impl crate::Writable for PR1INV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PR1INV to value 0"]
impl crate::Resettable for PR1INV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
