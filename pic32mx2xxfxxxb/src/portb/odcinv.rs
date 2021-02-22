#[doc = "Register `ODCINV` reader"]
pub struct R(crate::R<ODCINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODCINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ODCINV_SPEC>> for R {
    fn from(reader: crate::R<ODCINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODCINV` writer"]
pub struct W(crate::W<ODCINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODCINV_SPEC>;
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
impl core::convert::From<crate::W<ODCINV_SPEC>> for W {
    fn from(writer: crate::W<ODCINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ODCB0` reader - "]
pub struct ODCB0_R(crate::FieldReader<bool, bool>);
impl ODCB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB0` writer - "]
pub struct ODCB0_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB0_W<'a> {
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
#[doc = "Field `ODCB1` reader - "]
pub struct ODCB1_R(crate::FieldReader<bool, bool>);
impl ODCB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB1` writer - "]
pub struct ODCB1_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB1_W<'a> {
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
#[doc = "Field `ODCB2` reader - "]
pub struct ODCB2_R(crate::FieldReader<bool, bool>);
impl ODCB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB2` writer - "]
pub struct ODCB2_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB2_W<'a> {
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
#[doc = "Field `ODCB3` reader - "]
pub struct ODCB3_R(crate::FieldReader<bool, bool>);
impl ODCB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB3` writer - "]
pub struct ODCB3_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB3_W<'a> {
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
#[doc = "Field `ODCB4` reader - "]
pub struct ODCB4_R(crate::FieldReader<bool, bool>);
impl ODCB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB4` writer - "]
pub struct ODCB4_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB4_W<'a> {
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
#[doc = "Field `ODCB5` reader - "]
pub struct ODCB5_R(crate::FieldReader<bool, bool>);
impl ODCB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB5` writer - "]
pub struct ODCB5_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB5_W<'a> {
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
#[doc = "Field `ODCB7` reader - "]
pub struct ODCB7_R(crate::FieldReader<bool, bool>);
impl ODCB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB7` writer - "]
pub struct ODCB7_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB7_W<'a> {
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
#[doc = "Field `ODCB8` reader - "]
pub struct ODCB8_R(crate::FieldReader<bool, bool>);
impl ODCB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB8` writer - "]
pub struct ODCB8_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB8_W<'a> {
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
#[doc = "Field `ODCB9` reader - "]
pub struct ODCB9_R(crate::FieldReader<bool, bool>);
impl ODCB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB9` writer - "]
pub struct ODCB9_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB9_W<'a> {
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
#[doc = "Field `ODCB10` reader - "]
pub struct ODCB10_R(crate::FieldReader<bool, bool>);
impl ODCB10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB10` writer - "]
pub struct ODCB10_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB10_W<'a> {
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
#[doc = "Field `ODCB11` reader - "]
pub struct ODCB11_R(crate::FieldReader<bool, bool>);
impl ODCB11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB11` writer - "]
pub struct ODCB11_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB11_W<'a> {
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
#[doc = "Field `ODCB13` reader - "]
pub struct ODCB13_R(crate::FieldReader<bool, bool>);
impl ODCB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB13` writer - "]
pub struct ODCB13_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB13_W<'a> {
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
#[doc = "Field `ODCB14` reader - "]
pub struct ODCB14_R(crate::FieldReader<bool, bool>);
impl ODCB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB14` writer - "]
pub struct ODCB14_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB14_W<'a> {
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
#[doc = "Field `ODCB15` reader - "]
pub struct ODCB15_R(crate::FieldReader<bool, bool>);
impl ODCB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCB15` writer - "]
pub struct ODCB15_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCB15_W<'a> {
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
    pub fn odcb0(&self) -> ODCB0_R {
        ODCB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odcb1(&self) -> ODCB1_R {
        ODCB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odcb2(&self) -> ODCB2_R {
        ODCB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odcb3(&self) -> ODCB3_R {
        ODCB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odcb4(&self) -> ODCB4_R {
        ODCB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn odcb5(&self) -> ODCB5_R {
        ODCB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn odcb7(&self) -> ODCB7_R {
        ODCB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn odcb8(&self) -> ODCB8_R {
        ODCB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn odcb9(&self) -> ODCB9_R {
        ODCB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn odcb10(&self) -> ODCB10_R {
        ODCB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn odcb11(&self) -> ODCB11_R {
        ODCB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn odcb13(&self) -> ODCB13_R {
        ODCB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn odcb14(&self) -> ODCB14_R {
        ODCB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn odcb15(&self) -> ODCB15_R {
        ODCB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn odcb0(&mut self) -> ODCB0_W {
        ODCB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odcb1(&mut self) -> ODCB1_W {
        ODCB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odcb2(&mut self) -> ODCB2_W {
        ODCB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odcb3(&mut self) -> ODCB3_W {
        ODCB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odcb4(&mut self) -> ODCB4_W {
        ODCB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn odcb5(&mut self) -> ODCB5_W {
        ODCB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn odcb7(&mut self) -> ODCB7_W {
        ODCB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn odcb8(&mut self) -> ODCB8_W {
        ODCB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn odcb9(&mut self) -> ODCB9_W {
        ODCB9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn odcb10(&mut self) -> ODCB10_W {
        ODCB10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn odcb11(&mut self) -> ODCB11_W {
        ODCB11_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn odcb13(&mut self) -> ODCB13_W {
        ODCB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn odcb14(&mut self) -> ODCB14_W {
        ODCB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn odcb15(&mut self) -> ODCB15_W {
        ODCB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ODCBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odcinv](index.html) module"]
pub struct ODCINV_SPEC;
impl crate::RegisterSpec for ODCINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odcinv::R](R) reader structure"]
impl crate::Readable for ODCINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odcinv::W](W) writer structure"]
impl crate::Writable for ODCINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODCINV to value 0"]
impl crate::Resettable for ODCINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
