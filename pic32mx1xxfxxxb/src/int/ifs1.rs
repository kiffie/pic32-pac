#[doc = "Register `IFS1` reader"]
pub struct R(crate::R<IFS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IFS1_SPEC>> for R {
    fn from(reader: crate::R<IFS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFS1` writer"]
pub struct W(crate::W<IFS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS1_SPEC>;
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
impl core::convert::From<crate::W<IFS1_SPEC>> for W {
    fn from(writer: crate::W<IFS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1IF` reader - "]
pub struct CMP1IF_R(crate::FieldReader<bool, bool>);
impl CMP1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1IF` writer - "]
pub struct CMP1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IF_W<'a> {
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
#[doc = "Field `CMP2IF` reader - "]
pub struct CMP2IF_R(crate::FieldReader<bool, bool>);
impl CMP2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2IF` writer - "]
pub struct CMP2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IF_W<'a> {
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
#[doc = "Field `CMP3IF` reader - "]
pub struct CMP3IF_R(crate::FieldReader<bool, bool>);
impl CMP3IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3IF` writer - "]
pub struct CMP3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IF_W<'a> {
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
#[doc = "Field `SPI1EIF` reader - "]
pub struct SPI1EIF_R(crate::FieldReader<bool, bool>);
impl SPI1EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1EIF` writer - "]
pub struct SPI1EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EIF_W<'a> {
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
#[doc = "Field `SPI1RXIF` reader - "]
pub struct SPI1RXIF_R(crate::FieldReader<bool, bool>);
impl SPI1RXIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1RXIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1RXIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1RXIF` writer - "]
pub struct SPI1RXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RXIF_W<'a> {
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
#[doc = "Field `SPI1TXIF` reader - "]
pub struct SPI1TXIF_R(crate::FieldReader<bool, bool>);
impl SPI1TXIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1TXIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1TXIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1TXIF` writer - "]
pub struct SPI1TXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1TXIF_W<'a> {
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
#[doc = "Field `U1EIF` reader - "]
pub struct U1EIF_R(crate::FieldReader<bool, bool>);
impl U1EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        U1EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1EIF` writer - "]
pub struct U1EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> U1EIF_W<'a> {
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
#[doc = "Field `U1RXIF` reader - "]
pub struct U1RXIF_R(crate::FieldReader<bool, bool>);
impl U1RXIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        U1RXIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1RXIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1RXIF` writer - "]
pub struct U1RXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> U1RXIF_W<'a> {
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
#[doc = "Field `U1TXIF` reader - "]
pub struct U1TXIF_R(crate::FieldReader<bool, bool>);
impl U1TXIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        U1TXIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1TXIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1TXIF` writer - "]
pub struct U1TXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> U1TXIF_W<'a> {
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
#[doc = "Field `I2C1BIF` reader - "]
pub struct I2C1BIF_R(crate::FieldReader<bool, bool>);
impl I2C1BIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1BIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1BIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1BIF` writer - "]
pub struct I2C1BIF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1BIF_W<'a> {
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
#[doc = "Field `I2C1SIF` reader - "]
pub struct I2C1SIF_R(crate::FieldReader<bool, bool>);
impl I2C1SIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1SIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SIF` writer - "]
pub struct I2C1SIF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SIF_W<'a> {
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
#[doc = "Field `I2C1MIF` reader - "]
pub struct I2C1MIF_R(crate::FieldReader<bool, bool>);
impl I2C1MIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1MIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1MIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1MIF` writer - "]
pub struct I2C1MIF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1MIF_W<'a> {
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
#[doc = "Field `CNAIF` reader - "]
pub struct CNAIF_R(crate::FieldReader<bool, bool>);
impl CNAIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNAIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNAIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNAIF` writer - "]
pub struct CNAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CNAIF_W<'a> {
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
#[doc = "Field `CNBIF` reader - "]
pub struct CNBIF_R(crate::FieldReader<bool, bool>);
impl CNBIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNBIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNBIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNBIF` writer - "]
pub struct CNBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CNBIF_W<'a> {
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
#[doc = "Field `CNCIF` reader - "]
pub struct CNCIF_R(crate::FieldReader<bool, bool>);
impl CNCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNCIF` writer - "]
pub struct CNCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CNCIF_W<'a> {
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
#[doc = "Field `PMPIF` reader - "]
pub struct PMPIF_R(crate::FieldReader<bool, bool>);
impl PMPIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMPIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMPIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMPIF` writer - "]
pub struct PMPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPIF_W<'a> {
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
#[doc = "Field `PMPEIF` reader - "]
pub struct PMPEIF_R(crate::FieldReader<bool, bool>);
impl PMPEIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMPEIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMPEIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMPEIF` writer - "]
pub struct PMPEIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPEIF_W<'a> {
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
#[doc = "Field `SPI2EIF` reader - "]
pub struct SPI2EIF_R(crate::FieldReader<bool, bool>);
impl SPI2EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2EIF` writer - "]
pub struct SPI2EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2EIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `SPI2RXIF` reader - "]
pub struct SPI2RXIF_R(crate::FieldReader<bool, bool>);
impl SPI2RXIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2RXIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2RXIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2RXIF` writer - "]
pub struct SPI2RXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2RXIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SPI2TXIF` reader - "]
pub struct SPI2TXIF_R(crate::FieldReader<bool, bool>);
impl SPI2TXIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2TXIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2TXIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2TXIF` writer - "]
pub struct SPI2TXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2TXIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `U2EIF` reader - "]
pub struct U2EIF_R(crate::FieldReader<bool, bool>);
impl U2EIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        U2EIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2EIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2EIF` writer - "]
pub struct U2EIF_W<'a> {
    w: &'a mut W,
}
impl<'a> U2EIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `U2RXIF` reader - "]
pub struct U2RXIF_R(crate::FieldReader<bool, bool>);
impl U2RXIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        U2RXIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2RXIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2RXIF` writer - "]
pub struct U2RXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> U2RXIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `U2TXIF` reader - "]
pub struct U2TXIF_R(crate::FieldReader<bool, bool>);
impl U2TXIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        U2TXIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2TXIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2TXIF` writer - "]
pub struct U2TXIF_W<'a> {
    w: &'a mut W,
}
impl<'a> U2TXIF_W<'a> {
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
#[doc = "Field `I2C2BIF` reader - "]
pub struct I2C2BIF_R(crate::FieldReader<bool, bool>);
impl I2C2BIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2BIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2BIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2BIF` writer - "]
pub struct I2C2BIF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2BIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `I2C2SIF` reader - "]
pub struct I2C2SIF_R(crate::FieldReader<bool, bool>);
impl I2C2SIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2SIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2SIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2SIF` writer - "]
pub struct I2C2SIF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `I2C2MIF` reader - "]
pub struct I2C2MIF_R(crate::FieldReader<bool, bool>);
impl I2C2MIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2MIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2MIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2MIF` writer - "]
pub struct I2C2MIF_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2MIF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CTMUIF` reader - "]
pub struct CTMUIF_R(crate::FieldReader<bool, bool>);
impl CTMUIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTMUIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTMUIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTMUIF` writer - "]
pub struct CTMUIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUIF_W<'a> {
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
#[doc = "Field `DMA0IF` reader - "]
pub struct DMA0IF_R(crate::FieldReader<bool, bool>);
impl DMA0IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA0IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA0IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA0IF` writer - "]
pub struct DMA0IF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0IF_W<'a> {
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
#[doc = "Field `DMA1IF` reader - "]
pub struct DMA1IF_R(crate::FieldReader<bool, bool>);
impl DMA1IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1IF` writer - "]
pub struct DMA1IF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IF_W<'a> {
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
#[doc = "Field `DMA2IF` reader - "]
pub struct DMA2IF_R(crate::FieldReader<bool, bool>);
impl DMA2IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2IF` writer - "]
pub struct DMA2IF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IF_W<'a> {
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
#[doc = "Field `DMA3IF` reader - "]
pub struct DMA3IF_R(crate::FieldReader<bool, bool>);
impl DMA3IF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA3IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA3IF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA3IF` writer - "]
pub struct DMA3IF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA3IF_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmp1if(&self) -> CMP1IF_R {
        CMP1IF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmp2if(&self) -> CMP2IF_R {
        CMP2IF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmp3if(&self) -> CMP3IF_R {
        CMP3IF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi1eif(&self) -> SPI1EIF_R {
        SPI1EIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi1rxif(&self) -> SPI1RXIF_R {
        SPI1RXIF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi1txif(&self) -> SPI1TXIF_R {
        SPI1TXIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn u1eif(&self) -> U1EIF_R {
        U1EIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u1rxif(&self) -> U1RXIF_R {
        U1RXIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u1txif(&self) -> U1TXIF_R {
        U1TXIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c1bif(&self) -> I2C1BIF_R {
        I2C1BIF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c1sif(&self) -> I2C1SIF_R {
        I2C1SIF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c1mif(&self) -> I2C1MIF_R {
        I2C1MIF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnaif(&self) -> CNAIF_R {
        CNAIF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnbif(&self) -> CNBIF_R {
        CNBIF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cncif(&self) -> CNCIF_R {
        CNCIF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pmpif(&self) -> PMPIF_R {
        PMPIF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pmpeif(&self) -> PMPEIF_R {
        PMPEIF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi2eif(&self) -> SPI2EIF_R {
        SPI2EIF_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi2rxif(&self) -> SPI2RXIF_R {
        SPI2RXIF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi2txif(&self) -> SPI2TXIF_R {
        SPI2TXIF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn u2eif(&self) -> U2EIF_R {
        U2EIF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn u2rxif(&self) -> U2RXIF_R {
        U2RXIF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn u2txif(&self) -> U2TXIF_R {
        U2TXIF_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c2bif(&self) -> I2C2BIF_R {
        I2C2BIF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2c2sif(&self) -> I2C2SIF_R {
        I2C2SIF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn i2c2mif(&self) -> I2C2MIF_R {
        I2C2MIF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ctmuif(&self) -> CTMUIF_R {
        CTMUIF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dma0if(&self) -> DMA0IF_R {
        DMA0IF_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dma1if(&self) -> DMA1IF_R {
        DMA1IF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dma2if(&self) -> DMA2IF_R {
        DMA2IF_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dma3if(&self) -> DMA3IF_R {
        DMA3IF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmp1if(&mut self) -> CMP1IF_W {
        CMP1IF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmp2if(&mut self) -> CMP2IF_W {
        CMP2IF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmp3if(&mut self) -> CMP3IF_W {
        CMP3IF_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi1eif(&mut self) -> SPI1EIF_W {
        SPI1EIF_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi1rxif(&mut self) -> SPI1RXIF_W {
        SPI1RXIF_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi1txif(&mut self) -> SPI1TXIF_W {
        SPI1TXIF_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn u1eif(&mut self) -> U1EIF_W {
        U1EIF_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u1rxif(&mut self) -> U1RXIF_W {
        U1RXIF_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u1txif(&mut self) -> U1TXIF_W {
        U1TXIF_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c1bif(&mut self) -> I2C1BIF_W {
        I2C1BIF_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c1sif(&mut self) -> I2C1SIF_W {
        I2C1SIF_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c1mif(&mut self) -> I2C1MIF_W {
        I2C1MIF_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnaif(&mut self) -> CNAIF_W {
        CNAIF_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnbif(&mut self) -> CNBIF_W {
        CNBIF_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cncif(&mut self) -> CNCIF_W {
        CNCIF_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pmpif(&mut self) -> PMPIF_W {
        PMPIF_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pmpeif(&mut self) -> PMPEIF_W {
        PMPEIF_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi2eif(&mut self) -> SPI2EIF_W {
        SPI2EIF_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi2rxif(&mut self) -> SPI2RXIF_W {
        SPI2RXIF_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi2txif(&mut self) -> SPI2TXIF_W {
        SPI2TXIF_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn u2eif(&mut self) -> U2EIF_W {
        U2EIF_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn u2rxif(&mut self) -> U2RXIF_W {
        U2RXIF_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn u2txif(&mut self) -> U2TXIF_W {
        U2TXIF_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c2bif(&mut self) -> I2C2BIF_W {
        I2C2BIF_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2c2sif(&mut self) -> I2C2SIF_W {
        I2C2SIF_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn i2c2mif(&mut self) -> I2C2MIF_W {
        I2C2MIF_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ctmuif(&mut self) -> CTMUIF_W {
        CTMUIF_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dma0if(&mut self) -> DMA0IF_W {
        DMA0IF_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dma1if(&mut self) -> DMA1IF_W {
        DMA1IF_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dma2if(&mut self) -> DMA2IF_W {
        DMA2IF_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dma3if(&mut self) -> DMA3IF_W {
        DMA3IF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IFS1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs1](index.html) module"]
pub struct IFS1_SPEC;
impl crate::RegisterSpec for IFS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifs1::R](R) reader structure"]
impl crate::Readable for IFS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifs1::W](W) writer structure"]
impl crate::Writable for IFS1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS1 to value 0"]
impl crate::Resettable for IFS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
