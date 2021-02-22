#[doc = "Register `BMXCONSET` reader"]
pub struct R(crate::R<BMXCONSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMXCONSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BMXCONSET_SPEC>> for R {
    fn from(reader: crate::R<BMXCONSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMXCONSET` writer"]
pub struct W(crate::W<BMXCONSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMXCONSET_SPEC>;
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
impl core::convert::From<crate::W<BMXCONSET_SPEC>> for W {
    fn from(writer: crate::W<BMXCONSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BMXARB` reader - "]
pub struct BMXARB_R(crate::FieldReader<u8, u8>);
impl BMXARB_R {
    pub(crate) fn new(bits: u8) -> Self {
        BMXARB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXARB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXARB` writer - "]
pub struct BMXARB_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXARB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Field `BMXWSDRM` reader - "]
pub struct BMXWSDRM_R(crate::FieldReader<bool, bool>);
impl BMXWSDRM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXWSDRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXWSDRM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXWSDRM` writer - "]
pub struct BMXWSDRM_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXWSDRM_W<'a> {
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
#[doc = "Field `BMXERRIS` reader - "]
pub struct BMXERRIS_R(crate::FieldReader<bool, bool>);
impl BMXERRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXERRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXERRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXERRIS` writer - "]
pub struct BMXERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRIS_W<'a> {
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
#[doc = "Field `BMXERRDS` reader - "]
pub struct BMXERRDS_R(crate::FieldReader<bool, bool>);
impl BMXERRDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXERRDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXERRDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXERRDS` writer - "]
pub struct BMXERRDS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRDS_W<'a> {
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
#[doc = "Field `BMXERRDMA` reader - "]
pub struct BMXERRDMA_R(crate::FieldReader<bool, bool>);
impl BMXERRDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXERRDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXERRDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXERRDMA` writer - "]
pub struct BMXERRDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRDMA_W<'a> {
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
#[doc = "Field `BMXERRICD` reader - "]
pub struct BMXERRICD_R(crate::FieldReader<bool, bool>);
impl BMXERRICD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXERRICD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXERRICD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXERRICD` writer - "]
pub struct BMXERRICD_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRICD_W<'a> {
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
#[doc = "Field `BMXERRIXI` reader - "]
pub struct BMXERRIXI_R(crate::FieldReader<bool, bool>);
impl BMXERRIXI_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXERRIXI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXERRIXI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXERRIXI` writer - "]
pub struct BMXERRIXI_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXERRIXI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `BMXCHEDMA` reader - "]
pub struct BMXCHEDMA_R(crate::FieldReader<bool, bool>);
impl BMXCHEDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMXCHEDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMXCHEDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMXCHEDMA` writer - "]
pub struct BMXCHEDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXCHEDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn bmxarb(&self) -> BMXARB_R {
        BMXARB_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxwsdrm(&self) -> BMXWSDRM_R {
        BMXWSDRM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bmxerris(&self) -> BMXERRIS_R {
        BMXERRIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn bmxerrds(&self) -> BMXERRDS_R {
        BMXERRDS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bmxerrdma(&self) -> BMXERRDMA_R {
        BMXERRDMA_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bmxerricd(&self) -> BMXERRICD_R {
        BMXERRICD_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bmxerrixi(&self) -> BMXERRIXI_R {
        BMXERRIXI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bmxchedma(&self) -> BMXCHEDMA_R {
        BMXCHEDMA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn bmxarb(&mut self) -> BMXARB_W {
        BMXARB_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bmxwsdrm(&mut self) -> BMXWSDRM_W {
        BMXWSDRM_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn bmxerris(&mut self) -> BMXERRIS_W {
        BMXERRIS_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn bmxerrds(&mut self) -> BMXERRDS_W {
        BMXERRDS_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn bmxerrdma(&mut self) -> BMXERRDMA_W {
        BMXERRDMA_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bmxerricd(&mut self) -> BMXERRICD_W {
        BMXERRICD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn bmxerrixi(&mut self) -> BMXERRIXI_W {
        BMXERRIXI_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bmxchedma(&mut self) -> BMXCHEDMA_W {
        BMXCHEDMA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BMXCONSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmxconset](index.html) module"]
pub struct BMXCONSET_SPEC;
impl crate::RegisterSpec for BMXCONSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmxconset::R](R) reader structure"]
impl crate::Readable for BMXCONSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmxconset::W](W) writer structure"]
impl crate::Writable for BMXCONSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMXCONSET to value 0"]
impl crate::Resettable for BMXCONSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
