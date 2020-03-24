#[doc = "Reader of register IPC7INV"]
pub type R = crate::R<u32, super::IPC7INV>;
#[doc = "Writer for register IPC7INV"]
pub type W = crate::W<u32, super::IPC7INV>;
#[doc = "Register IPC7INV `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC7INV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP2IS`"]
pub type CMP2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP2IS`"]
pub struct CMP2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CMP2IP`"]
pub type CMP2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP2IP`"]
pub struct CMP2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `CMP3IS`"]
pub type CMP3IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP3IS`"]
pub struct CMP3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMP3IP`"]
pub type CMP3IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMP3IP`"]
pub struct CMP3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP3IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `USBIS`"]
pub type USBIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBIS`"]
pub struct USBIS_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `USBIP`"]
pub type USBIP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBIP`"]
pub struct USBIP_W<'a> {
    w: &'a mut W,
}
impl<'a> USBIP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPI1IS`"]
pub type SPI1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1IS`"]
pub struct SPI1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI1IP`"]
pub type SPI1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1IP`"]
pub struct SPI1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1IP_W<'a> {
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
    pub fn cmp2is(&self) -> CMP2IS_R {
        CMP2IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn cmp2ip(&self) -> CMP2IP_R {
        CMP2IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cmp3is(&self) -> CMP3IS_R {
        CMP3IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn cmp3ip(&self) -> CMP3IP_R {
        CMP3IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn usbis(&self) -> USBIS_R {
        USBIS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn usbip(&self) -> USBIP_R {
        USBIP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn spi1is(&self) -> SPI1IS_R {
        SPI1IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn spi1ip(&self) -> SPI1IP_R {
        SPI1IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cmp2is(&mut self) -> CMP2IS_W {
        CMP2IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn cmp2ip(&mut self) -> CMP2IP_W {
        CMP2IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cmp3is(&mut self) -> CMP3IS_W {
        CMP3IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn cmp3ip(&mut self) -> CMP3IP_W {
        CMP3IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn usbis(&mut self) -> USBIS_W {
        USBIS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn usbip(&mut self) -> USBIP_W {
        USBIP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn spi1is(&mut self) -> SPI1IS_W {
        SPI1IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn spi1ip(&mut self) -> SPI1IP_W {
        SPI1IP_W { w: self }
    }
}
