#[doc = "Register `IEC1SET` reader"]
pub struct R(crate::R<IEC1SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEC1SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IEC1SET_SPEC>> for R {
    fn from(reader: crate::R<IEC1SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEC1SET` writer"]
pub struct W(crate::W<IEC1SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEC1SET_SPEC>;
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
impl core::convert::From<crate::W<IEC1SET_SPEC>> for W {
    fn from(writer: crate::W<IEC1SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP1IE` reader - "]
pub struct CMP1IE_R(crate::FieldReader<bool, bool>);
impl CMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1IE` writer - "]
pub struct CMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP1IE_W<'a> {
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
#[doc = "Field `CMP2IE` reader - "]
pub struct CMP2IE_R(crate::FieldReader<bool, bool>);
impl CMP2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2IE` writer - "]
pub struct CMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IE_W<'a> {
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
#[doc = "Field `CMP3IE` reader - "]
pub struct CMP3IE_R(crate::FieldReader<bool, bool>);
impl CMP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3IE` writer - "]
pub struct CMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IE_W<'a> {
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
#[doc = "Field `SPI1EIE` reader - "]
pub struct SPI1EIE_R(crate::FieldReader<bool, bool>);
impl SPI1EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1EIE` writer - "]
pub struct SPI1EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1EIE_W<'a> {
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
#[doc = "Field `SPI1RXIE` reader - "]
pub struct SPI1RXIE_R(crate::FieldReader<bool, bool>);
impl SPI1RXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1RXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1RXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1RXIE` writer - "]
pub struct SPI1RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1RXIE_W<'a> {
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
#[doc = "Field `SPI1TXIE` reader - "]
pub struct SPI1TXIE_R(crate::FieldReader<bool, bool>);
impl SPI1TXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1TXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI1TXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1TXIE` writer - "]
pub struct SPI1TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1TXIE_W<'a> {
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
#[doc = "Field `U1EIE` reader - "]
pub struct U1EIE_R(crate::FieldReader<bool, bool>);
impl U1EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        U1EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1EIE` writer - "]
pub struct U1EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> U1EIE_W<'a> {
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
#[doc = "Field `U1RXIE` reader - "]
pub struct U1RXIE_R(crate::FieldReader<bool, bool>);
impl U1RXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        U1RXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1RXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1RXIE` writer - "]
pub struct U1RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> U1RXIE_W<'a> {
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
#[doc = "Field `U1TXIE` reader - "]
pub struct U1TXIE_R(crate::FieldReader<bool, bool>);
impl U1TXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        U1TXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U1TXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U1TXIE` writer - "]
pub struct U1TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> U1TXIE_W<'a> {
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
#[doc = "Field `I2C1BIE` reader - "]
pub struct I2C1BIE_R(crate::FieldReader<bool, bool>);
impl I2C1BIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1BIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1BIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1BIE` writer - "]
pub struct I2C1BIE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1BIE_W<'a> {
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
#[doc = "Field `I2C1SIE` reader - "]
pub struct I2C1SIE_R(crate::FieldReader<bool, bool>);
impl I2C1SIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1SIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1SIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SIE` writer - "]
pub struct I2C1SIE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SIE_W<'a> {
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
#[doc = "Field `I2C1MIE` reader - "]
pub struct I2C1MIE_R(crate::FieldReader<bool, bool>);
impl I2C1MIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1MIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C1MIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1MIE` writer - "]
pub struct I2C1MIE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1MIE_W<'a> {
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
#[doc = "Field `CNAIE` reader - "]
pub struct CNAIE_R(crate::FieldReader<bool, bool>);
impl CNAIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNAIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNAIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNAIE` writer - "]
pub struct CNAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNAIE_W<'a> {
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
#[doc = "Field `CNBIE` reader - "]
pub struct CNBIE_R(crate::FieldReader<bool, bool>);
impl CNBIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNBIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNBIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNBIE` writer - "]
pub struct CNBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNBIE_W<'a> {
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
#[doc = "Field `CNCIE` reader - "]
pub struct CNCIE_R(crate::FieldReader<bool, bool>);
impl CNCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNCIE` writer - "]
pub struct CNCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNCIE_W<'a> {
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
#[doc = "Field `PMPIE` reader - "]
pub struct PMPIE_R(crate::FieldReader<bool, bool>);
impl PMPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMPIE` writer - "]
pub struct PMPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPIE_W<'a> {
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
#[doc = "Field `PMPEIE` reader - "]
pub struct PMPEIE_R(crate::FieldReader<bool, bool>);
impl PMPEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMPEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMPEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMPEIE` writer - "]
pub struct PMPEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMPEIE_W<'a> {
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
#[doc = "Field `SPI2EIE` reader - "]
pub struct SPI2EIE_R(crate::FieldReader<bool, bool>);
impl SPI2EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2EIE` writer - "]
pub struct SPI2EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2EIE_W<'a> {
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
#[doc = "Field `SPI2RXIE` reader - "]
pub struct SPI2RXIE_R(crate::FieldReader<bool, bool>);
impl SPI2RXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2RXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2RXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2RXIE` writer - "]
pub struct SPI2RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2RXIE_W<'a> {
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
#[doc = "Field `SPI2TXIE` reader - "]
pub struct SPI2TXIE_R(crate::FieldReader<bool, bool>);
impl SPI2TXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI2TXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI2TXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI2TXIE` writer - "]
pub struct SPI2TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2TXIE_W<'a> {
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
#[doc = "Field `U2EIE` reader - "]
pub struct U2EIE_R(crate::FieldReader<bool, bool>);
impl U2EIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        U2EIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2EIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2EIE` writer - "]
pub struct U2EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> U2EIE_W<'a> {
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
#[doc = "Field `U2RXIE` reader - "]
pub struct U2RXIE_R(crate::FieldReader<bool, bool>);
impl U2RXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        U2RXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2RXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2RXIE` writer - "]
pub struct U2RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> U2RXIE_W<'a> {
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
#[doc = "Field `U2TXIE` reader - "]
pub struct U2TXIE_R(crate::FieldReader<bool, bool>);
impl U2TXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        U2TXIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for U2TXIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `U2TXIE` writer - "]
pub struct U2TXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> U2TXIE_W<'a> {
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
#[doc = "Field `I2C2BIE` reader - "]
pub struct I2C2BIE_R(crate::FieldReader<bool, bool>);
impl I2C2BIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2BIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2BIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2BIE` writer - "]
pub struct I2C2BIE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2BIE_W<'a> {
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
#[doc = "Field `I2C2SIE` reader - "]
pub struct I2C2SIE_R(crate::FieldReader<bool, bool>);
impl I2C2SIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2SIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2SIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2SIE` writer - "]
pub struct I2C2SIE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SIE_W<'a> {
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
#[doc = "Field `I2C2MIE` reader - "]
pub struct I2C2MIE_R(crate::FieldReader<bool, bool>);
impl I2C2MIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2MIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C2MIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2MIE` writer - "]
pub struct I2C2MIE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2MIE_W<'a> {
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
#[doc = "Field `CTMUIE` reader - "]
pub struct CTMUIE_R(crate::FieldReader<bool, bool>);
impl CTMUIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTMUIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTMUIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTMUIE` writer - "]
pub struct CTMUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUIE_W<'a> {
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
#[doc = "Field `DMA0IE` reader - "]
pub struct DMA0IE_R(crate::FieldReader<bool, bool>);
impl DMA0IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA0IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA0IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA0IE` writer - "]
pub struct DMA0IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0IE_W<'a> {
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
#[doc = "Field `DMA1IE` reader - "]
pub struct DMA1IE_R(crate::FieldReader<bool, bool>);
impl DMA1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA1IE` writer - "]
pub struct DMA1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IE_W<'a> {
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
#[doc = "Field `DMA2IE` reader - "]
pub struct DMA2IE_R(crate::FieldReader<bool, bool>);
impl DMA2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA2IE` writer - "]
pub struct DMA2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IE_W<'a> {
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
#[doc = "Field `DMA3IE` reader - "]
pub struct DMA3IE_R(crate::FieldReader<bool, bool>);
impl DMA3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA3IE` writer - "]
pub struct DMA3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA3IE_W<'a> {
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
    pub fn cmp1ie(&self) -> CMP1IE_R {
        CMP1IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMP2IE_R {
        CMP2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMP3IE_R {
        CMP3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi1eie(&self) -> SPI1EIE_R {
        SPI1EIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi1rxie(&self) -> SPI1RXIE_R {
        SPI1RXIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi1txie(&self) -> SPI1TXIE_R {
        SPI1TXIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn u1eie(&self) -> U1EIE_R {
        U1EIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u1rxie(&self) -> U1RXIE_R {
        U1RXIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u1txie(&self) -> U1TXIE_R {
        U1TXIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c1bie(&self) -> I2C1BIE_R {
        I2C1BIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c1sie(&self) -> I2C1SIE_R {
        I2C1SIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c1mie(&self) -> I2C1MIE_R {
        I2C1MIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnaie(&self) -> CNAIE_R {
        CNAIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnbie(&self) -> CNBIE_R {
        CNBIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cncie(&self) -> CNCIE_R {
        CNCIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pmpie(&self) -> PMPIE_R {
        PMPIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pmpeie(&self) -> PMPEIE_R {
        PMPEIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi2eie(&self) -> SPI2EIE_R {
        SPI2EIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi2rxie(&self) -> SPI2RXIE_R {
        SPI2RXIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi2txie(&self) -> SPI2TXIE_R {
        SPI2TXIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn u2eie(&self) -> U2EIE_R {
        U2EIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn u2rxie(&self) -> U2RXIE_R {
        U2RXIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn u2txie(&self) -> U2TXIE_R {
        U2TXIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c2bie(&self) -> I2C2BIE_R {
        I2C2BIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2c2sie(&self) -> I2C2SIE_R {
        I2C2SIE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn i2c2mie(&self) -> I2C2MIE_R {
        I2C2MIE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ctmuie(&self) -> CTMUIE_R {
        CTMUIE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dma0ie(&self) -> DMA0IE_R {
        DMA0IE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dma3ie(&self) -> DMA3IE_R {
        DMA3IE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cmp1ie(&mut self) -> CMP1IE_W {
        CMP1IE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cmp2ie(&mut self) -> CMP2IE_W {
        CMP2IE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cmp3ie(&mut self) -> CMP3IE_W {
        CMP3IE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi1eie(&mut self) -> SPI1EIE_W {
        SPI1EIE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn spi1rxie(&mut self) -> SPI1RXIE_W {
        SPI1RXIE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn spi1txie(&mut self) -> SPI1TXIE_W {
        SPI1TXIE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn u1eie(&mut self) -> U1EIE_W {
        U1EIE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u1rxie(&mut self) -> U1RXIE_W {
        U1RXIE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u1txie(&mut self) -> U1TXIE_W {
        U1TXIE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2c1bie(&mut self) -> I2C1BIE_W {
        I2C1BIE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2c1sie(&mut self) -> I2C1SIE_W {
        I2C1SIE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2c1mie(&mut self) -> I2C1MIE_W {
        I2C1MIE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cnaie(&mut self) -> CNAIE_W {
        CNAIE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cnbie(&mut self) -> CNBIE_W {
        CNBIE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cncie(&mut self) -> CNCIE_W {
        CNCIE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pmpie(&mut self) -> PMPIE_W {
        PMPIE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pmpeie(&mut self) -> PMPEIE_W {
        PMPEIE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn spi2eie(&mut self) -> SPI2EIE_W {
        SPI2EIE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn spi2rxie(&mut self) -> SPI2RXIE_W {
        SPI2RXIE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn spi2txie(&mut self) -> SPI2TXIE_W {
        SPI2TXIE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn u2eie(&mut self) -> U2EIE_W {
        U2EIE_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn u2rxie(&mut self) -> U2RXIE_W {
        U2RXIE_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn u2txie(&mut self) -> U2TXIE_W {
        U2TXIE_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c2bie(&mut self) -> I2C2BIE_W {
        I2C2BIE_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn i2c2sie(&mut self) -> I2C2SIE_W {
        I2C2SIE_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn i2c2mie(&mut self) -> I2C2MIE_W {
        I2C2MIE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ctmuie(&mut self) -> CTMUIE_W {
        CTMUIE_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dma0ie(&mut self) -> DMA0IE_W {
        DMA0IE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dma1ie(&mut self) -> DMA1IE_W {
        DMA1IE_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dma2ie(&mut self) -> DMA2IE_W {
        DMA2IE_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dma3ie(&mut self) -> DMA3IE_W {
        DMA3IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IEC1SET register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iec1set](index.html) module"]
pub struct IEC1SET_SPEC;
impl crate::RegisterSpec for IEC1SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iec1set::R](R) reader structure"]
impl crate::Readable for IEC1SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iec1set::W](W) writer structure"]
impl crate::Writable for IEC1SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEC1SET to value 0"]
impl crate::Resettable for IEC1SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
