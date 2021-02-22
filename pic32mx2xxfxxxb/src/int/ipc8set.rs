#[doc = "Register `IPC8SET` reader"]
pub struct R(crate::R<IPC8SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPC8SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IPC8SET_SPEC>> for R {
    fn from(reader: crate::R<IPC8SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPC8SET` writer"]
pub struct W(crate::W<IPC8SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPC8SET_SPEC>;
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
impl core::convert::From<crate::W<IPC8SET_SPEC>> for W {
    fn from(writer: crate::W<IPC8SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `U1IS` reader - "]
pub struct U1IS_R(crate::FieldReader<u8, u8>);
impl U1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        U1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1IS` writer - "]
pub struct U1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> U1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `U1IP` reader - "]
pub struct U1IP_R(crate::FieldReader<u8, u8>);
impl U1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        U1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1IP` writer - "]
pub struct U1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> U1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `I2C1IS` reader - "]
pub struct I2C1IS_R(crate::FieldReader<u8, u8>);
impl I2C1IS_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1IS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1IS` writer - "]
pub struct I2C1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `I2C1IP` reader - "]
pub struct I2C1IP_R(crate::FieldReader<u8, u8>);
impl I2C1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1IP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1IP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1IP` writer - "]
pub struct I2C1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `CNIS` reader - "]
pub struct CNIS_R(crate::FieldReader<u8, u8>);
impl CNIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIS` writer - "]
pub struct CNIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CNIP` reader - "]
pub struct CNIP_R(crate::FieldReader<u8, u8>);
impl CNIP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIP` writer - "]
pub struct CNIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `PMPIS` reader - "]
pub struct PMPIS_R(crate::FieldReader<u8, u8>);
impl PMPIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PMPIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMPIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMPIS` writer - "]
pub struct PMPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `PMPIP` reader - "]
pub struct PMPIP_R(crate::FieldReader<u8, u8>);
impl PMPIP_R {
    pub(crate) fn new(bits: u8) -> Self {
        PMPIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMPIP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMPIP` writer - "]
pub struct PMPIP_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPIP_W<'a> {
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
    pub fn u1is(&self) -> U1IS_R {
        U1IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn u1ip(&self) -> U1IP_R {
        U1IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn i2c1is(&self) -> I2C1IS_R {
        I2C1IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn i2c1ip(&self) -> I2C1IP_R {
        I2C1IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cnis(&self) -> CNIS_R {
        CNIS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn cnip(&self) -> CNIP_R {
        CNIP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pmpis(&self) -> PMPIS_R {
        PMPIS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn pmpip(&self) -> PMPIP_R {
        PMPIP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn u1is(&mut self) -> U1IS_W {
        U1IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn u1ip(&mut self) -> U1IP_W {
        U1IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn i2c1is(&mut self) -> I2C1IS_W {
        I2C1IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn i2c1ip(&mut self) -> I2C1IP_W {
        I2C1IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cnis(&mut self) -> CNIS_W {
        CNIS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn cnip(&mut self) -> CNIP_W {
        CNIP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pmpis(&mut self) -> PMPIS_W {
        PMPIS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn pmpip(&mut self) -> PMPIP_W {
        PMPIP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPC8SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipc8set](index.html) module"]
pub struct IPC8SET_SPEC;
impl crate::RegisterSpec for IPC8SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipc8set::R](R) reader structure"]
impl crate::Readable for IPC8SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipc8set::W](W) writer structure"]
impl crate::Writable for IPC8SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IPC8SET to value 0"]
impl crate::Resettable for IPC8SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
