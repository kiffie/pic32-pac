#[doc = "Reader of register PR3CLR"]
pub type R = crate::R<u32, super::PR3CLR>;
#[doc = "Writer for register PR3CLR"]
pub type W = crate::W<u32, super::PR3CLR>;
#[doc = "Register PR3CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::PR3CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PR3`"]
pub type PR3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PR3`"]
pub struct PR3_W<'a> {
    w: &'a mut W,
}
impl<'a> PR3_W<'a> {
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
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pr3(&mut self) -> PR3_W {
        PR3_W { w: self }
    }
}
