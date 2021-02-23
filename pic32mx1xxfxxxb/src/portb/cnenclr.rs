#[doc = "Register `CNENCLR` reader"]
pub struct R(crate::R<CNENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNENCLR_SPEC>> for R {
    fn from(reader: crate::R<CNENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNENCLR` writer"]
pub struct W(crate::W<CNENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNENCLR_SPEC>;
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
impl core::convert::From<crate::W<CNENCLR_SPEC>> for W {
    fn from(writer: crate::W<CNENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNIEB0` reader - "]
pub struct CNIEB0_R(crate::FieldReader<bool, bool>);
impl CNIEB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB0` writer - "]
pub struct CNIEB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB0_W<'a> {
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
#[doc = "Field `CNIEB1` reader - "]
pub struct CNIEB1_R(crate::FieldReader<bool, bool>);
impl CNIEB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB1` writer - "]
pub struct CNIEB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB1_W<'a> {
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
#[doc = "Field `CNIEB2` reader - "]
pub struct CNIEB2_R(crate::FieldReader<bool, bool>);
impl CNIEB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB2` writer - "]
pub struct CNIEB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB2_W<'a> {
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
#[doc = "Field `CNIEB3` reader - "]
pub struct CNIEB3_R(crate::FieldReader<bool, bool>);
impl CNIEB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB3` writer - "]
pub struct CNIEB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB3_W<'a> {
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
#[doc = "Field `CNIEB4` reader - "]
pub struct CNIEB4_R(crate::FieldReader<bool, bool>);
impl CNIEB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB4` writer - "]
pub struct CNIEB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB4_W<'a> {
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
#[doc = "Field `CNIEB5` reader - "]
pub struct CNIEB5_R(crate::FieldReader<bool, bool>);
impl CNIEB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB5` writer - "]
pub struct CNIEB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB5_W<'a> {
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
#[doc = "Field `CNIEB6` reader - "]
pub struct CNIEB6_R(crate::FieldReader<bool, bool>);
impl CNIEB6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB6` writer - "]
pub struct CNIEB6_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB6_W<'a> {
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
#[doc = "Field `CNIEB7` reader - "]
pub struct CNIEB7_R(crate::FieldReader<bool, bool>);
impl CNIEB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB7` writer - "]
pub struct CNIEB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB7_W<'a> {
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
#[doc = "Field `CNIEB8` reader - "]
pub struct CNIEB8_R(crate::FieldReader<bool, bool>);
impl CNIEB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB8` writer - "]
pub struct CNIEB8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB8_W<'a> {
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
#[doc = "Field `CNIEB9` reader - "]
pub struct CNIEB9_R(crate::FieldReader<bool, bool>);
impl CNIEB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB9` writer - "]
pub struct CNIEB9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB9_W<'a> {
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
#[doc = "Field `CNIEB10` reader - "]
pub struct CNIEB10_R(crate::FieldReader<bool, bool>);
impl CNIEB10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB10` writer - "]
pub struct CNIEB10_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB10_W<'a> {
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
#[doc = "Field `CNIEB11` reader - "]
pub struct CNIEB11_R(crate::FieldReader<bool, bool>);
impl CNIEB11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB11` writer - "]
pub struct CNIEB11_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB11_W<'a> {
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
#[doc = "Field `CNIEB12` reader - "]
pub struct CNIEB12_R(crate::FieldReader<bool, bool>);
impl CNIEB12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB12` writer - "]
pub struct CNIEB12_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB12_W<'a> {
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
#[doc = "Field `CNIEB13` reader - "]
pub struct CNIEB13_R(crate::FieldReader<bool, bool>);
impl CNIEB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB13` writer - "]
pub struct CNIEB13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB13_W<'a> {
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
#[doc = "Field `CNIEB14` reader - "]
pub struct CNIEB14_R(crate::FieldReader<bool, bool>);
impl CNIEB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB14` writer - "]
pub struct CNIEB14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB14_W<'a> {
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
#[doc = "Field `CNIEB15` reader - "]
pub struct CNIEB15_R(crate::FieldReader<bool, bool>);
impl CNIEB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEB15` writer - "]
pub struct CNIEB15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEB15_W<'a> {
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
    pub fn cnieb0(&self) -> CNIEB0_R {
        CNIEB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnieb1(&self) -> CNIEB1_R {
        CNIEB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnieb2(&self) -> CNIEB2_R {
        CNIEB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnieb3(&self) -> CNIEB3_R {
        CNIEB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnieb4(&self) -> CNIEB4_R {
        CNIEB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnieb5(&self) -> CNIEB5_R {
        CNIEB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cnieb6(&self) -> CNIEB6_R {
        CNIEB6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnieb7(&self) -> CNIEB7_R {
        CNIEB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnieb8(&self) -> CNIEB8_R {
        CNIEB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnieb9(&self) -> CNIEB9_R {
        CNIEB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cnieb10(&self) -> CNIEB10_R {
        CNIEB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cnieb11(&self) -> CNIEB11_R {
        CNIEB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cnieb12(&self) -> CNIEB12_R {
        CNIEB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnieb13(&self) -> CNIEB13_R {
        CNIEB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnieb14(&self) -> CNIEB14_R {
        CNIEB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnieb15(&self) -> CNIEB15_R {
        CNIEB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnieb0(&mut self) -> CNIEB0_W {
        CNIEB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnieb1(&mut self) -> CNIEB1_W {
        CNIEB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnieb2(&mut self) -> CNIEB2_W {
        CNIEB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnieb3(&mut self) -> CNIEB3_W {
        CNIEB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnieb4(&mut self) -> CNIEB4_W {
        CNIEB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnieb5(&mut self) -> CNIEB5_W {
        CNIEB5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cnieb6(&mut self) -> CNIEB6_W {
        CNIEB6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnieb7(&mut self) -> CNIEB7_W {
        CNIEB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnieb8(&mut self) -> CNIEB8_W {
        CNIEB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnieb9(&mut self) -> CNIEB9_W {
        CNIEB9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cnieb10(&mut self) -> CNIEB10_W {
        CNIEB10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cnieb11(&mut self) -> CNIEB11_W {
        CNIEB11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cnieb12(&mut self) -> CNIEB12_W {
        CNIEB12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnieb13(&mut self) -> CNIEB13_W {
        CNIEB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnieb14(&mut self) -> CNIEB14_W {
        CNIEB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnieb15(&mut self) -> CNIEB15_W {
        CNIEB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CNENBCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnenclr](index.html) module"]
pub struct CNENCLR_SPEC;
impl crate::RegisterSpec for CNENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnenclr::R](R) reader structure"]
impl crate::Readable for CNENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnenclr::W](W) writer structure"]
impl crate::Writable for CNENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNENCLR to value 0"]
impl crate::Resettable for CNENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
