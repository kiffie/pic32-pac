#[doc = "Reader of register CHEW1"]
pub type R = crate::R<u32, super::CHEW1>;
#[doc = "Writer for register CHEW1"]
pub type W = crate::W<u32, super::CHEW1>;
#[doc = "Register CHEW1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHEW1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHEW1`"]
pub type CHEW1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHEW1`"]
pub struct CHEW1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEW1_W<'a> {
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
    pub fn chew1(&self) -> CHEW1_R {
        CHEW1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn chew1(&mut self) -> CHEW1_W {
        CHEW1_W { w: self }
    }
}
