#[doc = "Register `CON1CLR` reader"]
pub struct R(crate::R<CON1CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CON1CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CON1CLR_SPEC>> for R {
    fn from(reader: crate::R<CON1CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CON1CLR` writer"]
pub struct W(crate::W<CON1CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CON1CLR_SPEC>;
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
impl core::convert::From<crate::W<CON1CLR_SPEC>> for W {
    fn from(writer: crate::W<CON1CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRXISEL` reader - "]
pub struct SRXISEL_R(crate::FieldReader<u8, u8>);
impl SRXISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRXISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRXISEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRXISEL` writer - "]
pub struct SRXISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRXISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Field `STXISEL` reader - "]
pub struct STXISEL_R(crate::FieldReader<u8, u8>);
impl STXISEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        STXISEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STXISEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STXISEL` writer - "]
pub struct STXISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STXISEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `DISSDI` reader - "]
pub struct DISSDI_R(crate::FieldReader<bool, bool>);
impl DISSDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISSDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISSDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISSDI` writer - "]
pub struct DISSDI_W<'a> {
    w: &'a mut W,
}
impl<'a> DISSDI_W<'a> {
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
#[doc = "Field `MSTEN` reader - "]
pub struct MSTEN_R(crate::FieldReader<bool, bool>);
impl MSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTEN` writer - "]
pub struct MSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTEN_W<'a> {
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
#[doc = "Field `CKP` reader - "]
pub struct CKP_R(crate::FieldReader<bool, bool>);
impl CKP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKP` writer - "]
pub struct CKP_W<'a> {
    w: &'a mut W,
}
impl<'a> CKP_W<'a> {
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
#[doc = "Field `SSEN` reader - "]
pub struct SSEN_R(crate::FieldReader<bool, bool>);
impl SSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSEN` writer - "]
pub struct SSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSEN_W<'a> {
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
#[doc = "Field `CKE` reader - "]
pub struct CKE_R(crate::FieldReader<bool, bool>);
impl CKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKE` writer - "]
pub struct CKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKE_W<'a> {
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
#[doc = "Field `SMP` reader - "]
pub struct SMP_R(crate::FieldReader<bool, bool>);
impl SMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP` writer - "]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
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
#[doc = "Field `MODE16` reader - "]
pub struct MODE16_R(crate::FieldReader<bool, bool>);
impl MODE16_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE16` writer - "]
pub struct MODE16_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE16_W<'a> {
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
#[doc = "Field `MODE32` reader - "]
pub struct MODE32_R(crate::FieldReader<bool, bool>);
impl MODE32_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE32` writer - "]
pub struct MODE32_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE32_W<'a> {
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
#[doc = "Field `DISSDO` reader - "]
pub struct DISSDO_R(crate::FieldReader<bool, bool>);
impl DISSDO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DISSDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISSDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DISSDO` writer - "]
pub struct DISSDO_W<'a> {
    w: &'a mut W,
}
impl<'a> DISSDO_W<'a> {
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
#[doc = "Field `FRZ` reader - "]
pub struct FRZ_R(crate::FieldReader<bool, bool>);
impl FRZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZ` writer - "]
pub struct FRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZ_W<'a> {
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
#[doc = "Field `ENHBUF` reader - "]
pub struct ENHBUF_R(crate::FieldReader<bool, bool>);
impl ENHBUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENHBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENHBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENHBUF` writer - "]
pub struct ENHBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> ENHBUF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `SPIFE` reader - "]
pub struct SPIFE_R(crate::FieldReader<bool, bool>);
impl SPIFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPIFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPIFE` writer - "]
pub struct SPIFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIFE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `MCLKSEL` reader - "]
pub struct MCLKSEL_R(crate::FieldReader<bool, bool>);
impl MCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLKSEL` writer - "]
pub struct MCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `FRMCNT` reader - "]
pub struct FRMCNT_R(crate::FieldReader<u8, u8>);
impl FRMCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRMCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMCNT` writer - "]
pub struct FRMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `FRMSYPW` reader - "]
pub struct FRMSYPW_R(crate::FieldReader<bool, bool>);
impl FRMSYPW_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMSYPW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMSYPW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMSYPW` writer - "]
pub struct FRMSYPW_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMSYPW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `MSSEN` reader - "]
pub struct MSSEN_R(crate::FieldReader<bool, bool>);
impl MSSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSSEN` writer - "]
pub struct MSSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `FRMPOL` reader - "]
pub struct FRMPOL_R(crate::FieldReader<bool, bool>);
impl FRMPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMPOL` writer - "]
pub struct FRMPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMPOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `FRMSYNC` reader - "]
pub struct FRMSYNC_R(crate::FieldReader<bool, bool>);
impl FRMSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMSYNC` writer - "]
pub struct FRMSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMSYNC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `FRMEN` reader - "]
pub struct FRMEN_R(crate::FieldReader<bool, bool>);
impl FRMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRMEN` writer - "]
pub struct FRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn srxisel(&self) -> SRXISEL_R {
        SRXISEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn stxisel(&self) -> STXISEL_R {
        STXISEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dissdi(&self) -> DISSDI_R {
        DISSDI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn msten(&self) -> MSTEN_R {
        MSTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ckp(&self) -> CKP_R {
        CKP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ssen(&self) -> SSEN_R {
        SSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mode16(&self) -> MODE16_R {
        MODE16_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mode32(&self) -> MODE32_R {
        MODE32_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dissdo(&self) -> DISSDO_R {
        DISSDO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&self) -> SIDL_R {
        SIDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn enhbuf(&self) -> ENHBUF_R {
        ENHBUF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spife(&self) -> SPIFE_R {
        SPIFE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mclksel(&self) -> MCLKSEL_R {
        MCLKSEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frmcnt(&self) -> FRMCNT_R {
        FRMCNT_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn frmsypw(&self) -> FRMSYPW_R {
        FRMSYPW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mssen(&self) -> MSSEN_R {
        MSSEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn frmpol(&self) -> FRMPOL_R {
        FRMPOL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn frmsync(&self) -> FRMSYNC_R {
        FRMSYNC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn frmen(&self) -> FRMEN_R {
        FRMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn srxisel(&mut self) -> SRXISEL_W {
        SRXISEL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn stxisel(&mut self) -> STXISEL_W {
        STXISEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dissdi(&mut self) -> DISSDI_W {
        DISSDI_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn msten(&mut self) -> MSTEN_W {
        MSTEN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ckp(&mut self) -> CKP_W {
        CKP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ssen(&mut self) -> SSEN_W {
        SSEN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W {
        CKE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mode16(&mut self) -> MODE16_W {
        MODE16_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mode32(&mut self) -> MODE32_W {
        MODE32_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dissdo(&mut self) -> DISSDO_W {
        DISSDO_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn sidl(&mut self) -> SIDL_W {
        SIDL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn frz(&mut self) -> FRZ_W {
        FRZ_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn enhbuf(&mut self) -> ENHBUF_W {
        ENHBUF_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn spife(&mut self) -> SPIFE_W {
        SPIFE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mclksel(&mut self) -> MCLKSEL_W {
        MCLKSEL_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn frmcnt(&mut self) -> FRMCNT_W {
        FRMCNT_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn frmsypw(&mut self) -> FRMSYPW_W {
        FRMSYPW_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn mssen(&mut self) -> MSSEN_W {
        MSSEN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn frmpol(&mut self) -> FRMPOL_W {
        FRMPOL_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn frmsync(&mut self) -> FRMSYNC_W {
        FRMSYNC_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn frmen(&mut self) -> FRMEN_W {
        FRMEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1CONCLR register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [con1clr](index.html) module"]
pub struct CON1CLR_SPEC;
impl crate::RegisterSpec for CON1CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [con1clr::R](R) reader structure"]
impl crate::Readable for CON1CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [con1clr::W](W) writer structure"]
impl crate::Writable for CON1CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CON1CLR to value 0"]
impl crate::Resettable for CON1CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
