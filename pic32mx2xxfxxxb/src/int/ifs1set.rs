#[doc = "Reader of register IFS1SET"]
pub type R = crate::R<u32, super::IFS1SET>;
#[doc = "Writer for register IFS1SET"]
pub type W = crate::W<u32, super::IFS1SET>;
#[doc = "Register IFS1SET `reset()`'s with value 0"]
impl crate::ResetValue for super::IFS1SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP1IF`"]
pub type CMP1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP1IF`"]
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
#[doc = "Reader of field `CMP2IF`"]
pub type CMP2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP2IF`"]
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
#[doc = "Reader of field `CMP3IF`"]
pub type CMP3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP3IF`"]
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
#[doc = "Reader of field `USBIF`"]
pub type USBIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBIF`"]
pub struct USBIF_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIF_W<'a> {
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
#[doc = "Reader of field `SPI1EIF`"]
pub type SPI1EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1EIF`"]
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
#[doc = "Reader of field `SPI1RXIF`"]
pub type SPI1RXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1RXIF`"]
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
#[doc = "Reader of field `SPI1TXIF`"]
pub type SPI1TXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1TXIF`"]
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
#[doc = "Reader of field `U1EIF`"]
pub type U1EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U1EIF`"]
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
#[doc = "Reader of field `U1RXIF`"]
pub type U1RXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U1RXIF`"]
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
#[doc = "Reader of field `U1TXIF`"]
pub type U1TXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U1TXIF`"]
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
#[doc = "Reader of field `I2C1BIF`"]
pub type I2C1BIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1BIF`"]
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
#[doc = "Reader of field `I2C1SIF`"]
pub type I2C1SIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1SIF`"]
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
#[doc = "Reader of field `I2C1MIF`"]
pub type I2C1MIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1MIF`"]
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
#[doc = "Reader of field `CNAIF`"]
pub type CNAIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNAIF`"]
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
#[doc = "Reader of field `CNBIF`"]
pub type CNBIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNBIF`"]
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
#[doc = "Reader of field `CNCIF`"]
pub type CNCIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNCIF`"]
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
#[doc = "Reader of field `PMPIF`"]
pub type PMPIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMPIF`"]
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
#[doc = "Reader of field `PMPEIF`"]
pub type PMPEIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMPEIF`"]
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
#[doc = "Reader of field `SPI2EIF`"]
pub type SPI2EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2EIF`"]
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
#[doc = "Reader of field `SPI2RXIF`"]
pub type SPI2RXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2RXIF`"]
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
#[doc = "Reader of field `SPI2TXIF`"]
pub type SPI2TXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2TXIF`"]
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
#[doc = "Reader of field `U2EIF`"]
pub type U2EIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U2EIF`"]
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
#[doc = "Reader of field `U2RXIF`"]
pub type U2RXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U2RXIF`"]
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
#[doc = "Reader of field `U2TXIF`"]
pub type U2TXIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U2TXIF`"]
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
#[doc = "Reader of field `I2C2BIF`"]
pub type I2C2BIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2BIF`"]
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
#[doc = "Reader of field `I2C2SIF`"]
pub type I2C2SIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2SIF`"]
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
#[doc = "Reader of field `I2C2MIF`"]
pub type I2C2MIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2MIF`"]
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
#[doc = "Reader of field `CTMUIF`"]
pub type CTMUIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTMUIF`"]
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
#[doc = "Reader of field `DMA0IF`"]
pub type DMA0IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA0IF`"]
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
#[doc = "Reader of field `DMA1IF`"]
pub type DMA1IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1IF`"]
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
#[doc = "Reader of field `DMA2IF`"]
pub type DMA2IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2IF`"]
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
#[doc = "Reader of field `DMA3IF`"]
pub type DMA3IF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA3IF`"]
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbif(&self) -> USBIF_R {
        USBIF_R::new(((self.bits >> 3) & 0x01) != 0)
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbif(&mut self) -> USBIF_W {
        USBIF_W { w: self }
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
}
