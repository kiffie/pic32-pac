#[doc = "Register `CNSTATCLR` reader"]
pub struct R(crate::R<CNSTATCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNSTATCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNSTATCLR_SPEC>> for R {
    fn from(reader: crate::R<CNSTATCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNSTATCLR` writer"]
pub struct W(crate::W<CNSTATCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNSTATCLR_SPEC>;
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
impl core::convert::From<crate::W<CNSTATCLR_SPEC>> for W {
    fn from(writer: crate::W<CNSTATCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNSTATA0` reader - "]
pub struct CNSTATA0_R(crate::FieldReader<bool, bool>);
impl CNSTATA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATA0` writer - "]
pub struct CNSTATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA0_W<'a> {
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
#[doc = "Field `CNSTATA1` reader - "]
pub struct CNSTATA1_R(crate::FieldReader<bool, bool>);
impl CNSTATA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATA1` writer - "]
pub struct CNSTATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA1_W<'a> {
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
#[doc = "Field `CNSTATA2` reader - "]
pub struct CNSTATA2_R(crate::FieldReader<bool, bool>);
impl CNSTATA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATA2` writer - "]
pub struct CNSTATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA2_W<'a> {
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
#[doc = "Field `CNSTATA3` reader - "]
pub struct CNSTATA3_R(crate::FieldReader<bool, bool>);
impl CNSTATA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATA3` writer - "]
pub struct CNSTATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA3_W<'a> {
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
#[doc = "Field `CNSTATA4` reader - "]
pub struct CNSTATA4_R(crate::FieldReader<bool, bool>);
impl CNSTATA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNSTATA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNSTATA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNSTATA4` writer - "]
pub struct CNSTATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNSTATA4_W<'a> {
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
    pub fn cnstata0(&self) -> CNSTATA0_R {
        CNSTATA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnstata1(&self) -> CNSTATA1_R {
        CNSTATA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnstata2(&self) -> CNSTATA2_R {
        CNSTATA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnstata3(&self) -> CNSTATA3_R {
        CNSTATA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnstata4(&self) -> CNSTATA4_R {
        CNSTATA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnstata0(&mut self) -> CNSTATA0_W {
        CNSTATA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnstata1(&mut self) -> CNSTATA1_W {
        CNSTATA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnstata2(&mut self) -> CNSTATA2_W {
        CNSTATA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnstata3(&mut self) -> CNSTATA3_W {
        CNSTATA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnstata4(&mut self) -> CNSTATA4_W {
        CNSTATA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CNSTATACLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnstatclr](index.html) module"]
pub struct CNSTATCLR_SPEC;
impl crate::RegisterSpec for CNSTATCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnstatclr::R](R) reader structure"]
impl crate::Readable for CNSTATCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnstatclr::W](W) writer structure"]
impl crate::Writable for CNSTATCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNSTATCLR to value 0"]
impl crate::Resettable for CNSTATCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
