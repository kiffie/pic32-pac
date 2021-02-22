#[doc = "Register `U1OTGIE` reader"]
pub struct R(crate::R<U1OTGIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1OTGIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1OTGIE_SPEC>> for R {
    fn from(reader: crate::R<U1OTGIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1OTGIE` writer"]
pub struct W(crate::W<U1OTGIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1OTGIE_SPEC>;
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
impl core::convert::From<crate::W<U1OTGIE_SPEC>> for W {
    fn from(writer: crate::W<U1OTGIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSVDIE` reader - "]
pub struct VBUSVDIE_R(crate::FieldReader<bool, bool>);
impl VBUSVDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSVDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSVDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSVDIE` writer - "]
pub struct VBUSVDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVDIE_W<'a> {
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
#[doc = "Field `SESENDIE` reader - "]
pub struct SESENDIE_R(crate::FieldReader<bool, bool>);
impl SESENDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESENDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESENDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SESENDIE` writer - "]
pub struct SESENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SESENDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SESVDIE` reader - "]
pub struct SESVDIE_R(crate::FieldReader<bool, bool>);
impl SESVDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESVDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESVDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SESVDIE` writer - "]
pub struct SESVDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SESVDIE_W<'a> {
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
#[doc = "Field `ACTVIE` reader - "]
pub struct ACTVIE_R(crate::FieldReader<bool, bool>);
impl ACTVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTVIE` writer - "]
pub struct ACTVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTVIE_W<'a> {
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
#[doc = "Field `LSTATEIE` reader - "]
pub struct LSTATEIE_R(crate::FieldReader<bool, bool>);
impl LSTATEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTATEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTATEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTATEIE` writer - "]
pub struct LSTATEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTATEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `T1MSECIE` reader - "]
pub struct T1MSECIE_R(crate::FieldReader<bool, bool>);
impl T1MSECIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        T1MSECIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1MSECIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1MSECIE` writer - "]
pub struct T1MSECIE_W<'a> {
    w: &'a mut W,
}
impl<'a> T1MSECIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IDIE` reader - "]
pub struct IDIE_R(crate::FieldReader<bool, bool>);
impl IDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDIE` writer - "]
pub struct IDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIE_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvdie(&self) -> VBUSVDIE_R {
        VBUSVDIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesendie(&self) -> SESENDIE_R {
        SESENDIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvdie(&self) -> SESVDIE_R {
        SESVDIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn actvie(&self) -> ACTVIE_R {
        ACTVIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstateie(&self) -> LSTATEIE_R {
        LSTATEIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t1msecie(&self) -> T1MSECIE_R {
        T1MSECIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvdie(&mut self) -> VBUSVDIE_W {
        VBUSVDIE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesendie(&mut self) -> SESENDIE_W {
        SESENDIE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvdie(&mut self) -> SESVDIE_W {
        SESVDIE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn actvie(&mut self) -> ACTVIE_W {
        ACTVIE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstateie(&mut self) -> LSTATEIE_W {
        LSTATEIE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t1msecie(&mut self) -> T1MSECIE_W {
        T1MSECIE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idie(&mut self) -> IDIE_W {
        IDIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1OTGIE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgie](index.html) module"]
pub struct U1OTGIE_SPEC;
impl crate::RegisterSpec for U1OTGIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1otgie::R](R) reader structure"]
impl crate::Readable for U1OTGIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1otgie::W](W) writer structure"]
impl crate::Writable for U1OTGIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1OTGIE to value 0"]
impl crate::Resettable for U1OTGIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
