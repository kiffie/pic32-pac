#[doc = "Reader of register RXREG"]
pub type R = crate::R<u32, super::RXREG>;
#[doc = "Writer for register RXREG"]
pub type W = crate::W<u32, super::RXREG>;
#[doc = "Register RXREG `reset()`'s with value 0"]
impl crate::ResetValue for super::RXREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXREG`"]
pub type RXREG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RXREG`"]
pub struct RXREG_W<'a> {
    w: &'a mut W,
}
impl<'a> RXREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxreg(&self) -> RXREG_R {
        RXREG_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxreg(&mut self) -> RXREG_W {
        RXREG_W { w: self }
    }
}
