#[doc = "Reader of register TMR3"]
pub type R = crate::R<u32, super::TMR3>;
#[doc = "Writer for register TMR3"]
pub type W = crate::W<u32, super::TMR3>;
#[doc = "Register TMR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMR3`"]
pub type TMR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TMR3`"]
pub struct TMR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR3_W<'a> {
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
    pub fn tmr3(&self) -> TMR3_R {
        TMR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr3(&mut self) -> TMR3_W {
        TMR3_W { w: self }
    }
}
