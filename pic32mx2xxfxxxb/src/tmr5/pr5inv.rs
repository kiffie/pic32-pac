#[doc = "Reader of register PR5INV"]
pub type R = crate::R<u32, super::PR5INV>;
#[doc = "Writer for register PR5INV"]
pub type W = crate::W<u32, super::PR5INV>;
#[doc = "Register PR5INV `reset()`'s with value 0"]
impl crate::ResetValue for super::PR5INV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PR5`"]
pub type PR5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PR5`"]
pub struct PR5_W<'a> {
    w: &'a mut W,
}
impl<'a> PR5_W<'a> {
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
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pr5(&mut self) -> PR5_W {
        PR5_W { w: self }
    }
}
