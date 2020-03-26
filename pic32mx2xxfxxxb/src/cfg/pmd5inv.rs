#[doc = "Reader of register PMD5INV"]
pub type R = crate::R<u32, super::PMD5INV>;
#[doc = "Writer for register PMD5INV"]
pub type W = crate::W<u32, super::PMD5INV>;
#[doc = "Register PMD5INV `reset()`'s with value 0"]
impl crate::ResetValue for super::PMD5INV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `U1MD`"]
pub type U1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U1MD`"]
pub struct U1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> U1MD_W<'a> {
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
#[doc = "Reader of field `U2MD`"]
pub type U2MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `U2MD`"]
pub struct U2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> U2MD_W<'a> {
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
#[doc = "Reader of field `SPI1MD`"]
pub type SPI1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI1MD`"]
pub struct SPI1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1MD_W<'a> {
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
#[doc = "Reader of field `SPI2MD`"]
pub type SPI2MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2MD`"]
pub struct SPI2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2MD_W<'a> {
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
#[doc = "Reader of field `I2C1MD`"]
pub type I2C1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1MD`"]
pub struct I2C1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1MD_W<'a> {
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
#[doc = "Reader of field `I2C2MD`"]
pub type I2C2MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2MD`"]
pub struct I2C2MD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2MD_W<'a> {
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
#[doc = "Reader of field `USB1MD`"]
pub type USB1MD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USB1MD`"]
pub struct USB1MD_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1MD_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u1md(&self) -> U1MD_R {
        U1MD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn u2md(&self) -> U2MD_R {
        U2MD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi1md(&self) -> SPI1MD_R {
        SPI1MD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi2md(&self) -> SPI2MD_R {
        SPI2MD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2c1md(&self) -> I2C1MD_R {
        I2C1MD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2c2md(&self) -> I2C2MD_R {
        I2C2MD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usb1md(&self) -> USB1MD_R {
        USB1MD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u1md(&mut self) -> U1MD_W {
        U1MD_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn u2md(&mut self) -> U2MD_W {
        U2MD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi1md(&mut self) -> SPI1MD_W {
        SPI1MD_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn spi2md(&mut self) -> SPI2MD_W {
        SPI2MD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn i2c1md(&mut self) -> I2C1MD_W {
        I2C1MD_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn i2c2md(&mut self) -> I2C2MD_W {
        I2C2MD_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usb1md(&mut self) -> USB1MD_W {
        USB1MD_W { w: self }
    }
}
