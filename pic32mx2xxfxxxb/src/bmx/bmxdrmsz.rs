#[doc = "Register `BMXDRMSZ` reader"]
pub struct R(crate::R<BMXDRMSZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMXDRMSZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMXDRMSZ_SPEC>> for R {
    fn from(reader: crate::R<BMXDRMSZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMXDRMSZ` writer"]
pub struct W(crate::W<BMXDRMSZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMXDRMSZ_SPEC>;
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
impl core::convert::From<crate::W<BMXDRMSZ_SPEC>> for W {
    fn from(writer: crate::W<BMXDRMSZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMXDRMSZ` reader - "]
pub struct BMXDRMSZ_R(crate::FieldReader<u32, u32>);
impl BMXDRMSZ_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMXDRMSZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXDRMSZ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXDRMSZ` writer - "]
pub struct BMXDRMSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXDRMSZ_W<'a> {
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
    pub fn bmxdrmsz(&self) -> BMXDRMSZ_R {
        BMXDRMSZ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxdrmsz(&mut self) -> BMXDRMSZ_W {
        BMXDRMSZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMXDRMSZ register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdrmsz](index.html) module"]
pub struct BMXDRMSZ_SPEC;
impl crate::RegisterSpec for BMXDRMSZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmxdrmsz::R](R) reader structure"]
impl crate::Readable for BMXDRMSZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmxdrmsz::W](W) writer structure"]
impl crate::Writable for BMXDRMSZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMXDRMSZ to value 0x0002_0000"]
impl crate::Resettable for BMXDRMSZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0002_0000
    }
}
