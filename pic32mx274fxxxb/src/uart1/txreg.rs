#[doc = "Reader of register TXREG"]
pub type R = crate::R<u32, super::TXREG>;
#[doc = "Writer for register TXREG"]
pub type W = crate::W<u32, super::TXREG>;
#[doc = "Register TXREG `reset()`'s with value 0"]
impl crate::ResetValue for super::TXREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXREG`"]
pub type TXREG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXREG`"]
pub struct TXREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TXREG_W<'a> {
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
    pub fn txreg(&self) -> TXREG_R {
        TXREG_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn txreg(&mut self) -> TXREG_W {
        TXREG_W { w: self }
    }
}
