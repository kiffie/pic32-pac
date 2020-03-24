#[doc = "Reader of register REFCLKIR"]
pub type R = crate::R<u32, super::REFCLKIR>;
#[doc = "Writer for register REFCLKIR"]
pub type W = crate::W<u32, super::REFCLKIR>;
#[doc = "Register REFCLKIR `reset()`'s with value 0"]
impl crate::ResetValue for super::REFCLKIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REFCLKIR`"]
pub type REFCLKIR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFCLKIR`"]
pub struct REFCLKIR_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCLKIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn refclkir(&self) -> REFCLKIR_R {
        REFCLKIR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn refclkir(&mut self) -> REFCLKIR_W {
        REFCLKIR_W { w: self }
    }
}
