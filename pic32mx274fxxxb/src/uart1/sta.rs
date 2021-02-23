#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STA_SPEC>> for R {
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STA` writer"]
pub struct W(crate::W<STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STA_SPEC>;
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
impl core::convert::From<crate::W<STA_SPEC>> for W {
    fn from(writer: crate::W<STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `URXDA` reader - "]
pub struct URXDA_R(crate::FieldReader<bool, bool>);
impl URXDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        URXDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URXDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URXDA` writer - "]
pub struct URXDA_W<'a> {
    w: &'a mut W,
}
impl<'a> URXDA_W<'a> {
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
#[doc = "Field `OERR` reader - "]
pub struct OERR_R(crate::FieldReader<bool, bool>);
impl OERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OERR` writer - "]
pub struct OERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OERR_W<'a> {
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
#[doc = "Field `FERR` reader - "]
pub struct FERR_R(crate::FieldReader<bool, bool>);
impl FERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERR` writer - "]
pub struct FERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FERR_W<'a> {
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
#[doc = "Field `PERR` reader - "]
pub struct PERR_R(crate::FieldReader<bool, bool>);
impl PERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERR` writer - "]
pub struct PERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERR_W<'a> {
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
#[doc = "Field `RIDLE` reader - "]
pub struct RIDLE_R(crate::FieldReader<bool, bool>);
impl RIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIDLE` writer - "]
pub struct RIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIDLE_W<'a> {
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
#[doc = "Field `ADDEN` reader - "]
pub struct ADDEN_R(crate::FieldReader<bool, bool>);
impl ADDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDEN` writer - "]
pub struct ADDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDEN_W<'a> {
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
#[doc = "Field `URXISEL` reader - "]
pub struct URXISEL_R(crate::FieldReader<u8, u8>);
impl URXISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        URXISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URXISEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URXISEL` writer - "]
pub struct URXISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> URXISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `TRMT` reader - "]
pub struct TRMT_R(crate::FieldReader<bool, bool>);
impl TRMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRMT` writer - "]
pub struct TRMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRMT_W<'a> {
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
#[doc = "Field `UTXBF` reader - "]
pub struct UTXBF_R(crate::FieldReader<bool, bool>);
impl UTXBF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTXBF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTXBF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTXBF` writer - "]
pub struct UTXBF_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXBF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `UTXEN` reader - "]
pub struct UTXEN_R(crate::FieldReader<bool, bool>);
impl UTXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTXEN` writer - "]
pub struct UTXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXEN_W<'a> {
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
#[doc = "Field `UTXBRK` reader - "]
pub struct UTXBRK_R(crate::FieldReader<bool, bool>);
impl UTXBRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTXBRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTXBRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTXBRK` writer - "]
pub struct UTXBRK_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXBRK_W<'a> {
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
#[doc = "Field `URXEN` reader - "]
pub struct URXEN_R(crate::FieldReader<bool, bool>);
impl URXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        URXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `URXEN` writer - "]
pub struct URXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URXEN_W<'a> {
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
#[doc = "Field `UTXINV` reader - "]
pub struct UTXINV_R(crate::FieldReader<bool, bool>);
impl UTXINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTXINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTXINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTXINV` writer - "]
pub struct UTXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXINV_W<'a> {
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
#[doc = "Field `UTXISEL` reader - "]
pub struct UTXISEL_R(crate::FieldReader<u8, u8>);
impl UTXISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UTXISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTXISEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTXISEL` writer - "]
pub struct UTXISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UTXISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `ADDR` reader - "]
pub struct ADDR_R(crate::FieldReader<u8, u8>);
impl ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - "]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `MASK` reader - "]
pub struct MASK_R(crate::FieldReader<u8, u8>);
impl MASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - "]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urxda(&self) -> URXDA_R {
        URXDA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn oerr(&self) -> OERR_R {
        OERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ridle(&self) -> RIDLE_R {
        RIDLE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn urxisel(&self) -> URXISEL_R {
        URXISEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trmt(&self) -> TRMT_R {
        TRMT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn utxbf(&self) -> UTXBF_R {
        UTXBF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn utxen(&self) -> UTXEN_R {
        UTXEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn utxbrk(&self) -> UTXBRK_R {
        UTXBRK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn urxen(&self) -> URXEN_R {
        URXEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn utxinv(&self) -> UTXINV_R {
        UTXINV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn utxisel(&self) -> UTXISEL_R {
        UTXISEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn urxda(&mut self) -> URXDA_W {
        URXDA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn oerr(&mut self) -> OERR_W {
        OERR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FERR_W {
        FERR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ridle(&mut self) -> RIDLE_W {
        RIDLE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adden(&mut self) -> ADDEN_W {
        ADDEN_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn urxisel(&mut self) -> URXISEL_W {
        URXISEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trmt(&mut self) -> TRMT_W {
        TRMT_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn utxbf(&mut self) -> UTXBF_W {
        UTXBF_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn utxen(&mut self) -> UTXEN_W {
        UTXEN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn utxbrk(&mut self) -> UTXBRK_W {
        UTXBRK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn urxen(&mut self) -> URXEN_W {
        URXEN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn utxinv(&mut self) -> UTXINV_W {
        UTXINV_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn utxisel(&mut self) -> UTXISEL_W {
        UTXISEL_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "U1STA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sta::W](W) writer structure"]
impl crate::Writable for STA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STA to value 0x0110"]
impl crate::Resettable for STA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0110
    }
}
