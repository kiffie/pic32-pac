#[doc = "Register `CNPU` reader"]
pub struct R(crate::R<CNPU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNPU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNPU_SPEC>> for R {
    fn from(reader: crate::R<CNPU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNPU` writer"]
pub struct W(crate::W<CNPU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNPU_SPEC>;
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
impl core::convert::From<crate::W<CNPU_SPEC>> for W {
    fn from(writer: crate::W<CNPU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNPUB0` reader - "]
pub struct CNPUB0_R(crate::FieldReader<bool, bool>);
impl CNPUB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB0` writer - "]
pub struct CNPUB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB0_W<'a> {
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
#[doc = "Field `CNPUB1` reader - "]
pub struct CNPUB1_R(crate::FieldReader<bool, bool>);
impl CNPUB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB1` writer - "]
pub struct CNPUB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB1_W<'a> {
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
#[doc = "Field `CNPUB2` reader - "]
pub struct CNPUB2_R(crate::FieldReader<bool, bool>);
impl CNPUB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB2` writer - "]
pub struct CNPUB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB2_W<'a> {
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
#[doc = "Field `CNPUB3` reader - "]
pub struct CNPUB3_R(crate::FieldReader<bool, bool>);
impl CNPUB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB3` writer - "]
pub struct CNPUB3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB3_W<'a> {
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
#[doc = "Field `CNPUB4` reader - "]
pub struct CNPUB4_R(crate::FieldReader<bool, bool>);
impl CNPUB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB4` writer - "]
pub struct CNPUB4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB4_W<'a> {
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
#[doc = "Field `CNPUB5` reader - "]
pub struct CNPUB5_R(crate::FieldReader<bool, bool>);
impl CNPUB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB5` writer - "]
pub struct CNPUB5_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB5_W<'a> {
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
#[doc = "Field `CNPUB7` reader - "]
pub struct CNPUB7_R(crate::FieldReader<bool, bool>);
impl CNPUB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB7` writer - "]
pub struct CNPUB7_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB7_W<'a> {
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
#[doc = "Field `CNPUB8` reader - "]
pub struct CNPUB8_R(crate::FieldReader<bool, bool>);
impl CNPUB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB8` writer - "]
pub struct CNPUB8_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB8_W<'a> {
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
#[doc = "Field `CNPUB9` reader - "]
pub struct CNPUB9_R(crate::FieldReader<bool, bool>);
impl CNPUB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB9` writer - "]
pub struct CNPUB9_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB9_W<'a> {
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
#[doc = "Field `CNPUB10` reader - "]
pub struct CNPUB10_R(crate::FieldReader<bool, bool>);
impl CNPUB10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB10` writer - "]
pub struct CNPUB10_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB10_W<'a> {
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
#[doc = "Field `CNPUB11` reader - "]
pub struct CNPUB11_R(crate::FieldReader<bool, bool>);
impl CNPUB11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB11` writer - "]
pub struct CNPUB11_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB11_W<'a> {
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
#[doc = "Field `CNPUB13` reader - "]
pub struct CNPUB13_R(crate::FieldReader<bool, bool>);
impl CNPUB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB13` writer - "]
pub struct CNPUB13_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB13_W<'a> {
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
#[doc = "Field `CNPUB14` reader - "]
pub struct CNPUB14_R(crate::FieldReader<bool, bool>);
impl CNPUB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB14` writer - "]
pub struct CNPUB14_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB14_W<'a> {
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
#[doc = "Field `CNPUB15` reader - "]
pub struct CNPUB15_R(crate::FieldReader<bool, bool>);
impl CNPUB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPUB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPUB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPUB15` writer - "]
pub struct CNPUB15_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPUB15_W<'a> {
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
    pub fn cnpub0(&self) -> CNPUB0_R {
        CNPUB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpub1(&self) -> CNPUB1_R {
        CNPUB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpub2(&self) -> CNPUB2_R {
        CNPUB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpub3(&self) -> CNPUB3_R {
        CNPUB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpub4(&self) -> CNPUB4_R {
        CNPUB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnpub5(&self) -> CNPUB5_R {
        CNPUB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnpub7(&self) -> CNPUB7_R {
        CNPUB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnpub8(&self) -> CNPUB8_R {
        CNPUB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnpub9(&self) -> CNPUB9_R {
        CNPUB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cnpub10(&self) -> CNPUB10_R {
        CNPUB10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cnpub11(&self) -> CNPUB11_R {
        CNPUB11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnpub13(&self) -> CNPUB13_R {
        CNPUB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnpub14(&self) -> CNPUB14_R {
        CNPUB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnpub15(&self) -> CNPUB15_R {
        CNPUB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpub0(&mut self) -> CNPUB0_W {
        CNPUB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpub1(&mut self) -> CNPUB1_W {
        CNPUB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpub2(&mut self) -> CNPUB2_W {
        CNPUB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpub3(&mut self) -> CNPUB3_W {
        CNPUB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpub4(&mut self) -> CNPUB4_W {
        CNPUB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cnpub5(&mut self) -> CNPUB5_W {
        CNPUB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cnpub7(&mut self) -> CNPUB7_W {
        CNPUB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cnpub8(&mut self) -> CNPUB8_W {
        CNPUB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cnpub9(&mut self) -> CNPUB9_W {
        CNPUB9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cnpub10(&mut self) -> CNPUB10_W {
        CNPUB10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cnpub11(&mut self) -> CNPUB11_W {
        CNPUB11_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnpub13(&mut self) -> CNPUB13_W {
        CNPUB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnpub14(&mut self) -> CNPUB14_W {
        CNPUB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cnpub15(&mut self) -> CNPUB15_W {
        CNPUB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CNPUB register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpu](index.html) module"]
pub struct CNPU_SPEC;
impl crate::RegisterSpec for CNPU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnpu::R](R) reader structure"]
impl crate::Readable for CNPU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnpu::W](W) writer structure"]
impl crate::Writable for CNPU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNPU to value 0"]
impl crate::Resettable for CNPU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
