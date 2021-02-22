#[doc = "Register `TMR2CLR` reader"]
pub struct R(crate::R<TMR2CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR2CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TMR2CLR_SPEC>> for R {
    fn from(reader: crate::R<TMR2CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR2CLR` writer"]
pub struct W(crate::W<TMR2CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR2CLR_SPEC>;
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
impl core::convert::From<crate::W<TMR2CLR_SPEC>> for W {
    fn from(writer: crate::W<TMR2CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR2` reader - "]
pub struct TMR2_R(crate::FieldReader<u32, u32>);
impl TMR2_R {
    pub(crate) fn new(bits: u32) -> Self {
        TMR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR2` writer - "]
pub struct TMR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2_W<'a> {
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
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr2(&mut self) -> TMR2_W {
        TMR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr2clr](index.html) module"]
pub struct TMR2CLR_SPEC;
impl crate::RegisterSpec for TMR2CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr2clr::R](R) reader structure"]
impl crate::Readable for TMR2CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr2clr::W](W) writer structure"]
impl crate::Writable for TMR2CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR2CLR to value 0"]
impl crate::Resettable for TMR2CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
