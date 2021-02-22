#[doc = "Register `IPC3CLR` reader"]
pub struct R(crate::R<IPC3CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC3CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC3CLR_SPEC>> for R {
    fn from(reader: crate::R<IPC3CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC3CLR` writer"]
pub struct W(crate::W<IPC3CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC3CLR_SPEC>;
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
impl core::convert::From<crate::W<IPC3CLR_SPEC>> for W {
    fn from(writer: crate::W<IPC3CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T3IS` reader - "]
pub struct T3IS_R(crate::FieldReader<u8, u8>);
impl T3IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        T3IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T3IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T3IS` writer - "]
pub struct T3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `T3IP` reader - "]
pub struct T3IP_R(crate::FieldReader<u8, u8>);
impl T3IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        T3IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T3IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T3IP` writer - "]
pub struct T3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T3IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `IC3IS` reader - "]
pub struct IC3IS_R(crate::FieldReader<u8, u8>);
impl IC3IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3IS` writer - "]
pub struct IC3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IC3IP` reader - "]
pub struct IC3IP_R(crate::FieldReader<u8, u8>);
impl IC3IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC3IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC3IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC3IP` writer - "]
pub struct IC3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC3IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `OC3IS` reader - "]
pub struct OC3IS_R(crate::FieldReader<u8, u8>);
impl OC3IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC3IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3IS` writer - "]
pub struct OC3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `OC3IP` reader - "]
pub struct OC3IP_R(crate::FieldReader<u8, u8>);
impl OC3IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC3IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC3IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC3IP` writer - "]
pub struct OC3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC3IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `INT3IS` reader - "]
pub struct INT3IS_R(crate::FieldReader<u8, u8>);
impl INT3IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT3IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT3IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT3IS` writer - "]
pub struct INT3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `INT3IP` reader - "]
pub struct INT3IP_R(crate::FieldReader<u8, u8>);
impl INT3IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT3IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT3IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT3IP` writer - "]
pub struct INT3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT3IP_W<'a> {
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
    pub fn t3is(&self) -> T3IS_R {
        T3IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t3ip(&self) -> T3IP_R {
        T3IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic3is(&self) -> IC3IS_R {
        IC3IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic3ip(&self) -> IC3IP_R {
        IC3IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc3is(&self) -> OC3IS_R {
        OC3IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc3ip(&self) -> OC3IP_R {
        OC3IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int3is(&self) -> INT3IS_R {
        INT3IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int3ip(&self) -> INT3IP_R {
        INT3IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t3is(&mut self) -> T3IS_W {
        T3IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t3ip(&mut self) -> T3IP_W {
        T3IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic3is(&mut self) -> IC3IS_W {
        IC3IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic3ip(&mut self) -> IC3IP_W {
        IC3IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc3is(&mut self) -> OC3IS_W {
        OC3IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc3ip(&mut self) -> OC3IP_W {
        OC3IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int3is(&mut self) -> INT3IS_W {
        INT3IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int3ip(&mut self) -> INT3IP_W {
        INT3IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC3CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc3clr](index.html) module"]
pub struct IPC3CLR_SPEC;
impl crate::RegisterSpec for IPC3CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc3clr::R](R) reader structure"]
impl crate::Readable for IPC3CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc3clr::W](W) writer structure"]
impl crate::Writable for IPC3CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC3CLR to value 0"]
impl crate::Resettable for IPC3CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
