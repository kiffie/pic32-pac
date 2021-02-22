#[doc = "Register `U1OTGCON` reader"]
pub struct R(crate::R<U1OTGCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1OTGCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1OTGCON_SPEC>> for R {
    fn from(reader: crate::R<U1OTGCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1OTGCON` writer"]
pub struct W(crate::W<U1OTGCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1OTGCON_SPEC>;
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
impl core::convert::From<crate::W<U1OTGCON_SPEC>> for W {
    fn from(writer: crate::W<U1OTGCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSDIS` reader - "]
pub struct VBUSDIS_R(crate::FieldReader<bool, bool>);
impl VBUSDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSDIS` writer - "]
pub struct VBUSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSDIS_W<'a> {
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
#[doc = "Field `VBUSCHG` reader - "]
pub struct VBUSCHG_R(crate::FieldReader<bool, bool>);
impl VBUSCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSCHG` writer - "]
pub struct VBUSCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSCHG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `OTGEN` reader - "]
pub struct OTGEN_R(crate::FieldReader<bool, bool>);
impl OTGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OTGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTGEN` writer - "]
pub struct OTGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGEN_W<'a> {
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
#[doc = "Field `VBUSON` reader - "]
pub struct VBUSON_R(crate::FieldReader<bool, bool>);
impl VBUSON_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSON` writer - "]
pub struct VBUSON_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSON_W<'a> {
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
#[doc = "Field `DMPULDWN` reader - "]
pub struct DMPULDWN_R(crate::FieldReader<bool, bool>);
impl DMPULDWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMPULDWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMPULDWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMPULDWN` writer - "]
pub struct DMPULDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMPULDWN_W<'a> {
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
#[doc = "Field `DPPULDWN` reader - "]
pub struct DPPULDWN_R(crate::FieldReader<bool, bool>);
impl DPPULDWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPPULDWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPPULDWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPPULDWN` writer - "]
pub struct DPPULDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPULDWN_W<'a> {
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
#[doc = "Field `DMPULUP` reader - "]
pub struct DMPULUP_R(crate::FieldReader<bool, bool>);
impl DMPULUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMPULUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMPULUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMPULUP` writer - "]
pub struct DMPULUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMPULUP_W<'a> {
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
#[doc = "Field `DPPULUP` reader - "]
pub struct DPPULUP_R(crate::FieldReader<bool, bool>);
impl DPPULUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPPULUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPPULUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPPULUP` writer - "]
pub struct DPPULUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPPULUP_W<'a> {
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
    pub fn vbusdis(&self) -> VBUSDIS_R {
        VBUSDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vbuschg(&self) -> VBUSCHG_R {
        VBUSCHG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbuson(&self) -> VBUSON_R {
        VBUSON_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dmpuldwn(&self) -> DMPULDWN_R {
        DMPULDWN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dppuldwn(&self) -> DPPULDWN_R {
        DPPULDWN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dmpulup(&self) -> DMPULUP_R {
        DMPULUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dppulup(&self) -> DPPULUP_R {
        DPPULUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusdis(&mut self) -> VBUSDIS_W {
        VBUSDIS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vbuschg(&mut self) -> VBUSCHG_W {
        VBUSCHG_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn otgen(&mut self) -> OTGEN_W {
        OTGEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbuson(&mut self) -> VBUSON_W {
        VBUSON_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dmpuldwn(&mut self) -> DMPULDWN_W {
        DMPULDWN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dppuldwn(&mut self) -> DPPULDWN_W {
        DPPULDWN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dmpulup(&mut self) -> DMPULUP_W {
        DMPULUP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dppulup(&mut self) -> DPPULUP_W {
        DPPULUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1OTGCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgcon](index.html) module"]
pub struct U1OTGCON_SPEC;
impl crate::RegisterSpec for U1OTGCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1otgcon::R](R) reader structure"]
impl crate::Readable for U1OTGCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1otgcon::W](W) writer structure"]
impl crate::Writable for U1OTGCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1OTGCON to value 0"]
impl crate::Resettable for U1OTGCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
