#[doc = "Register `RTCTIME` reader"]
pub struct R(crate::R<RTCTIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTCTIME_SPEC>> for R {
    fn from(reader: crate::R<RTCTIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCTIME` writer"]
pub struct W(crate::W<RTCTIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCTIME_SPEC>;
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
impl core::convert::From<crate::W<RTCTIME_SPEC>> for W {
    fn from(writer: crate::W<RTCTIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC01` reader - "]
pub struct SEC01_R(crate::FieldReader<u8, u8>);
impl SEC01_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC01` writer - "]
pub struct SEC01_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SEC10` reader - "]
pub struct SEC10_R(crate::FieldReader<u8, u8>);
impl SEC10_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC10` writer - "]
pub struct SEC10_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `MIN01` reader - "]
pub struct MIN01_R(crate::FieldReader<u8, u8>);
impl MIN01_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIN01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN01` writer - "]
pub struct MIN01_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `MIN10` reader - "]
pub struct MIN10_R(crate::FieldReader<u8, u8>);
impl MIN10_R {
    pub(crate) fn new(bits: u8) -> Self {
        MIN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIN10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIN10` writer - "]
pub struct MIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `HR01` reader - "]
pub struct HR01_R(crate::FieldReader<u8, u8>);
impl HR01_R {
    pub(crate) fn new(bits: u8) -> Self {
        HR01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HR01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HR01` writer - "]
pub struct HR01_W<'a> {
    w: &'a mut W,
}
impl<'a> HR01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `HR10` reader - "]
pub struct HR10_R(crate::FieldReader<u8, u8>);
impl HR10_R {
    pub(crate) fn new(bits: u8) -> Self {
        HR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HR10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HR10` writer - "]
pub struct HR10_W<'a> {
    w: &'a mut W,
}
impl<'a> HR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn sec01(&self) -> SEC01_R {
        SEC01_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn min01(&self) -> MIN01_R {
        MIN01_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn hr01(&self) -> HR01_R {
        HR01_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hr10(&self) -> HR10_R {
        HR10_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn sec01(&mut self) -> SEC01_W {
        SEC01_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sec10(&mut self) -> SEC10_W {
        SEC10_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn min01(&mut self) -> MIN01_W {
        MIN01_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn min10(&mut self) -> MIN10_W {
        MIN10_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn hr01(&mut self) -> HR01_W {
        HR01_W { w: self }
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hr10(&mut self) -> HR10_W {
        HR10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCTIME register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctime](index.html) module"]
pub struct RTCTIME_SPEC;
impl crate::RegisterSpec for RTCTIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtctime::R](R) reader structure"]
impl crate::Readable for RTCTIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtctime::W](W) writer structure"]
impl crate::Writable for RTCTIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCTIME to value 0"]
impl crate::Resettable for RTCTIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
