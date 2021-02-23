#[doc = "Register `PWRCONCLR` reader"]
pub struct R(crate::R<PWRCONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PWRCONCLR_SPEC>> for R {
    fn from(reader: crate::R<PWRCONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCONCLR` writer"]
pub struct W(crate::W<PWRCONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCONCLR_SPEC>;
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
impl core::convert::From<crate::W<PWRCONCLR_SPEC>> for W {
    fn from(writer: crate::W<PWRCONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREGS` reader - "]
pub struct VREGS_R(crate::FieldReader<bool, bool>);
impl VREGS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREGS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREGS` writer - "]
pub struct VREGS_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGS_W<'a> {
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
    pub fn vregs(&self) -> VREGS_R {
        VREGS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vregs(&mut self) -> VREGS_W {
        VREGS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrconclr](index.html) module"]
pub struct PWRCONCLR_SPEC;
impl crate::RegisterSpec for PWRCONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrconclr::R](R) reader structure"]
impl crate::Readable for PWRCONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrconclr::W](W) writer structure"]
impl crate::Writable for PWRCONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCONCLR to value 0"]
impl crate::Resettable for PWRCONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
