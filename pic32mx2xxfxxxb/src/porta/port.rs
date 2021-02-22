#[doc = "Register `PORT` reader"]
pub struct R(crate::R<PORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PORT_SPEC>> for R {
    fn from(reader: crate::R<PORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORT` writer"]
pub struct W(crate::W<PORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORT_SPEC>;
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
impl core::convert::From<crate::W<PORT_SPEC>> for W {
    fn from(writer: crate::W<PORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA0` reader - "]
pub struct RA0_R(crate::FieldReader<bool, bool>);
impl RA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA0` writer - "]
pub struct RA0_W<'a> {
    w: &'a mut W,
}
impl<'a> RA0_W<'a> {
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
#[doc = "Field `RA1` reader - "]
pub struct RA1_R(crate::FieldReader<bool, bool>);
impl RA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA1` writer - "]
pub struct RA1_W<'a> {
    w: &'a mut W,
}
impl<'a> RA1_W<'a> {
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
#[doc = "Field `RA2` reader - "]
pub struct RA2_R(crate::FieldReader<bool, bool>);
impl RA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA2` writer - "]
pub struct RA2_W<'a> {
    w: &'a mut W,
}
impl<'a> RA2_W<'a> {
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
#[doc = "Field `RA3` reader - "]
pub struct RA3_R(crate::FieldReader<bool, bool>);
impl RA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA3` writer - "]
pub struct RA3_W<'a> {
    w: &'a mut W,
}
impl<'a> RA3_W<'a> {
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
#[doc = "Field `RA4` reader - "]
pub struct RA4_R(crate::FieldReader<bool, bool>);
impl RA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA4` writer - "]
pub struct RA4_W<'a> {
    w: &'a mut W,
}
impl<'a> RA4_W<'a> {
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
    pub fn ra0(&self) -> RA0_R {
        RA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ra1(&self) -> RA1_R {
        RA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ra2(&self) -> RA2_R {
        RA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ra3(&self) -> RA3_R {
        RA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ra4(&self) -> RA4_R {
        RA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ra0(&mut self) -> RA0_W {
        RA0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ra1(&mut self) -> RA1_W {
        RA1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ra2(&mut self) -> RA2_W {
        RA2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ra3(&mut self) -> RA3_W {
        RA3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ra4(&mut self) -> RA4_W {
        RA4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORTA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port](index.html) module"]
pub struct PORT_SPEC;
impl crate::RegisterSpec for PORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [port::R](R) reader structure"]
impl crate::Readable for PORT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [port::W](W) writer structure"]
impl crate::Writable for PORT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORT to value 0"]
impl crate::Resettable for PORT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
