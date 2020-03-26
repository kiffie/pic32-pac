#[doc = "Reader of register IPC9"]
pub type R = crate::R<u32, super::IPC9>;
#[doc = "Writer for register IPC9"]
pub type W = crate::W<u32, super::IPC9>;
#[doc = "Register IPC9 `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC9 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI2IS`"]
pub type SPI2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI2IS`"]
pub struct SPI2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SPI2IP`"]
pub type SPI2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI2IP`"]
pub struct SPI2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `U2IS`"]
pub type U2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U2IS`"]
pub struct U2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> U2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `U2IP`"]
pub type U2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U2IP`"]
pub struct U2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> U2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `I2C2IS`"]
pub type I2C2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C2IS`"]
pub struct I2C2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2C2IP`"]
pub type I2C2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C2IP`"]
pub struct I2C2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `CTMUIS`"]
pub type CTMUIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTMUIS`"]
pub struct CTMUIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `CTMUIP`"]
pub type CTMUIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTMUIP`"]
pub struct CTMUIP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMUIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 26)) | (((value as u32) & 0x07) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi2is(&self) -> SPI2IS_R {
        SPI2IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn spi2ip(&self) -> SPI2IP_R {
        SPI2IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn u2is(&self) -> U2IS_R {
        U2IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn u2ip(&self) -> U2IP_R {
        U2IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2c2is(&self) -> I2C2IS_R {
        I2C2IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn i2c2ip(&self) -> I2C2IP_R {
        I2C2IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ctmuis(&self) -> CTMUIS_R {
        CTMUIS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn ctmuip(&self) -> CTMUIP_R {
        CTMUIP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn spi2is(&mut self) -> SPI2IS_W {
        SPI2IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn spi2ip(&mut self) -> SPI2IP_W {
        SPI2IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn u2is(&mut self) -> U2IS_W {
        U2IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn u2ip(&mut self) -> U2IP_W {
        U2IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn i2c2is(&mut self) -> I2C2IS_W {
        I2C2IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn i2c2ip(&mut self) -> I2C2IP_W {
        I2C2IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ctmuis(&mut self) -> CTMUIS_W {
        CTMUIS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn ctmuip(&mut self) -> CTMUIP_W {
        CTMUIP_W { w: self }
    }
}
