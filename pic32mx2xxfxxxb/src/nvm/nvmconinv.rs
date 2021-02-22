#[doc = "Register `NVMCONINV` reader"]
pub struct R(crate::R<NVMCONINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVMCONINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NVMCONINV_SPEC>> for R {
    fn from(reader: crate::R<NVMCONINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVMCONINV` writer"]
pub struct W(crate::W<NVMCONINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVMCONINV_SPEC>;
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
impl core::convert::From<crate::W<NVMCONINV_SPEC>> for W {
    fn from(writer: crate::W<NVMCONINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NVMOP` reader - "]
pub struct NVMOP_R(crate::FieldReader<u8, u8>);
impl NVMOP_R {
    pub(crate) fn new(bits: u8) -> Self {
        NVMOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVMOP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVMOP` writer - "]
pub struct NVMOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NVMOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Field `LVDSTAT` reader - "]
pub struct LVDSTAT_R(crate::FieldReader<bool, bool>);
impl LVDSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVDSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVDSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDSTAT` writer - "]
pub struct LVDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDSTAT_W<'a> {
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
#[doc = "Field `LVDERR` reader - "]
pub struct LVDERR_R(crate::FieldReader<bool, bool>);
impl LVDERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LVDERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LVDERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVDERR` writer - "]
pub struct LVDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LVDERR_W<'a> {
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
#[doc = "Field `WRERR` reader - "]
pub struct WRERR_R(crate::FieldReader<bool, bool>);
impl WRERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRERR` writer - "]
pub struct WRERR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRERR_W<'a> {
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
#[doc = "Field `WREN` reader - "]
pub struct WREN_R(crate::FieldReader<bool, bool>);
impl WREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WREN` writer - "]
pub struct WREN_W<'a> {
    w: &'a mut W,
}
impl<'a> WREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `WR` reader - "]
pub struct WR_R(crate::FieldReader<bool, bool>);
impl WR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR` writer - "]
pub struct WR_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_W<'a> {
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
    pub fn nvmop(&self) -> NVMOP_R {
        NVMOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lvdstat(&self) -> LVDSTAT_R {
        LVDSTAT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lvderr(&self) -> LVDERR_R {
        LVDERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn nvmop(&mut self) -> NVMOP_W {
        NVMOP_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn lvdstat(&mut self) -> LVDSTAT_W {
        LVDSTAT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lvderr(&mut self) -> LVDERR_W {
        LVDERR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn wrerr(&mut self) -> WRERR_W {
        WRERR_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr(&mut self) -> WR_W {
        WR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NVMCONINV register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvmconinv](index.html) module"]
pub struct NVMCONINV_SPEC;
impl crate::RegisterSpec for NVMCONINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvmconinv::R](R) reader structure"]
impl crate::Readable for NVMCONINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvmconinv::W](W) writer structure"]
impl crate::Writable for NVMCONINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVMCONINV to value 0"]
impl crate::Resettable for NVMCONINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
