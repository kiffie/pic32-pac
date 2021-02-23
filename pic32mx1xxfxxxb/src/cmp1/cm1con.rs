#[doc = "Register `CM1CON` reader"]
pub struct R(crate::R<CM1CON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM1CON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CM1CON_SPEC>> for R {
    fn from(reader: crate::R<CM1CON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM1CON` writer"]
pub struct W(crate::W<CM1CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM1CON_SPEC>;
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
impl core::convert::From<crate::W<CM1CON_SPEC>> for W {
    fn from(writer: crate::W<CM1CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCH` reader - "]
pub struct CCH_R(crate::FieldReader<u8, u8>);
impl CCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCH` writer - "]
pub struct CCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `CREF` reader - "]
pub struct CREF_R(crate::FieldReader<bool, bool>);
impl CREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CREF` writer - "]
pub struct CREF_W<'a> {
    w: &'a mut W,
}
impl<'a> CREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `EVPOL` reader - "]
pub struct EVPOL_R(crate::FieldReader<u8, u8>);
impl EVPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EVPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVPOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVPOL` writer - "]
pub struct EVPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> EVPOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `COUT` reader - "]
pub struct COUT_R(crate::FieldReader<bool, bool>);
impl COUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUT` writer - "]
pub struct COUT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUT_W<'a> {
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
#[doc = "Field `CPOL` reader - "]
pub struct CPOL_R(crate::FieldReader<bool, bool>);
impl CPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - "]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `COE` reader - "]
pub struct COE_R(crate::FieldReader<bool, bool>);
impl COE_R {
    pub(crate) fn new(bits: bool) -> Self {
        COE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COE` writer - "]
pub struct COE_W<'a> {
    w: &'a mut W,
}
impl<'a> COE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
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
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cch(&self) -> CCH_R {
        CCH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cref(&self) -> CREF_R {
        CREF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn evpol(&self) -> EVPOL_R {
        EVPOL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cch(&mut self) -> CCH_W {
        CCH_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cref(&mut self) -> CREF_W {
        CREF_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn evpol(&mut self) -> EVPOL_W {
        EVPOL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cout(&mut self) -> COUT_W {
        COUT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W {
        COE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM1CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm1con](index.html) module"]
pub struct CM1CON_SPEC;
impl crate::RegisterSpec for CM1CON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm1con::R](R) reader structure"]
impl crate::Readable for CM1CON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm1con::W](W) writer structure"]
impl crate::Writable for CM1CON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM1CON to value 0xc3"]
impl crate::Resettable for CM1CON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc3
    }
}
