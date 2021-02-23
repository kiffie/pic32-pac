#[doc = "Register `CON2SET` reader"]
pub struct R(crate::R<CON2SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CON2SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CON2SET_SPEC>> for R {
    fn from(reader: crate::R<CON2SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CON2SET` writer"]
pub struct W(crate::W<CON2SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CON2SET_SPEC>;
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
impl core::convert::From<crate::W<CON2SET_SPEC>> for W {
    fn from(writer: crate::W<CON2SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUDMOD` reader - "]
pub struct AUDMOD_R(crate::FieldReader<u8, u8>);
impl AUDMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        AUDMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUDMOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUDMOD` writer - "]
pub struct AUDMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `AUDMONO` reader - "]
pub struct AUDMONO_R(crate::FieldReader<bool, bool>);
impl AUDMONO_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUDMONO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUDMONO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUDMONO` writer - "]
pub struct AUDMONO_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDMONO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `AUDEN` reader - "]
pub struct AUDEN_R(crate::FieldReader<bool, bool>);
impl AUDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUDEN` writer - "]
pub struct AUDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `IGNTUR` reader - "]
pub struct IGNTUR_R(crate::FieldReader<bool, bool>);
impl IGNTUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        IGNTUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGNTUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGNTUR` writer - "]
pub struct IGNTUR_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNTUR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `IGNROV` reader - "]
pub struct IGNROV_R(crate::FieldReader<bool, bool>);
impl IGNROV_R {
    pub(crate) fn new(bits: bool) -> Self {
        IGNROV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGNROV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGNROV` writer - "]
pub struct IGNROV_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNROV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SPITUREN` reader - "]
pub struct SPITUREN_R(crate::FieldReader<bool, bool>);
impl SPITUREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPITUREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPITUREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPITUREN` writer - "]
pub struct SPITUREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPITUREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SPIROVEN` reader - "]
pub struct SPIROVEN_R(crate::FieldReader<bool, bool>);
impl SPIROVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIROVEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIROVEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIROVEN` writer - "]
pub struct SPIROVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIROVEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `FRMERREN` reader - "]
pub struct FRMERREN_R(crate::FieldReader<bool, bool>);
impl FRMERREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMERREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMERREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMERREN` writer - "]
pub struct FRMERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMERREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `SPISGNEXT` reader - "]
pub struct SPISGNEXT_R(crate::FieldReader<bool, bool>);
impl SPISGNEXT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPISGNEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPISGNEXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPISGNEXT` writer - "]
pub struct SPISGNEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPISGNEXT_W<'a> {
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
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn audmod(&self) -> AUDMOD_R {
        AUDMOD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn audmono(&self) -> AUDMONO_R {
        AUDMONO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn auden(&self) -> AUDEN_R {
        AUDEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn igntur(&self) -> IGNTUR_R {
        IGNTUR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ignrov(&self) -> IGNROV_R {
        IGNROV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spituren(&self) -> SPITUREN_R {
        SPITUREN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spiroven(&self) -> SPIROVEN_R {
        SPIROVEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frmerren(&self) -> FRMERREN_R {
        FRMERREN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spisgnext(&self) -> SPISGNEXT_R {
        SPISGNEXT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn audmod(&mut self) -> AUDMOD_W {
        AUDMOD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn audmono(&mut self) -> AUDMONO_W {
        AUDMONO_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn auden(&mut self) -> AUDEN_W {
        AUDEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn igntur(&mut self) -> IGNTUR_W {
        IGNTUR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ignrov(&mut self) -> IGNROV_W {
        IGNROV_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn spituren(&mut self) -> SPITUREN_W {
        SPITUREN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn spiroven(&mut self) -> SPIROVEN_W {
        SPIROVEN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn frmerren(&mut self) -> FRMERREN_W {
        FRMERREN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spisgnext(&mut self) -> SPISGNEXT_W {
        SPISGNEXT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1CON2SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con2set](index.html) module"]
pub struct CON2SET_SPEC;
impl crate::RegisterSpec for CON2SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [con2set::R](R) reader structure"]
impl crate::Readable for CON2SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [con2set::W](W) writer structure"]
impl crate::Writable for CON2SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CON2SET to value 0"]
impl crate::Resettable for CON2SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
