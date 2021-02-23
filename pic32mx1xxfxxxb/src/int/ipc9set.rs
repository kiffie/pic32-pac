#[doc = "Register `IPC9SET` reader"]
pub struct R(crate::R<IPC9SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC9SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC9SET_SPEC>> for R {
    fn from(reader: crate::R<IPC9SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC9SET` writer"]
pub struct W(crate::W<IPC9SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC9SET_SPEC>;
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
impl core::convert::From<crate::W<IPC9SET_SPEC>> for W {
    fn from(writer: crate::W<IPC9SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI2IS` reader - "]
pub struct SPI2IS_R(crate::FieldReader<u8, u8>);
impl SPI2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2IS` writer - "]
pub struct SPI2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `SPI2IP` reader - "]
pub struct SPI2IP_R(crate::FieldReader<u8, u8>);
impl SPI2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2IP` writer - "]
pub struct SPI2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `U2IS` reader - "]
pub struct U2IS_R(crate::FieldReader<u8, u8>);
impl U2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        U2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2IS` writer - "]
pub struct U2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> U2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `U2IP` reader - "]
pub struct U2IP_R(crate::FieldReader<u8, u8>);
impl U2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        U2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2IP` writer - "]
pub struct U2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> U2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `I2C2IS` reader - "]
pub struct I2C2IS_R(crate::FieldReader<u8, u8>);
impl I2C2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2IS` writer - "]
pub struct I2C2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `I2C2IP` reader - "]
pub struct I2C2IP_R(crate::FieldReader<u8, u8>);
impl I2C2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2IP` writer - "]
pub struct I2C2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `CTMUIS` reader - "]
pub struct CTMUIS_R(crate::FieldReader<u8, u8>);
impl CTMUIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTMUIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTMUIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTMUIS` writer - "]
pub struct CTMUIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CTMUIP` reader - "]
pub struct CTMUIP_R(crate::FieldReader<u8, u8>);
impl CTMUIP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTMUIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTMUIP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTMUIP` writer - "]
pub struct CTMUIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi2is(&self) -> SPI2IS_R {
        SPI2IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn spi2ip(&self) -> SPI2IP_R {
        SPI2IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn u2is(&self) -> U2IS_R {
        U2IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn u2ip(&self) -> U2IP_R {
        U2IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2c2is(&self) -> I2C2IS_R {
        I2C2IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn i2c2ip(&self) -> I2C2IP_R {
        I2C2IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ctmuis(&self) -> CTMUIS_R {
        CTMUIS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn ctmuip(&self) -> CTMUIP_R {
        CTMUIP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi2is(&mut self) -> SPI2IS_W {
        SPI2IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn spi2ip(&mut self) -> SPI2IP_W {
        SPI2IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn u2is(&mut self) -> U2IS_W {
        U2IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn u2ip(&mut self) -> U2IP_W {
        U2IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2c2is(&mut self) -> I2C2IS_W {
        I2C2IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn i2c2ip(&mut self) -> I2C2IP_W {
        I2C2IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ctmuis(&mut self) -> CTMUIS_W {
        CTMUIS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn ctmuip(&mut self) -> CTMUIP_W {
        CTMUIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC9SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc9set](index.html) module"]
pub struct IPC9SET_SPEC;
impl crate::RegisterSpec for IPC9SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc9set::R](R) reader structure"]
impl crate::Readable for IPC9SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc9set::W](W) writer structure"]
impl crate::Writable for IPC9SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC9SET to value 0"]
impl crate::Resettable for IPC9SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
