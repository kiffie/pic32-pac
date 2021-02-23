#[doc = "Register `IPC4SET` reader"]
pub struct R(crate::R<IPC4SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC4SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC4SET_SPEC>> for R {
    fn from(reader: crate::R<IPC4SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC4SET` writer"]
pub struct W(crate::W<IPC4SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC4SET_SPEC>;
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
impl core::convert::From<crate::W<IPC4SET_SPEC>> for W {
    fn from(writer: crate::W<IPC4SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T4IS` reader - "]
pub struct T4IS_R(crate::FieldReader<u8, u8>);
impl T4IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        T4IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T4IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T4IS` writer - "]
pub struct T4IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T4IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `T4IP` reader - "]
pub struct T4IP_R(crate::FieldReader<u8, u8>);
impl T4IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        T4IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T4IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T4IP` writer - "]
pub struct T4IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T4IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `IC4IS` reader - "]
pub struct IC4IS_R(crate::FieldReader<u8, u8>);
impl IC4IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC4IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4IS` writer - "]
pub struct IC4IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IC4IP` reader - "]
pub struct IC4IP_R(crate::FieldReader<u8, u8>);
impl IC4IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC4IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC4IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC4IP` writer - "]
pub struct IC4IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC4IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `OC4IS` reader - "]
pub struct OC4IS_R(crate::FieldReader<u8, u8>);
impl OC4IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC4IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4IS` writer - "]
pub struct OC4IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `OC4IP` reader - "]
pub struct OC4IP_R(crate::FieldReader<u8, u8>);
impl OC4IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC4IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC4IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC4IP` writer - "]
pub struct OC4IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC4IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `INT4IS` reader - "]
pub struct INT4IS_R(crate::FieldReader<u8, u8>);
impl INT4IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT4IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT4IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4IS` writer - "]
pub struct INT4IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `INT4IP` reader - "]
pub struct INT4IP_R(crate::FieldReader<u8, u8>);
impl INT4IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT4IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT4IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT4IP` writer - "]
pub struct INT4IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT4IP_W<'a> {
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
    pub fn t4is(&self) -> T4IS_R {
        T4IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t4ip(&self) -> T4IP_R {
        T4IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic4is(&self) -> IC4IS_R {
        IC4IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic4ip(&self) -> IC4IP_R {
        IC4IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc4is(&self) -> OC4IS_R {
        OC4IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc4ip(&self) -> OC4IP_R {
        OC4IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int4is(&self) -> INT4IS_R {
        INT4IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int4ip(&self) -> INT4IP_R {
        INT4IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t4is(&mut self) -> T4IS_W {
        T4IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t4ip(&mut self) -> T4IP_W {
        T4IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic4is(&mut self) -> IC4IS_W {
        IC4IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic4ip(&mut self) -> IC4IP_W {
        IC4IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc4is(&mut self) -> OC4IS_W {
        OC4IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc4ip(&mut self) -> OC4IP_W {
        OC4IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int4is(&mut self) -> INT4IS_W {
        INT4IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int4ip(&mut self) -> INT4IP_W {
        INT4IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC4SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc4set](index.html) module"]
pub struct IPC4SET_SPEC;
impl crate::RegisterSpec for IPC4SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc4set::R](R) reader structure"]
impl crate::Readable for IPC4SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc4set::W](W) writer structure"]
impl crate::Writable for IPC4SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC4SET to value 0"]
impl crate::Resettable for IPC4SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
