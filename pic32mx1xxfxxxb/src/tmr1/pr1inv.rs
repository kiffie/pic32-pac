#[doc = "Reader of register PR1INV"]
pub type R = crate::R<u32, super::PR1INV>;
#[doc = "Writer for register PR1INV"]
pub type W = crate::W<u32, super::PR1INV>;
#[doc = "Register PR1INV `reset()`'s with value 0"]
impl crate::ResetValue for super::PR1INV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PR1`"]
pub type PR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PR1`"]
pub struct PR1_W<'a> {
    w: &'a mut W,
}
impl<'a> PR1_W<'a> {
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
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pr1(&mut self) -> PR1_W {
        PR1_W { w: self }
    }
}