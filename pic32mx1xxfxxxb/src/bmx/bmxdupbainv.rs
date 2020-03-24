#[doc = "Reader of register BMXDUPBAINV"]
pub type R = crate::R<u32, super::BMXDUPBAINV>;
#[doc = "Writer for register BMXDUPBAINV"]
pub type W = crate::W<u32, super::BMXDUPBAINV>;
#[doc = "Register BMXDUPBAINV `reset()`'s with value 0"]
impl crate::ResetValue for super::BMXDUPBAINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BMXDUPBA`"]
pub type BMXDUPBA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BMXDUPBA`"]
pub struct BMXDUPBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXDUPBA_W<'a> {
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
    pub fn bmxdupba(&self) -> BMXDUPBA_R {
        BMXDUPBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxdupba(&mut self) -> BMXDUPBA_W {
        BMXDUPBA_W { w: self }
    }
}
