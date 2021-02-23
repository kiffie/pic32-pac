#[doc = "Register `AD1CON3INV` reader"]
pub struct R(crate::R<AD1CON3INV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AD1CON3INV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AD1CON3INV_SPEC>> for R {
    fn from(reader: crate::R<AD1CON3INV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AD1CON3INV` writer"]
pub struct W(crate::W<AD1CON3INV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AD1CON3INV_SPEC>;
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
impl core::convert::From<crate::W<AD1CON3INV_SPEC>> for W {
    fn from(writer: crate::W<AD1CON3INV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCS` reader - "]
pub struct ADCS_R(crate::FieldReader<u8, u8>);
impl ADCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCS` writer - "]
pub struct ADCS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Field `SAMC` reader - "]
pub struct SAMC_R(crate::FieldReader<u8, u8>);
impl SAMC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMC` writer - "]
pub struct SAMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `ADRC` reader - "]
pub struct ADRC_R(crate::FieldReader<bool, bool>);
impl ADRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADRC` writer - "]
pub struct ADRC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn adcs(&self) -> ADCS_R {
        ADCS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn samc(&self) -> SAMC_R {
        SAMC_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adrc(&self) -> ADRC_R {
        ADRC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn adcs(&mut self) -> ADCS_W {
        ADCS_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn samc(&mut self) -> SAMC_W {
        SAMC_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adrc(&mut self) -> ADRC_W {
        ADRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AD1CON3INV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con3inv](index.html) module"]
pub struct AD1CON3INV_SPEC;
impl crate::RegisterSpec for AD1CON3INV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ad1con3inv::R](R) reader structure"]
impl crate::Readable for AD1CON3INV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ad1con3inv::W](W) writer structure"]
impl crate::Writable for AD1CON3INV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AD1CON3INV to value 0"]
impl crate::Resettable for AD1CON3INV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
