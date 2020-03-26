#[doc = "Reader of register TMR4INV"]
pub type R = crate::R<u32, super::TMR4INV>;
#[doc = "Writer for register TMR4INV"]
pub type W = crate::W<u32, super::TMR4INV>;
#[doc = "Register TMR4INV `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR4INV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMR4`"]
pub type TMR4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TMR4`"]
pub struct TMR4_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr4(&self) -> TMR4_R {
        TMR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr4(&mut self) -> TMR4_W {
        TMR4_W { w: self }
    }
}
