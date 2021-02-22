#[doc = "Register `CNPDSET` reader"]
pub struct R(crate::R<CNPDSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNPDSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNPDSET_SPEC>> for R {
    fn from(reader: crate::R<CNPDSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNPDSET` writer"]
pub struct W(crate::W<CNPDSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNPDSET_SPEC>;
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
impl core::convert::From<crate::W<CNPDSET_SPEC>> for W {
    fn from(writer: crate::W<CNPDSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNPDB0` reader - "]
pub struct CNPDB0_R(crate::FieldReader<bool, bool>);
impl CNPDB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB0` writer - "]
pub struct CNPDB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB0_W<'a> {
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
#[doc = "Field `CNPDB1` reader - "]
pub struct CNPDB1_R(crate::FieldReader<bool, bool>);
impl CNPDB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB1` writer - "]
pub struct CNPDB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB1_W<'a> {
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
#[doc = "Field `CNPDB2` reader - "]
pub struct CNPDB2_R(crate::FieldReader<bool, bool>);
impl CNPDB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB2` writer - "]
pub struct CNPDB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB2_W<'a> {
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
#[doc = "Field `CNPDB3` reader - "]
pub struct CNPDB3_R(crate::FieldReader<bool, bool>);
impl CNPDB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB3` writer - "]
pub struct CNPDB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB3_W<'a> {
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
#[doc = "Field `CNPDB4` reader - "]
pub struct CNPDB4_R(crate::FieldReader<bool, bool>);
impl CNPDB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB4` writer - "]
pub struct CNPDB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB4_W<'a> {
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
#[doc = "Field `CNPDB5` reader - "]
pub struct CNPDB5_R(crate::FieldReader<bool, bool>);
impl CNPDB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB5` writer - "]
pub struct CNPDB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB5_W<'a> {
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
#[doc = "Field `CNPDB7` reader - "]
pub struct CNPDB7_R(crate::FieldReader<bool, bool>);
impl CNPDB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB7` writer - "]
pub struct CNPDB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB7_W<'a> {
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
#[doc = "Field `CNPDB8` reader - "]
pub struct CNPDB8_R(crate::FieldReader<bool, bool>);
impl CNPDB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB8` writer - "]
pub struct CNPDB8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB8_W<'a> {
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
#[doc = "Field `CNPDB9` reader - "]
pub struct CNPDB9_R(crate::FieldReader<bool, bool>);
impl CNPDB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB9` writer - "]
pub struct CNPDB9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB9_W<'a> {
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
#[doc = "Field `CNPDB10` reader - "]
pub struct CNPDB10_R(crate::FieldReader<bool, bool>);
impl CNPDB10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB10` writer - "]
pub struct CNPDB10_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB10_W<'a> {
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
#[doc = "Field `CNPDB11` reader - "]
pub struct CNPDB11_R(crate::FieldReader<bool, bool>);
impl CNPDB11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB11` writer - "]
pub struct CNPDB11_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB11_W<'a> {
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
#[doc = "Field `CNPDB13` reader - "]
pub struct CNPDB13_R(crate::FieldReader<bool, bool>);
impl CNPDB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB13` writer - "]
pub struct CNPDB13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB13_W<'a> {
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
#[doc = "Field `CNPDB14` reader - "]
pub struct CNPDB14_R(crate::FieldReader<bool, bool>);
impl CNPDB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB14` writer - "]
pub struct CNPDB14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB14_W<'a> {
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
#[doc = "Field `CNPDB15` reader - "]
pub struct CNPDB15_R(crate::FieldReader<bool, bool>);
impl CNPDB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDB15` writer - "]
pub struct CNPDB15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDB15_W<'a> {
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
    pub fn cnpdb0(&self) -> CNPDB0_R {
        CNPDB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpdb1(&self) -> CNPDB1_R {
        CNPDB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpdb2(&self) -> CNPDB2_R {
        CNPDB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpdb3(&self) -> CNPDB3_R {
        CNPDB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpdb4(&self) -> CNPDB4_R {
        CNPDB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnpdb5(&self) -> CNPDB5_R {
        CNPDB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnpdb7(&self) -> CNPDB7_R {
        CNPDB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnpdb8(&self) -> CNPDB8_R {
        CNPDB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnpdb9(&self) -> CNPDB9_R {
        CNPDB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cnpdb10(&self) -> CNPDB10_R {
        CNPDB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cnpdb11(&self) -> CNPDB11_R {
        CNPDB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnpdb13(&self) -> CNPDB13_R {
        CNPDB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnpdb14(&self) -> CNPDB14_R {
        CNPDB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnpdb15(&self) -> CNPDB15_R {
        CNPDB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpdb0(&mut self) -> CNPDB0_W {
        CNPDB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpdb1(&mut self) -> CNPDB1_W {
        CNPDB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpdb2(&mut self) -> CNPDB2_W {
        CNPDB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpdb3(&mut self) -> CNPDB3_W {
        CNPDB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpdb4(&mut self) -> CNPDB4_W {
        CNPDB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnpdb5(&mut self) -> CNPDB5_W {
        CNPDB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnpdb7(&mut self) -> CNPDB7_W {
        CNPDB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnpdb8(&mut self) -> CNPDB8_W {
        CNPDB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnpdb9(&mut self) -> CNPDB9_W {
        CNPDB9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cnpdb10(&mut self) -> CNPDB10_W {
        CNPDB10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cnpdb11(&mut self) -> CNPDB11_W {
        CNPDB11_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnpdb13(&mut self) -> CNPDB13_W {
        CNPDB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnpdb14(&mut self) -> CNPDB14_W {
        CNPDB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnpdb15(&mut self) -> CNPDB15_W {
        CNPDB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CNPDBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpdset](index.html) module"]
pub struct CNPDSET_SPEC;
impl crate::RegisterSpec for CNPDSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnpdset::R](R) reader structure"]
impl crate::Readable for CNPDSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnpdset::W](W) writer structure"]
impl crate::Writable for CNPDSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNPDSET to value 0"]
impl crate::Resettable for CNPDSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
