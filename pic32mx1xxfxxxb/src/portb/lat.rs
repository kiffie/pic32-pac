#[doc = "Register `LAT` reader"]
pub struct R(crate::R<LAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LAT_SPEC>> for R {
    fn from(reader: crate::R<LAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAT` writer"]
pub struct W(crate::W<LAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAT_SPEC>;
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
impl core::convert::From<crate::W<LAT_SPEC>> for W {
    fn from(writer: crate::W<LAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATB0` reader - "]
pub struct LATB0_R(crate::FieldReader<bool, bool>);
impl LATB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB0` writer - "]
pub struct LATB0_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB0_W<'a> {
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
#[doc = "Field `LATB1` reader - "]
pub struct LATB1_R(crate::FieldReader<bool, bool>);
impl LATB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB1` writer - "]
pub struct LATB1_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB1_W<'a> {
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
#[doc = "Field `LATB2` reader - "]
pub struct LATB2_R(crate::FieldReader<bool, bool>);
impl LATB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB2` writer - "]
pub struct LATB2_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB2_W<'a> {
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
#[doc = "Field `LATB3` reader - "]
pub struct LATB3_R(crate::FieldReader<bool, bool>);
impl LATB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB3` writer - "]
pub struct LATB3_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB3_W<'a> {
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
#[doc = "Field `LATB4` reader - "]
pub struct LATB4_R(crate::FieldReader<bool, bool>);
impl LATB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB4` writer - "]
pub struct LATB4_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB4_W<'a> {
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
#[doc = "Field `LATB5` reader - "]
pub struct LATB5_R(crate::FieldReader<bool, bool>);
impl LATB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB5` writer - "]
pub struct LATB5_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB5_W<'a> {
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
#[doc = "Field `LATB6` reader - "]
pub struct LATB6_R(crate::FieldReader<bool, bool>);
impl LATB6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB6` writer - "]
pub struct LATB6_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB6_W<'a> {
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
#[doc = "Field `LATB7` reader - "]
pub struct LATB7_R(crate::FieldReader<bool, bool>);
impl LATB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB7` writer - "]
pub struct LATB7_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB7_W<'a> {
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
#[doc = "Field `LATB8` reader - "]
pub struct LATB8_R(crate::FieldReader<bool, bool>);
impl LATB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB8` writer - "]
pub struct LATB8_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB8_W<'a> {
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
#[doc = "Field `LATB9` reader - "]
pub struct LATB9_R(crate::FieldReader<bool, bool>);
impl LATB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB9` writer - "]
pub struct LATB9_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB9_W<'a> {
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
#[doc = "Field `LATB10` reader - "]
pub struct LATB10_R(crate::FieldReader<bool, bool>);
impl LATB10_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB10` writer - "]
pub struct LATB10_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB10_W<'a> {
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
#[doc = "Field `LATB11` reader - "]
pub struct LATB11_R(crate::FieldReader<bool, bool>);
impl LATB11_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB11` writer - "]
pub struct LATB11_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB11_W<'a> {
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
#[doc = "Field `LATB12` reader - "]
pub struct LATB12_R(crate::FieldReader<bool, bool>);
impl LATB12_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB12` writer - "]
pub struct LATB12_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB12_W<'a> {
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
#[doc = "Field `LATB13` reader - "]
pub struct LATB13_R(crate::FieldReader<bool, bool>);
impl LATB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB13` writer - "]
pub struct LATB13_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB13_W<'a> {
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
#[doc = "Field `LATB14` reader - "]
pub struct LATB14_R(crate::FieldReader<bool, bool>);
impl LATB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB14` writer - "]
pub struct LATB14_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB14_W<'a> {
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
#[doc = "Field `LATB15` reader - "]
pub struct LATB15_R(crate::FieldReader<bool, bool>);
impl LATB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATB15` writer - "]
pub struct LATB15_W<'a> {
    w: &'a mut W,
}
impl<'a> LATB15_W<'a> {
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
    pub fn latb0(&self) -> LATB0_R {
        LATB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn latb1(&self) -> LATB1_R {
        LATB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn latb2(&self) -> LATB2_R {
        LATB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn latb3(&self) -> LATB3_R {
        LATB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn latb4(&self) -> LATB4_R {
        LATB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn latb5(&self) -> LATB5_R {
        LATB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn latb6(&self) -> LATB6_R {
        LATB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn latb7(&self) -> LATB7_R {
        LATB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn latb8(&self) -> LATB8_R {
        LATB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn latb9(&self) -> LATB9_R {
        LATB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn latb10(&self) -> LATB10_R {
        LATB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn latb11(&self) -> LATB11_R {
        LATB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn latb12(&self) -> LATB12_R {
        LATB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn latb13(&self) -> LATB13_R {
        LATB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn latb14(&self) -> LATB14_R {
        LATB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn latb15(&self) -> LATB15_R {
        LATB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn latb0(&mut self) -> LATB0_W {
        LATB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn latb1(&mut self) -> LATB1_W {
        LATB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn latb2(&mut self) -> LATB2_W {
        LATB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn latb3(&mut self) -> LATB3_W {
        LATB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn latb4(&mut self) -> LATB4_W {
        LATB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn latb5(&mut self) -> LATB5_W {
        LATB5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn latb6(&mut self) -> LATB6_W {
        LATB6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn latb7(&mut self) -> LATB7_W {
        LATB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn latb8(&mut self) -> LATB8_W {
        LATB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn latb9(&mut self) -> LATB9_W {
        LATB9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn latb10(&mut self) -> LATB10_W {
        LATB10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn latb11(&mut self) -> LATB11_W {
        LATB11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn latb12(&mut self) -> LATB12_W {
        LATB12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn latb13(&mut self) -> LATB13_W {
        LATB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn latb14(&mut self) -> LATB14_W {
        LATB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn latb15(&mut self) -> LATB15_W {
        LATB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LATB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lat](index.html) module"]
pub struct LAT_SPEC;
impl crate::RegisterSpec for LAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lat::R](R) reader structure"]
impl crate::Readable for LAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lat::W](W) writer structure"]
impl crate::Writable for LAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LAT to value 0"]
impl crate::Resettable for LAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
