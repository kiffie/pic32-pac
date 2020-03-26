#[doc = "Reader of register IEC1SET"]
pub type R = crate::R<u32, super::IEC1SET>;
#[doc = "Writer for register IEC1SET"]
pub type W = crate::W<u32, super::IEC1SET>;
#[doc = "Register IEC1SET `reset()`'s with value 0"]
impl crate::ResetValue for super::IEC1SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP1IE`"]
pub type CMP1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1IE`"]
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
#[doc = "Reader of field `CMP2IE`"]
pub type CMP2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP2IE`"]
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
#[doc = "Reader of field `CMP3IE`"]
pub type CMP3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP3IE`"]
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
#[doc = "Reader of field `USBIE`"]
pub type USBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBIE`"]
pub struct USBIE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIE_W<'a> {
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
#[doc = "Reader of field `SPI1EIE`"]
pub type SPI1EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1EIE`"]
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
#[doc = "Reader of field `SPI1RXIE`"]
pub type SPI1RXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1RXIE`"]
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
#[doc = "Reader of field `SPI1TXIE`"]
pub type SPI1TXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1TXIE`"]
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
#[doc = "Reader of field `U1EIE`"]
pub type U1EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U1EIE`"]
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
#[doc = "Reader of field `U1RXIE`"]
pub type U1RXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U1RXIE`"]
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
#[doc = "Reader of field `U1TXIE`"]
pub type U1TXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U1TXIE`"]
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
#[doc = "Reader of field `I2C1BIE`"]
pub type I2C1BIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1BIE`"]
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
#[doc = "Reader of field `I2C1SIE`"]
pub type I2C1SIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1SIE`"]
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
#[doc = "Reader of field `I2C1MIE`"]
pub type I2C1MIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1MIE`"]
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
#[doc = "Reader of field `CNAIE`"]
pub type CNAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNAIE`"]
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
#[doc = "Reader of field `CNBIE`"]
pub type CNBIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNBIE`"]
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
#[doc = "Reader of field `CNCIE`"]
pub type CNCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNCIE`"]
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
#[doc = "Reader of field `PMPIE`"]
pub type PMPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMPIE`"]
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
#[doc = "Reader of field `PMPEIE`"]
pub type PMPEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMPEIE`"]
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
#[doc = "Reader of field `SPI2EIE`"]
pub type SPI2EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2EIE`"]
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
#[doc = "Reader of field `SPI2RXIE`"]
pub type SPI2RXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2RXIE`"]
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
#[doc = "Reader of field `SPI2TXIE`"]
pub type SPI2TXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2TXIE`"]
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
#[doc = "Reader of field `U2EIE`"]
pub type U2EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U2EIE`"]
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
#[doc = "Reader of field `U2RXIE`"]
pub type U2RXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U2RXIE`"]
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
#[doc = "Reader of field `U2TXIE`"]
pub type U2TXIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U2TXIE`"]
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
#[doc = "Reader of field `I2C2BIE`"]
pub type I2C2BIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2BIE`"]
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
#[doc = "Reader of field `I2C2SIE`"]
pub type I2C2SIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2SIE`"]
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
#[doc = "Reader of field `I2C2MIE`"]
pub type I2C2MIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2MIE`"]
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
#[doc = "Reader of field `CTMUIE`"]
pub type CTMUIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMUIE`"]
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
#[doc = "Reader of field `DMA0IE`"]
pub type DMA0IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA0IE`"]
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
#[doc = "Reader of field `DMA1IE`"]
pub type DMA1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1IE`"]
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
#[doc = "Reader of field `DMA2IE`"]
pub type DMA2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2IE`"]
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
#[doc = "Reader of field `DMA3IE`"]
pub type DMA3IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA3IE`"]
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbie(&self) -> USBIE_R {
        USBIE_R::new(((self.bits >> 3) & 0x01) != 0)
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbie(&mut self) -> USBIE_W {
        USBIE_W { w: self }
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
}
