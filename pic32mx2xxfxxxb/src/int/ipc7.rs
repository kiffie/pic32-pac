#[doc = "Register `IPC7` reader"]
pub struct R(crate::R<IPC7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC7_SPEC>> for R {
    fn from(reader: crate::R<IPC7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC7` writer"]
pub struct W(crate::W<IPC7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC7_SPEC>;
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
impl core::convert::From<crate::W<IPC7_SPEC>> for W {
    fn from(writer: crate::W<IPC7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP2IS` reader - "]
pub struct CMP2IS_R(crate::FieldReader<u8, u8>);
impl CMP2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2IS` writer - "]
pub struct CMP2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `CMP2IP` reader - "]
pub struct CMP2IP_R(crate::FieldReader<u8, u8>);
impl CMP2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2IP` writer - "]
pub struct CMP2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `CMP3IS` reader - "]
pub struct CMP3IS_R(crate::FieldReader<u8, u8>);
impl CMP3IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP3IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3IS` writer - "]
pub struct CMP3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CMP3IP` reader - "]
pub struct CMP3IP_R(crate::FieldReader<u8, u8>);
impl CMP3IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP3IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3IP` writer - "]
pub struct CMP3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `USBIS` reader - "]
pub struct USBIS_R(crate::FieldReader<u8, u8>);
impl USBIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBIS` writer - "]
pub struct USBIS_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `USBIP` reader - "]
pub struct USBIP_R(crate::FieldReader<u8, u8>);
impl USBIP_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBIP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBIP` writer - "]
pub struct USBIP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `SPI1IS` reader - "]
pub struct SPI1IS_R(crate::FieldReader<u8, u8>);
impl SPI1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1IS` writer - "]
pub struct SPI1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `SPI1IP` reader - "]
pub struct SPI1IP_R(crate::FieldReader<u8, u8>);
impl SPI1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1IP` writer - "]
pub struct SPI1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1IP_W<'a> {
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
    pub fn cmp2is(&self) -> CMP2IS_R {
        CMP2IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn cmp2ip(&self) -> CMP2IP_R {
        CMP2IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cmp3is(&self) -> CMP3IS_R {
        CMP3IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn cmp3ip(&self) -> CMP3IP_R {
        CMP3IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn usbis(&self) -> USBIS_R {
        USBIS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn usbip(&self) -> USBIP_R {
        USBIP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn spi1is(&self) -> SPI1IS_R {
        SPI1IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn spi1ip(&self) -> SPI1IP_R {
        SPI1IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cmp2is(&mut self) -> CMP2IS_W {
        CMP2IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn cmp2ip(&mut self) -> CMP2IP_W {
        CMP2IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cmp3is(&mut self) -> CMP3IS_W {
        CMP3IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn cmp3ip(&mut self) -> CMP3IP_W {
        CMP3IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn usbis(&mut self) -> USBIS_W {
        USBIS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn usbip(&mut self) -> USBIP_W {
        USBIP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn spi1is(&mut self) -> SPI1IS_W {
        SPI1IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn spi1ip(&mut self) -> SPI1IP_W {
        SPI1IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC7 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc7](index.html) module"]
pub struct IPC7_SPEC;
impl crate::RegisterSpec for IPC7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc7::R](R) reader structure"]
impl crate::Readable for IPC7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc7::W](W) writer structure"]
impl crate::Writable for IPC7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC7 to value 0"]
impl crate::Resettable for IPC7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
