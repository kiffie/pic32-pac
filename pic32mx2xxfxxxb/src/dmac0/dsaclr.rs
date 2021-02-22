#[doc = "Register `DSACLR` reader"]
pub struct R(crate::R<DSACLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSACLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DSACLR_SPEC>> for R {
    fn from(reader: crate::R<DSACLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSACLR` writer"]
pub struct W(crate::W<DSACLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSACLR_SPEC>;
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
impl core::convert::From<crate::W<DSACLR_SPEC>> for W {
    fn from(writer: crate::W<DSACLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSA` reader - "]
pub struct DSA_R(crate::FieldReader<u32, u32>);
impl DSA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSA` writer - "]
pub struct DSA_W<'a> {
    w: &'a mut W,
}
impl<'a> DSA_W<'a> {
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
    pub fn dsa(&self) -> DSA_R {
        DSA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dsa(&mut self) -> DSA_W {
        DSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0DSACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsaclr](index.html) module"]
pub struct DSACLR_SPEC;
impl crate::RegisterSpec for DSACLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsaclr::R](R) reader structure"]
impl crate::Readable for DSACLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsaclr::W](W) writer structure"]
impl crate::Writable for DSACLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSACLR to value 0"]
impl crate::Resettable for DSACLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
