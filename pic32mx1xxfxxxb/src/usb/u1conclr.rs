#[doc = "Register `U1CONCLR` reader"]
pub struct R(crate::R<U1CONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1CONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1CONCLR_SPEC>> for R {
    fn from(reader: crate::R<U1CONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1CONCLR` writer"]
pub struct W(crate::W<U1CONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1CONCLR_SPEC>;
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
impl core::convert::From<crate::W<U1CONCLR_SPEC>> for W {
    fn from(writer: crate::W<U1CONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBEN_SOFEN` reader - "]
pub struct USBEN_SOFEN_R(crate::FieldReader<bool, bool>);
impl USBEN_SOFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBEN_SOFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBEN_SOFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBEN_SOFEN` writer - "]
pub struct USBEN_SOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEN_SOFEN_W<'a> {
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
#[doc = "Field `PPBRST` reader - "]
pub struct PPBRST_R(crate::FieldReader<bool, bool>);
impl PPBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPBRST` writer - "]
pub struct PPBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> PPBRST_W<'a> {
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
#[doc = "Field `RESUME` reader - "]
pub struct RESUME_R(crate::FieldReader<bool, bool>);
impl RESUME_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - "]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Field `HOSTEN` reader - "]
pub struct HOSTEN_R(crate::FieldReader<bool, bool>);
impl HOSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOSTEN` writer - "]
pub struct HOSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTEN_W<'a> {
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
#[doc = "Field `USBRST` reader - "]
pub struct USBRST_R(crate::FieldReader<bool, bool>);
impl USBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRST` writer - "]
pub struct USBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRST_W<'a> {
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
#[doc = "Field `PKTDIS_TOKBUSY` reader - "]
pub struct PKTDIS_TOKBUSY_R(crate::FieldReader<bool, bool>);
impl PKTDIS_TOKBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKTDIS_TOKBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTDIS_TOKBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKTDIS_TOKBUSY` writer - "]
pub struct PKTDIS_TOKBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTDIS_TOKBUSY_W<'a> {
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
#[doc = "Field `SE0` reader - "]
pub struct SE0_R(crate::FieldReader<bool, bool>);
impl SE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE0` writer - "]
pub struct SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0_W<'a> {
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
#[doc = "Field `JSTATE` reader - "]
pub struct JSTATE_R(crate::FieldReader<bool, bool>);
impl JSTATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        JSTATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JSTATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JSTATE` writer - "]
pub struct JSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> JSTATE_W<'a> {
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
    pub fn usben_sofen(&self) -> USBEN_SOFEN_R {
        USBEN_SOFEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ppbrst(&self) -> PPBRST_R {
        PPBRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hosten(&self) -> HOSTEN_R {
        HOSTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pktdis_tokbusy(&self) -> PKTDIS_TOKBUSY_R {
        PKTDIS_TOKBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jstate(&self) -> JSTATE_R {
        JSTATE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn usben_sofen(&mut self) -> USBEN_SOFEN_W {
        USBEN_SOFEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ppbrst(&mut self) -> PPBRST_W {
        PPBRST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hosten(&mut self) -> HOSTEN_W {
        HOSTEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pktdis_tokbusy(&mut self) -> PKTDIS_TOKBUSY_W {
        PKTDIS_TOKBUSY_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se0(&mut self) -> SE0_W {
        SE0_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jstate(&mut self) -> JSTATE_W {
        JSTATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1conclr](index.html) module"]
pub struct U1CONCLR_SPEC;
impl crate::RegisterSpec for U1CONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1conclr::R](R) reader structure"]
impl crate::Readable for U1CONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1conclr::W](W) writer structure"]
impl crate::Writable for U1CONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1CONCLR to value 0"]
impl crate::Resettable for U1CONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
