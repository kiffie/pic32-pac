#[doc = "Register `STATINV` reader"]
pub struct R(crate::R<STATINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STATINV_SPEC>> for R {
    fn from(reader: crate::R<STATINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATINV` writer"]
pub struct W(crate::W<STATINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATINV_SPEC>;
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
impl core::convert::From<crate::W<STATINV_SPEC>> for W {
    fn from(writer: crate::W<STATINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIRBF` reader - "]
pub struct SPIRBF_R(crate::FieldReader<bool, bool>);
impl SPIRBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIRBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIRBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIRBF` writer - "]
pub struct SPIRBF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRBF_W<'a> {
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
#[doc = "Field `SPITBF` reader - "]
pub struct SPITBF_R(crate::FieldReader<bool, bool>);
impl SPITBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPITBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPITBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPITBF` writer - "]
pub struct SPITBF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPITBF_W<'a> {
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
#[doc = "Field `SPITBE` reader - "]
pub struct SPITBE_R(crate::FieldReader<bool, bool>);
impl SPITBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPITBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPITBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPITBE` writer - "]
pub struct SPITBE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPITBE_W<'a> {
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
#[doc = "Field `SPIRBE` reader - "]
pub struct SPIRBE_R(crate::FieldReader<bool, bool>);
impl SPIRBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIRBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIRBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIRBE` writer - "]
pub struct SPIRBE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIRBE_W<'a> {
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
#[doc = "Field `SPIROV` reader - "]
pub struct SPIROV_R(crate::FieldReader<bool, bool>);
impl SPIROV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIROV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIROV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIROV` writer - "]
pub struct SPIROV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIROV_W<'a> {
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
#[doc = "Field `SRMT` reader - "]
pub struct SRMT_R(crate::FieldReader<bool, bool>);
impl SRMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRMT` writer - "]
pub struct SRMT_W<'a> {
    w: &'a mut W,
}
impl<'a> SRMT_W<'a> {
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
#[doc = "Field `SPITUR` reader - "]
pub struct SPITUR_R(crate::FieldReader<bool, bool>);
impl SPITUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPITUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPITUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPITUR` writer - "]
pub struct SPITUR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPITUR_W<'a> {
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
#[doc = "Field `SPIBUSY` reader - "]
pub struct SPIBUSY_R(crate::FieldReader<bool, bool>);
impl SPIBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIBUSY` writer - "]
pub struct SPIBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIBUSY_W<'a> {
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
#[doc = "Field `FRMERR` reader - "]
pub struct FRMERR_R(crate::FieldReader<bool, bool>);
impl FRMERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMERR` writer - "]
pub struct FRMERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMERR_W<'a> {
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
#[doc = "Field `TXBUFELM` reader - "]
pub struct TXBUFELM_R(crate::FieldReader<u8, u8>);
impl TXBUFELM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXBUFELM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBUFELM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBUFELM` writer - "]
pub struct TXBUFELM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBUFELM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `RXBUFELM` reader - "]
pub struct RXBUFELM_R(crate::FieldReader<u8, u8>);
impl RXBUFELM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXBUFELM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBUFELM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBUFELM` writer - "]
pub struct RXBUFELM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFELM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spirbf(&self) -> SPIRBF_R {
        SPIRBF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spitbf(&self) -> SPITBF_R {
        SPITBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spitbe(&self) -> SPITBE_R {
        SPITBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spirbe(&self) -> SPIRBE_R {
        SPIRBE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spirov(&self) -> SPIROV_R {
        SPIROV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn srmt(&self) -> SRMT_R {
        SRMT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spitur(&self) -> SPITUR_R {
        SPITUR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spibusy(&self) -> SPIBUSY_R {
        SPIBUSY_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frmerr(&self) -> FRMERR_R {
        FRMERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn txbufelm(&self) -> TXBUFELM_R {
        TXBUFELM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rxbufelm(&self) -> RXBUFELM_R {
        RXBUFELM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spirbf(&mut self) -> SPIRBF_W {
        SPIRBF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spitbf(&mut self) -> SPITBF_W {
        SPITBF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spitbe(&mut self) -> SPITBE_W {
        SPITBE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spirbe(&mut self) -> SPIRBE_W {
        SPIRBE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spirov(&mut self) -> SPIROV_W {
        SPIROV_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn srmt(&mut self) -> SRMT_W {
        SRMT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spitur(&mut self) -> SPITUR_W {
        SPITUR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spibusy(&mut self) -> SPIBUSY_W {
        SPIBUSY_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frmerr(&mut self) -> FRMERR_W {
        FRMERR_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn txbufelm(&mut self) -> TXBUFELM_W {
        TXBUFELM_W { w: self }
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rxbufelm(&mut self) -> RXBUFELM_W {
        RXBUFELM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1STATINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statinv](index.html) module"]
pub struct STATINV_SPEC;
impl crate::RegisterSpec for STATINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statinv::R](R) reader structure"]
impl crate::Readable for STATINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [statinv::W](W) writer structure"]
impl crate::Writable for STATINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATINV to value 0"]
impl crate::Resettable for STATINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
