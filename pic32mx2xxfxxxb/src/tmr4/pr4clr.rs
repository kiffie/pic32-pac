#[doc = "Reader of register PR4CLR"]
pub type R = crate::R<u32, super::PR4CLR>;
#[doc = "Writer for register PR4CLR"]
pub type W = crate::W<u32, super::PR4CLR>;
#[doc = "Register PR4CLR `reset()`'s with value 0"]
impl crate::ResetValue for super::PR4CLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PR4`"]
pub type PR4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PR4`"]
pub struct PR4_W<'a> {
    w: &'a mut W,
}
impl<'a> PR4_W<'a> {
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
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn pr4(&mut self) -> PR4_W {
        PR4_W { w: self }
    }
}
