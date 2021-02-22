#[doc = "Register `BMXPFMSZ` reader"]
pub struct R(crate::R<BMXPFMSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMXPFMSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMXPFMSZ_SPEC>> for R {
    fn from(reader: crate::R<BMXPFMSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMXPFMSZ` writer"]
pub struct W(crate::W<BMXPFMSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMXPFMSZ_SPEC>;
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
impl core::convert::From<crate::W<BMXPFMSZ_SPEC>> for W {
    fn from(writer: crate::W<BMXPFMSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMXPFMSZ` reader - "]
pub struct BMXPFMSZ_R(crate::FieldReader<u32, u32>);
impl BMXPFMSZ_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMXPFMSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXPFMSZ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXPFMSZ` writer - "]
pub struct BMXPFMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXPFMSZ_W<'a> {
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
    pub fn bmxpfmsz(&self) -> BMXPFMSZ_R {
        BMXPFMSZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxpfmsz(&mut self) -> BMXPFMSZ_W {
        BMXPFMSZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMXPFMSZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxpfmsz](index.html) module"]
pub struct BMXPFMSZ_SPEC;
impl crate::RegisterSpec for BMXPFMSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmxpfmsz::R](R) reader structure"]
impl crate::Readable for BMXPFMSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmxpfmsz::W](W) writer structure"]
impl crate::Writable for BMXPFMSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMXPFMSZ to value 0x0008_0000"]
impl crate::Resettable for BMXPFMSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_0000
    }
}
