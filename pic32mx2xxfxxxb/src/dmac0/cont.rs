#[doc = "Register `CONT` reader"]
pub struct R(crate::R<CONT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CONT_SPEC>> for R {
    fn from(reader: crate::R<CONT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONT` writer"]
pub struct W(crate::W<CONT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONT_SPEC>;
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
impl core::convert::From<crate::W<CONT_SPEC>> for W {
    fn from(writer: crate::W<CONT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHPRI` reader - "]
pub struct CHPRI_R(crate::FieldReader<u8, u8>);
impl CHPRI_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHPRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHPRI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHPRI` writer - "]
pub struct CHPRI_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `CHEDET` reader - "]
pub struct CHEDET_R(crate::FieldReader<bool, bool>);
impl CHEDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEDET` writer - "]
pub struct CHEDET_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEDET_W<'a> {
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
#[doc = "Field `CHAEN` reader - "]
pub struct CHAEN_R(crate::FieldReader<bool, bool>);
impl CHAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAEN` writer - "]
pub struct CHAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAEN_W<'a> {
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
#[doc = "Field `CHCHN` reader - "]
pub struct CHCHN_R(crate::FieldReader<bool, bool>);
impl CHCHN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHCHN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHCHN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHCHN` writer - "]
pub struct CHCHN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCHN_W<'a> {
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
#[doc = "Field `CHAED` reader - "]
pub struct CHAED_R(crate::FieldReader<bool, bool>);
impl CHAED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHAED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAED` writer - "]
pub struct CHAED_W<'a> {
    w: &'a mut W,
}
impl<'a> CHAED_W<'a> {
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
#[doc = "Field `CHEN` reader - "]
pub struct CHEN_R(crate::FieldReader<bool, bool>);
impl CHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHEN` writer - "]
pub struct CHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN_W<'a> {
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
#[doc = "Field `CHCHNS` reader - "]
pub struct CHCHNS_R(crate::FieldReader<bool, bool>);
impl CHCHNS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHCHNS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHCHNS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHCHNS` writer - "]
pub struct CHCHNS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCHNS_W<'a> {
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
#[doc = "Field `CHBUSY` reader - "]
pub struct CHBUSY_R(crate::FieldReader<bool, bool>);
impl CHBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHBUSY` writer - "]
pub struct CHBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> CHBUSY_W<'a> {
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
    pub fn chpri(&self) -> CHPRI_R {
        CHPRI_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chedet(&self) -> CHEDET_R {
        CHEDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn chaen(&self) -> CHAEN_R {
        CHAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn chchn(&self) -> CHCHN_R {
        CHCHN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chaed(&self) -> CHAED_R {
        CHAED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chen(&self) -> CHEN_R {
        CHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn chchns(&self) -> CHCHNS_R {
        CHCHNS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chbusy(&self) -> CHBUSY_R {
        CHBUSY_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chpri(&mut self) -> CHPRI_W {
        CHPRI_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn chedet(&mut self) -> CHEDET_W {
        CHEDET_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn chaen(&mut self) -> CHAEN_W {
        CHAEN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn chchn(&mut self) -> CHCHN_W {
        CHCHN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn chaed(&mut self) -> CHAED_W {
        CHAED_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn chen(&mut self) -> CHEN_W {
        CHEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn chchns(&mut self) -> CHCHNS_W {
        CHCHNS_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn chbusy(&mut self) -> CHBUSY_W {
        CHBUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCH0CON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cont](index.html) module"]
pub struct CONT_SPEC;
impl crate::RegisterSpec for CONT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cont::R](R) reader structure"]
impl crate::Readable for CONT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cont::W](W) writer structure"]
impl crate::Writable for CONT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONT to value 0"]
impl crate::Resettable for CONT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
