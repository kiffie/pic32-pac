#[doc = "Register `IPC0INV` reader"]
pub struct R(crate::R<IPC0INV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC0INV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC0INV_SPEC>> for R {
    fn from(reader: crate::R<IPC0INV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC0INV` writer"]
pub struct W(crate::W<IPC0INV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC0INV_SPEC>;
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
impl core::convert::From<crate::W<IPC0INV_SPEC>> for W {
    fn from(writer: crate::W<IPC0INV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIS` reader - "]
pub struct CTIS_R(crate::FieldReader<u8, u8>);
impl CTIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIS` writer - "]
pub struct CTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `CTIP` reader - "]
pub struct CTIP_R(crate::FieldReader<u8, u8>);
impl CTIP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIP` writer - "]
pub struct CTIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `CS0IS` reader - "]
pub struct CS0IS_R(crate::FieldReader<u8, u8>);
impl CS0IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS0IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS0IS` writer - "]
pub struct CS0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `CS0IP` reader - "]
pub struct CS0IP_R(crate::FieldReader<u8, u8>);
impl CS0IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS0IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS0IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS0IP` writer - "]
pub struct CS0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `CS1IS` reader - "]
pub struct CS1IS_R(crate::FieldReader<u8, u8>);
impl CS1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS1IS` writer - "]
pub struct CS1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CS1IP` reader - "]
pub struct CS1IP_R(crate::FieldReader<u8, u8>);
impl CS1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS1IP` writer - "]
pub struct CS1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `INT0IS` reader - "]
pub struct INT0IS_R(crate::FieldReader<u8, u8>);
impl INT0IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT0IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT0IS` writer - "]
pub struct INT0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `INT0IP` reader - "]
pub struct INT0IP_R(crate::FieldReader<u8, u8>);
impl INT0IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT0IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT0IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT0IP` writer - "]
pub struct INT0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT0IP_W<'a> {
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
    pub fn ctis(&self) -> CTIS_R {
        CTIS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn ctip(&self) -> CTIP_R {
        CTIP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs0is(&self) -> CS0IS_R {
        CS0IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn cs0ip(&self) -> CS0IP_R {
        CS0IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cs1is(&self) -> CS1IS_R {
        CS1IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn cs1ip(&self) -> CS1IP_R {
        CS1IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int0is(&self) -> INT0IS_R {
        INT0IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int0ip(&self) -> INT0IP_R {
        INT0IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ctis(&mut self) -> CTIS_W {
        CTIS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn ctip(&mut self) -> CTIP_W {
        CTIP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs0is(&mut self) -> CS0IS_W {
        CS0IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn cs0ip(&mut self) -> CS0IP_W {
        CS0IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cs1is(&mut self) -> CS1IS_W {
        CS1IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn cs1ip(&mut self) -> CS1IP_W {
        CS1IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int0is(&mut self) -> INT0IS_W {
        INT0IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int0ip(&mut self) -> INT0IP_W {
        INT0IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC0INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc0inv](index.html) module"]
pub struct IPC0INV_SPEC;
impl crate::RegisterSpec for IPC0INV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc0inv::R](R) reader structure"]
impl crate::Readable for IPC0INV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc0inv::W](W) writer structure"]
impl crate::Writable for IPC0INV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC0INV to value 0"]
impl crate::Resettable for IPC0INV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
