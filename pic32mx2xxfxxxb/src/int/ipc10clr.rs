#[doc = "Register `IPC10CLR` reader"]
pub struct R(crate::R<IPC10CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC10CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC10CLR_SPEC>> for R {
    fn from(reader: crate::R<IPC10CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC10CLR` writer"]
pub struct W(crate::W<IPC10CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC10CLR_SPEC>;
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
impl core::convert::From<crate::W<IPC10CLR_SPEC>> for W {
    fn from(writer: crate::W<IPC10CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA0IS` reader - "]
pub struct DMA0IS_R(crate::FieldReader<u8, u8>);
impl DMA0IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA0IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA0IS` writer - "]
pub struct DMA0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `DMA0IP` reader - "]
pub struct DMA0IP_R(crate::FieldReader<u8, u8>);
impl DMA0IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA0IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA0IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA0IP` writer - "]
pub struct DMA0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `DMA1IS` reader - "]
pub struct DMA1IS_R(crate::FieldReader<u8, u8>);
impl DMA1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1IS` writer - "]
pub struct DMA1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `DMA1IP` reader - "]
pub struct DMA1IP_R(crate::FieldReader<u8, u8>);
impl DMA1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1IP` writer - "]
pub struct DMA1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `DMA2IS` reader - "]
pub struct DMA2IS_R(crate::FieldReader<u8, u8>);
impl DMA2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2IS` writer - "]
pub struct DMA2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DMA2IP` reader - "]
pub struct DMA2IP_R(crate::FieldReader<u8, u8>);
impl DMA2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2IP` writer - "]
pub struct DMA2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `DMA3IS` reader - "]
pub struct DMA3IS_R(crate::FieldReader<u8, u8>);
impl DMA3IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA3IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA3IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA3IS` writer - "]
pub struct DMA3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `DMA3IP` reader - "]
pub struct DMA3IP_R(crate::FieldReader<u8, u8>);
impl DMA3IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMA3IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA3IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA3IP` writer - "]
pub struct DMA3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA3IP_W<'a> {
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
    pub fn dma0is(&self) -> DMA0IS_R {
        DMA0IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn dma0ip(&self) -> DMA0IP_R {
        DMA0IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dma1is(&self) -> DMA1IS_R {
        DMA1IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn dma1ip(&self) -> DMA1IP_R {
        DMA1IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dma2is(&self) -> DMA2IS_R {
        DMA2IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn dma2ip(&self) -> DMA2IP_R {
        DMA2IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn dma3is(&self) -> DMA3IS_R {
        DMA3IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn dma3ip(&self) -> DMA3IP_R {
        DMA3IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dma0is(&mut self) -> DMA0IS_W {
        DMA0IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn dma0ip(&mut self) -> DMA0IP_W {
        DMA0IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dma1is(&mut self) -> DMA1IS_W {
        DMA1IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn dma1ip(&mut self) -> DMA1IP_W {
        DMA1IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dma2is(&mut self) -> DMA2IS_W {
        DMA2IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn dma2ip(&mut self) -> DMA2IP_W {
        DMA2IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn dma3is(&mut self) -> DMA3IS_W {
        DMA3IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn dma3ip(&mut self) -> DMA3IP_W {
        DMA3IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC10CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc10clr](index.html) module"]
pub struct IPC10CLR_SPEC;
impl crate::RegisterSpec for IPC10CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc10clr::R](R) reader structure"]
impl crate::Readable for IPC10CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc10clr::W](W) writer structure"]
impl crate::Writable for IPC10CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC10CLR to value 0"]
impl crate::Resettable for IPC10CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
