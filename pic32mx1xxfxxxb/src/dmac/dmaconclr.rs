#[doc = "Register `DMACONCLR` reader"]
pub struct R(crate::R<DMACONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMACONCLR_SPEC>> for R {
    fn from(reader: crate::R<DMACONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACONCLR` writer"]
pub struct W(crate::W<DMACONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACONCLR_SPEC>;
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
impl core::convert::From<crate::W<DMACONCLR_SPEC>> for W {
    fn from(writer: crate::W<DMACONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMABUSY` reader - "]
pub struct DMABUSY_R(crate::FieldReader<bool, bool>);
impl DMABUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMABUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMABUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMABUSY` writer - "]
pub struct DMABUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABUSY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SUSPEND` reader - "]
pub struct SUSPEND_R(crate::FieldReader<bool, bool>);
impl SUSPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPEND` writer - "]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `FRZ` reader - "]
pub struct FRZ_R(crate::FieldReader<bool, bool>);
impl FRZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ` writer - "]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `ON` reader - "]
pub struct ON_R(crate::FieldReader<bool, bool>);
impl ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ON` writer - "]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dmabusy(&self) -> DMABUSY_R {
        DMABUSY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dmabusy(&mut self) -> DMABUSY_W {
        DMABUSY_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMACONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaconclr](index.html) module"]
pub struct DMACONCLR_SPEC;
impl crate::RegisterSpec for DMACONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaconclr::R](R) reader structure"]
impl crate::Readable for DMACONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaconclr::W](W) writer structure"]
impl crate::Writable for DMACONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACONCLR to value 0"]
impl crate::Resettable for DMACONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
