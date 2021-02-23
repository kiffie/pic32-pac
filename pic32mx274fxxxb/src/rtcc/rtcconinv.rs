#[doc = "Register `RTCCONINV` reader"]
pub struct R(crate::R<RTCCONINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCONINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTCCONINV_SPEC>> for R {
    fn from(reader: crate::R<RTCCONINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCONINV` writer"]
pub struct W(crate::W<RTCCONINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCONINV_SPEC>;
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
impl core::convert::From<crate::W<RTCCONINV_SPEC>> for W {
    fn from(writer: crate::W<RTCCONINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCOE` reader - "]
pub struct RTCOE_R(crate::FieldReader<bool, bool>);
impl RTCOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOE` writer - "]
pub struct RTCOE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOE_W<'a> {
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
#[doc = "Field `HALFSEC` reader - "]
pub struct HALFSEC_R(crate::FieldReader<bool, bool>);
impl HALFSEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALFSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALFSEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALFSEC` writer - "]
pub struct HALFSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFSEC_W<'a> {
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
#[doc = "Field `RTCSYNC` reader - "]
pub struct RTCSYNC_R(crate::FieldReader<bool, bool>);
impl RTCSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSYNC` writer - "]
pub struct RTCSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSYNC_W<'a> {
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
#[doc = "Field `RTCWREN` reader - "]
pub struct RTCWREN_R(crate::FieldReader<bool, bool>);
impl RTCWREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCWREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCWREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCWREN` writer - "]
pub struct RTCWREN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCWREN_W<'a> {
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
#[doc = "Field `RTCCLKON` reader - "]
pub struct RTCCLKON_R(crate::FieldReader<bool, bool>);
impl RTCCLKON_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCCLKON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCLKON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCLKON` writer - "]
pub struct RTCCLKON_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RTCOUTSEL` reader - "]
pub struct RTCOUTSEL_R(crate::FieldReader<u8, u8>);
impl RTCOUTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCOUTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOUTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOUTSEL` writer - "]
pub struct RTCOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOUTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `RTCCLKSEL` reader - "]
pub struct RTCCLKSEL_R(crate::FieldReader<u8, u8>);
impl RTCCLKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCLKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCLKSEL` writer - "]
pub struct RTCCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `SIDL` reader - "]
pub struct SIDL_R(crate::FieldReader<bool, bool>);
impl SIDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDL` writer - "]
pub struct SIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDL_W<'a> {
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
#[doc = "Field `CAL` reader - "]
pub struct CAL_R(crate::FieldReader<u16, u16>);
impl CAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL` writer - "]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtcoe(&self) -> RTCOE_R {
        RTCOE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn halfsec(&self) -> HALFSEC_R {
        HALFSEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtcsync(&self) -> RTCSYNC_R {
        RTCSYNC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtcwren(&self) -> RTCWREN_R {
        RTCWREN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtcclkon(&self) -> RTCCLKON_R {
        RTCCLKON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn rtcoutsel(&self) -> RTCOUTSEL_R {
        RTCOUTSEL_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rtcclksel(&self) -> RTCCLKSEL_R {
        RTCCLKSEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
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
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rtcoe(&mut self) -> RTCOE_W {
        RTCOE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn halfsec(&mut self) -> HALFSEC_W {
        HALFSEC_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtcsync(&mut self) -> RTCSYNC_W {
        RTCSYNC_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtcwren(&mut self) -> RTCWREN_W {
        RTCWREN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtcclkon(&mut self) -> RTCCLKON_W {
        RTCCLKON_W { w: self }
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn rtcoutsel(&mut self) -> RTCOUTSEL_W {
        RTCOUTSEL_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rtcclksel(&mut self) -> RTCCLKSEL_W {
        RTCCLKSEL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
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
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcconinv](index.html) module"]
pub struct RTCCONINV_SPEC;
impl crate::RegisterSpec for RTCCONINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcconinv::R](R) reader structure"]
impl crate::Readable for RTCCONINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcconinv::W](W) writer structure"]
impl crate::Writable for RTCCONINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCONINV to value 0"]
impl crate::Resettable for RTCCONINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
