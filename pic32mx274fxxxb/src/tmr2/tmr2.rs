#[doc = "Reader of register TMR2"]
pub type R = crate::R<u32, super::TMR2>;
#[doc = "Writer for register TMR2"]
pub type W = crate::W<u32, super::TMR2>;
#[doc = "Register TMR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMR2`"]
pub type TMR2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TMR2`"]
pub struct TMR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2_W<'a> {
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
    pub fn tmr2(&self) -> TMR2_R {
        TMR2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr2(&mut self) -> TMR2_W {
        TMR2_W { w: self }
    }
}
