#[doc = "Register `RTCDATEINV` reader"]
pub struct R(crate::R<RTCDATEINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCDATEINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTCDATEINV_SPEC>> for R {
    fn from(reader: crate::R<RTCDATEINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCDATEINV` writer"]
pub struct W(crate::W<RTCDATEINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCDATEINV_SPEC>;
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
impl core::convert::From<crate::W<RTCDATEINV_SPEC>> for W {
    fn from(writer: crate::W<RTCDATEINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDAY01` reader - "]
pub struct WDAY01_R(crate::FieldReader<u8, u8>);
impl WDAY01_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDAY01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDAY01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDAY01` writer - "]
pub struct WDAY01_W<'a> {
    w: &'a mut W,
}
impl<'a> WDAY01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Field `DAY01` reader - "]
pub struct DAY01_R(crate::FieldReader<u8, u8>);
impl DAY01_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAY01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY01` writer - "]
pub struct DAY01_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `DAY10` reader - "]
pub struct DAY10_R(crate::FieldReader<u8, u8>);
impl DAY10_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAY10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAY10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAY10` writer - "]
pub struct DAY10_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `MONTH01` reader - "]
pub struct MONTH01_R(crate::FieldReader<u8, u8>);
impl MONTH01_R {
    pub(crate) fn new(bits: u8) -> Self {
        MONTH01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH01` writer - "]
pub struct MONTH01_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `MONTH10` reader - "]
pub struct MONTH10_R(crate::FieldReader<u8, u8>);
impl MONTH10_R {
    pub(crate) fn new(bits: u8) -> Self {
        MONTH10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MONTH10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONTH10` writer - "]
pub struct MONTH10_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `YEAR01` reader - "]
pub struct YEAR01_R(crate::FieldReader<u8, u8>);
impl YEAR01_R {
    pub(crate) fn new(bits: u8) -> Self {
        YEAR01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YEAR01` writer - "]
pub struct YEAR01_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `YEAR10` reader - "]
pub struct YEAR10_R(crate::FieldReader<u8, u8>);
impl YEAR10_R {
    pub(crate) fn new(bits: u8) -> Self {
        YEAR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YEAR10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `YEAR10` writer - "]
pub struct YEAR10_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn wday01(&self) -> WDAY01_R {
        WDAY01_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn day01(&self) -> DAY01_R {
        DAY01_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn day10(&self) -> DAY10_R {
        DAY10_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn month01(&self) -> MONTH01_R {
        MONTH01_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn month10(&self) -> MONTH10_R {
        MONTH10_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn year01(&self) -> YEAR01_R {
        YEAR01_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn year10(&self) -> YEAR10_R {
        YEAR10_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn wday01(&mut self) -> WDAY01_W {
        WDAY01_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn day01(&mut self) -> DAY01_W {
        DAY01_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn day10(&mut self) -> DAY10_W {
        DAY10_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn month01(&mut self) -> MONTH01_W {
        MONTH01_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn month10(&mut self) -> MONTH10_W {
        MONTH10_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn year01(&mut self) -> YEAR01_W {
        YEAR01_W { w: self }
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn year10(&mut self) -> YEAR10_W {
        YEAR10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCDATEINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcdateinv](index.html) module"]
pub struct RTCDATEINV_SPEC;
impl crate::RegisterSpec for RTCDATEINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcdateinv::R](R) reader structure"]
impl crate::Readable for RTCDATEINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcdateinv::W](W) writer structure"]
impl crate::Writable for RTCDATEINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCDATEINV to value 0"]
impl crate::Resettable for RTCDATEINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
