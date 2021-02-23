#[doc = "Register `BMXBOOTSZ` reader"]
pub struct R(crate::R<BMXBOOTSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMXBOOTSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMXBOOTSZ_SPEC>> for R {
    fn from(reader: crate::R<BMXBOOTSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMXBOOTSZ` writer"]
pub struct W(crate::W<BMXBOOTSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMXBOOTSZ_SPEC>;
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
impl core::convert::From<crate::W<BMXBOOTSZ_SPEC>> for W {
    fn from(writer: crate::W<BMXBOOTSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMXBOOTSZ` reader - "]
pub struct BMXBOOTSZ_R(crate::FieldReader<u32, u32>);
impl BMXBOOTSZ_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMXBOOTSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXBOOTSZ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXBOOTSZ` writer - "]
pub struct BMXBOOTSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXBOOTSZ_W<'a> {
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
    pub fn bmxbootsz(&self) -> BMXBOOTSZ_R {
        BMXBOOTSZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxbootsz(&mut self) -> BMXBOOTSZ_W {
        BMXBOOTSZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMXBOOTSZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxbootsz](index.html) module"]
pub struct BMXBOOTSZ_SPEC;
impl crate::RegisterSpec for BMXBOOTSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmxbootsz::R](R) reader structure"]
impl crate::Readable for BMXBOOTSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmxbootsz::W](W) writer structure"]
impl crate::Writable for BMXBOOTSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMXBOOTSZ to value 0x3000"]
impl crate::Resettable for BMXBOOTSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3000
    }
}
