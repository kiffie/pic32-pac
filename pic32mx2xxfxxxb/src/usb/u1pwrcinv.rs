#[doc = "Register `U1PWRCINV` reader"]
pub struct R(crate::R<U1PWRCINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1PWRCINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1PWRCINV_SPEC>> for R {
    fn from(reader: crate::R<U1PWRCINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1PWRCINV` writer"]
pub struct W(crate::W<U1PWRCINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1PWRCINV_SPEC>;
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
impl core::convert::From<crate::W<U1PWRCINV_SPEC>> for W {
    fn from(writer: crate::W<U1PWRCINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBPWR` reader - "]
pub struct USBPWR_R(crate::FieldReader<bool, bool>);
impl USBPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBPWR` writer - "]
pub struct USBPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> USBPWR_W<'a> {
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
#[doc = "Field `USUSPEND` reader - "]
pub struct USUSPEND_R(crate::FieldReader<bool, bool>);
impl USUSPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        USUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USUSPEND` writer - "]
pub struct USUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> USUSPEND_W<'a> {
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
#[doc = "Field `USBBUSY` reader - "]
pub struct USBBUSY_R(crate::FieldReader<bool, bool>);
impl USBBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBBUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBBUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBBUSY` writer - "]
pub struct USBBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> USBBUSY_W<'a> {
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
#[doc = "Field `USLPGRD` reader - "]
pub struct USLPGRD_R(crate::FieldReader<bool, bool>);
impl USLPGRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        USLPGRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USLPGRD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USLPGRD` writer - "]
pub struct USLPGRD_W<'a> {
    w: &'a mut W,
}
impl<'a> USLPGRD_W<'a> {
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
#[doc = "Field `UACTPND` reader - "]
pub struct UACTPND_R(crate::FieldReader<bool, bool>);
impl UACTPND_R {
    pub(crate) fn new(bits: bool) -> Self {
        UACTPND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UACTPND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UACTPND` writer - "]
pub struct UACTPND_W<'a> {
    w: &'a mut W,
}
impl<'a> UACTPND_W<'a> {
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
    pub fn usbpwr(&self) -> USBPWR_R {
        USBPWR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ususpend(&self) -> USUSPEND_R {
        USUSPEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbbusy(&self) -> USBBUSY_R {
        USBBUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uslpgrd(&self) -> USLPGRD_R {
        USLPGRD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uactpnd(&self) -> UACTPND_R {
        UACTPND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn usbpwr(&mut self) -> USBPWR_W {
        USBPWR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ususpend(&mut self) -> USUSPEND_W {
        USUSPEND_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbbusy(&mut self) -> USBBUSY_W {
        USBBUSY_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn uslpgrd(&mut self) -> USLPGRD_W {
        USLPGRD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uactpnd(&mut self) -> UACTPND_W {
        UACTPND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1PWRCINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1pwrcinv](index.html) module"]
pub struct U1PWRCINV_SPEC;
impl crate::RegisterSpec for U1PWRCINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1pwrcinv::R](R) reader structure"]
impl crate::Readable for U1PWRCINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1pwrcinv::W](W) writer structure"]
impl crate::Writable for U1PWRCINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1PWRCINV to value 0"]
impl crate::Resettable for U1PWRCINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
