#[doc = "Reader of register DCRCXORINV"]
pub type R = crate::R<u32, super::DCRCXORINV>;
#[doc = "Writer for register DCRCXORINV"]
pub type W = crate::W<u32, super::DCRCXORINV>;
#[doc = "Register DCRCXORINV `reset()`'s with value 0"]
impl crate::ResetValue for super::DCRCXORINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DCRCXOR`"]
pub type DCRCXOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DCRCXOR`"]
pub struct DCRCXOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCXOR_W<'a> {
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
    pub fn dcrcxor(&self) -> DCRCXOR_R {
        DCRCXOR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dcrcxor(&mut self) -> DCRCXOR_W {
        DCRCXOR_W { w: self }
    }
}
