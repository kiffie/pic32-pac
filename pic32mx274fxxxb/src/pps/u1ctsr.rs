#[doc = "Reader of register U1CTSR"]
pub type R = crate::R<u32, super::U1CTSR>;
#[doc = "Writer for register U1CTSR"]
pub type W = crate::W<u32, super::U1CTSR>;
#[doc = "Register U1CTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::U1CTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `U1CTSR`"]
pub type U1CTSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1CTSR`"]
pub struct U1CTSR_W<'a> {
    w: &'a mut W,
}
impl<'a> U1CTSR_W<'a> {
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
    pub fn u1ctsr(&self) -> U1CTSR_R {
        U1CTSR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn u1ctsr(&mut self) -> U1CTSR_W {
        U1CTSR_W { w: self }
    }
}
