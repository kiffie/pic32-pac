#[doc = "Register `TRISINV` reader"]
pub struct R(crate::R<TRISINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRISINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TRISINV_SPEC>> for R {
    fn from(reader: crate::R<TRISINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRISINV` writer"]
pub struct W(crate::W<TRISINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRISINV_SPEC>;
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
impl core::convert::From<crate::W<TRISINV_SPEC>> for W {
    fn from(writer: crate::W<TRISINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRISB0` reader - "]
pub struct TRISB0_R(crate::FieldReader<bool, bool>);
impl TRISB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB0` writer - "]
pub struct TRISB0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB0_W<'a> {
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
#[doc = "Field `TRISB1` reader - "]
pub struct TRISB1_R(crate::FieldReader<bool, bool>);
impl TRISB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB1` writer - "]
pub struct TRISB1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB1_W<'a> {
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
#[doc = "Field `TRISB2` reader - "]
pub struct TRISB2_R(crate::FieldReader<bool, bool>);
impl TRISB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB2` writer - "]
pub struct TRISB2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB2_W<'a> {
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
#[doc = "Field `TRISB3` reader - "]
pub struct TRISB3_R(crate::FieldReader<bool, bool>);
impl TRISB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB3` writer - "]
pub struct TRISB3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB3_W<'a> {
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
#[doc = "Field `TRISB4` reader - "]
pub struct TRISB4_R(crate::FieldReader<bool, bool>);
impl TRISB4_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB4` writer - "]
pub struct TRISB4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB4_W<'a> {
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
#[doc = "Field `TRISB5` reader - "]
pub struct TRISB5_R(crate::FieldReader<bool, bool>);
impl TRISB5_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB5` writer - "]
pub struct TRISB5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB5_W<'a> {
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
#[doc = "Field `TRISB7` reader - "]
pub struct TRISB7_R(crate::FieldReader<bool, bool>);
impl TRISB7_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB7` writer - "]
pub struct TRISB7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB7_W<'a> {
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
#[doc = "Field `TRISB8` reader - "]
pub struct TRISB8_R(crate::FieldReader<bool, bool>);
impl TRISB8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB8` writer - "]
pub struct TRISB8_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB8_W<'a> {
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
#[doc = "Field `TRISB9` reader - "]
pub struct TRISB9_R(crate::FieldReader<bool, bool>);
impl TRISB9_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB9` writer - "]
pub struct TRISB9_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB9_W<'a> {
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
#[doc = "Field `TRISB13` reader - "]
pub struct TRISB13_R(crate::FieldReader<bool, bool>);
impl TRISB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB13` writer - "]
pub struct TRISB13_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB13_W<'a> {
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
#[doc = "Field `TRISB14` reader - "]
pub struct TRISB14_R(crate::FieldReader<bool, bool>);
impl TRISB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB14` writer - "]
pub struct TRISB14_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB14_W<'a> {
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
#[doc = "Field `TRISB15` reader - "]
pub struct TRISB15_R(crate::FieldReader<bool, bool>);
impl TRISB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRISB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRISB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRISB15` writer - "]
pub struct TRISB15_W<'a> {
    w: &'a mut W,
}
impl<'a> TRISB15_W<'a> {
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
    pub fn trisb0(&self) -> TRISB0_R {
        TRISB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trisb1(&self) -> TRISB1_R {
        TRISB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trisb2(&self) -> TRISB2_R {
        TRISB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trisb3(&self) -> TRISB3_R {
        TRISB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trisb4(&self) -> TRISB4_R {
        TRISB4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trisb5(&self) -> TRISB5_R {
        TRISB5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trisb7(&self) -> TRISB7_R {
        TRISB7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trisb8(&self) -> TRISB8_R {
        TRISB8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn trisb9(&self) -> TRISB9_R {
        TRISB9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn trisb13(&self) -> TRISB13_R {
        TRISB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn trisb14(&self) -> TRISB14_R {
        TRISB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn trisb15(&self) -> TRISB15_R {
        TRISB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn trisb0(&mut self) -> TRISB0_W {
        TRISB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn trisb1(&mut self) -> TRISB1_W {
        TRISB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn trisb2(&mut self) -> TRISB2_W {
        TRISB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trisb3(&mut self) -> TRISB3_W {
        TRISB3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trisb4(&mut self) -> TRISB4_W {
        TRISB4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trisb5(&mut self) -> TRISB5_W {
        TRISB5_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn trisb7(&mut self) -> TRISB7_W {
        TRISB7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trisb8(&mut self) -> TRISB8_W {
        TRISB8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn trisb9(&mut self) -> TRISB9_W {
        TRISB9_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn trisb13(&mut self) -> TRISB13_W {
        TRISB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn trisb14(&mut self) -> TRISB14_W {
        TRISB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn trisb15(&mut self) -> TRISB15_W {
        TRISB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRISBINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trisinv](index.html) module"]
pub struct TRISINV_SPEC;
impl crate::RegisterSpec for TRISINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trisinv::R](R) reader structure"]
impl crate::Readable for TRISINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trisinv::W](W) writer structure"]
impl crate::Writable for TRISINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRISINV to value 0"]
impl crate::Resettable for TRISINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
