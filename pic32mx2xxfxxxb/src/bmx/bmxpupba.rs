#[doc = "Reader of register BMXPUPBA"]
pub type R = crate::R<u32, super::BMXPUPBA>;
#[doc = "Writer for register BMXPUPBA"]
pub type W = crate::W<u32, super::BMXPUPBA>;
#[doc = "Register BMXPUPBA `reset()`'s with value 0"]
impl crate::ResetValue for super::BMXPUPBA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BMXPUPBA`"]
pub type BMXPUPBA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BMXPUPBA`"]
pub struct BMXPUPBA_W<'a> {
    w: &'a mut W,
}
impl<'a> BMXPUPBA_W<'a> {
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
    pub fn bmxpupba(&self) -> BMXPUPBA_R {
        BMXPUPBA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmxpupba(&mut self) -> BMXPUPBA_W {
        BMXPUPBA_W { w: self }
    }
}
