#[doc = "Register `CNPD` reader"]
pub struct R(crate::R<CNPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CNPD_SPEC>> for R {
    fn from(reader: crate::R<CNPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNPD` writer"]
pub struct W(crate::W<CNPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNPD_SPEC>;
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
impl core::convert::From<crate::W<CNPD_SPEC>> for W {
    fn from(writer: crate::W<CNPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNPDA0` reader - "]
pub struct CNPDA0_R(crate::FieldReader<bool, bool>);
impl CNPDA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDA0` writer - "]
pub struct CNPDA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA0_W<'a> {
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
#[doc = "Field `CNPDA1` reader - "]
pub struct CNPDA1_R(crate::FieldReader<bool, bool>);
impl CNPDA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDA1` writer - "]
pub struct CNPDA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA1_W<'a> {
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
#[doc = "Field `CNPDA2` reader - "]
pub struct CNPDA2_R(crate::FieldReader<bool, bool>);
impl CNPDA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDA2` writer - "]
pub struct CNPDA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA2_W<'a> {
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
#[doc = "Field `CNPDA3` reader - "]
pub struct CNPDA3_R(crate::FieldReader<bool, bool>);
impl CNPDA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDA3` writer - "]
pub struct CNPDA3_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA3_W<'a> {
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
#[doc = "Field `CNPDA4` reader - "]
pub struct CNPDA4_R(crate::FieldReader<bool, bool>);
impl CNPDA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNPDA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNPDA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNPDA4` writer - "]
pub struct CNPDA4_W<'a> {
    w: &'a mut W,
}
impl<'a> CNPDA4_W<'a> {
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
    pub fn cnpda0(&self) -> CNPDA0_R {
        CNPDA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpda1(&self) -> CNPDA1_R {
        CNPDA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpda2(&self) -> CNPDA2_R {
        CNPDA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpda3(&self) -> CNPDA3_R {
        CNPDA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpda4(&self) -> CNPDA4_R {
        CNPDA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnpda0(&mut self) -> CNPDA0_W {
        CNPDA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnpda1(&mut self) -> CNPDA1_W {
        CNPDA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cnpda2(&mut self) -> CNPDA2_W {
        CNPDA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cnpda3(&mut self) -> CNPDA3_W {
        CNPDA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cnpda4(&mut self) -> CNPDA4_W {
        CNPDA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CNPDA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnpd](index.html) module"]
pub struct CNPD_SPEC;
impl crate::RegisterSpec for CNPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnpd::R](R) reader structure"]
impl crate::Readable for CNPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnpd::W](W) writer structure"]
impl crate::Writable for CNPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNPD to value 0"]
impl crate::Resettable for CNPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
