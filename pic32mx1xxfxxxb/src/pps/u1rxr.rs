#[doc = "Reader of register U1RXR"]
pub type R = crate::R<u32, super::U1RXR>;
#[doc = "Writer for register U1RXR"]
pub type W = crate::W<u32, super::U1RXR>;
#[doc = "Register U1RXR `reset()`'s with value 0"]
impl crate::ResetValue for super::U1RXR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `U1RXR`"]
pub type U1RXR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1RXR`"]
pub struct U1RXR_W<'a> {
    w: &'a mut W,
}
impl<'a> U1RXR_W<'a> {
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
    pub fn u1rxr(&self) -> U1RXR_R {
        U1RXR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn u1rxr(&mut self) -> U1RXR_W {
        U1RXR_W { w: self }
    }
}
