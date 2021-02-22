#[doc = "Register `ODC` reader"]
pub struct R(crate::R<ODC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ODC_SPEC>> for R {
    fn from(reader: crate::R<ODC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ODC` writer"]
pub struct W(crate::W<ODC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODC_SPEC>;
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
impl core::convert::From<crate::W<ODC_SPEC>> for W {
    fn from(writer: crate::W<ODC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ODCA0` reader - "]
pub struct ODCA0_R(crate::FieldReader<bool, bool>);
impl ODCA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCA0` writer - "]
pub struct ODCA0_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA0_W<'a> {
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
#[doc = "Field `ODCA1` reader - "]
pub struct ODCA1_R(crate::FieldReader<bool, bool>);
impl ODCA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCA1` writer - "]
pub struct ODCA1_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA1_W<'a> {
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
#[doc = "Field `ODCA2` reader - "]
pub struct ODCA2_R(crate::FieldReader<bool, bool>);
impl ODCA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCA2` writer - "]
pub struct ODCA2_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA2_W<'a> {
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
#[doc = "Field `ODCA3` reader - "]
pub struct ODCA3_R(crate::FieldReader<bool, bool>);
impl ODCA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCA3` writer - "]
pub struct ODCA3_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA3_W<'a> {
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
#[doc = "Field `ODCA4` reader - "]
pub struct ODCA4_R(crate::FieldReader<bool, bool>);
impl ODCA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODCA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODCA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODCA4` writer - "]
pub struct ODCA4_W<'a> {
    w: &'a mut W,
}
impl<'a> ODCA4_W<'a> {
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
    pub fn odca0(&self) -> ODCA0_R {
        ODCA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odca1(&self) -> ODCA1_R {
        ODCA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odca2(&self) -> ODCA2_R {
        ODCA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odca3(&self) -> ODCA3_R {
        ODCA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odca4(&self) -> ODCA4_R {
        ODCA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn odca0(&mut self) -> ODCA0_W {
        ODCA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn odca1(&mut self) -> ODCA1_W {
        ODCA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn odca2(&mut self) -> ODCA2_W {
        ODCA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn odca3(&mut self) -> ODCA3_W {
        ODCA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn odca4(&mut self) -> ODCA4_W {
        ODCA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ODCA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odc](index.html) module"]
pub struct ODC_SPEC;
impl crate::RegisterSpec for ODC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odc::R](R) reader structure"]
impl crate::Readable for ODC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [odc::W](W) writer structure"]
impl crate::Writable for ODC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ODC to value 0"]
impl crate::Resettable for ODC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
