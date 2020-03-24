#[doc = "Reader of register DCH3SPTRINV"]
pub type R = crate::R<u32, super::DCH3SPTRINV>;
#[doc = "Writer for register DCH3SPTRINV"]
pub type W = crate::W<u32, super::DCH3SPTRINV>;
#[doc = "Register DCH3SPTRINV `reset()`'s with value 0"]
impl crate::ResetValue for super::DCH3SPTRINV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHSPTR`"]
pub type CHSPTR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CHSPTR`"]
pub struct CHSPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chsptr(&self) -> CHSPTR_R {
        CHSPTR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chsptr(&mut self) -> CHSPTR_W {
        CHSPTR_W { w: self }
    }
}
