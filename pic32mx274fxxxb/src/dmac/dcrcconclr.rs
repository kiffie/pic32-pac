#[doc = "Register `DCRCCONCLR` reader"]
pub struct R(crate::R<DCRCCONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCRCCONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DCRCCONCLR_SPEC>> for R {
    fn from(reader: crate::R<DCRCCONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCRCCONCLR` writer"]
pub struct W(crate::W<DCRCCONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCRCCONCLR_SPEC>;
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
impl core::convert::From<crate::W<DCRCCONCLR_SPEC>> for W {
    fn from(writer: crate::W<DCRCCONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCCH` reader - "]
pub struct CRCCH_R(crate::FieldReader<u8, u8>);
impl CRCCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CRCCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCCH` writer - "]
pub struct CRCCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Field `CRCTYP` reader - "]
pub struct CRCTYP_R(crate::FieldReader<bool, bool>);
impl CRCTYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCTYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCTYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCTYP` writer - "]
pub struct CRCTYP_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCTYP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CRCAPP` reader - "]
pub struct CRCAPP_R(crate::FieldReader<bool, bool>);
impl CRCAPP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCAPP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCAPP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCAPP` writer - "]
pub struct CRCAPP_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCAPP_W<'a> {
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
#[doc = "Field `CRCEN` reader - "]
pub struct CRCEN_R(crate::FieldReader<bool, bool>);
impl CRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCEN` writer - "]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
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
#[doc = "Field `PLEN` reader - "]
pub struct PLEN_R(crate::FieldReader<u8, u8>);
impl PLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLEN` writer - "]
pub struct PLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `BITO` reader - "]
pub struct BITO_R(crate::FieldReader<bool, bool>);
impl BITO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BITO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BITO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BITO` writer - "]
pub struct BITO_W<'a> {
    w: &'a mut W,
}
impl<'a> BITO_W<'a> {
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
#[doc = "Field `WBO` reader - "]
pub struct WBO_R(crate::FieldReader<bool, bool>);
impl WBO_R {
    pub(crate) fn new(bits: bool) -> Self {
        WBO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WBO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WBO` writer - "]
pub struct WBO_W<'a> {
    w: &'a mut W,
}
impl<'a> WBO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `BYTO` reader - "]
pub struct BYTO_R(crate::FieldReader<u8, u8>);
impl BYTO_R {
    pub(crate) fn new(bits: u8) -> Self {
        BYTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BYTO` writer - "]
pub struct BYTO_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn crcch(&self) -> CRCCH_R {
        CRCCH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crctyp(&self) -> CRCTYP_R {
        CRCTYP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crcapp(&self) -> CRCAPP_R {
        CRCAPP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn plen(&self) -> PLEN_R {
        PLEN_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn bito(&self) -> BITO_R {
        BITO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn wbo(&self) -> WBO_R {
        WBO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn byto(&self) -> BYTO_R {
        BYTO_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn crcch(&mut self) -> CRCCH_W {
        CRCCH_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crctyp(&mut self) -> CRCTYP_W {
        CRCTYP_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crcapp(&mut self) -> CRCAPP_W {
        CRCAPP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn plen(&mut self) -> PLEN_W {
        PLEN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn bito(&mut self) -> BITO_W {
        BITO_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn wbo(&mut self) -> WBO_W {
        WBO_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn byto(&mut self) -> BYTO_W {
        BYTO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCRCCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcrcconclr](index.html) module"]
pub struct DCRCCONCLR_SPEC;
impl crate::RegisterSpec for DCRCCONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcrcconclr::R](R) reader structure"]
impl crate::Readable for DCRCCONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcrcconclr::W](W) writer structure"]
impl crate::Writable for DCRCCONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCRCCONCLR to value 0"]
impl crate::Resettable for DCRCCONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
