#[doc = "Register `LATINV` reader"]
pub struct R(crate::R<LATINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LATINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LATINV_SPEC>> for R {
    fn from(reader: crate::R<LATINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LATINV` writer"]
pub struct W(crate::W<LATINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LATINV_SPEC>;
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
impl core::convert::From<crate::W<LATINV_SPEC>> for W {
    fn from(writer: crate::W<LATINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATA0` reader - "]
pub struct LATA0_R(crate::FieldReader<bool, bool>);
impl LATA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATA0` writer - "]
pub struct LATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA0_W<'a> {
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
#[doc = "Field `LATA1` reader - "]
pub struct LATA1_R(crate::FieldReader<bool, bool>);
impl LATA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATA1` writer - "]
pub struct LATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA1_W<'a> {
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
#[doc = "Field `LATA2` reader - "]
pub struct LATA2_R(crate::FieldReader<bool, bool>);
impl LATA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATA2` writer - "]
pub struct LATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA2_W<'a> {
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
#[doc = "Field `LATA3` reader - "]
pub struct LATA3_R(crate::FieldReader<bool, bool>);
impl LATA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATA3` writer - "]
pub struct LATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA3_W<'a> {
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
#[doc = "Field `LATA4` reader - "]
pub struct LATA4_R(crate::FieldReader<bool, bool>);
impl LATA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LATA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LATA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LATA4` writer - "]
pub struct LATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> LATA4_W<'a> {
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
    pub fn lata0(&self) -> LATA0_R {
        LATA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lata1(&self) -> LATA1_R {
        LATA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lata2(&self) -> LATA2_R {
        LATA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lata3(&self) -> LATA3_R {
        LATA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lata4(&self) -> LATA4_R {
        LATA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lata0(&mut self) -> LATA0_W {
        LATA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lata1(&mut self) -> LATA1_W {
        LATA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lata2(&mut self) -> LATA2_W {
        LATA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lata3(&mut self) -> LATA3_W {
        LATA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lata4(&mut self) -> LATA4_W {
        LATA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LATAINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [latinv](index.html) module"]
pub struct LATINV_SPEC;
impl crate::RegisterSpec for LATINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [latinv::R](R) reader structure"]
impl crate::Readable for LATINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [latinv::W](W) writer structure"]
impl crate::Writable for LATINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LATINV to value 0"]
impl crate::Resettable for LATINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
