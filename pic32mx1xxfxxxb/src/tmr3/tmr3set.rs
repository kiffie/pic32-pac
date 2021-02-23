#[doc = "Register `TMR3SET` reader"]
pub struct R(crate::R<TMR3SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR3SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TMR3SET_SPEC>> for R {
    fn from(reader: crate::R<TMR3SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR3SET` writer"]
pub struct W(crate::W<TMR3SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR3SET_SPEC>;
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
impl core::convert::From<crate::W<TMR3SET_SPEC>> for W {
    fn from(writer: crate::W<TMR3SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR3` reader - "]
pub struct TMR3_R(crate::FieldReader<u32, u32>);
impl TMR3_R {
    pub(crate) fn new(bits: u32) -> Self {
        TMR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR3` writer - "]
pub struct TMR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3_W<'a> {
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
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr3(&mut self) -> TMR3_W {
        TMR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMR3SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr3set](index.html) module"]
pub struct TMR3SET_SPEC;
impl crate::RegisterSpec for TMR3SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr3set::R](R) reader structure"]
impl crate::Readable for TMR3SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr3set::W](W) writer structure"]
impl crate::Writable for TMR3SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TMR3SET to value 0"]
impl crate::Resettable for TMR3SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
