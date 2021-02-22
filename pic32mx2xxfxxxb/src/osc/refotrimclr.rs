#[doc = "Register `REFOTRIMCLR` reader"]
pub struct R(crate::R<REFOTRIMCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFOTRIMCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<REFOTRIMCLR_SPEC>> for R {
    fn from(reader: crate::R<REFOTRIMCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFOTRIMCLR` writer"]
pub struct W(crate::W<REFOTRIMCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFOTRIMCLR_SPEC>;
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
impl core::convert::From<crate::W<REFOTRIMCLR_SPEC>> for W {
    fn from(writer: crate::W<REFOTRIMCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROTRIM` reader - "]
pub struct ROTRIM_R(crate::FieldReader<u16, u16>);
impl ROTRIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        ROTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROTRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROTRIM` writer - "]
pub struct ROTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 23)) | (((value as u32) & 0x01ff) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn rotrim(&self) -> ROTRIM_R {
        ROTRIM_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn rotrim(&mut self) -> ROTRIM_W {
        ROTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "REFOTRIMCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refotrimclr](index.html) module"]
pub struct REFOTRIMCLR_SPEC;
impl crate::RegisterSpec for REFOTRIMCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refotrimclr::R](R) reader structure"]
impl crate::Readable for REFOTRIMCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refotrimclr::W](W) writer structure"]
impl crate::Writable for REFOTRIMCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REFOTRIMCLR to value 0"]
impl crate::Resettable for REFOTRIMCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
