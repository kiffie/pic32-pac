#[doc = "Register `IPC6` reader"]
pub struct R(crate::R<IPC6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC6_SPEC>> for R {
    fn from(reader: crate::R<IPC6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC6` writer"]
pub struct W(crate::W<IPC6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC6_SPEC>;
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
impl core::convert::From<crate::W<IPC6_SPEC>> for W {
    fn from(writer: crate::W<IPC6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSCMIS` reader - "]
pub struct FSCMIS_R(crate::FieldReader<u8, u8>);
impl FSCMIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSCMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSCMIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSCMIS` writer - "]
pub struct FSCMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCMIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `FSCMIP` reader - "]
pub struct FSCMIP_R(crate::FieldReader<u8, u8>);
impl FSCMIP_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSCMIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSCMIP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSCMIP` writer - "]
pub struct FSCMIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSCMIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `RTCCIS` reader - "]
pub struct RTCCIS_R(crate::FieldReader<u8, u8>);
impl RTCCIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCCIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCIS` writer - "]
pub struct RTCCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `RTCCIP` reader - "]
pub struct RTCCIP_R(crate::FieldReader<u8, u8>);
impl RTCCIP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCCIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCCIP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCIP` writer - "]
pub struct RTCCIP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `FCEIS` reader - "]
pub struct FCEIS_R(crate::FieldReader<u8, u8>);
impl FCEIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCEIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCEIS` writer - "]
pub struct FCEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `FCEIP` reader - "]
pub struct FCEIP_R(crate::FieldReader<u8, u8>);
impl FCEIP_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCEIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCEIP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCEIP` writer - "]
pub struct FCEIP_W<'a> {
    w: &'a mut W,
}
impl<'a> FCEIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `CMP1IS` reader - "]
pub struct CMP1IS_R(crate::FieldReader<u8, u8>);
impl CMP1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1IS` writer - "]
pub struct CMP1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `CMP1IP` reader - "]
pub struct CMP1IP_R(crate::FieldReader<u8, u8>);
impl CMP1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1IP` writer - "]
pub struct CMP1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IP_W<'a> {
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
    pub fn fscmis(&self) -> FSCMIS_R {
        FSCMIS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn fscmip(&self) -> FSCMIP_R {
        FSCMIP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rtccis(&self) -> RTCCIS_R {
        RTCCIS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn rtccip(&self) -> RTCCIP_R {
        RTCCIP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fceis(&self) -> FCEIS_R {
        FCEIS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn fceip(&self) -> FCEIP_R {
        FCEIP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn cmp1is(&self) -> CMP1IS_R {
        CMP1IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn cmp1ip(&self) -> CMP1IP_R {
        CMP1IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn fscmis(&mut self) -> FSCMIS_W {
        FSCMIS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn fscmip(&mut self) -> FSCMIP_W {
        FSCMIP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rtccis(&mut self) -> RTCCIS_W {
        RTCCIS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn rtccip(&mut self) -> RTCCIP_W {
        RTCCIP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fceis(&mut self) -> FCEIS_W {
        FCEIS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn fceip(&mut self) -> FCEIP_W {
        FCEIP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn cmp1is(&mut self) -> CMP1IS_W {
        CMP1IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn cmp1ip(&mut self) -> CMP1IP_W {
        CMP1IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC6 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc6](index.html) module"]
pub struct IPC6_SPEC;
impl crate::RegisterSpec for IPC6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc6::R](R) reader structure"]
impl crate::Readable for IPC6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc6::W](W) writer structure"]
impl crate::Writable for IPC6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC6 to value 0"]
impl crate::Resettable for IPC6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
