#[doc = "Register `BMXDUDBACLR` reader"]
pub struct R(crate::R<BMXDUDBACLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMXDUDBACLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMXDUDBACLR_SPEC>> for R {
    fn from(reader: crate::R<BMXDUDBACLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMXDUDBACLR` writer"]
pub struct W(crate::W<BMXDUDBACLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMXDUDBACLR_SPEC>;
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
impl core::convert::From<crate::W<BMXDUDBACLR_SPEC>> for W {
    fn from(writer: crate::W<BMXDUDBACLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMXDUDBA` reader - "]
pub struct BMXDUDBA_R(crate::FieldReader<u32, u32>);
impl BMXDUDBA_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMXDUDBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXDUDBA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXDUDBA` writer - "]
pub struct BMXDUDBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXDUDBA_W<'a> {
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
    pub fn bmxdudba(&self) -> BMXDUDBA_R {
        BMXDUDBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxdudba(&mut self) -> BMXDUDBA_W {
        BMXDUDBA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMXDUDBACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxdudbaclr](index.html) module"]
pub struct BMXDUDBACLR_SPEC;
impl crate::RegisterSpec for BMXDUDBACLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmxdudbaclr::R](R) reader structure"]
impl crate::Readable for BMXDUDBACLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmxdudbaclr::W](W) writer structure"]
impl crate::Writable for BMXDUDBACLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMXDUDBACLR to value 0"]
impl crate::Resettable for BMXDUDBACLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
