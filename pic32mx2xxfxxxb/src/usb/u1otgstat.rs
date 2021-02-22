#[doc = "Register `U1OTGSTAT` reader"]
pub struct R(crate::R<U1OTGSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1OTGSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1OTGSTAT_SPEC>> for R {
    fn from(reader: crate::R<U1OTGSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1OTGSTAT` writer"]
pub struct W(crate::W<U1OTGSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1OTGSTAT_SPEC>;
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
impl core::convert::From<crate::W<U1OTGSTAT_SPEC>> for W {
    fn from(writer: crate::W<U1OTGSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSVD` reader - "]
pub struct VBUSVD_R(crate::FieldReader<bool, bool>);
impl VBUSVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSVD` writer - "]
pub struct VBUSVD_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVD_W<'a> {
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
#[doc = "Field `SESEND` reader - "]
pub struct SESEND_R(crate::FieldReader<bool, bool>);
impl SESEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SESEND` writer - "]
pub struct SESEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SESEND_W<'a> {
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
#[doc = "Field `SESVD` reader - "]
pub struct SESVD_R(crate::FieldReader<bool, bool>);
impl SESVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SESVD` writer - "]
pub struct SESVD_W<'a> {
    w: &'a mut W,
}
impl<'a> SESVD_W<'a> {
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
#[doc = "Field `LSTATE` reader - "]
pub struct LSTATE_R(crate::FieldReader<bool, bool>);
impl LSTATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTATE` writer - "]
pub struct LSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTATE_W<'a> {
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
#[doc = "Field `ID` reader - "]
pub struct ID_R(crate::FieldReader<bool, bool>);
impl ID_R {
    pub(crate) fn new(bits: bool) -> Self {
        ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ID` writer - "]
pub struct ID_W<'a> {
    w: &'a mut W,
}
impl<'a> ID_W<'a> {
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
    pub fn vbusvd(&self) -> VBUSVD_R {
        VBUSVD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesend(&self) -> SESEND_R {
        SESEND_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvd(&self) -> SESVD_R {
        SESVD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstate(&self) -> LSTATE_R {
        LSTATE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvd(&mut self) -> VBUSVD_W {
        VBUSVD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesend(&mut self) -> SESEND_W {
        SESEND_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvd(&mut self) -> SESVD_W {
        SESVD_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstate(&mut self) -> LSTATE_W {
        LSTATE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1OTGSTAT register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgstat](index.html) module"]
pub struct U1OTGSTAT_SPEC;
impl crate::RegisterSpec for U1OTGSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1otgstat::R](R) reader structure"]
impl crate::Readable for U1OTGSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1otgstat::W](W) writer structure"]
impl crate::Writable for U1OTGSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1OTGSTAT to value 0"]
impl crate::Resettable for U1OTGSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
