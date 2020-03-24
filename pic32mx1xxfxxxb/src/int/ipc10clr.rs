#[doc = "Reader of register IPC10CLR"]
pub type R = crate::R<u32, super::IPC10CLR>;
#[doc = "Writer for register IPC10CLR"]
pub type W = crate::W<u32, super::IPC10CLR>;
#[doc = "Register IPC10CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::IPC10CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMA0IS`"]
pub type DMA0IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA0IS`"]
pub struct DMA0IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DMA0IP`"]
pub type DMA0IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA0IP`"]
pub struct DMA0IP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | (((value as u32) & 0x07) << 2);
        self.w
    }
}
#[doc = "Reader of field `DMA1IS`"]
pub type DMA1IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA1IS`"]
pub struct DMA1IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `DMA1IP`"]
pub type DMA1IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA1IP`"]
pub struct DMA1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Reader of field `DMA2IS`"]
pub type DMA2IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA2IS`"]
pub struct DMA2IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DMA2IP`"]
pub type DMA2IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA2IP`"]
pub struct DMA2IP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `DMA3IS`"]
pub type DMA3IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA3IS`"]
pub struct DMA3IS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA3IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `DMA3IP`"]
pub type DMA3IP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DMA3IP`"]
pub struct DMA3IP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA3IP_W<'a> {
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
    pub fn dma0is(&self) -> DMA0IS_R {
        DMA0IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn dma0ip(&self) -> DMA0IP_R {
        DMA0IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dma1is(&self) -> DMA1IS_R {
        DMA1IS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn dma1ip(&self) -> DMA1IP_R {
        DMA1IP_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dma2is(&self) -> DMA2IS_R {
        DMA2IS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn dma2ip(&self) -> DMA2IP_R {
        DMA2IP_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn dma3is(&self) -> DMA3IS_R {
        DMA3IS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn dma3ip(&self) -> DMA3IP_R {
        DMA3IP_R::new(((self.bits >> 26) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dma0is(&mut self) -> DMA0IS_W {
        DMA0IS_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn dma0ip(&mut self) -> DMA0IP_W {
        DMA0IP_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dma1is(&mut self) -> DMA1IS_W {
        DMA1IS_W { w: self }
    }
    #[doc = "Bits 10:12"]
    #[inline(always)]
    pub fn dma1ip(&mut self) -> DMA1IP_W {
        DMA1IP_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dma2is(&mut self) -> DMA2IS_W {
        DMA2IS_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn dma2ip(&mut self) -> DMA2IP_W {
        DMA2IP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn dma3is(&mut self) -> DMA3IS_W {
        DMA3IS_W { w: self }
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn dma3ip(&mut self) -> DMA3IP_W {
        DMA3IP_W { w: self }
    }
}
