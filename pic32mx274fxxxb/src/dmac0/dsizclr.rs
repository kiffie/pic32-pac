#[doc = "Register `DSIZCLR` reader"]
pub struct R(crate::R<DSIZCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSIZCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DSIZCLR_SPEC>> for R {
    fn from(reader: crate::R<DSIZCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSIZCLR` writer"]
pub struct W(crate::W<DSIZCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSIZCLR_SPEC>;
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
impl core::convert::From<crate::W<DSIZCLR_SPEC>> for W {
    fn from(writer: crate::W<DSIZCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHDSIZ` reader - "]
pub struct CHDSIZ_R(crate::FieldReader<u16, u16>);
impl CHDSIZ_R {
    pub(crate) fn new(bits: u16) -> Self {
        CHDSIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHDSIZ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHDSIZ` writer - "]
pub struct CHDSIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDSIZ_W<'a> {
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
    pub fn chdsiz(&self) -> CHDSIZ_R {
        CHDSIZ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chdsiz(&mut self) -> CHDSIZ_W {
        CHDSIZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0DSIZCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsizclr](index.html) module"]
pub struct DSIZCLR_SPEC;
impl crate::RegisterSpec for DSIZCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsizclr::R](R) reader structure"]
impl crate::Readable for DSIZCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsizclr::W](W) writer structure"]
impl crate::Writable for DSIZCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSIZCLR to value 0"]
impl crate::Resettable for DSIZCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
