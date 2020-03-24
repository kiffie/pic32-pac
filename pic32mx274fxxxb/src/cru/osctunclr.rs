#[doc = "Reader of register OSCTUNCLR"]
pub type R = crate::R<u32, super::OSCTUNCLR>;
#[doc = "Writer for register OSCTUNCLR"]
pub type W = crate::W<u32, super::OSCTUNCLR>;
#[doc = "Register OSCTUNCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCTUNCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TUN`"]
pub type TUN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUN`"]
pub struct TUN_W<'a> {
    w: &'a mut W,
}
impl<'a> TUN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tun(&self) -> TUN_R {
        TUN_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tun(&mut self) -> TUN_W {
        TUN_W { w: self }
    }
}
