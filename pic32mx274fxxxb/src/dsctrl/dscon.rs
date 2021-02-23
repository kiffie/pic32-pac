#[doc = "Register `DSCON` reader"]
pub struct R(crate::R<DSCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DSCON_SPEC>> for R {
    fn from(reader: crate::R<DSCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCON` writer"]
pub struct W(crate::W<DSCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCON_SPEC>;
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
impl core::convert::From<crate::W<DSCON_SPEC>> for W {
    fn from(writer: crate::W<DSCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELEASE` reader - "]
pub struct RELEASE_R(crate::FieldReader<bool, bool>);
impl RELEASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RELEASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELEASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELEASE` writer - "]
pub struct RELEASE_W<'a> {
    w: &'a mut W,
}
impl<'a> RELEASE_W<'a> {
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
#[doc = "Field `DSBOR` reader - "]
pub struct DSBOR_R(crate::FieldReader<bool, bool>);
impl DSBOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSBOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSBOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSBOR` writer - "]
pub struct DSBOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSBOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `WAKEDIS` reader - "]
pub struct WAKEDIS_R(crate::FieldReader<bool, bool>);
impl WAKEDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEDIS` writer - "]
pub struct WAKEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEDIS_W<'a> {
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
#[doc = "Field `RTCCWDIS` reader - "]
pub struct RTCCWDIS_R(crate::FieldReader<bool, bool>);
impl RTCCWDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCWDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCWDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCWDIS` writer - "]
pub struct RTCCWDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCWDIS_W<'a> {
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
#[doc = "Field `RTCDIS` reader - "]
pub struct RTCDIS_R(crate::FieldReader<bool, bool>);
impl RTCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCDIS` writer - "]
pub struct RTCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCDIS_W<'a> {
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
#[doc = "Field `DSGPREN` reader - "]
pub struct DSGPREN_R(crate::FieldReader<bool, bool>);
impl DSGPREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSGPREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSGPREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSGPREN` writer - "]
pub struct DSGPREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSGPREN_W<'a> {
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
#[doc = "Field `DSEN` reader - "]
pub struct DSEN_R(crate::FieldReader<bool, bool>);
impl DSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSEN` writer - "]
pub struct DSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSEN_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn release(&self) -> RELEASE_R {
        RELEASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dsbor(&self) -> DSBOR_R {
        DSBOR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wakedis(&self) -> WAKEDIS_R {
        WAKEDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtccwdis(&self) -> RTCCWDIS_R {
        RTCCWDIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtcdis(&self) -> RTCDIS_R {
        RTCDIS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dsgpren(&self) -> DSGPREN_R {
        DSGPREN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn release(&mut self) -> RELEASE_W {
        RELEASE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dsbor(&mut self) -> DSBOR_W {
        DSBOR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wakedis(&mut self) -> WAKEDIS_W {
        WAKEDIS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtccwdis(&mut self) -> RTCCWDIS_W {
        RTCCWDIS_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rtcdis(&mut self) -> RTCDIS_W {
        RTCDIS_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dsgpren(&mut self) -> DSGPREN_W {
        DSGPREN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dsen(&mut self) -> DSEN_W {
        DSEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscon](index.html) module"]
pub struct DSCON_SPEC;
impl crate::RegisterSpec for DSCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscon::R](R) reader structure"]
impl crate::Readable for DSCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscon::W](W) writer structure"]
impl crate::Writable for DSCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSCON to value 0"]
impl crate::Resettable for DSCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
