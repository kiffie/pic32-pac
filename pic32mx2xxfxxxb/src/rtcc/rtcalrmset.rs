#[doc = "Register `RTCALRMSET` reader"]
pub struct R(crate::R<RTCALRMSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCALRMSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTCALRMSET_SPEC>> for R {
    fn from(reader: crate::R<RTCALRMSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCALRMSET` writer"]
pub struct W(crate::W<RTCALRMSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCALRMSET_SPEC>;
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
impl core::convert::From<crate::W<RTCALRMSET_SPEC>> for W {
    fn from(writer: crate::W<RTCALRMSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARPT` reader - "]
pub struct ARPT_R(crate::FieldReader<u8, u8>);
impl ARPT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ARPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARPT` writer - "]
pub struct ARPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ARPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Field `AMASK` reader - "]
pub struct AMASK_R(crate::FieldReader<u8, u8>);
impl AMASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        AMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMASK` writer - "]
pub struct AMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> AMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `ALRMSYNC` reader - "]
pub struct ALRMSYNC_R(crate::FieldReader<bool, bool>);
impl ALRMSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRMSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRMSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRMSYNC` writer - "]
pub struct ALRMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRMSYNC_W<'a> {
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
#[doc = "Field `PIV` reader - "]
pub struct PIV_R(crate::FieldReader<bool, bool>);
impl PIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIV` writer - "]
pub struct PIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PIV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CHIME` reader - "]
pub struct CHIME_R(crate::FieldReader<bool, bool>);
impl CHIME_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIME` writer - "]
pub struct CHIME_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIME_W<'a> {
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
#[doc = "Field `ALRMEN` reader - "]
pub struct ALRMEN_R(crate::FieldReader<bool, bool>);
impl ALRMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRMEN` writer - "]
pub struct ALRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRMEN_W<'a> {
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
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn arpt(&self) -> ARPT_R {
        ARPT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amask(&self) -> AMASK_R {
        AMASK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn alrmsync(&self) -> ALRMSYNC_R {
        ALRMSYNC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn piv(&self) -> PIV_R {
        PIV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn chime(&self) -> CHIME_R {
        CHIME_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn alrmen(&self) -> ALRMEN_R {
        ALRMEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn arpt(&mut self) -> ARPT_W {
        ARPT_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn amask(&mut self) -> AMASK_W {
        AMASK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn alrmsync(&mut self) -> ALRMSYNC_W {
        ALRMSYNC_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn piv(&mut self) -> PIV_W {
        PIV_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn chime(&mut self) -> CHIME_W {
        CHIME_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn alrmen(&mut self) -> ALRMEN_W {
        ALRMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCALRMSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcalrmset](index.html) module"]
pub struct RTCALRMSET_SPEC;
impl crate::RegisterSpec for RTCALRMSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcalrmset::R](R) reader structure"]
impl crate::Readable for RTCALRMSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcalrmset::W](W) writer structure"]
impl crate::Writable for RTCALRMSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCALRMSET to value 0"]
impl crate::Resettable for RTCALRMSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
