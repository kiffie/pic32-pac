#[doc = "Register `IPC5CLR` reader"]
pub struct R(crate::R<IPC5CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC5CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC5CLR_SPEC>> for R {
    fn from(reader: crate::R<IPC5CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC5CLR` writer"]
pub struct W(crate::W<IPC5CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC5CLR_SPEC>;
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
impl core::convert::From<crate::W<IPC5CLR_SPEC>> for W {
    fn from(writer: crate::W<IPC5CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T5IS` reader - "]
pub struct T5IS_R(crate::FieldReader<u8, u8>);
impl T5IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        T5IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T5IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T5IS` writer - "]
pub struct T5IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T5IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `T5IP` reader - "]
pub struct T5IP_R(crate::FieldReader<u8, u8>);
impl T5IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        T5IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T5IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T5IP` writer - "]
pub struct T5IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T5IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `IC5IS` reader - "]
pub struct IC5IS_R(crate::FieldReader<u8, u8>);
impl IC5IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC5IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC5IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC5IS` writer - "]
pub struct IC5IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IC5IP` reader - "]
pub struct IC5IP_R(crate::FieldReader<u8, u8>);
impl IC5IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC5IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC5IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC5IP` writer - "]
pub struct IC5IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC5IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `OC5IS` reader - "]
pub struct OC5IS_R(crate::FieldReader<u8, u8>);
impl OC5IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC5IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5IS` writer - "]
pub struct OC5IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `OC5IP` reader - "]
pub struct OC5IP_R(crate::FieldReader<u8, u8>);
impl OC5IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC5IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC5IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC5IP` writer - "]
pub struct OC5IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC5IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `AD1IS` reader - "]
pub struct AD1IS_R(crate::FieldReader<u8, u8>);
impl AD1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        AD1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1IS` writer - "]
pub struct AD1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `AD1IP` reader - "]
pub struct AD1IP_R(crate::FieldReader<u8, u8>);
impl AD1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        AD1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD1IP` writer - "]
pub struct AD1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> AD1IP_W<'a> {
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
    pub fn t5is(&self) -> T5IS_R {
        T5IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t5ip(&self) -> T5IP_R {
        T5IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic5is(&self) -> IC5IS_R {
        IC5IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic5ip(&self) -> IC5IP_R {
        IC5IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc5is(&self) -> OC5IS_R {
        OC5IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc5ip(&self) -> OC5IP_R {
        OC5IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ad1is(&self) -> AD1IS_R {
        AD1IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn ad1ip(&self) -> AD1IP_R {
        AD1IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t5is(&mut self) -> T5IS_W {
        T5IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t5ip(&mut self) -> T5IP_W {
        T5IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic5is(&mut self) -> IC5IS_W {
        IC5IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic5ip(&mut self) -> IC5IP_W {
        IC5IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc5is(&mut self) -> OC5IS_W {
        OC5IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc5ip(&mut self) -> OC5IP_W {
        OC5IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ad1is(&mut self) -> AD1IS_W {
        AD1IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn ad1ip(&mut self) -> AD1IP_W {
        AD1IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC5CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc5clr](index.html) module"]
pub struct IPC5CLR_SPEC;
impl crate::RegisterSpec for IPC5CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc5clr::R](R) reader structure"]
impl crate::Readable for IPC5CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc5clr::W](W) writer structure"]
impl crate::Writable for IPC5CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC5CLR to value 0"]
impl crate::Resettable for IPC5CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
