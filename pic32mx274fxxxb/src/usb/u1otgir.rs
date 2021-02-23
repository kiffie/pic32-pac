#[doc = "Register `U1OTGIR` reader"]
pub struct R(crate::R<U1OTGIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1OTGIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1OTGIR_SPEC>> for R {
    fn from(reader: crate::R<U1OTGIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1OTGIR` writer"]
pub struct W(crate::W<U1OTGIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1OTGIR_SPEC>;
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
impl core::convert::From<crate::W<U1OTGIR_SPEC>> for W {
    fn from(writer: crate::W<U1OTGIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSVDIF` reader - "]
pub struct VBUSVDIF_R(crate::FieldReader<bool, bool>);
impl VBUSVDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSVDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSVDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBUSVDIF` writer - "]
pub struct VBUSVDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSVDIF_W<'a> {
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
#[doc = "Field `SESENDIF` reader - "]
pub struct SESENDIF_R(crate::FieldReader<bool, bool>);
impl SESENDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESENDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESENDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SESENDIF` writer - "]
pub struct SESENDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SESENDIF_W<'a> {
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
#[doc = "Field `SESVDIF` reader - "]
pub struct SESVDIF_R(crate::FieldReader<bool, bool>);
impl SESVDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SESVDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SESVDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SESVDIF` writer - "]
pub struct SESVDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SESVDIF_W<'a> {
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
#[doc = "Field `ACTVIF` reader - "]
pub struct ACTVIF_R(crate::FieldReader<bool, bool>);
impl ACTVIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTVIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTVIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTVIF` writer - "]
pub struct ACTVIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTVIF_W<'a> {
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
#[doc = "Field `LSTATEIF` reader - "]
pub struct LSTATEIF_R(crate::FieldReader<bool, bool>);
impl LSTATEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSTATEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSTATEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSTATEIF` writer - "]
pub struct LSTATEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSTATEIF_W<'a> {
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
#[doc = "Field `T1MSECIF` reader - "]
pub struct T1MSECIF_R(crate::FieldReader<bool, bool>);
impl T1MSECIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        T1MSECIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T1MSECIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T1MSECIF` writer - "]
pub struct T1MSECIF_W<'a> {
    w: &'a mut W,
}
impl<'a> T1MSECIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `IDIF` reader - "]
pub struct IDIF_R(crate::FieldReader<bool, bool>);
impl IDIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDIF` writer - "]
pub struct IDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIF_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvdif(&self) -> VBUSVDIF_R {
        VBUSVDIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesendif(&self) -> SESENDIF_R {
        SESENDIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvdif(&self) -> SESVDIF_R {
        SESVDIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn actvif(&self) -> ACTVIF_R {
        ACTVIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstateif(&self) -> LSTATEIF_R {
        LSTATEIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t1msecif(&self) -> T1MSECIF_R {
        T1MSECIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idif(&self) -> IDIF_R {
        IDIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vbusvdif(&mut self) -> VBUSVDIF_W {
        VBUSVDIF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesendif(&mut self) -> SESENDIF_W {
        SESENDIF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sesvdif(&mut self) -> SESVDIF_W {
        SESVDIF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn actvif(&mut self) -> ACTVIF_W {
        ACTVIF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lstateif(&mut self) -> LSTATEIF_W {
        LSTATEIF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn t1msecif(&mut self) -> T1MSECIF_W {
        T1MSECIF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn idif(&mut self) -> IDIF_W {
        IDIF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1OTGIR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1otgir](index.html) module"]
pub struct U1OTGIR_SPEC;
impl crate::RegisterSpec for U1OTGIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1otgir::R](R) reader structure"]
impl crate::Readable for U1OTGIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1otgir::W](W) writer structure"]
impl crate::Writable for U1OTGIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1OTGIR to value 0"]
impl crate::Resettable for U1OTGIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
