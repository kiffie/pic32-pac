#[doc = "Register `SPLLCONINV` reader"]
pub struct R(crate::R<SPLLCONINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLLCONINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPLLCONINV_SPEC>> for R {
    fn from(reader: crate::R<SPLLCONINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPLLCONINV` writer"]
pub struct W(crate::W<SPLLCONINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLLCONINV_SPEC>;
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
impl core::convert::From<crate::W<SPLLCONINV_SPEC>> for W {
    fn from(writer: crate::W<SPLLCONINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLICLK` reader - "]
pub struct PLLICLK_R(crate::FieldReader<bool, bool>);
impl PLLICLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLICLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLICLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLICLK` writer - "]
pub struct PLLICLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLICLK_W<'a> {
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
#[doc = "Field `PLLIDIV` reader - "]
pub struct PLLIDIV_R(crate::FieldReader<u8, u8>);
impl PLLIDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLIDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLIDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLIDIV` writer - "]
pub struct PLLIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLIDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `PLLMULT` reader - "]
pub struct PLLMULT_R(crate::FieldReader<u8, u8>);
impl PLLMULT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLMULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLMULT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLMULT` writer - "]
pub struct PLLMULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `PLLODIV` reader - "]
pub struct PLLODIV_R(crate::FieldReader<u8, u8>);
impl PLLODIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLODIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLLODIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLODIV` writer - "]
pub struct PLLODIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLODIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn plliclk(&self) -> PLLICLK_R {
        PLLICLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pllidiv(&self) -> PLLIDIV_R {
        PLLIDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn pllmult(&self) -> PLLMULT_R {
        PLLMULT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pllodiv(&self) -> PLLODIV_R {
        PLLODIV_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn plliclk(&mut self) -> PLLICLK_W {
        PLLICLK_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pllidiv(&mut self) -> PLLIDIV_W {
        PLLIDIV_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn pllmult(&mut self) -> PLLMULT_W {
        PLLMULT_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pllodiv(&mut self) -> PLLODIV_W {
        PLLODIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPLLCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spllconinv](index.html) module"]
pub struct SPLLCONINV_SPEC;
impl crate::RegisterSpec for SPLLCONINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spllconinv::R](R) reader structure"]
impl crate::Readable for SPLLCONINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spllconinv::W](W) writer structure"]
impl crate::Writable for SPLLCONINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPLLCONINV to value 0"]
impl crate::Resettable for SPLLCONINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
