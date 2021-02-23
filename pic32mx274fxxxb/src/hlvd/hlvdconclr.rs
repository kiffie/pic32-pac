#[doc = "Register `HLVDCONCLR` reader"]
pub struct R(crate::R<HLVDCONCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HLVDCONCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HLVDCONCLR_SPEC>> for R {
    fn from(reader: crate::R<HLVDCONCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HLVDCONCLR` writer"]
pub struct W(crate::W<HLVDCONCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HLVDCONCLR_SPEC>;
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
impl core::convert::From<crate::W<HLVDCONCLR_SPEC>> for W {
    fn from(writer: crate::W<HLVDCONCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HLVDL` reader - "]
pub struct HLVDL_R(crate::FieldReader<u8, u8>);
impl HLVDL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HLVDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLVDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HLVDL` writer - "]
pub struct HLVDL_W<'a> {
    w: &'a mut W,
}
impl<'a> HLVDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Field `HLEVT` reader - "]
pub struct HLEVT_R(crate::FieldReader<bool, bool>);
impl HLEVT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HLEVT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HLEVT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HLEVT` writer - "]
pub struct HLEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> HLEVT_W<'a> {
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
#[doc = "Field `BGVST` reader - "]
pub struct BGVST_R(crate::FieldReader<bool, bool>);
impl BGVST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BGVST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BGVST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BGVST` writer - "]
pub struct BGVST_W<'a> {
    w: &'a mut W,
}
impl<'a> BGVST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `VDIR` reader - "]
pub struct VDIR_R(crate::FieldReader<bool, bool>);
impl VDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDIR` writer - "]
pub struct VDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> VDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SIDL` reader - "]
pub struct SIDL_R(crate::FieldReader<bool, bool>);
impl SIDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIDL` writer - "]
pub struct SIDL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIDL_W<'a> {
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
#[doc = "Field `ON` reader - "]
pub struct ON_R(crate::FieldReader<bool, bool>);
impl ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ON` writer - "]
pub struct ON_W<'a> {
    w: &'a mut W,
}
impl<'a> ON_W<'a> {
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
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hlvdl(&self) -> HLVDL_R {
        HLVDL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hlevt(&self) -> HLEVT_R {
        HLEVT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bgvst(&self) -> BGVST_R {
        BGVST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn vdir(&self) -> VDIR_R {
        VDIR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hlvdl(&mut self) -> HLVDL_W {
        HLVDL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hlevt(&mut self) -> HLEVT_W {
        HLEVT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bgvst(&mut self) -> BGVST_W {
        BGVST_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn vdir(&mut self) -> VDIR_W {
        VDIR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HLVDCONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hlvdconclr](index.html) module"]
pub struct HLVDCONCLR_SPEC;
impl crate::RegisterSpec for HLVDCONCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hlvdconclr::R](R) reader structure"]
impl crate::Readable for HLVDCONCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hlvdconclr::W](W) writer structure"]
impl crate::Writable for HLVDCONCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HLVDCONCLR to value 0"]
impl crate::Resettable for HLVDCONCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
