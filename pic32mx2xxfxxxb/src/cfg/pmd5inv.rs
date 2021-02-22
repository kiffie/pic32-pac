#[doc = "Register `PMD5INV` reader"]
pub struct R(crate::R<PMD5INV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMD5INV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMD5INV_SPEC>> for R {
    fn from(reader: crate::R<PMD5INV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMD5INV` writer"]
pub struct W(crate::W<PMD5INV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMD5INV_SPEC>;
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
impl core::convert::From<crate::W<PMD5INV_SPEC>> for W {
    fn from(writer: crate::W<PMD5INV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U1MD` reader - "]
pub struct U1MD_R(crate::FieldReader<bool, bool>);
impl U1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        U1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1MD` writer - "]
pub struct U1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> U1MD_W<'a> {
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
#[doc = "Field `U2MD` reader - "]
pub struct U2MD_R(crate::FieldReader<bool, bool>);
impl U2MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        U2MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2MD` writer - "]
pub struct U2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> U2MD_W<'a> {
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
#[doc = "Field `SPI1MD` reader - "]
pub struct SPI1MD_R(crate::FieldReader<bool, bool>);
impl SPI1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1MD` writer - "]
pub struct SPI1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1MD_W<'a> {
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
#[doc = "Field `SPI2MD` reader - "]
pub struct SPI2MD_R(crate::FieldReader<bool, bool>);
impl SPI2MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2MD` writer - "]
pub struct SPI2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2MD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `I2C1MD` reader - "]
pub struct I2C1MD_R(crate::FieldReader<bool, bool>);
impl I2C1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1MD` writer - "]
pub struct I2C1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1MD_W<'a> {
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
#[doc = "Field `I2C2MD` reader - "]
pub struct I2C2MD_R(crate::FieldReader<bool, bool>);
impl I2C2MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2MD` writer - "]
pub struct I2C2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2MD_W<'a> {
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
#[doc = "Field `USB1MD` reader - "]
pub struct USB1MD_R(crate::FieldReader<bool, bool>);
impl USB1MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB1MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB1MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB1MD` writer - "]
pub struct USB1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1MD_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u1md(&self) -> U1MD_R {
        U1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn u2md(&self) -> U2MD_R {
        U2MD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi1md(&self) -> SPI1MD_R {
        SPI1MD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi2md(&self) -> SPI2MD_R {
        SPI2MD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2c1md(&self) -> I2C1MD_R {
        I2C1MD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2c2md(&self) -> I2C2MD_R {
        I2C2MD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usb1md(&self) -> USB1MD_R {
        USB1MD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u1md(&mut self) -> U1MD_W {
        U1MD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn u2md(&mut self) -> U2MD_W {
        U2MD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi1md(&mut self) -> SPI1MD_W {
        SPI1MD_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi2md(&mut self) -> SPI2MD_W {
        SPI2MD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2c1md(&mut self) -> I2C1MD_W {
        I2C1MD_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2c2md(&mut self) -> I2C2MD_W {
        I2C2MD_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usb1md(&mut self) -> USB1MD_W {
        USB1MD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMD5INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmd5inv](index.html) module"]
pub struct PMD5INV_SPEC;
impl crate::RegisterSpec for PMD5INV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmd5inv::R](R) reader structure"]
impl crate::Readable for PMD5INV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmd5inv::W](W) writer structure"]
impl crate::Writable for PMD5INV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMD5INV to value 0"]
impl crate::Resettable for PMD5INV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
