#[doc = "Register `ANSELSET` reader"]
pub struct R(crate::R<ANSELSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANSELSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ANSELSET_SPEC>> for R {
    fn from(reader: crate::R<ANSELSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANSELSET` writer"]
pub struct W(crate::W<ANSELSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANSELSET_SPEC>;
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
impl core::convert::From<crate::W<ANSELSET_SPEC>> for W {
    fn from(writer: crate::W<ANSELSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANSB0` reader - "]
pub struct ANSB0_R(crate::FieldReader<bool, bool>);
impl ANSB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSB0` writer - "]
pub struct ANSB0_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB0_W<'a> {
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
#[doc = "Field `ANSB1` reader - "]
pub struct ANSB1_R(crate::FieldReader<bool, bool>);
impl ANSB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSB1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSB1` writer - "]
pub struct ANSB1_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB1_W<'a> {
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
#[doc = "Field `ANSB2` reader - "]
pub struct ANSB2_R(crate::FieldReader<bool, bool>);
impl ANSB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSB2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSB2` writer - "]
pub struct ANSB2_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB2_W<'a> {
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
#[doc = "Field `ANSB3` reader - "]
pub struct ANSB3_R(crate::FieldReader<bool, bool>);
impl ANSB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSB3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSB3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSB3` writer - "]
pub struct ANSB3_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB3_W<'a> {
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
#[doc = "Field `ANSB12` reader - "]
pub struct ANSB12_R(crate::FieldReader<bool, bool>);
impl ANSB12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSB12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSB12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSB12` writer - "]
pub struct ANSB12_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB12_W<'a> {
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
#[doc = "Field `ANSB13` reader - "]
pub struct ANSB13_R(crate::FieldReader<bool, bool>);
impl ANSB13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSB13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSB13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSB13` writer - "]
pub struct ANSB13_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB13_W<'a> {
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
#[doc = "Field `ANSB14` reader - "]
pub struct ANSB14_R(crate::FieldReader<bool, bool>);
impl ANSB14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSB14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSB14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSB14` writer - "]
pub struct ANSB14_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB14_W<'a> {
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
#[doc = "Field `ANSB15` reader - "]
pub struct ANSB15_R(crate::FieldReader<bool, bool>);
impl ANSB15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANSB15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANSB15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANSB15` writer - "]
pub struct ANSB15_W<'a> {
    w: &'a mut W,
}
impl<'a> ANSB15_W<'a> {
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
    pub fn ansb0(&self) -> ANSB0_R {
        ANSB0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ansb1(&self) -> ANSB1_R {
        ANSB1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ansb2(&self) -> ANSB2_R {
        ANSB2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ansb3(&self) -> ANSB3_R {
        ANSB3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ansb12(&self) -> ANSB12_R {
        ANSB12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ansb13(&self) -> ANSB13_R {
        ANSB13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ansb14(&self) -> ANSB14_R {
        ANSB14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ansb15(&self) -> ANSB15_R {
        ANSB15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ansb0(&mut self) -> ANSB0_W {
        ANSB0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ansb1(&mut self) -> ANSB1_W {
        ANSB1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ansb2(&mut self) -> ANSB2_W {
        ANSB2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ansb3(&mut self) -> ANSB3_W {
        ANSB3_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ansb12(&mut self) -> ANSB12_W {
        ANSB12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ansb13(&mut self) -> ANSB13_W {
        ANSB13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ansb14(&mut self) -> ANSB14_W {
        ANSB14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ansb15(&mut self) -> ANSB15_W {
        ANSB15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ANSELBSET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anselset](index.html) module"]
pub struct ANSELSET_SPEC;
impl crate::RegisterSpec for ANSELSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anselset::R](R) reader structure"]
impl crate::Readable for ANSELSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anselset::W](W) writer structure"]
impl crate::Writable for ANSELSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANSELSET to value 0"]
impl crate::Resettable for ANSELSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
