#[doc = "Reader of register WDTCONSET"]
pub type R = crate::R<u32, super::WDTCONSET>;
#[doc = "Writer for register WDTCONSET"]
pub type W = crate::W<u32, super::WDTCONSET>;
#[doc = "Register WDTCONSET `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTCONSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDTCLR`"]
pub type WDTCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTCLR`"]
pub struct WDTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCLR_W<'a> {
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
#[doc = "Reader of field `WDTWINEN`"]
pub type WDTWINEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTWINEN`"]
pub struct WDTWINEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTWINEN_W<'a> {
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
#[doc = "Reader of field `SWDTPS`"]
pub type SWDTPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWDTPS`"]
pub struct SWDTPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDTPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 2)) | (((value as u32) & 0x1f) << 2);
        self.w
    }
}
#[doc = "Reader of field `ON`"]
pub type ON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ON`"]
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdtclr(&self) -> WDTCLR_R {
        WDTCLR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wdtwinen(&self) -> WDTWINEN_R {
        WDTWINEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn swdtps(&self) -> SWDTPS_R {
        SWDTPS_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&self) -> ON_R {
        ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdtclr(&mut self) -> WDTCLR_W {
        WDTCLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wdtwinen(&mut self) -> WDTWINEN_W {
        WDTWINEN_W { w: self }
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn swdtps(&mut self) -> SWDTPS_W {
        SWDTPS_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn on(&mut self) -> ON_W {
        ON_W { w: self }
    }
}
