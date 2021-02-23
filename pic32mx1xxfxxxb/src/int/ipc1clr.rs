#[doc = "Register `IPC1CLR` reader"]
pub struct R(crate::R<IPC1CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC1CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC1CLR_SPEC>> for R {
    fn from(reader: crate::R<IPC1CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC1CLR` writer"]
pub struct W(crate::W<IPC1CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC1CLR_SPEC>;
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
impl core::convert::From<crate::W<IPC1CLR_SPEC>> for W {
    fn from(writer: crate::W<IPC1CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T1IS` reader - "]
pub struct T1IS_R(crate::FieldReader<u8, u8>);
impl T1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        T1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1IS` writer - "]
pub struct T1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `T1IP` reader - "]
pub struct T1IP_R(crate::FieldReader<u8, u8>);
impl T1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        T1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1IP` writer - "]
pub struct T1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `IC1IS` reader - "]
pub struct IC1IS_R(crate::FieldReader<u8, u8>);
impl IC1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1IS` writer - "]
pub struct IC1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IC1IP` reader - "]
pub struct IC1IP_R(crate::FieldReader<u8, u8>);
impl IC1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC1IP` writer - "]
pub struct IC1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `OC1IS` reader - "]
pub struct OC1IS_R(crate::FieldReader<u8, u8>);
impl OC1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1IS` writer - "]
pub struct OC1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `OC1IP` reader - "]
pub struct OC1IP_R(crate::FieldReader<u8, u8>);
impl OC1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC1IP` writer - "]
pub struct OC1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `INT1IS` reader - "]
pub struct INT1IS_R(crate::FieldReader<u8, u8>);
impl INT1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1IS` writer - "]
pub struct INT1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `INT1IP` reader - "]
pub struct INT1IP_R(crate::FieldReader<u8, u8>);
impl INT1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT1IP` writer - "]
pub struct INT1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT1IP_W<'a> {
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
    pub fn t1is(&self) -> T1IS_R {
        T1IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t1ip(&self) -> T1IP_R {
        T1IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic1is(&self) -> IC1IS_R {
        IC1IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic1ip(&self) -> IC1IP_R {
        IC1IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc1is(&self) -> OC1IS_R {
        OC1IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc1ip(&self) -> OC1IP_R {
        OC1IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int1is(&self) -> INT1IS_R {
        INT1IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int1ip(&self) -> INT1IP_R {
        INT1IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t1is(&mut self) -> T1IS_W {
        T1IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t1ip(&mut self) -> T1IP_W {
        T1IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic1is(&mut self) -> IC1IS_W {
        IC1IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic1ip(&mut self) -> IC1IP_W {
        IC1IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc1is(&mut self) -> OC1IS_W {
        OC1IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc1ip(&mut self) -> OC1IP_W {
        OC1IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int1is(&mut self) -> INT1IS_W {
        INT1IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int1ip(&mut self) -> INT1IP_W {
        INT1IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC1CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc1clr](index.html) module"]
pub struct IPC1CLR_SPEC;
impl crate::RegisterSpec for IPC1CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc1clr::R](R) reader structure"]
impl crate::Readable for IPC1CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc1clr::W](W) writer structure"]
impl crate::Writable for IPC1CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC1CLR to value 0"]
impl crate::Resettable for IPC1CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
