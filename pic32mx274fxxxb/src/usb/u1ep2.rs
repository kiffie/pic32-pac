#[doc = "Register `U1EP2` reader"]
pub struct R(crate::R<U1EP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1EP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1EP2_SPEC>> for R {
    fn from(reader: crate::R<U1EP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1EP2` writer"]
pub struct W(crate::W<U1EP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1EP2_SPEC>;
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
impl core::convert::From<crate::W<U1EP2_SPEC>> for W {
    fn from(writer: crate::W<U1EP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPHSHK` reader - "]
pub struct EPHSHK_R(crate::FieldReader<bool, bool>);
impl EPHSHK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPHSHK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPHSHK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPHSHK` writer - "]
pub struct EPHSHK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPHSHK_W<'a> {
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
#[doc = "Field `EPSTALL` reader - "]
pub struct EPSTALL_R(crate::FieldReader<bool, bool>);
impl EPSTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPSTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPSTALL` writer - "]
pub struct EPSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSTALL_W<'a> {
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
#[doc = "Field `EPTXEN` reader - "]
pub struct EPTXEN_R(crate::FieldReader<bool, bool>);
impl EPTXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPTXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPTXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTXEN` writer - "]
pub struct EPTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTXEN_W<'a> {
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
#[doc = "Field `EPRXEN` reader - "]
pub struct EPRXEN_R(crate::FieldReader<bool, bool>);
impl EPRXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRXEN` writer - "]
pub struct EPRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRXEN_W<'a> {
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
#[doc = "Field `EPCONDIS` reader - "]
pub struct EPCONDIS_R(crate::FieldReader<bool, bool>);
impl EPCONDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPCONDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPCONDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPCONDIS` writer - "]
pub struct EPCONDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPCONDIS_W<'a> {
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
    pub fn ephshk(&self) -> EPHSHK_R {
        EPHSHK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epstall(&self) -> EPSTALL_R {
        EPSTALL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn eptxen(&self) -> EPTXEN_R {
        EPTXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn eprxen(&self) -> EPRXEN_R {
        EPRXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn epcondis(&self) -> EPCONDIS_R {
        EPCONDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ephshk(&mut self) -> EPHSHK_W {
        EPHSHK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epstall(&mut self) -> EPSTALL_W {
        EPSTALL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn eptxen(&mut self) -> EPTXEN_W {
        EPTXEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn eprxen(&mut self) -> EPRXEN_W {
        EPRXEN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn epcondis(&mut self) -> EPCONDIS_W {
        EPCONDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1EP2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1ep2](index.html) module"]
pub struct U1EP2_SPEC;
impl crate::RegisterSpec for U1EP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1ep2::R](R) reader structure"]
impl crate::Readable for U1EP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1ep2::W](W) writer structure"]
impl crate::Writable for U1EP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1EP2 to value 0"]
impl crate::Resettable for U1EP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
