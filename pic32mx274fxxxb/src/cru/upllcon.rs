#[doc = "Reader of register UPLLCON"]
pub type R = crate::R<u32, super::UPLLCON>;
#[doc = "Writer for register UPLLCON"]
pub type W = crate::W<u32, super::UPLLCON>;
#[doc = "Register UPLLCON `reset()`'s with value 0x0100_0100"]
impl crate::ResetValue for super::UPLLCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0100
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
