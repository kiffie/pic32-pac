#[doc = "Register `CFGCON` reader"]
pub struct R(crate::R<CFGCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CFGCON_SPEC>> for R {
    fn from(reader: crate::R<CFGCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGCON` writer"]
pub struct W(crate::W<CFGCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGCON_SPEC>;
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
impl core::convert::From<crate::W<CFGCON_SPEC>> for W {
    fn from(writer: crate::W<CFGCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDOEN` reader - "]
pub struct TDOEN_R(crate::FieldReader<bool, bool>);
impl TDOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDOEN` writer - "]
pub struct TDOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TDOEN_W<'a> {
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
#[doc = "Field `FAEN` reader - "]
pub struct FAEN_R(crate::FieldReader<bool, bool>);
impl FAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAEN` writer - "]
pub struct FAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAEN_W<'a> {
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
#[doc = "Field `JTAGEN` reader - "]
pub struct JTAGEN_R(crate::FieldReader<bool, bool>);
impl JTAGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAGEN` writer - "]
pub struct JTAGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAGEN_W<'a> {
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
#[doc = "Field `RPFA` reader - "]
pub struct RPFA_R(crate::FieldReader<bool, bool>);
impl RPFA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPFA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPFA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPFA` writer - "]
pub struct RPFA_W<'a> {
    w: &'a mut W,
}
impl<'a> RPFA_W<'a> {
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
#[doc = "Field `PMDLOCK` reader - "]
pub struct PMDLOCK_R(crate::FieldReader<bool, bool>);
impl PMDLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMDLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMDLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMDLOCK` writer - "]
pub struct PMDLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PMDLOCK_W<'a> {
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
#[doc = "Field `IOLOCK` reader - "]
pub struct IOLOCK_R(crate::FieldReader<bool, bool>);
impl IOLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOLOCK` writer - "]
pub struct IOLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLOCK_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdoen(&self) -> TDOEN_R {
        TDOEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn faen(&self) -> FAEN_R {
        FAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn jtagen(&self) -> JTAGEN_R {
        JTAGEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rpfa(&self) -> RPFA_R {
        RPFA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pmdlock(&self) -> PMDLOCK_R {
        PMDLOCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdoen(&mut self) -> TDOEN_W {
        TDOEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn faen(&mut self) -> FAEN_W {
        FAEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn jtagen(&mut self) -> JTAGEN_W {
        JTAGEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rpfa(&mut self) -> RPFA_W {
        RPFA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pmdlock(&mut self) -> PMDLOCK_W {
        PMDLOCK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W {
        IOLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFGCON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgcon](index.html) module"]
pub struct CFGCON_SPEC;
impl crate::RegisterSpec for CFGCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgcon::R](R) reader structure"]
impl crate::Readable for CFGCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgcon::W](W) writer structure"]
impl crate::Writable for CFGCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGCON to value 0x08"]
impl crate::Resettable for CFGCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
