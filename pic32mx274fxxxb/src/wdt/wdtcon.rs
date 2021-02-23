#[doc = "Register `WDTCON` reader"]
pub struct R(crate::R<WDTCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WDTCON_SPEC>> for R {
    fn from(reader: crate::R<WDTCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCON` writer"]
pub struct W(crate::W<WDTCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCON_SPEC>;
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
impl core::convert::From<crate::W<WDTCON_SPEC>> for W {
    fn from(writer: crate::W<WDTCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTWINEN` reader - "]
pub struct WDTWINEN_R(crate::FieldReader<bool, bool>);
impl WDTWINEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTWINEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTWINEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTWINEN` writer - "]
pub struct WDTWINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTWINEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Field `RUNDIV` reader - "]
pub struct RUNDIV_R(crate::FieldReader<u8, u8>);
impl RUNDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RUNDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNDIV` writer - "]
pub struct RUNDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `ON` reader - "]
pub struct ON_R(crate::FieldReader<bool, bool>);
impl ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ON` writer - "]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `WDTCLRKEY` reader - "]
pub struct WDTCLRKEY_R(crate::FieldReader<u16, u16>);
impl WDTCLRKEY_R {
    pub(crate) fn new(bits: u16) -> Self {
        WDTCLRKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTCLRKEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTCLRKEY` writer - "]
pub struct WDTCLRKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCLRKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdtwinen(&self) -> WDTWINEN_R {
        WDTWINEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rundiv(&self) -> RUNDIV_R {
        RUNDIV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wdtclrkey(&self) -> WDTCLRKEY_R {
        WDTCLRKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdtwinen(&mut self) -> WDTWINEN_W {
        WDTWINEN_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rundiv(&mut self) -> RUNDIV_W {
        RUNDIV_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn wdtclrkey(&mut self) -> WDTCLRKEY_W {
        WDTCLRKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDTCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtcon](index.html) module"]
pub struct WDTCON_SPEC;
impl crate::RegisterSpec for WDTCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtcon::R](R) reader structure"]
impl crate::Readable for WDTCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtcon::W](W) writer structure"]
impl crate::Writable for WDTCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCON to value 0"]
impl crate::Resettable for WDTCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
