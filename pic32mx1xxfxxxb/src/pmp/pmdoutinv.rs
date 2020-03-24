#[doc = "Reader of register PMDOUTINV"]
pub type R = crate::R<u32, super::PMDOUTINV>;
#[doc = "Writer for register PMDOUTINV"]
pub type W = crate::W<u32, super::PMDOUTINV>;
#[doc = "Register PMDOUTINV `reset()`'s with value 0"]
impl crate::ResetValue for super::PMDOUTINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATAOUT`"]
pub type DATAOUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATAOUT`"]
pub struct DATAOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAOUT_W<'a> {
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
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dataout(&mut self) -> DATAOUT_W {
        DATAOUT_W { w: self }
    }
}
