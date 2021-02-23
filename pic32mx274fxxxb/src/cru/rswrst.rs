#[doc = "Register `RSWRST` reader"]
pub struct R(crate::R<RSWRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSWRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RSWRST_SPEC>> for R {
    fn from(reader: crate::R<RSWRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSWRST` writer"]
pub struct W(crate::W<RSWRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSWRST_SPEC>;
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
impl core::convert::From<crate::W<RSWRST_SPEC>> for W {
    fn from(writer: crate::W<RSWRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - "]
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRST` writer - "]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSWRST register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswrst](index.html) module"]
pub struct RSWRST_SPEC;
impl crate::RegisterSpec for RSWRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rswrst::R](R) reader structure"]
impl crate::Readable for RSWRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rswrst::W](W) writer structure"]
impl crate::Writable for RSWRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSWRST to value 0"]
impl crate::Resettable for RSWRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
