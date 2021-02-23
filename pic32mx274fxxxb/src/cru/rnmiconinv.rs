#[doc = "Register `RNMICONINV` reader"]
pub struct R(crate::R<RNMICONINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNMICONINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RNMICONINV_SPEC>> for R {
    fn from(reader: crate::R<RNMICONINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNMICONINV` writer"]
pub struct W(crate::W<RNMICONINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNMICONINV_SPEC>;
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
impl core::convert::From<crate::W<RNMICONINV_SPEC>> for W {
    fn from(writer: crate::W<RNMICONINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NMICNT` reader - "]
pub struct NMICNT_R(crate::FieldReader<u16, u16>);
impl NMICNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        NMICNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMICNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMICNT` writer - "]
pub struct NMICNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NMICNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Field `WDTS` reader - "]
pub struct WDTS_R(crate::FieldReader<bool, bool>);
impl WDTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTS` writer - "]
pub struct WDTS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CF` reader - "]
pub struct CF_R(crate::FieldReader<bool, bool>);
impl CF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CF` writer - "]
pub struct CF_W<'a> {
    w: &'a mut W,
}
impl<'a> CF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `HLVD` reader - "]
pub struct HLVD_R(crate::FieldReader<bool, bool>);
impl HLVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        HLVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HLVD` writer - "]
pub struct HLVD_W<'a> {
    w: &'a mut W,
}
impl<'a> HLVD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `GNMI` reader - "]
pub struct GNMI_R(crate::FieldReader<bool, bool>);
impl GNMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        GNMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GNMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GNMI` writer - "]
pub struct GNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> GNMI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SWNMI` reader - "]
pub struct SWNMI_R(crate::FieldReader<bool, bool>);
impl SWNMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWNMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWNMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWNMI` writer - "]
pub struct SWNMI_W<'a> {
    w: &'a mut W,
}
impl<'a> SWNMI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `WDTO` reader - "]
pub struct WDTO_R(crate::FieldReader<bool, bool>);
impl WDTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTO` writer - "]
pub struct WDTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nmicnt(&self) -> NMICNT_R {
        NMICNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wdts(&self) -> WDTS_R {
        WDTS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hlvd(&self) -> HLVD_R {
        HLVD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gnmi(&self) -> GNMI_R {
        GNMI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swnmi(&self) -> SWNMI_R {
        SWNMI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdto(&self) -> WDTO_R {
        WDTO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn nmicnt(&mut self) -> NMICNT_W {
        NMICNT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wdts(&mut self) -> WDTS_W {
        WDTS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hlvd(&mut self) -> HLVD_W {
        HLVD_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gnmi(&mut self) -> GNMI_W {
        GNMI_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swnmi(&mut self) -> SWNMI_W {
        SWNMI_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wdto(&mut self) -> WDTO_W {
        WDTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNMICONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rnmiconinv](index.html) module"]
pub struct RNMICONINV_SPEC;
impl crate::RegisterSpec for RNMICONINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rnmiconinv::R](R) reader structure"]
impl crate::Readable for RNMICONINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rnmiconinv::W](W) writer structure"]
impl crate::Writable for RNMICONINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNMICONINV to value 0"]
impl crate::Resettable for RNMICONINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
