#[doc = "Register `PORTCLR` reader"]
pub struct R(crate::R<PORTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PORTCLR_SPEC>> for R {
    fn from(reader: crate::R<PORTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTCLR` writer"]
pub struct W(crate::W<PORTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTCLR_SPEC>;
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
impl core::convert::From<crate::W<PORTCLR_SPEC>> for W {
    fn from(writer: crate::W<PORTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB0` reader - "]
pub struct RB0_R(crate::FieldReader<bool, bool>);
impl RB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB0` writer - "]
pub struct RB0_W<'a> {
    w: &'a mut W,
}
impl<'a> RB0_W<'a> {
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
#[doc = "Field `RB1` reader - "]
pub struct RB1_R(crate::FieldReader<bool, bool>);
impl RB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB1` writer - "]
pub struct RB1_W<'a> {
    w: &'a mut W,
}
impl<'a> RB1_W<'a> {
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
#[doc = "Field `RB2` reader - "]
pub struct RB2_R(crate::FieldReader<bool, bool>);
impl RB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB2` writer - "]
pub struct RB2_W<'a> {
    w: &'a mut W,
}
impl<'a> RB2_W<'a> {
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
#[doc = "Field `RB3` reader - "]
pub struct RB3_R(crate::FieldReader<bool, bool>);
impl RB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB3` writer - "]
pub struct RB3_W<'a> {
    w: &'a mut W,
}
impl<'a> RB3_W<'a> {
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
#[doc = "Field `RB4` reader - "]
pub struct RB4_R(crate::FieldReader<bool, bool>);
impl RB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB4` writer - "]
pub struct RB4_W<'a> {
    w: &'a mut W,
}
impl<'a> RB4_W<'a> {
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
#[doc = "Field `RB5` reader - "]
pub struct RB5_R(crate::FieldReader<bool, bool>);
impl RB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB5` writer - "]
pub struct RB5_W<'a> {
    w: &'a mut W,
}
impl<'a> RB5_W<'a> {
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
#[doc = "Field `RB7` reader - "]
pub struct RB7_R(crate::FieldReader<bool, bool>);
impl RB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB7` writer - "]
pub struct RB7_W<'a> {
    w: &'a mut W,
}
impl<'a> RB7_W<'a> {
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
#[doc = "Field `RB8` reader - "]
pub struct RB8_R(crate::FieldReader<bool, bool>);
impl RB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB8` writer - "]
pub struct RB8_W<'a> {
    w: &'a mut W,
}
impl<'a> RB8_W<'a> {
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
#[doc = "Field `RB9` reader - "]
pub struct RB9_R(crate::FieldReader<bool, bool>);
impl RB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB9` writer - "]
pub struct RB9_W<'a> {
    w: &'a mut W,
}
impl<'a> RB9_W<'a> {
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
#[doc = "Field `RB13` reader - "]
pub struct RB13_R(crate::FieldReader<bool, bool>);
impl RB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB13` writer - "]
pub struct RB13_W<'a> {
    w: &'a mut W,
}
impl<'a> RB13_W<'a> {
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
#[doc = "Field `RB14` reader - "]
pub struct RB14_R(crate::FieldReader<bool, bool>);
impl RB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB14` writer - "]
pub struct RB14_W<'a> {
    w: &'a mut W,
}
impl<'a> RB14_W<'a> {
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
#[doc = "Field `RB15` reader - "]
pub struct RB15_R(crate::FieldReader<bool, bool>);
impl RB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB15` writer - "]
pub struct RB15_W<'a> {
    w: &'a mut W,
}
impl<'a> RB15_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rb0(&self) -> RB0_R {
        RB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rb1(&self) -> RB1_R {
        RB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rb2(&self) -> RB2_R {
        RB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rb3(&self) -> RB3_R {
        RB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rb4(&self) -> RB4_R {
        RB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rb5(&self) -> RB5_R {
        RB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rb7(&self) -> RB7_R {
        RB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rb8(&self) -> RB8_R {
        RB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rb9(&self) -> RB9_R {
        RB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rb13(&self) -> RB13_R {
        RB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rb14(&self) -> RB14_R {
        RB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rb15(&self) -> RB15_R {
        RB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rb0(&mut self) -> RB0_W {
        RB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rb1(&mut self) -> RB1_W {
        RB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rb2(&mut self) -> RB2_W {
        RB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rb3(&mut self) -> RB3_W {
        RB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rb4(&mut self) -> RB4_W {
        RB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rb5(&mut self) -> RB5_W {
        RB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rb7(&mut self) -> RB7_W {
        RB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rb8(&mut self) -> RB8_W {
        RB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rb9(&mut self) -> RB9_W {
        RB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rb13(&mut self) -> RB13_W {
        RB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rb14(&mut self) -> RB14_W {
        RB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rb15(&mut self) -> RB15_W {
        RB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORTBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portclr](index.html) module"]
pub struct PORTCLR_SPEC;
impl crate::RegisterSpec for PORTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portclr::R](R) reader structure"]
impl crate::Readable for PORTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portclr::W](W) writer structure"]
impl crate::Writable for PORTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTCLR to value 0"]
impl crate::Resettable for PORTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
