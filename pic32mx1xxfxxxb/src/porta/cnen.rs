#[doc = "Register `CNEN` reader"]
pub struct R(crate::R<CNEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNEN_SPEC>> for R {
    fn from(reader: crate::R<CNEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNEN` writer"]
pub struct W(crate::W<CNEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNEN_SPEC>;
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
impl core::convert::From<crate::W<CNEN_SPEC>> for W {
    fn from(writer: crate::W<CNEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNIEA0` reader - "]
pub struct CNIEA0_R(crate::FieldReader<bool, bool>);
impl CNIEA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEA0` writer - "]
pub struct CNIEA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA0_W<'a> {
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
#[doc = "Field `CNIEA1` reader - "]
pub struct CNIEA1_R(crate::FieldReader<bool, bool>);
impl CNIEA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEA1` writer - "]
pub struct CNIEA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA1_W<'a> {
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
#[doc = "Field `CNIEA2` reader - "]
pub struct CNIEA2_R(crate::FieldReader<bool, bool>);
impl CNIEA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEA2` writer - "]
pub struct CNIEA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA2_W<'a> {
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
#[doc = "Field `CNIEA3` reader - "]
pub struct CNIEA3_R(crate::FieldReader<bool, bool>);
impl CNIEA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEA3` writer - "]
pub struct CNIEA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA3_W<'a> {
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
#[doc = "Field `CNIEA4` reader - "]
pub struct CNIEA4_R(crate::FieldReader<bool, bool>);
impl CNIEA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNIEA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNIEA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNIEA4` writer - "]
pub struct CNIEA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIEA4_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cniea0(&self) -> CNIEA0_R {
        CNIEA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cniea1(&self) -> CNIEA1_R {
        CNIEA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cniea2(&self) -> CNIEA2_R {
        CNIEA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cniea3(&self) -> CNIEA3_R {
        CNIEA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cniea4(&self) -> CNIEA4_R {
        CNIEA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cniea0(&mut self) -> CNIEA0_W {
        CNIEA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cniea1(&mut self) -> CNIEA1_W {
        CNIEA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cniea2(&mut self) -> CNIEA2_W {
        CNIEA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cniea3(&mut self) -> CNIEA3_W {
        CNIEA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cniea4(&mut self) -> CNIEA4_W {
        CNIEA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CNENA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnen](index.html) module"]
pub struct CNEN_SPEC;
impl crate::RegisterSpec for CNEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnen::R](R) reader structure"]
impl crate::Readable for CNEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnen::W](W) writer structure"]
impl crate::Writable for CNEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNEN to value 0"]
impl crate::Resettable for CNEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
