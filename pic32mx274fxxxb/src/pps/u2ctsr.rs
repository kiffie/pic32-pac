#[doc = "Reader of register U2CTSR"]
pub type R = crate::R<u32, super::U2CTSR>;
#[doc = "Writer for register U2CTSR"]
pub type W = crate::W<u32, super::U2CTSR>;
#[doc = "Register U2CTSR `reset()`'s with value 0"]
impl crate::ResetValue for super::U2CTSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `U2CTSR`"]
pub type U2CTSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U2CTSR`"]
pub struct U2CTSR_W<'a> {
    w: &'a mut W,
}
impl<'a> U2CTSR_W<'a> {
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
    pub fn u2ctsr(&self) -> U2CTSR_R {
        U2CTSR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn u2ctsr(&mut self) -> U2CTSR_W {
        U2CTSR_W { w: self }
    }
}
