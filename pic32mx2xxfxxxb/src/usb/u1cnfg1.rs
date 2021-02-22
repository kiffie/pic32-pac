#[doc = "Register `U1CNFG1` reader"]
pub struct R(crate::R<U1CNFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U1CNFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<U1CNFG1_SPEC>> for R {
    fn from(reader: crate::R<U1CNFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `U1CNFG1` writer"]
pub struct W(crate::W<U1CNFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<U1CNFG1_SPEC>;
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
impl core::convert::From<crate::W<U1CNFG1_SPEC>> for W {
    fn from(writer: crate::W<U1CNFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UASUSPND` reader - "]
pub struct UASUSPND_R(crate::FieldReader<bool, bool>);
impl UASUSPND_R {
    pub(crate) fn new(bits: bool) -> Self {
        UASUSPND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UASUSPND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UASUSPND` writer - "]
pub struct UASUSPND_W<'a> {
    w: &'a mut W,
}
impl<'a> UASUSPND_W<'a> {
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
#[doc = "Field `USBSIDL` reader - "]
pub struct USBSIDL_R(crate::FieldReader<bool, bool>);
impl USBSIDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBSIDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBSIDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSIDL` writer - "]
pub struct USBSIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSIDL_W<'a> {
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
#[doc = "Field `USBFRZ` reader - "]
pub struct USBFRZ_R(crate::FieldReader<bool, bool>);
impl USBFRZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBFRZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBFRZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBFRZ` writer - "]
pub struct USBFRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFRZ_W<'a> {
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
#[doc = "Field `UOEMON` reader - "]
pub struct UOEMON_R(crate::FieldReader<bool, bool>);
impl UOEMON_R {
    pub(crate) fn new(bits: bool) -> Self {
        UOEMON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UOEMON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UOEMON` writer - "]
pub struct UOEMON_W<'a> {
    w: &'a mut W,
}
impl<'a> UOEMON_W<'a> {
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
#[doc = "Field `UTEYE` reader - "]
pub struct UTEYE_R(crate::FieldReader<bool, bool>);
impl UTEYE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTEYE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTEYE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTEYE` writer - "]
pub struct UTEYE_W<'a> {
    w: &'a mut W,
}
impl<'a> UTEYE_W<'a> {
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
    pub fn uasuspnd(&self) -> UASUSPND_R {
        UASUSPND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbsidl(&self) -> USBSIDL_R {
        USBSIDL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn usbfrz(&self) -> USBFRZ_R {
        USBFRZ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uoemon(&self) -> UOEMON_R {
        UOEMON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uteye(&self) -> UTEYE_R {
        UTEYE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uasuspnd(&mut self) -> UASUSPND_W {
        UASUSPND_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbsidl(&mut self) -> USBSIDL_W {
        USBSIDL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn usbfrz(&mut self) -> USBFRZ_W {
        USBFRZ_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn uoemon(&mut self) -> UOEMON_W {
        UOEMON_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn uteye(&mut self) -> UTEYE_W {
        UTEYE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1CNFG1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u1cnfg1](index.html) module"]
pub struct U1CNFG1_SPEC;
impl crate::RegisterSpec for U1CNFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u1cnfg1::R](R) reader structure"]
impl crate::Readable for U1CNFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [u1cnfg1::W](W) writer structure"]
impl crate::Writable for U1CNFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets U1CNFG1 to value 0"]
impl crate::Resettable for U1CNFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
