#[doc = "Reader of register SPLLCON"]
pub type R = crate::R<u32, super::SPLLCON>;
#[doc = "Writer for register SPLLCON"]
pub type W = crate::W<u32, super::SPLLCON>;
#[doc = "Register SPLLCON `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::SPLLCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `PLLICLK`"]
pub type PLLICLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLLICLK`"]
pub struct PLLICLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLICLK_W<'a> {
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
#[doc = "Reader of field `PLLIDIV`"]
pub type PLLIDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLIDIV`"]
pub struct PLLIDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLIDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `PLLMULT`"]
pub type PLLMULT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLMULT`"]
pub struct PLLMULT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `PLLODIV`"]
pub type PLLODIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLLODIV`"]
pub struct PLLODIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLODIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn plliclk(&self) -> PLLICLK_R {
        PLLICLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pllidiv(&self) -> PLLIDIV_R {
        PLLIDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn pllmult(&self) -> PLLMULT_R {
        PLLMULT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pllodiv(&self) -> PLLODIV_R {
        PLLODIV_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn plliclk(&mut self) -> PLLICLK_W {
        PLLICLK_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pllidiv(&mut self) -> PLLIDIV_W {
        PLLIDIV_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn pllmult(&mut self) -> PLLMULT_W {
        PLLMULT_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pllodiv(&mut self) -> PLLODIV_W {
        PLLODIV_W { w: self }
    }
}
