#[doc = "Register `IPC2CLR` reader"]
pub struct R(crate::R<IPC2CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC2CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC2CLR_SPEC>> for R {
    fn from(reader: crate::R<IPC2CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC2CLR` writer"]
pub struct W(crate::W<IPC2CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC2CLR_SPEC>;
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
impl core::convert::From<crate::W<IPC2CLR_SPEC>> for W {
    fn from(writer: crate::W<IPC2CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T2IS` reader - "]
pub struct T2IS_R(crate::FieldReader<u8, u8>);
impl T2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        T2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T2IS` writer - "]
pub struct T2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> T2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `T2IP` reader - "]
pub struct T2IP_R(crate::FieldReader<u8, u8>);
impl T2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        T2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T2IP` writer - "]
pub struct T2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> T2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `IC2IS` reader - "]
pub struct IC2IS_R(crate::FieldReader<u8, u8>);
impl IC2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2IS` writer - "]
pub struct IC2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `IC2IP` reader - "]
pub struct IC2IP_R(crate::FieldReader<u8, u8>);
impl IC2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC2IP` writer - "]
pub struct IC2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IC2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `OC2IS` reader - "]
pub struct OC2IS_R(crate::FieldReader<u8, u8>);
impl OC2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2IS` writer - "]
pub struct OC2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `OC2IP` reader - "]
pub struct OC2IP_R(crate::FieldReader<u8, u8>);
impl OC2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        OC2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC2IP` writer - "]
pub struct OC2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> OC2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `INT2IS` reader - "]
pub struct INT2IS_R(crate::FieldReader<u8, u8>);
impl INT2IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT2IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT2IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2IS` writer - "]
pub struct INT2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `INT2IP` reader - "]
pub struct INT2IP_R(crate::FieldReader<u8, u8>);
impl INT2IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT2IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT2IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT2IP` writer - "]
pub struct INT2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> INT2IP_W<'a> {
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
    pub fn t2is(&self) -> T2IS_R {
        T2IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t2ip(&self) -> T2IP_R {
        T2IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic2is(&self) -> IC2IS_R {
        IC2IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic2ip(&self) -> IC2IP_R {
        IC2IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc2is(&self) -> OC2IS_R {
        OC2IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc2ip(&self) -> OC2IP_R {
        OC2IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int2is(&self) -> INT2IS_R {
        INT2IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int2ip(&self) -> INT2IP_R {
        INT2IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn t2is(&mut self) -> T2IS_W {
        T2IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn t2ip(&mut self) -> T2IP_W {
        T2IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ic2is(&mut self) -> IC2IS_W {
        IC2IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn ic2ip(&mut self) -> IC2IP_W {
        IC2IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn oc2is(&mut self) -> OC2IS_W {
        OC2IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn oc2ip(&mut self) -> OC2IP_W {
        OC2IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn int2is(&mut self) -> INT2IS_W {
        INT2IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn int2ip(&mut self) -> INT2IP_W {
        INT2IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc2clr](index.html) module"]
pub struct IPC2CLR_SPEC;
impl crate::RegisterSpec for IPC2CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc2clr::R](R) reader structure"]
impl crate::Readable for IPC2CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc2clr::W](W) writer structure"]
impl crate::Writable for IPC2CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC2CLR to value 0"]
impl crate::Resettable for IPC2CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
