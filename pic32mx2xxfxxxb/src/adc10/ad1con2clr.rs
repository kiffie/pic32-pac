#[doc = "Register `AD1CON2CLR` reader"]
pub struct R(crate::R<AD1CON2CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AD1CON2CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AD1CON2CLR_SPEC>> for R {
    fn from(reader: crate::R<AD1CON2CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AD1CON2CLR` writer"]
pub struct W(crate::W<AD1CON2CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AD1CON2CLR_SPEC>;
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
impl core::convert::From<crate::W<AD1CON2CLR_SPEC>> for W {
    fn from(writer: crate::W<AD1CON2CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALTS` reader - "]
pub struct ALTS_R(crate::FieldReader<bool, bool>);
impl ALTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALTS` writer - "]
pub struct ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTS_W<'a> {
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
#[doc = "Field `BUFM` reader - "]
pub struct BUFM_R(crate::FieldReader<bool, bool>);
impl BUFM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFM` writer - "]
pub struct BUFM_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFM_W<'a> {
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
#[doc = "Field `SMPI` reader - "]
pub struct SMPI_R(crate::FieldReader<u8, u8>);
impl SMPI_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPI` writer - "]
pub struct SMPI_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Field `BUFS` reader - "]
pub struct BUFS_R(crate::FieldReader<bool, bool>);
impl BUFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFS` writer - "]
pub struct BUFS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFS_W<'a> {
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
#[doc = "Field `CSCNA` reader - "]
pub struct CSCNA_R(crate::FieldReader<bool, bool>);
impl CSCNA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSCNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSCNA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSCNA` writer - "]
pub struct CSCNA_W<'a> {
    w: &'a mut W,
}
impl<'a> CSCNA_W<'a> {
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
#[doc = "Field `OFFCAL` reader - "]
pub struct OFFCAL_R(crate::FieldReader<bool, bool>);
impl OFFCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OFFCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFCAL` writer - "]
pub struct OFFCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFCAL_W<'a> {
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
#[doc = "Field `VCFG` reader - "]
pub struct VCFG_R(crate::FieldReader<u8, u8>);
impl VCFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        VCFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCFG` writer - "]
pub struct VCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> VCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alts(&self) -> ALTS_R {
        ALTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bufm(&self) -> BUFM_R {
        BUFM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn smpi(&self) -> SMPI_R {
        SMPI_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bufs(&self) -> BUFS_R {
        BUFS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cscna(&self) -> CSCNA_R {
        CSCNA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn offcal(&self) -> OFFCAL_R {
        OFFCAL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn vcfg(&self) -> VCFG_R {
        VCFG_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn alts(&mut self) -> ALTS_W {
        ALTS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bufm(&mut self) -> BUFM_W {
        BUFM_W { w: self }
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn smpi(&mut self) -> SMPI_W {
        SMPI_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bufs(&mut self) -> BUFS_W {
        BUFS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cscna(&mut self) -> CSCNA_W {
        CSCNA_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn offcal(&mut self) -> OFFCAL_W {
        OFFCAL_W { w: self }
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn vcfg(&mut self) -> VCFG_W {
        VCFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AD1CON2CLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ad1con2clr](index.html) module"]
pub struct AD1CON2CLR_SPEC;
impl crate::RegisterSpec for AD1CON2CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ad1con2clr::R](R) reader structure"]
impl crate::Readable for AD1CON2CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ad1con2clr::W](W) writer structure"]
impl crate::Writable for AD1CON2CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AD1CON2CLR to value 0"]
impl crate::Resettable for AD1CON2CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
