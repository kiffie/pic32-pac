#[doc = "Reader of register TMR5CLR"]
pub type R = crate::R<u32, super::TMR5CLR>;
#[doc = "Writer for register TMR5CLR"]
pub type W = crate::W<u32, super::TMR5CLR>;
#[doc = "Register TMR5CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::TMR5CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMR5`"]
pub type TMR5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TMR5`"]
pub struct TMR5_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR5_W<'a> {
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
    pub fn tmr5(&self) -> TMR5_R {
        TMR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmr5(&mut self) -> TMR5_W {
        TMR5_W { w: self }
    }
}
