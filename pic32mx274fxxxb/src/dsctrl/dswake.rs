#[doc = "Register `DSWAKE` reader"]
pub struct R(crate::R<DSWAKE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSWAKE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DSWAKE_SPEC>> for R {
    fn from(reader: crate::R<DSWAKE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSWAKE` writer"]
pub struct W(crate::W<DSWAKE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSWAKE_SPEC>;
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
impl core::convert::From<crate::W<DSWAKE_SPEC>> for W {
    fn from(writer: crate::W<DSWAKE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSMCLR` reader - "]
pub struct DSMCLR_R(crate::FieldReader<bool, bool>);
impl DSMCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSMCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSMCLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSMCLR` writer - "]
pub struct DSMCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSMCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DSRTC` reader - "]
pub struct DSRTC_R(crate::FieldReader<bool, bool>);
impl DSRTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSRTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRTC` writer - "]
pub struct DSRTC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DSWDT` reader - "]
pub struct DSWDT_R(crate::FieldReader<bool, bool>);
impl DSWDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSWDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSWDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSWDT` writer - "]
pub struct DSWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSWDT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DSFLT` reader - "]
pub struct DSFLT_R(crate::FieldReader<bool, bool>);
impl DSFLT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSFLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSFLT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSFLT` writer - "]
pub struct DSFLT_W<'a> {
    w: &'a mut W,
}
impl<'a> DSFLT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DSINT0` reader - "]
pub struct DSINT0_R(crate::FieldReader<bool, bool>);
impl DSINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSINT0` writer - "]
pub struct DSINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DSINT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dsmclr(&self) -> DSMCLR_R {
        DSMCLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dsrtc(&self) -> DSRTC_R {
        DSRTC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dswdt(&self) -> DSWDT_R {
        DSWDT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dsflt(&self) -> DSFLT_R {
        DSFLT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dsint0(&self) -> DSINT0_R {
        DSINT0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dsmclr(&mut self) -> DSMCLR_W {
        DSMCLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dsrtc(&mut self) -> DSRTC_W {
        DSRTC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dswdt(&mut self) -> DSWDT_W {
        DSWDT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dsflt(&mut self) -> DSFLT_W {
        DSFLT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dsint0(&mut self) -> DSINT0_W {
        DSINT0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSWAKE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dswake](index.html) module"]
pub struct DSWAKE_SPEC;
impl crate::RegisterSpec for DSWAKE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dswake::R](R) reader structure"]
impl crate::Readable for DSWAKE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dswake::W](W) writer structure"]
impl crate::Writable for DSWAKE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSWAKE to value 0"]
impl crate::Resettable for DSWAKE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
