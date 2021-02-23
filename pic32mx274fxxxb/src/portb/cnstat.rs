#[doc = "Register `CNSTAT` reader"]
pub struct R(crate::R<CNSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNSTAT_SPEC>> for R {
    fn from(reader: crate::R<CNSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNSTAT` writer"]
pub struct W(crate::W<CNSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNSTAT_SPEC>;
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
impl core::convert::From<crate::W<CNSTAT_SPEC>> for W {
    fn from(writer: crate::W<CNSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNSTATB0` reader - "]
pub struct CNSTATB0_R(crate::FieldReader<bool, bool>);
impl CNSTATB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB0` writer - "]
pub struct CNSTATB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB0_W<'a> {
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
#[doc = "Field `CNSTATB1` reader - "]
pub struct CNSTATB1_R(crate::FieldReader<bool, bool>);
impl CNSTATB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB1` writer - "]
pub struct CNSTATB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB1_W<'a> {
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
#[doc = "Field `CNSTATB2` reader - "]
pub struct CNSTATB2_R(crate::FieldReader<bool, bool>);
impl CNSTATB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB2` writer - "]
pub struct CNSTATB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB2_W<'a> {
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
#[doc = "Field `CNSTATB3` reader - "]
pub struct CNSTATB3_R(crate::FieldReader<bool, bool>);
impl CNSTATB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB3` writer - "]
pub struct CNSTATB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB3_W<'a> {
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
#[doc = "Field `CNSTATB4` reader - "]
pub struct CNSTATB4_R(crate::FieldReader<bool, bool>);
impl CNSTATB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB4` writer - "]
pub struct CNSTATB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB4_W<'a> {
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
#[doc = "Field `CNSTATB5` reader - "]
pub struct CNSTATB5_R(crate::FieldReader<bool, bool>);
impl CNSTATB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB5` writer - "]
pub struct CNSTATB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB5_W<'a> {
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
#[doc = "Field `CNSTATB7` reader - "]
pub struct CNSTATB7_R(crate::FieldReader<bool, bool>);
impl CNSTATB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB7` writer - "]
pub struct CNSTATB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB7_W<'a> {
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
#[doc = "Field `CNSTATB8` reader - "]
pub struct CNSTATB8_R(crate::FieldReader<bool, bool>);
impl CNSTATB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB8` writer - "]
pub struct CNSTATB8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB8_W<'a> {
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
#[doc = "Field `CNSTATB9` reader - "]
pub struct CNSTATB9_R(crate::FieldReader<bool, bool>);
impl CNSTATB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB9` writer - "]
pub struct CNSTATB9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB9_W<'a> {
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
#[doc = "Field `CNSTATB13` reader - "]
pub struct CNSTATB13_R(crate::FieldReader<bool, bool>);
impl CNSTATB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB13` writer - "]
pub struct CNSTATB13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB13_W<'a> {
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
#[doc = "Field `CNSTATB14` reader - "]
pub struct CNSTATB14_R(crate::FieldReader<bool, bool>);
impl CNSTATB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB14` writer - "]
pub struct CNSTATB14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB14_W<'a> {
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
#[doc = "Field `CNSTATB15` reader - "]
pub struct CNSTATB15_R(crate::FieldReader<bool, bool>);
impl CNSTATB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATB15` writer - "]
pub struct CNSTATB15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATB15_W<'a> {
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
    pub fn cnstatb0(&self) -> CNSTATB0_R {
        CNSTATB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnstatb1(&self) -> CNSTATB1_R {
        CNSTATB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnstatb2(&self) -> CNSTATB2_R {
        CNSTATB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnstatb3(&self) -> CNSTATB3_R {
        CNSTATB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnstatb4(&self) -> CNSTATB4_R {
        CNSTATB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnstatb5(&self) -> CNSTATB5_R {
        CNSTATB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnstatb7(&self) -> CNSTATB7_R {
        CNSTATB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnstatb8(&self) -> CNSTATB8_R {
        CNSTATB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnstatb9(&self) -> CNSTATB9_R {
        CNSTATB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnstatb13(&self) -> CNSTATB13_R {
        CNSTATB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnstatb14(&self) -> CNSTATB14_R {
        CNSTATB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnstatb15(&self) -> CNSTATB15_R {
        CNSTATB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnstatb0(&mut self) -> CNSTATB0_W {
        CNSTATB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnstatb1(&mut self) -> CNSTATB1_W {
        CNSTATB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnstatb2(&mut self) -> CNSTATB2_W {
        CNSTATB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnstatb3(&mut self) -> CNSTATB3_W {
        CNSTATB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnstatb4(&mut self) -> CNSTATB4_W {
        CNSTATB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnstatb5(&mut self) -> CNSTATB5_W {
        CNSTATB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnstatb7(&mut self) -> CNSTATB7_W {
        CNSTATB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnstatb8(&mut self) -> CNSTATB8_W {
        CNSTATB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnstatb9(&mut self) -> CNSTATB9_W {
        CNSTATB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnstatb13(&mut self) -> CNSTATB13_W {
        CNSTATB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnstatb14(&mut self) -> CNSTATB14_W {
        CNSTATB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnstatb15(&mut self) -> CNSTATB15_W {
        CNSTATB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CNSTATB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnstat](index.html) module"]
pub struct CNSTAT_SPEC;
impl crate::RegisterSpec for CNSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnstat::R](R) reader structure"]
impl crate::Readable for CNSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnstat::W](W) writer structure"]
impl crate::Writable for CNSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNSTAT to value 0"]
impl crate::Resettable for CNSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
